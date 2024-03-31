use crate::error::Error;

#[async_trait::async_trait]
pub trait Repo {
    async fn save(&self) -> Result<(), Error>;
}

pub mod dynamodb {
    use std::sync::Arc;

    use crate::{
        datasource::{self, Connector},
        error::Error,
    };
    use aws_sdk_dynamodb::{
        error::SdkError, operation::delete_table::DeleteTableError, types::AttributeValue,
    };
    use aws_smithy_runtime_api::http::Response;
    use aws_smithy_types::Blob;

    const BOARD_TABLE_NAME: &str = "article";
    const BOARD_TABLE_KEY_NAME: &str = "id";

    pub struct RepoImpl {
        connector: Arc<dyn Connector<aws_sdk_dynamodb::Client>>,
    }

    #[async_trait::async_trait]
    impl super::Repo for RepoImpl {
        async fn save(&self) -> Result<(), crate::error::Error> {
            let stmt = format!(
                r#"
                INSERT INTO {} VALUE {{
                    '{}': ?
                }}"#,
                BOARD_TABLE_NAME, BOARD_TABLE_KEY_NAME
            );

            let article_id = ulid::Ulid::new();

            let conn = self.connector.get_conn();
            match conn
                .execute_statement()
                .statement(stmt)
                .set_parameters(vec![AttributeValue::B(Blob::new(article_id.to_bytes()))].into())
                .send()
                .await
            {
                Ok(_) => Ok(()),
                Err(e) => Err(Error::AwsSdkError(e.into())),
            }
        }
    }

    impl RepoImpl {
        async fn new(
            connector: &Arc<dyn Connector<aws_sdk_dynamodb::Client>>,
        ) -> Result<impl super::Repo, Error> {
            if let Err(e) =
                datasource::dynamodb::delete_table(connector.get_conn(), BOARD_TABLE_NAME).await
            {
                if is_resource_not_found_exception(&e) {
                    return Err(e);
                }
            }

            datasource::dynamodb::create_table(
                connector.get_conn(),
                BOARD_TABLE_NAME,
                BOARD_TABLE_KEY_NAME,
            )
            .await?;

            Ok(RepoImpl {
                connector: Arc::clone(connector),
            })
        }
    }

    fn is_resource_not_found_exception(e: &Error) -> bool {
        #[allow(irrefutable_let_patterns)]
        if let Error::AwsSdkError(e) = e {
            let e = e
                .downcast_ref::<SdkError<DeleteTableError, Response>>()
                .unwrap();
            if let Some(e) = e.as_service_error() {
                !e.is_resource_not_found_exception()
            } else {
                true
            }
        } else {
            false
        }
    }

    pub async fn new(
        connector: &Arc<dyn Connector<aws_sdk_dynamodb::Client>>,
    ) -> Result<impl super::Repo, Error> {
        RepoImpl::new(connector).await
    }
}
