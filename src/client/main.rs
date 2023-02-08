use my_service::{my_service_client::MyServiceClient, MyRequest};
use tonic::Request;

pub mod my_service {
    tonic::include_proto!("my_service");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = MyServiceClient::connect("http://127.0.0.1:50051").await?;

    let request = Request::new(MyRequest {
        name: "RS_PROTOBUF".into()
    });

    let response = client.my_method(request).await?;

    println!("response: {:?}", response);

    Ok(())
}
