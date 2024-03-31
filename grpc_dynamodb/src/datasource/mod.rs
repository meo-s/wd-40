pub mod dynamodb;

// TODO(meos): Sync & Send trait에 대해 더 자세히 공부하기
pub trait Connector<T>: Sync + Send {
    fn get_conn<'borrow>(&'borrow self) -> &'borrow T;
}
