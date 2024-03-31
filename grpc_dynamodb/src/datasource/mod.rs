pub mod dynamodb;

pub trait Connector<T> {
    fn get_conn<'borrow>(&'borrow self) -> &'borrow T;
}
