pub mod message;
pub mod contract;

use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
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
