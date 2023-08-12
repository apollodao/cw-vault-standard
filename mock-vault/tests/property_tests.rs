use cosmwasm_std::coin;
use cosmwasm_std::Decimal;
use cw_it::test_tube::Account;
use cw_it::traits::CwItRunner;
use cw_mock_vault::test_helpers;
use cw_vault_standard_test_helpers::traits::CwVaultStandardRobot;
use proptest::prelude::*;
use proptest::proptest;

proptest! {
        #![proptest_config(ProptestConfig {
            cases: 64,
            max_local_rejects: 100000,
            max_global_rejects: 100000,
            max_shrink_iters: 512,
            ..ProptestConfig::default()
        })]

    /// # Property: Subsequent deposits without rewards accruing in between should receive the same
    /// ratio of vault tokens
    #[test]
    fn subsequent_deposits_receive_same_ratio_of_vault_tokens(
        init_amount in prop_oneof![Just(0u128), 100..1000000000u128].prop_flat_map(Just),
        amount1 in 100..1000000000u128,
        amount2 in 100..1000000000u128,
    ) {
        let runner = test_helpers::get_test_runner();
        let accs = runner.init_default_accounts().unwrap();
        let admin = &accs[0];
        let user1 = &accs[1];
        let user2 = &accs[2];
        let base_token = "uosmo";
        let robot = test_helpers::MockVaultRobot::instantiate(&runner, admin, base_token, Some(coin(10000000, "uosmo")));

        if init_amount != 0 {
            robot
            .deposit(init_amount, None, &[coin(init_amount, base_token)],  &admin);
        }

        robot
            .deposit(amount1, None, &[coin(amount1, base_token)], user1)
            .deposit(amount2, None, &[coin(amount2, base_token)], user2);

        let user1_vault_token_balance = robot.query_vault_token_balance(user1.address());
        let user2_vault_token_balance = robot.query_vault_token_balance(user2.address());
        let user1_ratio = Decimal::from_ratio(user1_vault_token_balance, amount1);
        let user2_ratio = Decimal::from_ratio(user2_vault_token_balance, amount2);
        test_helpers::assert_almost_eq(user1_ratio, user2_ratio, "0.00001");
    }
}
