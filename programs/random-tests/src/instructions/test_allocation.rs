use std::iter;

use anchor_lang::{
    prelude::*
};

pub fn test_allocation_handler(_:Context<TestAllocationAccounts>, size:usize)-> ! {
    let initial_vector = iter::repeat_n(1_2_3_4, size).collect::<Vec<_>>();

    let mut final_vector:Vec<i32> = vec![];

    loop{
        final_vector.extend_from_slice(&initial_vector);
    };

}

#[derive(Accounts)]
pub struct TestAllocationAccounts{

}