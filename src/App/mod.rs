#![allow(dead_code)]
use std::{collections::HashMap, net::TcpListener};

use crate::{
    RouteManager::RouterManager, util::append_vec::append_vec, HandleConnection::RequestProcessing,
    Request::get_http_data::HeaderData, Response::ResponseTool,
};

pub type HandleCallback = fn(&HeaderData, &mut ResponseTool, &mut RouterManager) -> (); // request: HeaderData, response: TcpStream
pub type HandlerType = HashMap<HandlerHashmapKeyString, Vec<HandleCallback>>;
#[derive(Clone)]
#[doc = include_str!("../../doc/HttpHandler.md")]
pub struct HttpHandler {
    handler: HandlerType,
    not_found_handler: HandleCallback,
    threading: bool,
}

pub fn default_not_found_handler(
    _req: &HeaderData,
    response: &mut ResponseTool,
    _: &mut RouterManager,
) {
    let not_found_contents = "<h1>404 Page Not Found</h1>";
    response.status(404);
    response.send(not_found_contents.to_string(), true);
}

impl HttpHandler {
    pub fn new() -> HttpHandler {
        return HttpHandler {
            handler: (HashMap::new()),
            not_found_handler: default_not_found_handler,
            threading: false,
        };
    }

    pub fn turn_threading(&mut self) -> &mut Self {
        self.threading = !self.threading;
        self
    }

    #[doc = "to register the post handler ex : handle.post('/'.to_string() , <your handler>)"]
    pub fn post(&mut self, path: String, handler: HandleCallback) {
        let key = get_key("POST".to_string(), path);
        self.add_handle(key, handler);
    }

    #[doc = "to register the get handler ex : handle.get('/'.to_string() , <your handler>)"]
    pub fn get(&mut self, path: String, handler: HandleCallback) {
        let key = get_key("GET".to_string(), path);
        self.add_handle(key, handler);
    }

    #[doc = "to register the put handler ex : handle.put('/'.to_string() , <your handler>)"]
    pub fn put(&mut self, path: String, handler: HandleCallback) {
        let key = get_key("PUT".to_owned(), path);
        self.add_handle(key, handler);
    }

    #[doc = "to register the patch handler ex : handle.patch('/'.to_string() , <your handler>)"]
    pub fn patch(&mut self, path: String, handler: HandleCallback) {
        let key = get_key("PATCH".to_string(), path);
        self.add_handle(key, handler);
    }

    #[doc = "to register the delete handler ex : handle.delete('/'.to_string() , <your handler>)"]
    pub fn delete(&mut self, path: String, handler: HandleCallback) {
        let key = get_key("DELETE".to_string(), path);
        self.add_handle(key, handler);
    }

    #[doc = "to register the handler for all method like:GET,POST ex : handle.all_method('/'.to_string() , <your handler>)"]
    pub fn all_method(&mut self, path: String, handler: HandleCallback) {
        let key = get_key("*".to_string(), path);
        self.add_handle(key, handler);
    }

    #[doc = "to register the handler for not found case ex : handle.not_found(<your handler>)"]
    pub fn not_found(&mut self, handler: HandleCallback) {
        self.not_found_handler = handler;
    }

    #[doc = "to register the handler for all http request ex : handle.all(<your handler>)"]
    pub fn all(&mut self, handler: HandleCallback) {
        let key = get_key("*".to_string(), "*".to_string());
        self.add_handle(key, handler);
    }

    pub fn handle_http_request(&mut self, listener: TcpListener) {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut process = RequestProcessing::new(self.not_found_handler);
            process.preProcessing(&self.handler, &mut stream);
            if self.threading {
                let han = &mut self.handler;
                std::thread::spawn(move || process.processing(&mut stream))
                    .join()
                    .and_then(|f| {
                        f.ProcessingRouter(han);
                        Ok(())
                    })
                    .expect("đen thôi đỏ là red");
            } else {
                process
                    .processing(&mut stream)
                    .ProcessingRouter(&mut self.handler);
            }
            // stream
            //     .shutdown(std::net::Shutdown::Both)
            //     .expect("shutdown call failed");
        }
    }

    #[doc(hidden)]
    pub fn handlers(&self) -> &HandlerType {
        &self.handler
    }
    
    pub fn route(&mut self, path: String, route: &Self) {
        self.add_route(path, route)
    }
}

trait Handle {
    fn add_handle(&mut self, key: HandlerHashmapKeyString, handler: HandleCallback);
    fn add_multiple_handler(&mut self, key: HandlerHashmapKeyString, handler: Vec<HandleCallback>);
}
impl Handle for HttpHandler {
    fn add_handle(&mut self, key: HandlerHashmapKeyString, handler: HandleCallback) {
        self.add_multiple_handler(key, vec![handler])
    }

    fn add_multiple_handler(&mut self, key: HandlerHashmapKeyString, handler: Vec<HandleCallback>) {
        pub_add_multiple_handler(&mut self.handler, key, handler);
    }
}

pub fn pub_add_multiple_handler(
    handlers: &mut HandlerType,
    key: HandlerHashmapKeyString,
    handler: Vec<HandleCallback>,
) {
    let handle = handlers.get_mut(&key);
    match handle {
        Some(handle) => append_vec(handle, &handler),
        None => {
            handlers.insert(key, handler);
        }
    }
}
pub fn get_key(method: String, path: String) -> HandlerHashmapKeyString {
    let key = format!("{}{}", method, path);
    return HandlerHashmapKeyString {
        data: key,
        method,
        path,
    };
}
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct HandlerHashmapKeyString {
    pub data: String,
    pub method: String,
    pub path: String,
}

trait Router {
    fn add_route(&mut self, path: String, route: &Self);
}
impl Router for HttpHandler {
    fn add_route(&mut self, path: String, route: &Self) {
        add_route_pub(&mut self.handler, path, route.handlers());
    }
}

pub fn add_route_pub(hander: &mut HandlerType, path: String, route: &HandlerType) {
    for n in route.keys() {
        let mut new_path = path.clone();
        new_path.push_str(
            &(match new_path.ends_with("/") {
                true => n.path.clone(),
                false => match n.path.starts_with("/") {
                    true => n.path.clone(),
                    false => "/".to_owned() + &n.path,
                },
            }),
        );
        let key = get_key(n.method.clone(), new_path);
        pub_add_multiple_handler(hander, key, route.get(n).unwrap().to_vec());
    }
}
