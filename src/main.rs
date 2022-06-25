use std::cell::RefCell;
use std::io::Read;
use std::ops::Deref;
use std::panic::panic_any;
use aws_config::meta::region::future::ProvideRegion;
use aws_config::meta::region::RegionProviderChain;
use aws_config::profile::ProfileFileCredentialsProvider;
use aws_config::provider_config::ProviderConfig;
use aws_sdk_route53::Error;
use aws_types::app_name::AppName;
use aws_types::region::Region;
use aws_types::SdkConfig;
use tokio::task;
use crate::types::InfraError;
use crate::vpc::VpcClient;


mod domains;
mod types;
mod vpc;

async fn create_config() -> SdkConfig {
    let config = aws_config::ConfigLoader::default().credentials_provider(ProfileFileCredentialsProvider::builder().profile_name("infra").build()).load().await;
    config

}

#[tokio::main]
async fn main() -> Result<(),InfraError> {
    let config = create_config().await;
    let vpc_client = VpcClient::new(&config).await.expect("Couldn't create vpc client");
    let t1 = vpc_client.create_vpc("10.0.1.0/24".to_string(),"production".to_string(),"main".to_string());
    let t2 = vpc_client.create_vpc("10.0.7.0/24".to_string(),"test".to_string(),"main".to_string());
    let (r1,r2) = tokio::join!(t1,t2);

    match r1.and_then(|s|  r2) {
        Ok(e) => Ok(()),
        Err(e) => panic_any(e)
    }

}
