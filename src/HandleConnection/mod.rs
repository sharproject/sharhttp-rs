#![allow(dead_code)]
use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::TcpStream,
};

use crate::{
    routeManager::RouterManager,
    util::append_vec::append_vec,
    App::{get_key, HandlerType},
    Request::get_http_data::{GetRequest, HeaderData},
    Response::ResponseTool,
};

use crate::App::HandleCallback;

pub struct RequestProcessing {
    ProcessingHandler: Vec<HandleCallback>,
    not_found_handler: HandleCallback,
    httpData: HeaderData,
}

impl RequestProcessing {
    pub fn new(not_found_handler: HandleCallback) -> RequestProcessing {
        RequestProcessing {
            ProcessingHandler: Vec::new(),
            not_found_handler,
            httpData: HeaderData::Default(),
        }
    }
    pub fn preProcessing(&mut self, handlers: &HandlerType, stream: &mut TcpStream) {
        self.get_request_data(stream);
        self.get_handler(handlers);
    }
    pub fn processing(&mut self, stream: &mut TcpStream) -> RouterManager {
        self.handle_connection(stream)
    }
}

pub trait HandleConnection {
    fn handle_connection(&mut self, stream: &mut TcpStream) -> RouterManager;
    fn get_request_data(&mut self, stream: &TcpStream);
    fn get_handler(&mut self, handlers: &HandlerType);
}
impl HandleConnection for RequestProcessing {
    fn handle_connection(&mut self, stream: &mut TcpStream) -> RouterManager {
        let mut response = ResponseTool {
            stream,
            response: false,
            status: 200,
            content: "".to_string(),
            header: &mut HashMap::new(),
            Request: self.httpData.clone(),
            cookie: &mut Vec::new(),
        };
        let mut routerM = RouterManager::new();

        routerM.setLocalPath(self.httpData.path.clone());
        response.Setup();

        if self.ProcessingHandler.len() <= 0 {
            (self.not_found_handler)(&self.httpData, &mut response, &mut routerM);
            return routerM;
        };

        for h in &self.ProcessingHandler {
            if response.response {
                return routerM;
            }
            (&h)(&self.httpData, &mut response, &mut routerM);
        }
        if response.response {
            return routerM;
        }
        (self.not_found_handler)(&self.httpData, &mut response, &mut routerM);
        return routerM;
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
}
