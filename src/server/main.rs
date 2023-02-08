use tonic::{transport::Server, Request, Response, Status};

pub mod my_service {
    tonic::include_proto!("my_service");
}

use my_service::{
    my_service_server::{MyService, MyServiceServer},
    MyRequest, MyResponse,
};

#[derive(Default)]
pub struct MyServiceImpl {}

#[tonic::async_trait]
impl MyService for MyServiceImpl {
    async fn my_method(&self, request: Request<MyRequest>) -> Result<Response<MyResponse>, Status> {
        let response = MyResponse {
            message: format!("Hello, {}!", request.into_inner().name),
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    let service = MyServiceServer::new(MyServiceImpl::default());

    Server::builder().add_service(service).serve(addr).await?;

    Ok(())
}
