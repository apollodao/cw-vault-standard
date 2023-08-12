use std::str::FromStr;

use cosmwasm_std::{coin, Coin, Decimal, Uint128};
use cw_it::cw_multi_test::{ContractWrapper, StargateKeeper, StargateMessageHandler};
use cw_it::multi_test::modules::TokenFactory;
use cw_it::multi_test::MultiTestRunner;
use cw_it::robot::TestRobot;
use cw_it::test_tube::{Account, Module, SigningAccount, Wasm};
use cw_it::traits::CwItRunner;
use cw_it::{Artifact, ContractType, TestRunner};
use cw_vault_standard_test_helpers::traits::CwVaultStandardRobot;

pub const MOCK_VAULT_TOKEN_SUBDENOM: &str = "vault-token";

const TOKEN_FACTORY: &TokenFactory =
    &TokenFactory::new("factory", 32, 16, 59 + 16, "10000000uosmo");

pub fn get_test_runner<'a>() -> TestRunner<'a> {
    match option_env!("TEST_RUNNER_TYPE").unwrap_or("multi-test") {
        "multi-test" => {
            let mut stargate_keeper = StargateKeeper::new();
            TOKEN_FACTORY.register_msgs(&mut stargate_keeper);

            TestRunner::MultiTest(MultiTestRunner::new_with_stargate("osmo", stargate_keeper))
        }
        #[cfg(feature = "osmosis-test-tube")]
        "osmosis-test-app" => {
            TestRunner::OsmosisTestApp(cw_it::osmosis_test_tube::OsmosisTestApp::new())
        }
        _ => panic!("Unsupported test runner type"),
    }
}

pub const DEFAULT_ARTIFACTS_DIR: &str = "../../artifacts/";

pub fn get_wasm_path(contract_name: &str) -> String {
    let artifacts_dir = option_env!("ARTIFACTS_DIR").unwrap_or(DEFAULT_ARTIFACTS_DIR);

    let mut path = format!("{}/{}", artifacts_dir, contract_name.replace('-', "_"));

    // If path doesn't exist, try appending the CPU architecture
    if !std::path::Path::new(&format!("{}.wasm", path)).exists() {
        path = format!("{}-{}", path, std::env::consts::ARCH);
    }

    format!("{}.wasm", path)
}

pub fn get_mock_vault_contract() -> ContractType {
    match option_env!("TEST_RUNNER_TYPE").unwrap_or("multi-test") {
        "multi-test" => ContractType::MultiTestContract(Box::new(ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        ))),
        _ => ContractType::Artifact(Artifact::Local(get_wasm_path("mock-vault"))),
    }
}

pub fn assert_almost_eq(left: Decimal, right: Decimal, max_rel_diff: &str) {
    let max_rel_diff = Decimal::from_str(max_rel_diff).unwrap();

    let largest = std::cmp::max(left, right);
    let rel_diff = left.abs_diff(right) / largest;

    if rel_diff > max_rel_diff {
        panic!(
            "assertion failed: `(left ≈ right)`\nleft: {}\nright: {}\nrelative difference: {}\nmax allowed relative difference: {}\n",
            left, right, rel_diff, max_rel_diff
        )
    }
}

/// A simple testing robot for testing vault contracts.
pub struct MockVaultRobot<'a, R: CwItRunner<'a>> {
    pub runner: &'a R,
    pub admin: &'a SigningAccount,
    pub vault_addr: String,
}

impl<'a, R: CwItRunner<'a>> CwVaultStandardRobot<'a, R> for MockVaultRobot<'a, R> {
    fn vault_addr(&self) -> String {
        self.vault_addr.clone()
    }

    fn query_base_token_balance(&self, address: impl Into<String>) -> Uint128 {
        let base_token_denom = self.base_token();
        self.query_native_token_balance(address, base_token_denom)
    }
}

impl<'a, R: CwItRunner<'a>> MockVaultRobot<'a, R> {
    /// Uploads and instantiates the vault contract and returns a new instance of the robot.
    pub fn instantiate(
        runner: &'a R,
        admin: &'a SigningAccount,
        base_token: &str,
        denom_creation_fee: Option<Coin>,
    ) -> MockVaultRobot<'a, R>
    where
        Self: Sized,
    {
        let wasm = Wasm::new(runner);

        let mock_vault = get_mock_vault_contract();
        let code_id = runner.store_code(mock_vault, admin).unwrap();

        let msg = crate::msg::InstantiateMsg {
            base_token: base_token.to_string(),
        };
        let vault_addr = wasm
            .instantiate(
                code_id,
                &msg,
                Some(&admin.address()),
                Some("mock_vault"),
                &denom_creation_fee.map_or_else(|| vec![], |f| vec![f]),
                admin,
            )
            .unwrap()
            .data
            .address;

        MockVaultRobot {
            runner,
            admin,
            vault_addr,
        }
    }
}

impl<'a, R: CwItRunner<'a>> TestRobot<'a, R> for MockVaultRobot<'a, R> {
    fn runner(&self) -> &'a R {
        self.runner
    }
}
