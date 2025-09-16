pub mod instructions;
pub use instructions::*;

declare_id!("GQAkDF5S91CSJSQHDbsZDoWqtzQdeDguN8Dp94eMcJ6q");

#[program]
pub mod random_tests {
    use super::*;

    pub fn test_ser_de_compute(ctx: Context<TestSerDeComputeAccounts>) -> Result<()> {
        test_ser_de_compute_handler(ctx)
    }

    #[allow(unreachable_code)]
    pub fn test_allocation(ctx: Context<TestAllocationAccounts>, size:usize) -> Result<()>{
        test_allocation_handler(ctx, size);

        Ok(())
    }

    #[allow(unreachable_code)]
    pub fn test_loop(ctx:Context<TestLoopAccounts>) -> Result<()>{
        test_loop_handler(ctx);

        Ok(())
    }

    #[allow(unreachable_code)]
    pub fn test_loop_with_log(ctx:Context<TestLoopWithLogAccounts>) -> Result<()>{
        test_loop_with_log_handler(ctx);

        Ok(())
    }

    pub fn test_print_transaction(ctx:Context<TestPrintTransactionAccounts>) -> Result<()>{
        test_print_transaction_handler(ctx)
    }

    #[allow(unreachable_code)]
    pub fn test_function_recurse(ctx:Context<TestFunctionRecurseAccounts>) -> Result<()>{
        test_function_recurse_handler(ctx);

        Ok(())
    }

    #[allow(unreachable_code)]
    pub fn test_function_recurse_with_log(ctx:Context<TestFunctionRecurseWithLogAccounts>) -> Result<()>{
        test_function_recurse_with_log_handler(ctx);

        Ok(())
    }
}
