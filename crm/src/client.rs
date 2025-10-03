use anyhow::Result;
use tonic::Request;

use crm::pb::{user_service_client::UserServiceClient, CreateUserRequest};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = UserServiceClient::connect("http://[::1]:50051").await?;
    let request = Request::new(CreateUserRequest {
        name: "Alice".into(),
        email: "alice@acme.org".into(),
    });

    let response = client.create_user(request).await?;
    // let response = client.create_user(request).await?.into_inner();

    println!("response: {:?}", response);

    Ok(())
}
