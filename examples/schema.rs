use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use ibc_nameservice::msg::{
    ExecuteMsg, GetBaseDenomResponse, GetIbcDenomsResponse, InstantiateMsg, IsEqualResponse,
    Namespace, PaginationArgs, QueryMsg,
};
use ibc_nameservice::state::{Config, IBCDenomInfo};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(Config), &out_dir);
    export_schema(&schema_for!(IBCDenomInfo), &out_dir);
    export_schema(&schema_for!(GetIbcDenomsResponse), &out_dir);
    export_schema(&schema_for!(GetBaseDenomResponse), &out_dir);
    export_schema(&schema_for!(IsEqualResponse), &out_dir);
    export_schema(&schema_for!(Namespace), &out_dir);
    export_schema(&schema_for!(PaginationArgs), &out_dir);
}
