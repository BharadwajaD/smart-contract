use cosmwasm_std::Addr;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstantiateMessage{
    pub admins : Vec<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryMessage{
    Greet {},
    AdminsList {},
}

#[derive(Debug,PartialEq, Eq, Serialize, Deserialize)]
pub struct GreetResp{
    pub message: String
}

#[derive(Debug,PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminListResp{
    pub admins: Vec<Addr>,
}

#[derive(Clone, Debug,PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecuteMessage{
    AddMember {admin: String},
    Leave {},
}
