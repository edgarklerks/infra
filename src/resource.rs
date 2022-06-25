use std::any::Any;
use aws_types::SdkConfig;
use crate::InfraError;
use async_trait::async_trait;

#[async_trait]
pub trait InfraResource<ClientType> {
    type ResourceType;
    async fn init(config: &SdkConfig) -> Result<ClientType, InfraError>;
}

#[async_trait]
pub trait InfraCreatable<ClientType : InfraResource<ClientType>, Output>  {
    async fn created(&self, resource :  &<ClientType as InfraResource<ClientType>>::ResourceType) -> Result<bool, InfraError>;
    async fn create(&self, resource : &<ClientType as InfraResource<ClientType>>::ResourceType) -> Result<Output, InfraError>;
}

#[async_trait]
pub trait InfraDeletable<ClientType : InfraResource<ClientType>, Output> {
    async fn delete(&self, resource : &<ClientType as InfraResource<ClientType>>::ResourceType) -> Result<Output, InfraError>;
}