// The repository layer is where you will be writing queries to call the database
// In this layer we DO NOT process the data retrieved from the database
// We will simply handle the queries and return the data as is
// Any database related errors should be handled here
// In the repository layer I am using anyhow to handle errors as it is more convenient

use crate::utils::dynamodb_helpers::log_sdk_error;
use crate::utils::dynamodb_helpers::DynamoItem;
use crate::utils::dynamodb_helpers::IntoAttributeValue;
use aws_config::SdkConfig;
use aws_sdk_dynamodb::Client;

#[derive(Clone)]
pub struct UserRepository {
    client: Client,
    table_name: String,
    pub max_retries: Option<u32>,
}

impl UserRepository {
    pub async fn new(shared_config: &SdkConfig, max_retries: Option<u32>) -> Self {
        let client = Client::new(shared_config);

        Self {
            client,
            // For the sake of simplicity we will hardcode the table name here
            // You can also use environment variable to store the table name such as user_table_name
            table_name: "users".to_string(),
            max_retries,
        }
    }

    pub async fn get_user_by_id(self, id: String) -> anyhow::Result<Option<DynamoItem>> {
        let res = self
            .client
            .get_item()
            .table_name(self.table_name)
            .key("id", id.into_av()) // into_av() is a helper function that converts a primitive value into dynamodb's AttributeValue
            .send()
            .await;

        match res {
            Ok(res) => Ok(res.item),
            Err(e) => {
                log_sdk_error(e);
                Err(anyhow::anyhow!("Error while getting data"))
            }
        }
    }
}
