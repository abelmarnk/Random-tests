use std::iter;

use anchor_lang::{
    prelude::*,
    solana_program::{
        compute_units::sol_remaining_compute_units, log::{
            sol_log
        }
    },
};
use borsh::{BorshDeserialize, BorshSerialize};

#[allow(unused_assignments)]
pub fn test_ser_de_compute_handler(_: Context<TestSerDeComputeAccounts>) -> Result<()> {

        // Construct a sample struct
        let test_struct = TestSerDeComputeStruct {
            field_1: 123,
            field_2: 12345,
            field_3: 1234567,
            field_4: 123456789,
            field_5: 123456789_10,
            field_6: [0; 32],
            field_7: "Hello World".to_string(),
            field_8: iter::repeat_n(Pubkey::default(), 200).collect::<Vec<_>>(),
            field_9: TestSerDeComputeEnum::Type25,
        };
        
        let mut buffer: Vec<u8> = Vec::with_capacity(10_000);
        let mut buffer_2: Vec<u8> = Vec::with_capacity(10_000);

        let mut base_ = 0;

        let mut base = 0;

        base_ = sol_remaining_compute_units();

        base = sol_remaining_compute_units();

        sol_log(format!("Base compute units for getting compute units: {}", 
            (base_ - base)).as_str());

        // Track compute before struct serialization
        base_ = sol_remaining_compute_units();

        // Serialize entire struct
        test_struct.serialize(&mut buffer)?;

        // Track compute after struct serialization
        base = sol_remaining_compute_units();

        sol_log(format!("Compute units used: {}", (base_ - base)).as_str());

        // Track compute before field serialization
        base_ = sol_remaining_compute_units();


        test_struct.field_1.serialize(&mut buffer_2)?;
        test_struct.field_2.serialize(&mut buffer_2)?;
        test_struct.field_3.serialize(&mut buffer_2)?;
        test_struct.field_4.serialize(&mut buffer_2)?;
        test_struct.field_5.serialize(&mut buffer_2)?;
        test_struct.field_6.serialize(&mut buffer_2)?;
        test_struct.field_7.serialize(&mut buffer_2)?;
        test_struct.field_8.serialize(&mut buffer_2)?;
        test_struct.field_9.serialize(&mut buffer_2)?;

        // Track compute after field serialization
        base = sol_remaining_compute_units();

        sol_log(format!("Compute units used: {}", (base_ - base)).as_str());


        Ok(())
}


#[derive(Accounts)]
pub struct TestSerDeComputeAccounts {

}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub enum TestSerDeComputeEnum {
    Type1,
    Type2,
    Type3,
    Type4,
    Type5,
    Type6,
    Type7,
    Type8,
    Type9,
    Type10,
    Type11,
    Type12,
    Type13,
    Type14,
    Type15,
    Type16,
    Type17,
    Type18,
    Type19,
    Type20,
    Type21,
    Type22,
    Type23,
    Type24,
    Type25,
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct TestSerDeComputeStruct {
    pub field_1: u8,
    pub field_2: u16,
    pub field_3: u32,
    pub field_4: u64,
    pub field_5: u128,
    pub field_6: [u8; 32],
    pub field_7: String,
    pub field_8: Vec<Pubkey>,
    pub field_9: TestSerDeComputeEnum,
}

// Expansion of derive for the TestSerDeCompute
// Based on the expansion of the BorshSerialize derive, the two code parts above 
// are equivalent, except for the function call. Now, when the tests are run, they consume 
// the same amount of compute units. So, what that implies is that it's either there is no cost 
// associated with the function call, or the compiler inlined the function call.
/* 
impl borsh::ser::BorshSerialize for TestSerDeComputeStruct
where
    u8: borsh::ser::BorshSerialize,
    u16: borsh::ser::BorshSerialize,
    u32: borsh::ser::BorshSerialize,
    u64: borsh::ser::BorshSerialize,
    u128: borsh::ser::BorshSerialize,
    [u8; 32]: borsh::ser::BorshSerialize,
    String: borsh::ser::BorshSerialize,
    Vec<Pubkey>: borsh::ser::BorshSerialize,
    TestEnum: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.field_1, writer)?;
        borsh::BorshSerialize::serialize(&self.field_2, writer)?;
        borsh::BorshSerialize::serialize(&self.field_3, writer)?;
        borsh::BorshSerialize::serialize(&self.field_4, writer)?;
        borsh::BorshSerialize::serialize(&self.field_5, writer)?;
        borsh::BorshSerialize::serialize(&self.field_6, writer)?;
        borsh::BorshSerialize::serialize(&self.field_7, writer)?;
        borsh::BorshSerialize::serialize(&self.field_8, writer)?;
        borsh::BorshSerialize::serialize(&self.field_9, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for TestSerDeComputeStruct
where
    u8: borsh::BorshDeserialize,
    u16: borsh::BorshDeserialize,
    u32: borsh::BorshDeserialize,
    u64: borsh::BorshDeserialize,
    u128: borsh::BorshDeserialize,
    [u8; 32]: borsh::BorshDeserialize,
    String: borsh::BorshDeserialize,
    Vec<Pubkey>: borsh::BorshDeserialize,
    TestEnum: borsh::BorshDeserialize,
{
    fn deserialize_reader<R: borsh::maybestd::io::Read>(
        reader: &mut R,
    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            field_1: borsh::BorshDeserialize::deserialize_reader(reader)?,
            field_2: borsh::BorshDeserialize::deserialize_reader(reader)?,
            field_3: borsh::BorshDeserialize::deserialize_reader(reader)?,
            field_4: borsh::BorshDeserialize::deserialize_reader(reader)?,
            field_5: borsh::BorshDeserialize::deserialize_reader(reader)?,
            field_6: borsh::BorshDeserialize::deserialize_reader(reader)?,
            field_7: borsh::BorshDeserialize::deserialize_reader(reader)?,
            field_8: borsh::BorshDeserialize::deserialize_reader(reader)?,
            field_9: borsh::BorshDeserialize::deserialize_reader(reader)?,
        })
    }
*/