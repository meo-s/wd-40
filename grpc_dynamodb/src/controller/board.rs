use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};

pub mod pb {
    tonic::include_proto!("board");
}

struct BoardServiceImpl;
impl BoardServiceImpl {
    fn new() -> BoardServiceImpl {
        BoardServiceImpl {}
    }
}

#[async_trait]
impl pb::board_service_server::BoardService for BoardServiceImpl {
    async fn read_article(
        &self,
        _req: Request<pb::ReadArticleRequest>,
    ) -> Result<Response<pb::ReadArticleResponse>, Status> {
        let resp = pb::ReadArticleResponse {
            id: 0,
            title: "".into(),
        };
        Ok(Response::new(resp))
    }

    async fn write_article(
        &self,
        _req: Request<pb::WriteArticleRequest>,
    ) -> Result<Response<pb::WriteArticleResponse>, Status> {
        let resp = pb::WriteArticleResponse { id: 0 };
        Ok(Response::new(resp))
    }
}

pub fn new(
) -> pb::board_service_server::BoardServiceServer<impl pb::board_service_server::BoardService> {
    let svc = Arc::new(BoardServiceImpl::new());
    pb::board_service_server::BoardServiceServer::from_arc(svc)
}
