#[cfg(test)]
mod tests {
    use crate::msg::InstantiateMsg;
    use crate::msg::{
        ExecuteMsg, GetBaseDenomResponse, GetIbcDenomsResponse, IsEqualResponse, PaginationArgs,
        QueryMsg,
    };
    use cosmwasm_std::{Addr, Coin, Empty, StdResult, Uint128};
    use cw_multi_test::{App, AppBuilder, AppResponse, Contract, ContractWrapper, Executor};

    pub fn contract_template() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    const USER: &str = "jeff-vader"; // not admin
    const ADMIN: &str = "ADMIN"; // contract admin via --admin flag
    const NATIVE_DENOM: &str = "denom";
    const ADMIN_ADDR: &str = "contractaddr0"; // e.g. dao/multisig

    fn mock_app() -> App {
        AppBuilder::new().build(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(USER),
                    vec![Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(1),
                    }],
                )
                .unwrap();
        })
    }

    fn proper_instantiate() -> (App, Addr) {
        let mut app = mock_app();
        let cw_template_id = app.store_code(contract_template());

        let msg = InstantiateMsg {
            admin_address: ADMIN_ADDR.to_string(),
            namespace: None,
        };
        let contract_addr = app
            .instantiate_contract(
                cw_template_id,
                Addr::unchecked(ADMIN),
                &msg,
                &[],
                "test",
                None,
            )
            .unwrap();

        (app, contract_addr)
    }

    fn add_ibc_denom(
        app: &mut App,
        contract_addr: &Addr,
        sender: &str,
        ibc_denom: &str,
        base_denom: &str,
    ) -> anyhow::Result<AppResponse> {
        let msg = ExecuteMsg::AddIbcDenom {
            ibc_denom: ibc_denom.to_string(),
            base_denom: base_denom.to_string(),
        };

        app.execute_contract(Addr::unchecked(sender), contract_addr.clone(), &msg, &[])
    }

    fn remove_ibc_denom(
        app: &mut App,
        contract_addr: &Addr,
        sender: &str,
        ibc_denom: &str,
    ) -> anyhow::Result<AppResponse> {
        let msg = ExecuteMsg::RemoveIbcDenom {
            ibc_denom: ibc_denom.to_string(),
        };

        app.execute_contract(Addr::unchecked(sender), contract_addr.clone(), &msg, &[])
    }

    fn query_ibc_denoms(
        app: &mut App,
        contract_addr: &Addr,
        base_denom: &str,
        pagination_args: Option<PaginationArgs>,
    ) -> StdResult<GetIbcDenomsResponse> {
        let denom_query = QueryMsg::GetIbcDenoms {
            base_denom: base_denom.to_string(),
            pagination_args,
        };

        app.wrap().query_wasm_smart(contract_addr, &denom_query)
    }

    fn query_base_denom(
        app: &mut App,
        contract_addr: &Addr,
        ibc_denom: &str,
    ) -> StdResult<GetBaseDenomResponse> {
        let denom_query = QueryMsg::GetBaseDenom {
            ibc_denom: ibc_denom.to_string(),
        };

        app.wrap().query_wasm_smart(contract_addr, &denom_query)
    }

    fn query_equality(
        app: &mut App,
        contract_addr: &Addr,
        denom_one: &str,
        denom_two: &str,
    ) -> StdResult<IsEqualResponse> {
        let denom_query = QueryMsg::IsEqual {
            ibc_denom_one: denom_one.to_string(),
            ibc_denom_two: denom_two.to_string(),
        };

        app.wrap().query_wasm_smart(contract_addr, &denom_query)
    }

    mod basic_functionality {
        use super::*;

        #[test]
        fn non_admin_cant_add() {
            let (mut app, contract_addr) = proper_instantiate();

            add_ibc_denom(&mut app, &contract_addr, USER, "ibc/yolo", "juno").unwrap_err();
        }

        #[test]
        fn add_several_denoms() {
            let (mut app, contract_addr) = proper_instantiate();

            add_ibc_denom(&mut app, &contract_addr, ADMIN_ADDR, "ibc/yolo", "juno").unwrap();
            add_ibc_denom(&mut app, &contract_addr, ADMIN_ADDR, "ibc/wagmi", "juno").unwrap();

            let ibc_denoms = query_ibc_denoms(&mut app, &contract_addr, "juno", None).unwrap();
            assert_eq!(ibc_denoms.ibc_denoms, vec!["ibc/yolo", "ibc/wagmi"]);
        }

        #[test]
        fn add_several_denoms_then_remove() {
            let (mut app, contract_addr) = proper_instantiate();

            add_ibc_denom(&mut app, &contract_addr, ADMIN_ADDR, "ibc/yolo", "juno").unwrap();
            add_ibc_denom(&mut app, &contract_addr, ADMIN_ADDR, "ibc/wagmi", "juno").unwrap();

            remove_ibc_denom(&mut app, &contract_addr, ADMIN_ADDR, "ibc/yolo").unwrap();

            let ibc_denoms = query_ibc_denoms(&mut app, &contract_addr, "juno", None).unwrap();
            assert_eq!(ibc_denoms.ibc_denoms, vec!["ibc/wagmi"]);
        }

        #[test]
        fn non_admin_cant_remove() {
            let (mut app, contract_addr) = proper_instantiate();

            add_ibc_denom(&mut app, &contract_addr, ADMIN_ADDR, "ibc/yolo", "juno").unwrap();
            add_ibc_denom(&mut app, &contract_addr, ADMIN_ADDR, "ibc/wagmi", "juno").unwrap();

            remove_ibc_denom(&mut app, &contract_addr, USER, "ibc/yolo").unwrap_err();

            let ibc_denoms = query_ibc_denoms(&mut app, &contract_addr, "juno", None).unwrap();
            assert_eq!(ibc_denoms.ibc_denoms, vec!["ibc/yolo", "ibc/wagmi"]);
        }

        #[test]
        fn get_denom_check() {
            let (mut app, contract_addr) = proper_instantiate();

            add_ibc_denom(&mut app, &contract_addr, ADMIN_ADDR, "ibc/yolo", "juno").unwrap();
            add_ibc_denom(&mut app, &contract_addr, ADMIN_ADDR, "ibc/wagmi", "juno").unwrap();

            let base_denom_one = query_base_denom(&mut app, &contract_addr, "ibc/yolo").unwrap();
            let base_denom_two = query_base_denom(&mut app, &contract_addr, "ibc/wagmi").unwrap();

            assert_eq!(base_denom_one, base_denom_two);
            assert_eq!(base_denom_one.base_denom, "juno");
        }

        #[test]
        #[allow(clippy::bool_assert_comparison)]
        fn equality_check() {
            let (mut app, contract_addr) = proper_instantiate();

            add_ibc_denom(&mut app, &contract_addr, ADMIN_ADDR, "ibc/yolo", "juno").unwrap();
            add_ibc_denom(&mut app, &contract_addr, ADMIN_ADDR, "ibc/wagmi", "juno").unwrap();

            let equal_response =
                query_equality(&mut app, &contract_addr, "ibc/yolo", "ibc/wagmi").unwrap();
            assert_eq!(equal_response.is_equal, true);
        }
    }
}
