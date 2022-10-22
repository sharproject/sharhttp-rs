use regex::Regex;
use std::fmt::Debug;

pub type HeaderType = std::collections::HashMap<String, String>;
pub type CookieType = Vec<CookieValue>;

#[derive(Clone)]
pub struct CookieValue {
    pub name: String,
    pub value: String,
    pub Max_Age: i32,
}
impl CookieValue {
    pub fn header(&self) -> String {
        format!("{}={};Max-Age={}", self.name, self.value, self.Max_Age).to_string()
    }
}

#[derive(Clone)]
pub struct HeaderData {
    pub method: String,
    pub path: String,
    pub http_version: String,
    pub header: HeaderType,
}
impl HeaderData {
    pub fn Default() -> Self {
        Self {
            method: "".to_string(),
            path: "".to_owned(),
            http_version: "".to_string(),
            header: std::collections::HashMap::new(),
        }
    }
}

impl Debug for HeaderData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HeaderData")
            .field("method", &self.method)
            .field("path", &self.path)
            .field("httpVersion", &self.http_version)
            .finish()
    }
}

pub fn GetRequest(http_request: &Vec<String>) -> HeaderData {
    let http_request_path_method_regex =
        Regex::new(r"(POST|GET|PUT|PATCH|DELETE)(.+)(HTTP(/)\d.\d)").unwrap();
    let http_request_header_regex = Regex::new(r"(.+)\s?:\s(.+)").unwrap();
    let mut data: HeaderData = HeaderData {
        method: String::from(""),
        path: String::from(""),
        http_version: String::from(""),
        header: std::collections::HashMap::new(),
    };
    for e in http_request {
        if http_request_path_method_regex.is_match(&e) {
            for cap in http_request_path_method_regex.captures_iter(&e) {
                data.method = cap[1].to_string();
                data.path = cap[2].trim().to_string();
                data.http_version = cap[3].to_string();
            }
            continue;
        }
        if http_request_header_regex.is_match(&e) {
            for cap in http_request_header_regex.captures_iter(&e) {
                match cap[1].to_string().as_str() {
                    "Cookie" => {}
                    _ => {
                        data.header.insert(cap[1].to_string(), cap[2].to_string());
                    }
                }
            }
        }
    }
    return data;
}
