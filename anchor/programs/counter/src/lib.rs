#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, mint, token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked}};

declare_id!("FqzkXZdwYjurnUKetJCAvaUw5WAqbwzU6gZEwydeEfqS");

#[program]
pub mod counter {
    use std::string;
    use anchor_lang::accounts;
    use anchor_spl::token_interface;

    use super::*;

    // create a vesting account
    pub fn create_vesting_account(
        ctx:Context<CreateVestingAccount>,
        company_name:String
    ) -> Result<()>{
        *ctx.accounts.vesting_account = VestingAccount {
            company_name,
            owner: ctx.accounts.owner.key(),
            mint: ctx.accounts.mint.key(),
            treasury_token_account: ctx.accounts.treasury_token_account.key(),
            bump: ctx.bumps.vesting_account,
            treasury_bump: ctx.bumps.treasury_token_account
        };

        Ok(())
    }

    // create a employee account
    pub fn create_employee_account(
        ctx:Context<CreateEmployeeAccount>,
        start_time: i64,
        end_time: i64,
        cliff_time: i64,
        total_amount: u64            
    ) -> Result<()>{
        *ctx.accounts.employee_account = EmployeeAccount {
            beneficiary: ctx.accounts.beneficiary.key(),
            vesting_account: ctx.accounts.vesting_account.key(),
            start_time,
            end_time,
            cliff_time,
            total_withdrawn: 0,
            total_amount,
            bump: ctx.bumps.employee_account
        };
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(company_name:String)]
pub struct CreateVestingAccount<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    pub mint: InterfaceAccount<'info,Mint>,

    #[account(
        init,
        payer = owner,
        space = 8 + VestingAccount::INIT_SPACE,
        seeds = [company_name.as_ref()],
        bump
    )]
    pub vesting_account: Account<'info,VestingAccount>,

    #[account(
        init,
        payer = owner,
        token::mint = mint,
        token::authority = treasury_token_account,
        seeds = [b"vesting_treasury",company_name.as_bytes()],
        bump
    )]
    pub treasury_token_account: InterfaceAccount<'info,TokenAccount>,

    pub system_program: Program<'info,System>,
    pub token_program: Interface<'info,TokenInterface>
}

#[derive(Accounts)]
pub struct CreateEmployeeAccount<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    pub beneficiary: SystemAccount<'info>,

    #[account(
        has_one = owner
    )]
    pub vesting_account: Account<'info,VestingAccount>,

    #[account(
        init,
        payer =  owner,
        space = 8 + EmployeeAccount::INIT_SPACE,
        seeds = [b"employee_vesting",beneficiary.key().as_ref(),vesting_account.key().as_ref()],
        bump
    )]
    pub employee_account: Account<'info,EmployeeAccount>,
    pub system_program: Program<'info,System>
}

#[account]
#[derive(InitSpace)]
pub struct VestingAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub treasury_token_account:Pubkey,
    #[max_len(20)]
    pub company_name: String,
    pub treasury_bump: u8,
    pub bump: u8
}

#[account]
#[derive(InitSpace)]
pub struct EmployeeAccount {
    pub beneficiary: Pubkey,
    pub start_time: i64,
    pub end_time: i64,
    pub cliff_time: i64,
    pub total_amount: u64,
    pub total_withdrawn: u64,
    pub vesting_account: Pubkey,
    pub bump: u8
}