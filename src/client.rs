mod compiled_protos;
use compiled_protos::helloworld::greeter_client::GreeterClient;
use compiled_protos::helloworld::HelloRequest;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_file(false)
        .init();

    let mut client = GreeterClient::connect("http://localhost:8080")
        .await
        .unwrap();

    let req = tonic::Request::new(HelloRequest {
        name: "gabriel".to_string(),
    });

    let response = client.say_hello(req).await;
    tracing::info!("response: {response:?}");
}
