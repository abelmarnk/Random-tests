use anchor_lang::{
    prelude::*,
    solana_program::log::sol_log_compute_units
};

#[allow(unconditional_recursion)]
pub fn test_function_recurse_with_log_handler(ctx:Context<TestFunctionRecurseWithLogAccounts>)-> ! {
    sol_log_compute_units();
    test_function_recurse_with_log_handler(ctx)
}


#[derive(Accounts)]
pub struct TestFunctionRecurseWithLogAccounts{

}