use anchor_lang::{
    prelude::*,
    solana_program::log::sol_log_compute_units
};

pub fn test_loop_with_log_handler(_:Context<TestLoopWithLogAccounts>)-> ! {
    loop{
        sol_log_compute_units();
    }
}


#[derive(Accounts)]
pub struct TestLoopWithLogAccounts{

}