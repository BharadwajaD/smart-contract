use cosmwasm_std::{Addr, Storage};
use cw_storage_plus::Item;

use crate::error::ContractError;

pub const ADMINS: Item<Vec<Addr>> = Item::new("admins");

pub fn add_admin(storage: &mut dyn Storage, sender_addr: Addr, addr: Addr) -> Result<(), ContractError>{

    let mut admins = ADMINS.load(storage)?;
    if !admins.contains(&sender_addr) {
        return Err(ContractError::Unauthorized { sender: sender_addr })
    }
    admins.push(addr);
    ADMINS.save(storage, &admins)?;
    return Ok(());
}

pub fn remove_admin(storage: &mut dyn Storage, sender_addr: Addr) -> Result<(), ContractError>{
    let admins = ADMINS.load(storage)?;
    if !admins.contains(&sender_addr) {
        return Err(ContractError::Unauthorized { sender: sender_addr })
    }
    let admins = admins.into_iter().filter(|addr| *addr != sender_addr).collect();
    ADMINS.save(storage, &admins)?;
    return Ok(());
}
