use aws_sdk_ec2::model::ResourceType;
use aws_sdk_ec2::model::Tag;
use aws_sdk_ec2::model::TagSpecification;
use aws_sdk_ec2::operation::CreateVpc;
use aws_types::SdkConfig;
use crate::types::InfraError;

pub struct VpcClient(aws_sdk_ec2::Client);

impl VpcClient {
    pub async fn new(config : &SdkConfig) -> Result<VpcClient, InfraError> {
        let s = aws_sdk_ec2::Client::new(config);
        Ok(VpcClient(s))
    }
    pub async fn exists_vpc(&self, name : String) -> Result<bool, InfraError> {
        unimplemented!()
    }
    pub async fn create_vpc(&self, cidr: String, env : String, name : String) -> Result<(), InfraError> {
        let out = self.0.create_vpc().set_cidr_block(Some(cidr))
            .set_amazon_provided_ipv6_cidr_block(Some(true))
            .set_tag_specifications(Some(vec![
                TagSpecification::builder()
                    .set_resource_type(Some(ResourceType::Vpc))
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
