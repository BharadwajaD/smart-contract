use cosmwasm_std::{
    Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult, to_json_binary,
};

use crate::message::{self, QueryMessage};


pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}

pub fn query(
    _deps: Deps,
    _env: Env,
    msg: QueryMessage
) -> StdResult<Binary> {
    return match msg{
        QueryMessage::Greet {  } => 
        {
            to_json_binary(&message::GreetResp{message: String::from("Hello World")})
        }
    }
}

#[allow(dead_code)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty
) -> StdResult<Response> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, ContractWrapper, Executor};

    use crate::message::GreetResp;

    use super::*;

    #[test]
    fn greet_query() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &Empty {},
                &[],
                "Contract",
                None,
            )
            .unwrap();

        let resp: GreetResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMessage::Greet {})
            .unwrap();

        assert_eq!(
            resp,
            GreetResp{message: "Hello World".to_owned()}
        );
    }
}
