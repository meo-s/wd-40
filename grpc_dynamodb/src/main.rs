use std::sync::Arc;

mod controller;
mod datasource;
mod error;
mod repository;

async fn work() -> Result<(), Box<dyn std::error::Error>> {
    use repository::board::Repo;

    // https://github.com/rust-lang/rust/issues/116095
    // let connector = Arc::<dyn datasource::Connector<aws_sdk_dynamodb::Client>::new( ... );  <-- compile error
    let connector = datasource::dynamodb::Connector::from_env().await;
    let connector: Arc<dyn datasource::Connector<aws_sdk_dynamodb::Client>> = Arc::new(connector);
    let board_repo = repository::board::dynamodb::new(&Arc::new(connector)).await?;
    board_repo.save().await?;

    let addr = "[::]:8080".parse().unwrap();
    let board_controller = controller::board::new();
    tonic::transport::Server::builder()
        .add_service(board_controller)
        .serve(addr)
        .await?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(work())
}
