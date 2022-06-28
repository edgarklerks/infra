use aws_config::profile::ProfileFileCredentialsProvider;
use aws_types::SdkConfig;
use log::info;
use crate::resource::{InfraCreatable, InfraResource};
use crate::types::{InfraError, unwrap_with_infra_error};
use crate::vpc::{VpcClient, VpcResource};

mod resource;
mod domains;
mod types;
mod vpc;

async fn create_config() -> SdkConfig {
    let config = aws_config::ConfigLoader::default().credentials_provider(ProfileFileCredentialsProvider::builder().profile_name("infra").build()).load().await;
    config

}

#[tokio::main]
async fn main() -> Result<(),InfraError> {
    pretty_env_logger::init();
    let config = create_config().await;
    let vpc_client = unwrap_with_infra_error(VpcClient::init(&config).await);
    let vpcProd = vpc_client.create(&VpcResource{
        cidr: "10.0.1.0/24".to_string(),
        env: "production".to_string(),
        name: "main".to_string()
    }).await;
    let vpcTest = vpc_client.create(&VpcResource{
        cidr: "10.0.7.0/24".to_string(),
        env: "test".to_string(),
        name: "main".to_string()

    }).await;

    info!("Result: {:?},{:?}", unwrap_with_infra_error(vpcProd), unwrap_with_infra_error(vpcTest));
    Ok(())


}
