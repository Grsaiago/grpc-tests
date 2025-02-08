mod compiled_protos;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::atomic::AtomicU64,
};

use compiled_protos::helloworld::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};
use http::{HeaderMap, Request};
use tonic::{transport::Server, Response};
use tower::ServiceBuilder;
use tower_http::trace::{DefaultOnRequest, TraceLayer};
use tracing::{info_span, Level};

#[derive(Debug, Default)]
pub struct MyGreeter {
    counter: AtomicU64,
}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: tonic::Request<HelloRequest>,
    ) -> std::result::Result<tonic::Response<HelloReply>, tonic::Status> {
        tracing::info!("Got a new request! {request:?}");
        self.counter
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        let reply = HelloReply {
            message: format!("hello {}", &request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .compact()
        .with_file(false)
        .with_target(false)
        .init();

    let port = 8080;
    let host: Ipv4Addr = "0.0.0.0".parse().unwrap();
    let addr = SocketAddr::new(IpAddr::V4(host), port);

    let greeter_server = MyGreeter::default();

    let server = Server::builder()
        .layer(
            ServiceBuilder::new().layer(
                TraceLayer::new_for_grpc()
                    .make_span_with(|req: &Request<_>| {
                        info_span!("grpc_request",
                            id = %format_request_id(req.headers())
                        )
                    })
                    .on_request(DefaultOnRequest::new().level(Level::INFO)),
            ),
        )
        .add_service(GreeterServer::new(greeter_server));

    tracing::info!("starting server on {}:{}", host, port);
    if let Err(err) = server.serve(addr).await {
        tracing::error!("server start failed {err:?}");
    }
    return;
}

pub fn format_request_id(headers: &HeaderMap) -> String {
    headers
        .get("X-Request-Id")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("<none>")
        .to_string()
}
