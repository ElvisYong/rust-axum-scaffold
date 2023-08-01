use aws_sdk_dynamodb::error::SdkError::{
    ConstructionFailure, DispatchFailure, ResponseError, ServiceError, TimeoutError,
};
use aws_sdk_dynamodb::{error::SdkError, primitives::Blob, types::AttributeValue};
use std::collections::HashMap;
use std::fmt::Debug;
use tracing::log::error;

pub type DynamoItem = HashMap<String, AttributeValue>;

/// Custom helpers for dynamodb to quickly convert primitives into AttributeValue
/// Only a couple of types are implemented here, but you can add more as you need
pub trait IntoAttributeValue {
    fn into_av(self) -> AttributeValue;
}

impl IntoAttributeValue for u64 {
    fn into_av(self) -> AttributeValue {
        AttributeValue::N(self.to_string())
    }
}

impl IntoAttributeValue for String {
    fn into_av(self) -> AttributeValue {
        AttributeValue::S(self)
    }
}

impl IntoAttributeValue for Vec<u8> {
    fn into_av(self) -> AttributeValue {
        AttributeValue::B(Blob::new(self))
    }
}

impl IntoAttributeValue for bool {
    fn into_av(self) -> AttributeValue {
        AttributeValue::Bool(self)
    }
}

/// Helper function to help log the errors from dynamodb sdk
pub fn log_sdk_error<T>(error: SdkError<T>)
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