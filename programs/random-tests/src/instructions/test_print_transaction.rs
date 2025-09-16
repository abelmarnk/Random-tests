pub use anchor_lang::{
    prelude::*,
    solana_program::sysvar::instructions::{
        ID as INSTRUCTION_SYSVAR_ADDRESS,
        load_instruction_at_checked
    }
};

pub fn test_print_transaction_handler(ctx:Context<TestPrintTransactionAccounts>)->Result<()>{
    let instructions_sysvar_account = &ctx.accounts.instruction_sysvar_account;

    let instruction_count:[u8;2] = instructions_sysvar_account.try_borrow_data().
        unwrap()[..2].try_into().unwrap(); // Just unwrap, give no errors for now.

    let instruction_count = u16::from_le_bytes(instruction_count);

    msg!("Printing transaction......\n\n");

    for index in 0..usize::from(instruction_count){
        msg!("Instruction - {}:\n\n", index);
        let instruction = load_instruction_at_checked(index,
            &instructions_sysvar_account).
            unwrap(); // Just unwrap give no errors for now.
        
        msg!("Program id: {:?}\n\n\n", instruction.program_id);

        msg!("Accounts: \n\n");
        for account_meta in &instruction.accounts{
            msg!("Key: {:?}\n\n", account_meta.pubkey);
            msg!("Is signer: {:?}\n\n", account_meta.is_signer);
            msg!("Is writable: {:?}\n\n", account_meta.is_writable);
        }
        msg!("\n");

        msg!("Data: {:?}", instruction.data);
    }


    Ok(())
}

#[derive(Accounts)]
pub struct TestPrintTransactionAccounts<'info>{
    #[account(
        address = INSTRUCTION_SYSVAR_ADDRESS
    )]
    pub instruction_sysvar_account:UncheckedAccount<'info>
}