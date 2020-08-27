use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Match {
    pub id: i32,
}

impl Match
{
     pub async fn list() -> Self {
         Match {
             id: 111,
         }
    }
}