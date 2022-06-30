use crate::msg::{
    Direction, GetBaseDenomResponse, GetIbcDenomsResponse, IsEqualResponse, PaginationArgs,
};
use crate::state::ibc_tokens;
use cosmwasm_std::{Deps, Order, StdResult};
use cw_storage_plus::Bound;

const DEFAULT_LIMIT: u32 = 10;
const MAX_LIMIT: u32 = 30;
const DEFAULT_PAGE: u32 = 0;

struct RangeQueryArgs {
    pub start_after: Option<String>,
    pub limit: usize,
    pub page: usize,
    pub order: Order,
}

fn unpack_pagination_args(pagination_args: Option<PaginationArgs>) -> RangeQueryArgs {
    let pargs = match pagination_args {
        Some(pa) => {
            let start_after = pa.start_after;
            let limit = pa.limit;
            let page = pa.page;
            let order = pa.order;

            PaginationArgs {
                start_after,
                limit,
                page,
                order,
            }
        }
        None => PaginationArgs {
            start_after: None,
            limit: None,
            page: None,
            order: Direction::Desc,
        },
    };

    let order_type = match pargs.order {
        Direction::Asc => Order::Ascending,
        Direction::Desc => Order::Descending,
    };

    let limit = pargs.limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

    RangeQueryArgs {
        start_after: pargs.start_after,
        limit,
        page: pargs.page.unwrap_or(DEFAULT_PAGE) as usize * limit,
        order: order_type,
    }
}

pub fn get_ibc_denoms(
    deps: Deps,
    base_denom: String,
    pagination_args: Option<PaginationArgs>,
) -> StdResult<GetIbcDenomsResponse> {
    let pa = unpack_pagination_args(pagination_args);

    let limit = pa.limit;
    let page = pa.page;
    let order = pa.order;

    let start_after = pa.start_after;
    let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));

    let ibc_denoms: Vec<String> = ibc_tokens()
        .idx
        .base_denom
        .prefix(base_denom)
        .range(deps.storage, start, None, order)
        .skip(page)
        .take(limit)
        .map(|item| item.map(|(key, _ibc_denom_info)| key))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(GetIbcDenomsResponse { ibc_denoms })
}

pub fn get_base_denom(deps: Deps, ibc_denom: String) -> StdResult<GetBaseDenomResponse> {
    let ibc_denom_info = ibc_tokens().load(deps.storage, &ibc_denom)?;
    Ok(GetBaseDenomResponse {
        base_denom: ibc_denom_info.base_denom,
    })
}

pub fn is_equal(
    deps: Deps,
    ibc_denom_one: String,
    ibc_denom_two: String,
) -> StdResult<IsEqualResponse> {
    let base_denom_one = ibc_tokens().load(deps.storage, &ibc_denom_one)?;
    let base_denom_two = ibc_tokens().load(deps.storage, &ibc_denom_two)?;
    Ok(IsEqualResponse {
        is_equal: (base_denom_one == base_denom_two),
    })
}
