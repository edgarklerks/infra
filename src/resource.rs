use aws_types::SdkConfig;
use crate::InfraError;
use async_trait::async_trait;

#[derive(Clone,Debug, PartialEq)]
pub enum ResourceState {
        Alive,
        Dead,
        GoingQuantum
}

#[async_trait]
pub trait InfraResource<ClientType> {
    type ResourceType;
    type SdkType;
    async fn init(config: &SdkConfig) -> Result<ClientType, InfraError>;
    async fn state(&self, resource : &Self::ResourceType) -> Result<ResourceState, InfraError>;
    async fn describe(&self, resource : &Self::ResourceType) -> Result<Option<Self::SdkType>, InfraError>;
}

#[async_trait]
pub trait InfraCreatable<ClientType : InfraResource<ClientType>>  {
    async fn create(&self, resource : &<ClientType as InfraResource<ClientType>>::ResourceType) -> Result<ResourceState, InfraError>;
}
#[async_trait]
pub trait InfraDeletable<ClientType : InfraResource<ClientType>> {
    async fn delete(&self, resource : &<ClientType as InfraResource<ClientType>>::ResourceType) -> Result<ResourceState, InfraError>;
}