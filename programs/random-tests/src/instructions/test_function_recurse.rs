use anchor_lang::{
    prelude::*
};

#[allow(unconditional_recursion)]
pub fn test_function_recurse_handler(ctx:Context<TestFunctionRecurseAccounts>)-> ! {
    test_function_recurse_handler(ctx)
}


#[derive(Accounts)]
pub struct TestFunctionRecurseAccounts{

}