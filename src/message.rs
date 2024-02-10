use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryMessage{
    Greet {}
}

#[derive(Debug,PartialEq, Eq, Serialize, Deserialize)]
pub struct GreetResp{
    pub message: String
}
