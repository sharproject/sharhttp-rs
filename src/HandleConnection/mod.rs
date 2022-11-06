#![allow(dead_code)]
use std::{
    collections::BTreeMap,
    io::{BufRead, BufReader},
    net::TcpStream,
};

use crate::{
    util::append_vec::append_vec,
    App::{get_key, HandlerType, RequestCaching},
    Request::get_http_data::{GetRequest, HeaderData},
    Response::ResponseTool,
    RouteManager::RouterManager,
};

use crate::App::HandleCallback;

pub struct ProcessReturnValue(RouterManager, Option<String>, String);

impl ProcessReturnValue {
    pub fn ProcessingRouter(&mut self, handler: &mut HandlerType) {
        self.0.ProcessingRouter(handler)
    }
    pub fn setData(&mut self, handler: &mut HandlerType, cachingData: &mut RequestCaching) {
        self.ProcessingRouter(handler);
        self.ProcessingCache(cachingData);
    }
    pub fn ProcessingCache(&mut self, cachingData: &mut RequestCaching) {
        if self.1 != None {
            cachingData.insert(self.2.clone(), self.1.clone().unwrap());
        }
    }
}

pub struct RequestProcessing {
    ProcessingHandler: Vec<HandleCallback>,
    not_found_handler: HandleCallback,
    httpData: HeaderData,
    startTime: std::time::Instant,
    finalFunction: HandleCallback,
    cacheData: Option<String>,
}

impl RequestProcessing {
    pub fn new(
        not_found_handler: HandleCallback,
        StartTime: std::time::Instant,
        finalFunction: HandleCallback,
    ) -> RequestProcessing {
        RequestProcessing {
            ProcessingHandler: Vec::new(),
            not_found_handler,
            httpData: HeaderData::Default(),
            startTime: StartTime,
            finalFunction,
            cacheData: None,
        }
    }
    pub fn preProcessing(
        &mut self,
        handlers: &HandlerType,
        stream: &mut TcpStream,
        cache: &RequestCaching,
    ) {
        self.get_request_data(stream);
        self.cacheData = match cache.get(&self.httpData.path.clone()) {
            Some(a) => Some(a.clone()),
            None => None,
        };
        self.get_handler(handlers);
    }
    pub fn processing(&mut self, stream: &mut TcpStream) -> ProcessReturnValue {
        self.handle_connection(stream)
    }
}

pub trait HandleConnection {
    fn handle_connection(&mut self, stream: &mut TcpStream) -> ProcessReturnValue;
    fn get_request_data(&mut self, stream: &TcpStream);
    fn get_handler(&mut self, handlers: &HandlerType);
    fn get_return_value(
        r: RouterManager,
        Response: ResponseTool,
        path: String,
    ) -> ProcessReturnValue;
}
impl HandleConnection for RequestProcessing {
    fn handle_connection(&mut self, stream: &mut TcpStream) -> ProcessReturnValue {
        let mut response = ResponseTool {
            stream,
            response: false,
            status: 200,
            content: "".to_string(),
            header: &mut BTreeMap::new(),
            Request: &self.httpData.clone(),
            cookie: &mut Vec::new(),
            StartTime: self.startTime,
            finalFunction: self.finalFunction,
            caching: false,
        };
        let mut routerM = RouterManager::new();

        response.Setup();
        if self.cacheData != None {
            response.send(self.cacheData.as_ref().unwrap().to_string(), true);
            (response.finalFunction)(&mut self.httpData, &mut response, &mut routerM);
            return Self::get_return_value(routerM, response, self.httpData.path.clone());
        }

        if self.ProcessingHandler.len() <= 0 {
            (self.not_found_handler)(&mut self.httpData, &mut response, &mut routerM);
            (response.finalFunction)(&mut self.httpData, &mut response, &mut routerM);
            return Self::get_return_value(routerM, response, self.httpData.path.clone());
        };

        for h in &self.ProcessingHandler {
            if response.response {
                (response.finalFunction)(&mut self.httpData, &mut response, &mut routerM);
                return Self::get_return_value(routerM, response, self.httpData.path.clone());
            }
            (&h)(&mut self.httpData, &mut response, &mut routerM);
        }
        if response.response {
            (response.finalFunction)(&mut self.httpData, &mut response, &mut routerM);
            return Self::get_return_value(routerM, response, self.httpData.path.clone());
        }
        (self.not_found_handler)(&mut self.httpData, &mut response, &mut routerM);
        (response.finalFunction)(&mut self.httpData, &mut response, &mut routerM);
        return Self::get_return_value(routerM, response, self.httpData.path.clone());
    }

    fn get_request_data(&mut self, stream: &TcpStream) {
        let http_request: Vec<_> = BufReader::new(stream.try_clone().unwrap())
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        self.httpData = GetRequest(&http_request);
    }

    fn get_handler(&mut self, handlers: &HandlerType) {
        if self.cacheData != None {
            return;
        }
        let path_use = String::from(format!(
            "{}",
            format_args!(
                "{}{}",
                self.httpData.path.clone(),
                match self.httpData.path.ends_with("/") {
                    true => "*",
                    false => "/*",
                }
            )
        ));
        let mut path_list: Vec<String> =
            vec!["*".to_string(), path_use, self.httpData.path.clone()];
        if !self.httpData.path.clone().ends_with("/") {
            path_list.push(format!("{}/", self.httpData.path.clone()));
        }

        let method_list = vec!["*", &self.httpData.method];
        for path in path_list {
            for method in &method_list {
                let key = get_key(method.to_string(), path.to_string());
                if handlers.contains_key(&key) {
                    let rest_handlers = handlers.get(&key).unwrap();
                    append_vec(&mut self.ProcessingHandler, rest_handlers);
                }
            }
        }
    }

    fn get_return_value(
        r: RouterManager,
        Response: ResponseTool,
        path: String,
    ) -> ProcessReturnValue {
        if Response.caching {
            ProcessReturnValue(r, Some(Response.content), path)
        } else {
            ProcessReturnValue(r, None, path)
        }
    }
}
