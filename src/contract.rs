use cosmwasm_std::entry_point;
use cw2::set_contract_version;

use crate::execute::{add_ibc_denom, remove_ibc_denom};
use crate::query::{get_base_denom, get_ibc_denoms, is_equal};

use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SudoMsg};
use crate::state::{Config, CONFIG};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:ibc-nameservice";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let validated_address = deps.api.addr_validate(&msg.admin_address)?;

    let config = Config {
        admin_address: validated_address,
        namespace: msg.namespace,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("sender", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, _env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::AddIbcDenom {
            base_denom,
            ibc_denom,
        } => add_ibc_denom(deps, base_denom, ibc_denom),
        SudoMsg::RemoveIbcDenom { ibc_denom } => remove_ibc_denom(deps, ibc_denom),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Err(ContractError::Unauthorized {})
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetIbcDenoms {
            base_denom,
            pagination_args,
        } => to_binary(&get_ibc_denoms(deps, base_denom, pagination_args)?),
        QueryMsg::GetBaseDenom { ibc_denom } => to_binary(&get_base_denom(deps, ibc_denom)?),
        QueryMsg::IsEqual {
            ibc_denom_one,
            ibc_denom_two,
        } => to_binary(&is_equal(deps, ibc_denom_one, ibc_denom_two)?),
    }
}
