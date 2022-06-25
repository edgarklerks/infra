use std::borrow::Borrow;
use std::cell::RefCell;
use std::ops::Deref;
use aws_sdk_ec2::model::Tag;
use aws_sdk_ec2::model::TagSpecification;
use aws_sdk_ec2::model::Filter;
use aws_sdk_ec2::operation::CreateVpc;
use aws_sdk_ec2::output::{CreateVpcOutput, DescribeVpcsOutput};
use aws_types::SdkConfig;
use crate::resource::{InfraCreatable, InfraResource};
use crate::types::InfraError;
use async_trait::async_trait;

pub struct VpcClient(aws_sdk_ec2::Client);

pub struct VpcResource{
    pub cidr : String,
    pub env : String,
    pub name : String
}

#[async_trait]
impl InfraResource<VpcClient> for VpcClient {
    type ResourceType = VpcResource;

    async fn init(config: &SdkConfig) -> Result<VpcClient, InfraError> {
        let s = aws_sdk_ec2::Client::new(config);
        Ok(VpcClient(s))
    }
}

#[async_trait]
impl InfraCreatable<VpcClient, CreateVpcOutput> for VpcClient {
    async fn created(&self, resource : &VpcResource) -> Result<bool,InfraError> {
        println!("Filter on tag:{:?}","name");
        let name_filter = Filter::builder()
            .set_name(Some(format!("tag:{}", "name")))
                .set_values(Some(vec![resource.name.clone()])).build();
        let env_filter = Filter::builder()
            .set_name(Some(format!("tag:{}", "env")))
            .set_values(Some(vec![resource.env.clone()])).build();
        let result = self.0.describe_vpcs().set_filters(Some(vec![name_filter, env_filter])).send().await;
        match result {
            Ok(DescribeVpcsOutput{ vpcs, next_token,.. }) => Ok(vpcs.is_some() && vpcs.unwrap().len() == 1),
            Err(e) => panic!("Error: {:?}", e)
        }
    }

    async fn create(&self, resource : &VpcResource) -> Result<CreateVpcOutput,InfraError> {
        todo!()
    }
}

impl VpcClient {
    pub async fn new(config : &SdkConfig) -> Result<VpcClient, InfraError> {
        let s = aws_sdk_ec2::Client::new(config);
        Ok(VpcClient(s))
    }
    pub async fn create_vpc(&self, cidr: String, env : String, name : String) -> Result<(), InfraError> {
        let out =
            self.0.create_vpc().set_cidr_block(Some(cidr))
                .set_amazon_provided_ipv6_cidr_block(Some(true))
                .set_tag_specifications(Some(vec![
                    TagSpecification::builder()
                        .set_resource_type(Some(aws_sdk_ec2::model::ResourceType::Vpc))
                        .set_tags(Some(vec![
                            Tag::builder().set_key(Some("name".to_string())).set_value(Some(name)).build(),
                            Tag::builder().set_key(Some("env".to_string())).set_value(Some(env)).build()
                        ]))
                        .build()
                ])).send().await;
        println!("Create VPC output: {:?}", out);
        Ok(())
    }

}
