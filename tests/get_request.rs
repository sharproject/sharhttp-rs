use rust_http_web_lib::Request::get_http_data::GetRequest;
#[test]
fn test_get_request() {
    let request_data = vec![
        "GET / HTTP/1.1",
        "Host: 127.0.0.1:8000",
        "Connection: keep-alive",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let request = GetRequest(&request_data);
    assert_eq!(request.method, "GET");
    assert_eq!(request.path, "/");
    assert_eq!(request.http_version, "HTTP/1.1");
    assert_eq!(
        request.header.get("Host"),
        Some(&"127.0.0.1:8000".to_owned())
    );
    assert_eq!(
        request.header.get("Connection"),
        Some(&"keep-alive".to_owned())
    )
}
