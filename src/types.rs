use std::error::Error;
use std::fmt::{Display, Formatter, Write};
use crate::InfraError::{CannotCreateResource, CannotDescribeResource, MultipleResultsReturned};
use crate::types::InfraError::{CannotCreateClient};

#[derive(Debug, Clone)]
pub enum InfraError {
    CannotCreateClient(String),
    CannotCreateResource(String),
    MultipleResultsReturned(String),
    CannotDescribeResource(String)
}


pub fn unwrap_with_infra_error<T>(r : Result<T, InfraError>) -> T{
    match r {
        Err(e) => panic!("{:?}", e),
        Ok(t) => t
    }
}

impl Error for InfraError {
    
}

impl Display for InfraError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CannotCreateClient(g) => {
                f.write_str("CannotCreateClient(")?;
                f.write_str(g.as_str())?;
                f.write_char(')')?;
                f.pad(" ")
            },
            CannotCreateResource(g) => {
                f.write_str("CannotCreateResource(")?;
                f.write_str(g.as_str())?;
                f.write_char(')')?;
                f.pad(" ")
            },
            CannotDescribeResource(g) =>{
               f.write_str("CannotDescribeResource(")?;
                f.write_str(g.as_str())?;
                f.write_char(')')?;
                f.pad("")

            },
            MultipleResultsReturned(g ) => {
                f.write_str("MultipleResultsReturned(")?;
                f.write_str(g.as_str())?;
                f.write_char(')')?;
                f.pad(" ")
            }
        }
    }
}
