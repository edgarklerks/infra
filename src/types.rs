use std::error::Error;
use std::fmt::{Display, Formatter, Write};
use crate::types::InfraError::{CannotCreateClient, CannotCreateHostedZone};

#[derive(Debug, Clone)]
pub enum InfraError {
    CannotCreateClient(String),
    CannotCreateHostedZone(String)
}

impl Error for InfraError {
    
}

impl Display for InfraError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CannotCreateClient(g) => {
                f.write_str("CannotCreateClient(");
                f.write_str(g.as_str());
                f.write_char(')');
                f.pad(" ")
            }
            CannotCreateHostedZone(g) => {
                f.write_str("CannotCreateHostedZone(");
                f.write_str(g.as_str());
                f.write_char(')');
                f.pad(" ")

            }
        }
    }
}
