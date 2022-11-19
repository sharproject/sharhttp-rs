use std::{io::Write, net::TcpStream};

use crate::{
    status::canonical_reason,
    App::HandleCallback,
    Request::{
        get_default_header::default_header,
        get_http_data::{CookieType, CookieValue},
    },
};

pub struct ResponseTool {
    pub stream: TcpStream,
    pub responded: bool,
    pub status: u128,
    pub content: String,
    pub header: crate::Request::get_http_data::HeaderType,
    pub Request: crate::Request::get_http_data::HeaderData,
    pub cookie: crate::Request::get_http_data::CookieType,
    pub StartTime: std::time::Instant,
    pub finalFunction: HandleCallback,
    #[doc = "set this attr to enable and disable caching"]
    pub caching: bool,
    #[doc = "if value is true app don't run next handler"]
    pub ended: bool,
    pub requestCookie: CookieType,
    pub removeCookieHeader: CookieType,
    pub content_bytes: Vec<u8>,
}

impl ResponseTool {
    pub fn status(&mut self, status: u128) -> &mut Self {
        self.status = status;
        return self;
    }
    pub fn end(&mut self) {
        if self.responded {
            panic!("was response");
        }
        self.preProcessing();
        let mut response_bytes = Vec::default();

        if self.content.len() > 0 {
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
            for k in self.removeCookieHeader.clone() {
                response_data.push_str(&format!("Set-Cookie: {}\n", k.delete_cookie_header()));
            }
            response_data.push_str("\r\n");
            response_data.push_str(&self.content);
            response_bytes = response_data.as_bytes().to_vec();
        } else {
            response_bytes = self.content_bytes.clone();
        }


        match self.stream.write_all(&response_bytes.as_slice()) {
            Ok(_) => {
                self.responded = true;
            }
            Err(error) => {
                panic!("have the error when response {:#?}", error);
            }
        };
    }
    pub fn send(&mut self, content: String, end: bool) {
        if self.content_bytes.len() > 0 {
            println!("warning: some things will wrong when framwork try to send response , send function");
        }

        self.content = content;
        if end {
            self.end()
        }
    }
    pub fn send_bytes(&mut self, content: &[u8], end: bool) {
        if self.content.len() > 0 {
            println!("warning: some things will wrong when framwork try to send response , send_bytes function");
        }

        self.content_bytes = content.to_vec();
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
        for n in &self.Request.cookie {
            self.requestCookie.push(CookieValue {
                name: n.0.to_string(),
                value: n.1.to_string(),
                Max_Age: None,
            })
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
        for k in &self.requestCookie {
            if k.name == n {
                self.removeCookieHeader.push(k.clone())
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

impl Process for ResponseTool {
    fn preProcessing(&mut self) {}
}

pub fn getResponseTool(
    stream: TcpStream,
    httpData: &crate::Request::get_http_data::HeaderData,
    startTime: std::time::Instant,
    finalFunction: HandleCallback,
) -> ResponseTool {
    return ResponseTool {
        stream,
        responded: false,
        status: 200,
        content: "".to_string(),
        header: std::collections::BTreeMap::new(),
        Request: httpData.clone(),
        cookie: Vec::new(),
        StartTime: startTime,
        finalFunction,
        caching: false,
        ended: false,
        requestCookie: CookieType::default(),
        removeCookieHeader: CookieType::default(),
        content_bytes: Default::default(),
    };
}
