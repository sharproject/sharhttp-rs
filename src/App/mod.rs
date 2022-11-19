#![allow(dead_code)]
use std::{collections::BTreeMap, net::TcpListener};

use crate::{
    util::append_vec::append_vec,
    HandleConnection::{ProcessReturnValue, RequestProcessing},
    Request::get_http_data::HeaderData,
    Response::ResponseTool,
    RouteManager::RouterManager,
};

pub type HandleCallback = fn(&mut HeaderData, &mut ResponseTool, &mut RouterManager) -> (); // request: HeaderData, response: TcpStream

pub type HandlerType = BTreeMap<HandlerBTreeMapKeyString, Vec<HandleCallback>>;
pub type RequestCaching = BTreeMap<String, String>;
#[doc = include_str!("../../doc/HttpHandler.md")]
#[derive(Clone)]
pub struct HttpHandler {
    handler: HandlerType,
    not_found_handler: HandleCallback,
    threading: bool,
    finalHandler: HandleCallback,
    request_caching: RequestCaching,
    cache_not_found_case: bool,
}

pub fn default_not_found_handler(
    _req: &mut HeaderData,
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
            handler: (BTreeMap::new()),
            not_found_handler: default_not_found_handler,
            threading: false,
            finalHandler: |_, _, _| {},
            request_caching: RequestCaching::new(),
            cache_not_found_case: false,
        };
    }

    #[doc = "to turn on or off threading ex : handle.turn_threading()"]
    pub fn turn_threading(&mut self) -> &mut Self {
        self.threading = !self.threading;
        self
    }

    #[doc = "to register the post handler ex : handle.post('/'.to_string() , <your handler>)"]
    pub fn post(&mut self, path: String, handler: HandleCallback) -> &mut Self {
        self.register_handler("POST".to_string(), path, handler)
    }

    #[doc = "to register the get handler ex : handle.get('/'.to_string() , <your handler>)"]
    pub fn get(&mut self, path: String, handler: HandleCallback) -> &mut Self {
        self.register_handler("GET".to_string(), path, handler)
    }

    #[doc = "to register the put handler ex : handle.put('/'.to_string() , <your handler>)"]
    pub fn put(&mut self, path: String, handler: HandleCallback) -> &mut Self {
        self.register_handler("PUT".to_owned(), path, handler)
    }

    #[doc = "to register the patch handler ex : handle.patch('/'.to_string() , <your handler>)"]
    pub fn patch(&mut self, path: String, handler: HandleCallback) -> &mut Self {
        self.register_handler("PATCH".to_string(), path, handler)
    }

    #[doc = "to register the delete handler ex : handle.delete('/'.to_string() , <your handler>)"]
    pub fn delete(&mut self, path: String, handler: HandleCallback) -> &mut Self {
        self.register_handler("DELETE".to_string(), path, handler)
    }

    #[doc = "to register the handler for all method like:GET,POST ex : handle.all_method('/'.to_string() , <your handler>)"]
    pub fn all_method(&mut self, path: String, handler: HandleCallback) -> &mut Self {
        self.register_handler(String::from("*"), path, handler)
    }

    #[doc = "to register the handler for not found case ex : handle.not_found(<your handler>)"]
    pub fn not_found(&mut self, handler: HandleCallback) -> &mut Self {
        self.not_found_handler = handler;
        self
    }

    #[doc = "to register the handler for run when response case ex : handle.finalFn(<your handler>)"]
    pub fn finalFn(&mut self, handler: HandleCallback) -> &mut Self {
        self.finalHandler = handler;
        self
    }

    #[doc = "to register the handler for all http request ex : handle.all(<your handler>)"]
    pub fn all(&mut self, handler: HandleCallback) -> &mut Self {
        self.register_handler(String::from("*"), String::from("*"), handler)
    }

    pub fn handle_http_request(&mut self, listener: TcpListener) {
        for stream in listener.incoming() {
            let now = std::time::Instant::now();
            {
                let mut stream = stream.unwrap();
                let mut process = RequestProcessing::new(
                    self.not_found_handler,
                    now,
                    self.finalHandler,
                    self.cache_not_found_case,
                );
                process.preProcessing(&self.handler, &mut stream, &self.request_caching);

                if self.threading {
                    let (sender, receiver) = std::sync::mpsc::channel::<ProcessReturnValue>();

                    std::thread::spawn(move || sender.send(process.processing(stream)));
                    // threadHan
                    //     .join()
                    //     .and_then(|f| {
                    //         f.ProcessingRouter(han);
                    //         Ok(())
                    //     })
                    //     .expect("đen thôi đỏ là red");
                    receiver
                        .recv()
                        .unwrap()
                        .setData(&mut self.handler, &mut self.request_caching);
                } else {
                    process
                        .processing(stream)
                        .setData(&mut self.handler, &mut self.request_caching);
                }
            }
        }
    }

    #[doc(hidden)]
    pub fn handlers(&self) -> &HandlerType {
        &self.handler
    }

    pub fn route(&mut self, path: String, route: &Self) -> &mut Self {
        self.add_route(path, route)
    }
}

trait Handle {
    fn add_handle(&mut self, key: HandlerBTreeMapKeyString, handler: HandleCallback);
    fn add_multiple_handler(&mut self, key: HandlerBTreeMapKeyString, handler: Vec<HandleCallback>);
    fn register_handler(
        &mut self,
        method: String,
        key: String,
        handler: HandleCallback,
    ) -> &mut Self;
}
impl Handle for HttpHandler {
    fn add_handle(&mut self, key: HandlerBTreeMapKeyString, handler: HandleCallback) {
        self.add_multiple_handler(key, vec![handler])
    }

    fn add_multiple_handler(
        &mut self,
        key: HandlerBTreeMapKeyString,
        handler: Vec<HandleCallback>,
    ) {
        pub_add_multiple_handler(&mut self.handler, key.clone(), handler);
    }
    fn register_handler(
        &mut self,
        method: String,
        path: String,
        handler: HandleCallback,
    ) -> &mut Self {
        let key = get_key(method, path);
        self.add_handle(key, handler);
        self
    }
}

pub fn pub_add_multiple_handler(
    handlers: &mut HandlerType,
    key: HandlerBTreeMapKeyString,
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
pub fn get_key(method: String, path: String) -> HandlerBTreeMapKeyString {
    let key = format!("{}{}", method, path);
    return HandlerBTreeMapKeyString {
        data: key,
        method,
        path,
    };
}
#[derive(Clone, Ord, PartialOrd, PartialEq, Eq, Debug)]
pub struct HandlerBTreeMapKeyString {
    pub data: String,
    pub method: String,
    pub path: String,
}

trait Router {
    fn add_route(&mut self, path: String, route: &Self) -> &mut Self;
}
impl Router for HttpHandler {
    fn add_route(&mut self, path: String, route: &Self) -> &mut Self {
        add_route_pub(&mut self.handler, path, route.handlers());
        self
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
