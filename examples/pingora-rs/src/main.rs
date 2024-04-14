use pingora::apps::http_app::ServeHttp;
use pingora::protocols::http::ServerSession;
use http::{Response, StatusCode};
use async_trait::async_trait;
use pingora::server::Server;
use std::sync::Arc;
use bytes::Bytes;
use std::time::Duration;
use pingora::server::configuration::Opt;
use pingora::prelude::timeout;
use pingora::services::listening::Service;
use structopt::StructOpt;

pub struct HttpHelloworldApp;

#[async_trait]
impl ServeHttp for HttpHelloworldApp {
        async fn response(&self, http_stream: &mut ServerSession) -> Response<Vec<u8>> {
                let rd_timeout = 1000;
                let body = match timeout(
                        Duration::from_millis(rd_timeout),
                        http_stream.read_request_body(),
                ).await
                {
                        Ok(_) => {
                                // println!("Recieved request");
                                Bytes::from("Hello, World from first server!\n")
                        },
                        Err(_)  => {
                                panic!("Timeout!");
                        }
                };
                Response::builder()
                        .status(StatusCode::OK)
                        .header(http::header::CONTENT_TYPE, "text/html")
                        .header(http::header::CONTENT_LENGTH, body.len())
                        .body(body.to_vec())
                        .unwrap()
        }
}

fn new_http_hello_app() -> Arc<HttpHelloworldApp> {
        Arc::new(HttpHelloworldApp{})
}

fn echo_service_http() -> Service<HttpHelloworldApp> {
        Service::new("Helloworld app".to_string(), new_http_hello_app())
}

pub fn main() {

        let opt = Some(Opt::from_args());
        let mut server = Server::new(opt).unwrap();
        server.bootstrap();

        let mut echo_hello = echo_service_http();
        echo_hello.add_tcp("0.0.0.0:8080");

        println!("Listening for connections on port: {}", 8080);
        server.add_service(echo_hello);
        server.run_forever();
}
