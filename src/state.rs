use cw_storage_plus::{Index, IndexList, IndexedMap, MultiIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

use crate::msg::Namespace;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin_address: Addr,
    pub namespace: Option<Namespace>,
}

pub const CONFIG: Item<Config> = Item::new("config");

/// This is the data stored against the registered IBC denom - just used for indexing for now
/// it can be expanded in future
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IBCDenomInfo {
    /// base denom identifier e.g. juno. Probably it will be the Bech32 HRP
    pub base_denom: String,
}

/// A mapping of IBC denom -> Denom info, where in denom info there is a secondary index
/// allowing one to look up the base denom
/// i.e.
/// ibc/DENOMHASH -> { base_denom: "juno" }
/// ibc/DENOMHASH_TWO -> { base_denom: "juno" }
pub struct IBCTokenIndexes<'a> {
    pub base_denom: MultiIndex<'a, String, IBCDenomInfo, String>,
}

impl<'a> IndexList<IBCDenomInfo> for IBCTokenIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<IBCDenomInfo>> + '_> {
        let v: Vec<&dyn Index<IBCDenomInfo>> = vec![&self.base_denom];
        Box::new(v.into_iter())
    }
}

pub fn ibc_tokens<'a>() -> IndexedMap<'a, &'a str, IBCDenomInfo, IBCTokenIndexes<'a>> {
    let indexes = IBCTokenIndexes {
        base_denom: MultiIndex::new(
            |d: &IBCDenomInfo| d.base_denom.clone(),
            "ibc_tokens",
            "ibc_tokens__base_denom",
        ),
    };
    IndexedMap::new("ibc_tokens", indexes)
}
