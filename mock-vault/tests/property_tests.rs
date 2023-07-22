use cosmwasm_std::Decimal;
use cw_it::robot::TestRobot;
use cw_it::test_tube::Account;
use cw_it::traits::CwItRunner;
use proptest::prelude::*;
use proptest::proptest;

mod test_helpers;

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
        let robot = test_helpers::VaultRobot::new(&runner, admin, "uosmo");

        if init_amount != 0 {
            robot
            .send_native_tokens(admin, &robot.vault_addr, init_amount, "uosmo")
            .deposit_to_vault(init_amount, admin);
        }

        robot
            .deposit_to_vault(amount1, user1)
            .deposit_to_vault(amount2, user2);

        let user1_vault_token_balance = robot.query_vault_token_balance(user1.address());
        let user2_vault_token_balance = robot.query_vault_token_balance(user2.address());
        let user1_ratio = Decimal::from_ratio(user1_vault_token_balance, amount1);
        let user2_ratio = Decimal::from_ratio(user2_vault_token_balance, amount2);
        test_helpers::assert_almost_eq(user1_ratio, user2_ratio, "0.00001");
    }
}
