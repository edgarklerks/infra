use aws_sdk_ec2::model::Tag;
use aws_sdk_ec2::model::TagSpecification;
use aws_sdk_ec2::model::Filter;
use aws_sdk_ec2::model::Vpc;
use aws_sdk_ec2::output::DescribeVpcsOutput;
use aws_types::SdkConfig;
use crate::resource::{InfraCreatable, InfraResource, ResourceState};
use crate::types::{InfraError, unwrap_with_infra_error};
use async_trait::async_trait;
use log::info;
use crate::InfraError::{ CannotDescribeResource, MultipleResultsReturned};
use crate::resource::ResourceState::{Alive, Dead, GoingQuantum};

pub struct VpcClient(aws_sdk_ec2::Client);

#[derive(Clone, Debug)]
pub struct VpcResource{
    pub cidr : String,
    pub env : String,
    pub name : String
}


#[async_trait]
impl InfraResource<VpcClient> for VpcClient {
    type ResourceType = VpcResource;
    type SdkType = Vpc;

    async fn init(config: &SdkConfig) -> Result<VpcClient, InfraError> {
        let s = aws_sdk_ec2::Client::new(config);
        Ok(VpcClient(s))
    }
    async fn state(&self, resource: &VpcResource) -> Result<ResourceState, InfraError> {
        let result = self.describe(resource).await;
        match result {
            Err(MultipleResultsReturned(s)) => Ok(GoingQuantum),
            Err(e) => Err(e),
            Ok(Some(_v)) => Ok(Alive),
            Ok(None) => Ok(Dead)
        }
    }

    async fn describe(&self, resource: &VpcResource) -> Result<Option<Self::SdkType>, InfraError> {
        info!( "Filter on tag:{}={}", "name",resource.name.clone());
        info!( "Filter on tag:{}={}", "env",resource.env.clone());
        let name_filter = Filter::builder()
            .set_name(Some(format!("tag:{}", "name")))
            .set_values(Some(vec![resource.name.clone()])).build();
        let env_filter = Filter::builder()
            .set_name(Some(format!("tag:{}", "env")))
            .set_values(Some(vec![resource.env.clone()])).build();
        let result = self.0.describe_vpcs().set_filters(Some(vec![name_filter, env_filter])).send().await;
        match result {
            Ok(DescribeVpcsOutput { vpcs, next_token ,..}) => {
                match vpcs {
                    Some(vec) if vec.len() == 1 => {
                        let value: Option<Vpc>= vec.get(0).map(|s|s.clone());
                        Ok(value)
                    },
                    Some(vec) if vec.len() > 0 => Err(MultipleResultsReturned(format!("{:?}", vec))),
                    Some(vec) if vec.len() == 0 => Ok(None),
                    Some(_) => todo!(),
                    None => Ok(None)
                }
            }
            Err(e) => Err(CannotDescribeResource(format!("{:?}",e)))
        }
    }
}

#[async_trait]
impl InfraCreatable<VpcClient> for VpcClient  {

    async fn create(&self, resource: &VpcResource) -> Result<ResourceState, InfraError> {
        let already_created = self.state(resource).await;
        if unwrap_with_infra_error(already_created) == Alive {
            Ok(Alive)
        } else {

                let cidr = resource.cidr.clone();
                let out =
                    self.0.create_vpc().set_cidr_block(Some(cidr))
                        .set_amazon_provided_ipv6_cidr_block(Some(true))
                        .set_tag_specifications(Some(vec![
                            TagSpecification::builder()
                                .set_resource_type(Some(aws_sdk_ec2::model::ResourceType::Vpc))
                                .set_tags(Some(vec![
                                    Tag::builder().set_key(Some("name".to_string())).set_value(Some(resource.name.clone())).build(),
                                    Tag::builder().set_key(Some("env".to_string())).set_value(Some(resource.env.clone())).build()
                                ]))
                                .build()
                        ])).send().await;
                Ok(Alive)
            }
    }
}
