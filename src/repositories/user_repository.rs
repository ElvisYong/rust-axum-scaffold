use aws_config::SdkConfig;
use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::error::SdkError::{
    ConstructionFailure, DispatchFailure, ResponseError, ServiceError, TimeoutError,
};
use aws_sdk_dynamodb::operation::put_item::PutItemOutput;
use aws_sdk_dynamodb::types::{AttributeValue, WriteRequest};
use aws_sdk_dynamodb::Client;
use futures_util::future::join_all;
use std::cmp;
use std::collections::HashMap;
use std::fmt::Debug;
use std::time::Duration;
use tokio::time::sleep;
use tracing::log::{error, warn};

const INITIAL_DELAY: Duration = Duration::from_secs(1);
const MAX_DELAY: Duration = Duration::from_secs(300);

pub type DynamoItem = HashMap<String, AttributeValue>;

#[derive(Clone)]
pub struct UserRepository {
    client: Client,
    pub max_retries: Option<u32>,
}

impl UserRepository {
    pub async fn new(shared_config: &SdkConfig, max_retries: Option<u32>) -> Self {
        let client = Client::new(shared_config);

        Self {
            client,
            max_retries,
        }
    }

    pub fn get_client(self) -> Client {
        self.client
    }

    pub async fn full_scan(self, table_name: &String) -> anyhow::Result<Option<Vec<DynamoItem>>> {
        let res = self.client.scan().table_name(table_name).send().await;

        match res {
            Ok(res) => Ok(res.items),
            Err(e) => {
                log_sdk_error(e);
                Err(anyhow::anyhow!("Error while scanning data"))
            }
        }
    }

    pub async fn query_table(
        self,
        table_name: &String,
        index_name: Option<String>,
        key_condition_expression: String,
        expression_attribute_names: Option<HashMap<String, String>>,
        expression_attribute_values: HashMap<String, AttributeValue>,
        scan_index_forward: bool,
        limit: Option<i32>,
    ) -> anyhow::Result<Option<Vec<DynamoItem>>> {
        let res = self
            .client
            .query()
            .table_name(table_name)
            .set_index_name(index_name)
            .set_key_condition_expression(Some(key_condition_expression))
            .set_expression_attribute_names(expression_attribute_names)
            .set_expression_attribute_values(Some(expression_attribute_values))
            .scan_index_forward(scan_index_forward)
            .limit(limit.unwrap_or(i32::MAX))
            .send()
            .await;

        match res {
            Ok(res) => Ok(res.items),
            Err(e) => {
                log_sdk_error(e);
                Err(anyhow::anyhow!("Error while querying data"))
            }
        }
    }

    pub async fn insert(
        self,
        table_name: String,
        aws_hash_map: HashMap<String, AttributeValue>,
    ) -> anyhow::Result<PutItemOutput> {
        let res = self
            .client
            .put_item()
            .set_table_name(Some(table_name))
            .set_item(Some(aws_hash_map))
            .send()
            .await;

        match res {
            Ok(res) => Ok(res),
            Err(e) => {
                log_sdk_error(e);
                Err(anyhow::anyhow!("Error while inserting data"))
            }
        }
    }

    pub async fn batch_insert(self, table_name: String, write_requests: Vec<WriteRequest>) {
        let chunks = write_requests.chunks(25);

        for chunk in chunks {
            Self::ingest_batch_chunk(&self.client, &table_name, chunk.to_vec(), self.max_retries)
                .await;
        }
    }

    pub async fn par_batch_insert(self, table_name: String, write_requests: Vec<WriteRequest>) {
        let chunks = write_requests.chunks(25);
        let mut insert_tasks = Vec::new();

        for chunk in chunks {
            let cloned_client = self.client.clone();
            let cloned_table_name = table_name.clone();
            let cloned_chunk = chunk.to_vec();
            let max_retries = self.max_retries;

            insert_tasks.push(tokio::spawn(async move {
                Self::ingest_batch_chunk(
                    &cloned_client,
                    &cloned_table_name,
                    cloned_chunk,
                    max_retries,
                )
                .await;
            }));
        }

        let completed_tasks = join_all(insert_tasks).await;
        for task in completed_tasks {
            if let Err(e) = task {
                error!("Error while inserting: {:?}", e);
            }
        }
    }

    async fn ingest_batch_chunk(
        client: &Client,
        table_name: &str,
        write_requests: Vec<WriteRequest>,
        max_retries: Option<u32>,
    ) {
        let mut current_retries = 0;
        let mut items_to_process = write_requests.clone();
        let mut retry_on_error = false;

        while !items_to_process.is_empty() {
            let response = client
                .batch_write_item()
                .request_items(table_name, items_to_process.clone())
                .send()
                .await;

            match response {
                Ok(result) => match result.unprocessed_items() {
                    Some(unprocessed_items) if !unprocessed_items.is_empty() => {
                        match unprocessed_items.get(table_name) {
                            Some(retry_items) => {
                                items_to_process = retry_items.clone();
                            }
                            None => {
                                items_to_process = vec![];
                            }
                        }
                    }
                    _ => {
                        items_to_process = vec![];
                    }
                },
                Err(e) => {
                    log_sdk_error(e);
                    retry_on_error = true;
                }
            }

            if retry_on_error {
                current_retries += 1;
                if let Some(max_retries) = max_retries {
                    if current_retries > max_retries {
                        error!("Exceeded maximum number of retries, giving up.");
                        break;
                    }
                }
                let delay_time = cmp::min(INITIAL_DELAY * 2u32.pow(current_retries), MAX_DELAY);
                sleep(delay_time).await;
                warn!("Retrying after {} seconds...", delay_time.as_secs());
                items_to_process = write_requests.clone();
                retry_on_error = false;
            }
        }
    }
}

fn log_sdk_error<T>(error: SdkError<T>)
where
    T: Debug,
{
    match error {
        // The request failed during construction. It was not dispatched over the network.
        ConstructionFailure(construction_failure) => {
            error!("Construction failure: {:?}", construction_failure)
        }
        // The request failed due to a timeout. The request MAY have been sent and received.
        TimeoutError(timeout_error) => {
            error!("Timeout error: {:?}", timeout_error);
        }
        // The request failed during dispatch. An HTTP response was not received. The request MAY
        // have been sent.
        DispatchFailure(dispatch_failure) => {
            error!("Dispatch failure: {:?}", dispatch_failure);
        }
        // A response was received but it was not parse-able according the the protocol (for example
        // the server hung up while the body was being read)
        ResponseError(response_error) => {
            error!("Response error: {:?}", response_error);
        }
        // An error response was received from the service
        ServiceError(service_error) => {
            error!("Service error: {:?}", service_error);
        }
        _ => {
            error!("Error while ingesting: {:?}", error);
        }
    }
}
