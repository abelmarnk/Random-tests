use anchor_lang::{
    Discriminator, 
    InstructionData,
    solana_program::{
        sysvar::{
        instructions::ID as INSTRUCTION_SYSVAR_ADDRESS,
        ID as SYSVAR_ADDRESS
        },
        instruction::InstructionError
    }
};
use mollusk_svm::{
    Mollusk, result::Check
};
use solana_sdk::{
    account::Account, instruction::{
        AccountMeta, Instruction
    }
};

#[test]
pub fn allocation_test(){
    let mollusk = Mollusk::new(
        &random_tests::ID, "random_tests");

    let test_instruction =  Instruction{
        program_id:random_tests::ID,
        accounts:vec![],
        data:random_tests::instruction::TestAllocation{
            size:10
        }.data()
    };

    mollusk.process_and_validate_instruction(&test_instruction, &[], 
        &[Check::instruction_err(InstructionError::ProgramFailedToComplete)]);
}

#[test]
/// There seems to be one more step missing when executing this, the logs in the other variant
/// show that there are 61 calls and there is the call from the instruction handler, and the call 
/// from the part that distuinghes which instruction to call based on the instruction discriminator, 
/// making 63, rather than the 64 stated here:-
/// (https://github.com/anza-xyz/agave/blob/d5a84daebd2a7225684aa3f722b330e9d5381e76/compute-budget/src/compute_budget.rs#L186)
pub fn function_recurse_test(){
    let mut mollusk = Mollusk::new(
        &random_tests::ID, "random_tests");
    
    let test_instruction = Instruction{
        program_id:random_tests::ID,
        accounts:vec![],
        data:random_tests::instruction::TestFunctionRecurse::DISCRIMINATOR.to_vec()
    };

    mollusk.compute_budget.compute_unit_limit = 1_400_000;

    mollusk.process_and_validate_instruction(&test_instruction, &[],
        &[Check::instruction_err(InstructionError::ProgramFailedToComplete)]);
}

#[test]
/// There seems to be one more step missing when executing this, the logs show that there are 61 calls
/// and there is the call from the instruction handler, and the call from the part that distuinghes
/// which instruction to call based on the instruction discriminator, making 63, rather than the 64 stated
/// here:-(https://github.com/anza-xyz/agave/blob/d5a84daebd2a7225684aa3f722b330e9d5381e76/compute-budget/src/compute_budget.rs#L186)
pub fn function_recurse_with_log_test(){
    let mut mollusk = Mollusk::new(
        &random_tests::ID, "random_tests");

    let test_instruction = Instruction{
        program_id:random_tests::ID,
        accounts:vec![],
        data:random_tests::instruction::TestFunctionRecurseWithLog::DISCRIMINATOR.to_vec()
    };

    mollusk.compute_budget.compute_unit_limit = 1_400_000;

    mollusk.process_and_validate_instruction(&test_instruction, &[], 
        &[Check::instruction_err(InstructionError::ProgramFailedToComplete)]);
}


#[test]
/// Why does this result in a different error from the other variant?
pub fn loop_test(){
    let mut mollusk = Mollusk::new(
        &random_tests::ID, "random_tests");
    
    let test_instruction = Instruction{
        program_id:random_tests::ID,
        accounts:vec![],
        data:random_tests::instruction::TestLoop::DISCRIMINATOR.to_vec()
    };

    mollusk.compute_budget.compute_unit_limit = 1_400_000;
    
    mollusk.process_and_validate_instruction(&test_instruction, &[],
    &[Check::instruction_err(InstructionError::ProgramFailedToComplete)]);
}

#[test]
pub fn loop_with_log_test(){
    let mut mollusk = Mollusk::new(
        &random_tests::ID, "random_tests");

    let test_instruction = Instruction{
        program_id:random_tests::ID,
        accounts:vec![],
        data:random_tests::instruction::TestLoopWithLog::DISCRIMINATOR.to_vec()
    };

    mollusk.compute_budget.compute_unit_limit = 1_400_000;

    mollusk.process_and_validate_instruction(&test_instruction, &[], 
        &[Check::instruction_err(InstructionError::ComputationalBudgetExceeded)]);
}

#[test]
#[ignore = "Sysvar not implemented yet"]
pub fn print_transaction_test(){
    let mollusk = Mollusk::new(
        &random_tests::ID, "random_tests");

    let test_instruction = Instruction{
        program_id:random_tests::ID,
        accounts:vec![
            AccountMeta{
                pubkey:INSTRUCTION_SYSVAR_ADDRESS,
                is_signer:false,
                is_writable:false
            }
        ],
        data:random_tests::instruction::TestPrintTransaction::DISCRIMINATOR.to_vec()
    };

    let instruction_sysvar = (INSTRUCTION_SYSVAR_ADDRESS, 
        Account::new(0, 0, &SYSVAR_ADDRESS));

    mollusk.process_and_validate_instruction_chain(
        &[(&test_instruction, &[Check::success()]),
        (&test_instruction, &[Check::success()]),
        (&test_instruction, &[Check::success()])], 
        &[instruction_sysvar]
    );        
}

#[test]
pub fn ser_deser_compute_test(){
    let mollusk = Mollusk::new(
        &random_tests::ID, "random_tests");

    let test_instruction = Instruction{
        program_id:random_tests::ID,
        accounts: vec![],
        data:random_tests::instruction::TestSerDeCompute::DISCRIMINATOR.to_vec()
    };

    mollusk.process_and_validate_instruction(&test_instruction, 
        &[], &[Check::success()]);
}
