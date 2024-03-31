use crate::error::Error;
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{
    types::{AttributeDefinition, BillingMode, KeySchemaElement, KeyType, ScalarAttributeType},
    Client, Config,
};

#[derive(Default)]
struct ConfigLoader;

impl ConfigLoader {
    async fn from_env() -> Config {
        // TODO(meos): 배포시의 설정법을 찾아봐야 함
        let aws_cfg = aws_config::defaults(BehaviorVersion::latest())
            .test_credentials()
            .load()
            .await;

        let mut builder = aws_sdk_dynamodb::config::Builder::from(&aws_cfg);
        if let Ok(region_name) = std::env::var("AWS_REGION") {
            builder.set_region(aws_config::Region::new(region_name).into());
        }
        if let Ok(endpoint_url) = std::env::var("AWS_ENDPOINT_URL") {
            builder.set_endpoint_url(endpoint_url.into());
        }

        builder.build()
    }
}

pub struct Connector {
    conn: Client,
}

impl Connector {
    pub async fn from_env() -> Connector {
        let dynamodb_cfg = ConfigLoader::from_env().await;
        Connector {
            conn: Client::from_conf(dynamodb_cfg),
        }
    }
}

impl super::Connector<Client> for Connector {
    fn get_conn<'a>(&'a self) -> &'a Client {
        &self.conn
    }
}

pub async fn create_table(conn: &Client, table_name: &str, key_name: &str) -> Result<(), Error> {
    let ad = AttributeDefinition::builder()
        .attribute_name(key_name.to_string())
        .attribute_type(ScalarAttributeType::B)
        .build()
        .map_err(|e| Error::AwsSdkError(e.into()))?;

    let ks = KeySchemaElement::builder()
        .attribute_name(key_name.to_string())
        .key_type(KeyType::Hash)
        .build()
        .map_err(|e| Error::AwsSdkError(e.into()))?;

    conn.create_table()
        .table_name(table_name.to_string())
        .key_schema(ks)
        .attribute_definitions(ad)
        .billing_mode(BillingMode::PayPerRequest)
        .send()
        .await
        .map_err(|e| Error::AwsSdkError(e.into()))?;
    Ok(())
}

pub async fn delete_table(conn: &Client, table_name: &str) -> Result<(), Error> {
    conn.delete_table()
        .table_name(table_name.to_string())
        .send()
        .await
        .map_err(|e| Error::AwsSdkError(e.into()))?;
    Ok(())
}
