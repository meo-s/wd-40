mod controller;
mod datasource;
mod error;
mod repository;

async fn work() -> Result<(), Box<dyn std::error::Error>> {
    use datasource::Connector;
    let connector = datasource::dynamodb::Connector::from_env().await;

    let mut board_repo = repository::board::dynamodb::new(&connector).await?;

    

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
