use anyhow::Result;
use crm::pb::{
    user_service_server::{UserService, UserServiceServer},
    CreateUserRequest, GetUserRequest, User,
};
use tonic::{async_trait, transport::Server, Request, Response, Status};

// cd to dir crm, then call "cargo run"
pub mod pb {
    // include!(concat!(env!("OUT_DIR"), "/crm.rs"));
}

#[derive(Default)]
pub struct UserServer;

#[async_trait]
impl UserService for UserServer {
    async fn get_user(&self, request: Request<GetUserRequest>) -> Result<Response<User>, Status> {
        let input = request.into_inner();
        println!("Get user: {:?}", input);

        Ok(Response::new(User::default()))
    }
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<User>, Status> {
        let input = request.into_inner();
        println!("Create user: {:?}", input);
        let user = User::new(1, &input.name, &input.email);

        Ok(Response::new(user))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // let user = User::new(1, "Alice", "alice@acme.org");
    // let encoded = user.encode_to_vec();
    // let decoded = User::decode(&encoded[..]).unwrap();
    // println!("user: {:?}", user);
    let addr = "[::1]:50051".parse().expect("should parse addr ok");
    let svc = UserServer;

    println!("UserServer is listening to {}", addr);

    Server::builder()
        .add_service(UserServiceServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}
