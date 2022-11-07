use std::{io::Write, net::TcpStream};

use crate::{
    status::canonical_reason,
    App::HandleCallback,
    Request::{
        get_default_header::default_header,
        get_http_data::{CookieType, CookieValue},
    },
};

pub struct ResponseTool<'a> {
    pub(crate) stream: &'a mut TcpStream,
    pub responded: bool,
    pub(crate) status: u128,
    pub content: String,
    pub header: &'a mut crate::Request::get_http_data::HeaderType,
    pub Request: &'a crate::Request::get_http_data::HeaderData,
    pub cookie: &'a mut crate::Request::get_http_data::CookieType,
    pub StartTime: std::time::Instant,
    pub finalFunction: HandleCallback,
    #[doc = "set this attr to enable and disable caching"]
    pub caching: bool,
    #[doc="if value is true app don't run next handler"]
    pub ended: bool,
}

impl ResponseTool<'_> {
    pub fn status(&mut self, status: u128) -> &mut Self {
        self.status = status;
        return self;
    }
    pub fn end(&mut self) {
        if self.responded {
            panic!("was response");
        }
        self.preProcessing();
        let status_line = format!(
            "HTTP/1.1 {} {}",
            self.status,
            canonical_reason(self.status.try_into().unwrap()).unwrap()
        );

        let length = self.content.len();

        let mut response_data = format!("{status_line}\r\nContent-Length: {length}\r\n");
        for (header_key, header_value) in self.header.clone() {
            response_data.push_str(&format!("{header_key}:{header_value}\r\n"));
        }
        for k in self.cookie.clone() {
            response_data.push_str(&format!("Set-Cookie: {}\n", k.header()));
        }
        response_data.push_str("\r\n");
        response_data.push_str(&self.content);

        match self.stream.write_all(response_data.as_bytes()) {
            Ok(_) => {
                self.responded = true;
            }
            Err(error) => {
                panic!("have the error when response {:?}", error);
            }
        };
    }
    pub fn send(&mut self, content: String, end: bool) {
        self.content = content;
        if end {
            self.end()
        }
    }
    pub fn Setup(&mut self) -> &mut Self {
        let default_header = default_header(&(self.Request.header));
        for n in default_header.keys() {
            self.header
                .insert(n.to_string(), default_header.get(n).unwrap().to_string());
        }
        self
    }
    pub fn set_header(&mut self, k: String, v: String) {
        self.header.insert(k, v);
    }
    pub fn remove_header(&mut self, k: String) {
        if self.header.contains_key(&k) {
            self.header.remove(&k);
        }
    }
    pub fn set_cookie(&mut self, v: &CookieValue) {
        self.cookie.push(v.clone());
    }
    pub fn remove_cookie(&mut self, n: String) {
        let mut newVec: CookieType = Vec::new();
        for k in self.cookie.clone() {
            if k.name != n {
                newVec.push(k);
            }
        }
        self.cookie.clone_from(&newVec);
    }

    #[doc = "to register the handler for not found case \n , warn : this function will overwrite all set before \n ,ex : handle.not_found(<your handler>)"]
    pub fn finalFn(&mut self, han: HandleCallback) {
        self.finalFunction = han;
    }
}

trait Process {
    fn preProcessing(&mut self);
}

impl Process for ResponseTool<'_> {
    fn preProcessing(&mut self) {}
}
