use crate::state::{ibc_tokens, IBCDenomInfo, CONFIG};
use crate::ContractError;
use cosmwasm_std::{ensure_eq, DepsMut, MessageInfo, Response};

pub fn add_ibc_denom(
    deps: DepsMut,
    info: MessageInfo,
    base_denom: String,
    ibc_denom: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let contract_admin = config.admin_address;

    ensure_eq!(info.sender, contract_admin, ContractError::Unauthorized {});

    ibc_tokens().update(deps.storage, &ibc_denom, |old| match old {
        Some(_) => Err(ContractError::AlreadyRegistered {}),
        None => Ok(IBCDenomInfo { base_denom }),
    })?;

    Ok(Response::new()
        .add_attribute("method", "add_ibc_denom")
        .add_attribute("added", ibc_denom))
}

pub fn remove_ibc_denom(
    deps: DepsMut,
    info: MessageInfo,
    ibc_denom: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let contract_admin = config.admin_address;

    ensure_eq!(info.sender, contract_admin, ContractError::Unauthorized {});

    ibc_tokens().remove(deps.storage, &ibc_denom)?;

    Ok(Response::new()
        .add_attribute("method", "remove_ibc_denom")
        .add_attribute("removed", ibc_denom))
}
