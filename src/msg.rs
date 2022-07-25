use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Eq)]
pub struct PaginationArgs {
    pub start_after: Option<String>,
    pub limit: Option<u32>,
    pub page: Option<u32>,
    pub order: Direction,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Asc,
    Desc,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Namespace {
    /// This is an optional whoami namespace to use
    /// if set, records will be generated on the target
    /// contract as well
    pub base_namespace: String,
    /// The target whoami contract
    /// needs to be set if base namespace is set
    pub whoami_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// This is the address, DAO or multisig
    /// that can add new entries
    pub admin_address: String,
    /// Optional namespace arguments
    /// not yet used for anything
    pub namespace: Option<Namespace>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SudoMsg {
    AddIbcDenom {
        ibc_denom: String,
        base_denom: String,
    },
    RemoveIbcDenom {
        ibc_denom: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Takes a short base_denom and returns all known IBC variants
    GetIbcDenoms {
        base_denom: String, // e.g. pass in 'juno'
        pagination_args: Option<PaginationArgs>,
    },
    /// Takes an IBC variant and returns the short base_denom
    GetBaseDenom {
        ibc_denom: String, // e.g. a voucher token like ibc/DENOMHASH
    },
    /// Takes two IBC denoms and returns if they are equal
    IsEqual {
        ibc_denom_one: String,
        ibc_denom_two: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetIbcDenomsResponse {
    pub ibc_denoms: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetBaseDenomResponse {
    pub base_denom: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IsEqualResponse {
    pub is_equal: bool,
}
