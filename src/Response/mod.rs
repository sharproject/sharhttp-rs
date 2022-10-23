use std::{io::Write, net::TcpStream};

use crate::Request::{
    get_default_header::default_header,
    get_http_data::{CookieType, CookieValue},
};

pub struct ResponseTool<'a> {
    pub(crate) stream: &'a mut TcpStream,
    pub response: bool,
    pub(crate) status: i128,
    pub content: String,
    pub header: &'a mut crate::Request::get_http_data::HeaderType,
    pub Request: &'a crate::Request::get_http_data::HeaderData,
    pub cookie: &'a mut crate::Request::get_http_data::CookieType,
}

impl ResponseTool<'_> {
    pub fn status(&mut self, status: i128) -> &mut Self {
        self.status = status;
        return self;
    }
    pub fn end(&mut self) {
        if self.response {
            panic!("was response");
        }
        self.preProcessing();
        let mut status_line = "".to_owned();
        status_line.push_str("HTTP/1.1 ");
        status_line.push_str(&self.status.to_string());
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
                self.response = true;
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
}

trait Process {
    fn preProcessing(&mut self);
}

impl Process for ResponseTool<'_> {
    fn preProcessing(&mut self) {
    }
}
