use cosmwasm_std::{
    Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_json_binary,
};

use crate::{message::{self, QueryMessage, InstantiateMessage, ExecuteMessage}, state::{ADMINS, add_admin, remove_admin}, error::ContractError};


pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMessage,
) -> StdResult<Response> {
    let admins: StdResult<Vec<_>> = msg.admins.into_iter()
        .map(|addr| deps.api.addr_validate(&addr))
        .collect();

    ADMINS.save(deps.storage, &admins?)?;
    return Ok(Response::new());
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
        QueryMessage::AdminsList {  } => {
            let admins = ADMINS.load(_deps.storage)?;
            to_json_binary(&message::AdminListResp{admins})
        }
    }
}

pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMessage
) -> Result<Response, ContractError> {

    match msg {
        ExecuteMessage::AddMember { admin } => {
            let admin = deps.api.addr_validate(&admin)?;
            add_admin(deps.storage, info.sender, admin)?
        }
        ExecuteMessage::Leave {  } => {
            remove_admin(deps.storage, info.sender)?
        }
    }
    return Ok(Response::new());
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, ContractWrapper, Executor};

    use crate::message::{GreetResp, AdminListResp};

    use super::*;

    //TODO: Creating new smart contract for each state
    //instead create a global contract and use it for all tests
    struct ContractApp{
        app: App, 
        addr: Addr
    }

    impl ContractApp {
        fn new() -> Self{

            let mut app = App::default();

            let code = ContractWrapper::new(execute, instantiate, query);
            let code_id = app.store_code(Box::new(code));
            let addr = app
                .instantiate_contract(
                    code_id,
                    Addr::unchecked("owner"),
                    &InstantiateMessage {admins: vec!["admin1".to_owned(), "admin2".to_owned()]},
                    &[],
                    "Contract",
                    None,
                )
                .unwrap();

            return Self{
                app, 
                addr
            }

        }
    }


    #[test]
    fn greet_query() {
        let contract_test = ContractApp::new();
        let app = contract_test.app;
        let addr = contract_test.addr;
        let resp: GreetResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMessage::Greet {})
            .unwrap();

        assert_eq!(
            resp,
            GreetResp{message: "Hello World".to_owned()}
        );
    }

    #[test]
    fn admins_query(){
        let contract_test = ContractApp::new();
        let app = contract_test.app;
        let addr = contract_test.addr;

        let resp: AdminListResp = app.wrap()
            .query_wasm_smart(&addr, &QueryMessage::AdminsList {  })
            .unwrap();

        assert_eq!(
            resp, 
            AdminListResp{admins: vec![Addr::unchecked("admin1"), Addr::unchecked("admin2")]}
            );
    }

    #[test]
    fn admins_execute() {

        let contract_test = ContractApp::new();
        let mut app = contract_test.app;
        let addr = contract_test.addr;
        //add admins
        app.execute_contract(
            Addr::unchecked("admin1"), addr.clone(), &ExecuteMessage::AddMember { admin:  "admin3".to_owned()}, &[]
            ).unwrap();

        let resp: AdminListResp = app.wrap()
            .query_wasm_smart(&addr, &QueryMessage::AdminsList {  })
            .unwrap();

        assert_eq!(
            resp, 
            AdminListResp{admins: vec![Addr::unchecked("admin1"), Addr::unchecked("admin2"), Addr::unchecked("admin3")]}
            );

        //leave as admin
        app.execute_contract(
            Addr::unchecked("admin1"), addr.clone(), &ExecuteMessage::Leave {  }, &[]
            ).unwrap();

        let resp: AdminListResp = app.wrap()
            .query_wasm_smart(&addr, &QueryMessage::AdminsList {  })
            .unwrap();

        assert_eq!(
            resp, 
            AdminListResp{admins: vec![Addr::unchecked("admin2"), Addr::unchecked("admin3")]}
            );

        //unauthorized error
        let err = app.execute_contract(
            Addr::unchecked("user"), addr.clone(), &ExecuteMessage::AddMember { admin: "admin4".to_owned() } , &[]
            ).unwrap_err();

        assert_eq!(
            ContractError::Unauthorized { sender: Addr::unchecked("user") },
            err.downcast().unwrap()
            );

    }
}
