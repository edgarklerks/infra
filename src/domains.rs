use aws_sdk_route53::model::{HostedZoneConfig, Vpc};
use aws_sdk_route53::output::CreateHostedZoneOutput;
use aws_types::SdkConfig;
use crate::InfraError::CannotCreateResource;
use crate::types::InfraError;

pub struct DomainClient(aws_sdk_route53::Client);

impl DomainClient {
    async fn new(config : SdkConfig) -> Result<DomainClient,InfraError> {
           let route53 = aws_sdk_route53::Client::new(&config);
            Ok(DomainClient(route53))
    }
    async fn create_hosted_zone(&self, name : String, vpc : Vpc) -> Result<CreateHostedZoneOutput, InfraError> {
        let zone_config = HostedZoneConfig::builder()
            .set_comment(Some(format!("Hosted zone for {:?}",name)))
            .build();
        let result = self.0.create_hosted_zone().set_name(Some(name))
            .set_hosted_zone_config(Some(zone_config))
            .set_vpc(Some(vpc)).send().await;
        match result {
            Err(e)=> Err(CannotCreateResource(e.to_string())),

            Ok(s)=> Ok(s)
        }
    }

}
