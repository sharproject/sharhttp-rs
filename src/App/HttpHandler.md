# this is the doc for HttpHandler

## info

- HttpHandler is the main of struct for init the new tcp handler
- this is the "main" handler or sub handler if you want (route method)

## create handler

- this is the recommend way to create the new server:

  ```rs
    use rust_http_web_lib::routeManager::RouterManager;
    use rust_http_web_lib::App::HttpHandler;
    use rust_http_web_lib::Request::get_http_data::HeaderData;
    use rust_http_web_lib::Response::ResponseTool;
    // we use the std::net::TcpListener to create the tcp server
    use std::net::TcpListener;

    fn main() {
        let mut handle = HttpHandler::new();
        const PORT: i32 = 8000;

        handle.get("/".to_owned(), home_handler);

        // log 
        handle.all(log_request);

        let listener = TcpListener::bind(format!("127.0.0.1:{PORT}"));
        match listener {
            Ok(listener) => {
                println!(
                    "server is open at {} link : http://{}",
                    PORT,
                    listener.local_addr().unwrap()
                );
                handle.handle_http_request(listener);
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }
    fn home_handler(_request: &HeaderData, response: &mut ResponseTool, _: &mut RouterManager) {
        response.send("<h1>hello world</h1>".to_string(), true);
    }
    fn log_request(request: &HeaderData, _response: &mut ResponseTool, _: &mut RouterManager) {
        println!("method: {}, path: {}", request.method, request.path);
    }

  ```
