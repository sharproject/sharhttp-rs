use regex::Regex;
use std::{fmt::Debug};

pub type HeaderType = std::collections::BTreeMap<String, String>;
pub type CookieType = Vec<CookieValue>;

#[derive(Clone)]
pub struct CookieValue {
    pub name: String,
    pub value: String,
    pub Max_Age: Option<i32>,
}
impl CookieValue {
    pub fn header(&self) -> String {
        Self::create_header(self.name.clone(), self.value.clone(), self.Max_Age)
    }
    pub fn delete_cookie_header(&self) -> String {
        Self::create_header(self.name.clone(), "deleted".to_string(), None)
    }
    pub fn create_header(name: String, value: String, Max_Age: Option<i32>) -> String {
        format!(
            "{}={};{}",
            name,
            value,
            match Max_Age {
                Some(a) => format!("Max-Age={}", a),
                None => "".to_string(),
            }
        )
        .to_string()
    }
}

#[derive(Clone,Default)]
pub struct HeaderData {
    pub method: String,
    pub path: String,
    pub http_version: String,
    pub header: HeaderType,
    pub cookie: HeaderType,
    pub rawData : Vec<String>,

    pub LocalData: serde_json::Map<String, serde_json::Value>,

    pub LocalLibParseData: serde_json::Map<String, serde_json::Value>,
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
    let mut data: HeaderData = HeaderData::default();
    data.rawData = http_request.clone();
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
                    "Cookie" => {
                        let path = cap[2].split("=").collect::<Vec<&str>>();
                        data.cookie.insert(path[0].to_string(), path[1].to_string());
                    }
                    _ => {
                        data.header.insert(cap[1].to_string(), cap[2].to_string());
                    }
                }
            }
        }
    }
    return data;
}
