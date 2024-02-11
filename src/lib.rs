pub mod message;
pub mod contract;
pub mod state;
pub mod error;

use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use error::ContractError;
use message::{InstantiateMessage, ExecuteMessage};


#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMessage,
) -> StdResult<Response> {
    return contract::instantiate(_deps, _env, _info, _msg);
}

#[entry_point]
pub fn query(
    _deps: Deps,
    _env: Env,
    _msg: message::QueryMessage,
) -> StdResult<Binary> {
    return contract::query(_deps, _env, _msg);
}

#[entry_point]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMessage
) -> Result<Response, ContractError> {
    return contract::execute(_deps, _env, _info, msg);
}
