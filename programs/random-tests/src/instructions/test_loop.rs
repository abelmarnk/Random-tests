use anchor_lang::{
    prelude::*
};

pub fn test_loop_handler(_:Context<TestLoopAccounts>)-> ! {
    loop{

    }
}


#[derive(Accounts)]
pub struct TestLoopAccounts{

}