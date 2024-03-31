use crate::error::Error;

pub trait Repo<'c> {
    async fn save(&'c self) -> Result<(), Error>;
}

pub mod dynamodb {
    use crate::{
        datasource::{self, dynamodb::Connector, Connector as GenericConnector},
        error::Error,
    };
    use aws_sdk_dynamodb::{
        error::SdkError, operation::delete_table::DeleteTableError, types::AttributeValue,
    };
    use aws_smithy_runtime_api::http::Response;
    use aws_smithy_types::Blob;

    const BOARD_TABLE_NAME: &str = "article";
    const BOARD_TABLE_KEY_NAME: &str = "id";

    struct RepoImpl<'c> {
        connector: &'c Connector,
    }

    impl<'c> RepoImpl<'c> {
        async fn new(connector: &'c Connector) -> Result<RepoImpl<'c>, Error> {
            if let Err(Error::AwsSdkError(e)) =
                datasource::dynamodb::delete_table(connector.get_conn(), BOARD_TABLE_NAME).await
            {
                let unexpected = {
                    let e = e
                        .downcast_ref::<SdkError<DeleteTableError, Response>>()
                        .unwrap();
                    if let Some(e) = e.as_service_error() {
                        !e.is_resource_not_found_exception()
                    } else {
                        true
                    }
                };
                if unexpected {
                    return Err(Error::AwsSdkError(e));
                }
            }

            datasource::dynamodb::create_table(
                connector.get_conn(),
                BOARD_TABLE_NAME,
                BOARD_TABLE_KEY_NAME,
            )
            .await?;

            Ok(RepoImpl { connector })
        }
    }

    impl<'c> super::Repo<'c> for RepoImpl<'c> {
        async fn save(&'c self) -> Result<(), crate::error::Error> {
            let conn = self.connector.get_conn();
            let stmt = format!(r#"INSERT INTO {} VALUES {{ "id": ? }}"#, BOARD_TABLE_NAME);

            let article_id = ulid::Ulid::new();
            match conn
                .execute_statement()
                .statement(stmt)
                .set_parameters(vec![AttributeValue::B(Blob::new(article_id.to_bytes()))].into())
                .send()
                .await
            {
                Ok(_) => {
                    println!("{} saved !!", article_id.to_string());
                    Ok(())
                }
                Err(e) => {
                    println!("failed to save");
                    Err(Error::AwsSdkError(e.into()))
                }
            }
        }
    }

    pub async fn new<'c>(connector: &'c Connector) -> Result<impl super::Repo<'c>, Error> {
        RepoImpl::new(connector).await
    }
}
