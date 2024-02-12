use serde::{Deserialize, Serialize};
use shrinkwraprs::Shrinkwrap;

#[derive(Shrinkwrap, Serialize, Deserialize, Debug, Clone)]
pub struct Username(String);

impl Username {
    pub fn parse(s: String) -> Result<Self, String> {
        Ok(Self(s))
    }
}
