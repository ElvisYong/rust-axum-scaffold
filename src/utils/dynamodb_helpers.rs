use aws_sdk_dynamodb::{types::AttributeValue, primitives::Blob};


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