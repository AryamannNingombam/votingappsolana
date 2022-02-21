use anchor_lang::prelude::*;
use std::collections::HashMap;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod votingapp {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, proposal_names: Vec<String>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.chairperson = "".to_string(); //replace with the one who aclled the function
        let chairperson = base_account.chairperson.clone();
        base_account.voters.insert(
            chairperson,
            Voter {
                weight: 1,
                voted: false,
                vote: 0,
                delegate: "".to_string(),
            },
        );
        for proposal in proposal_names {
            base_account.proposals.push(Proposal {
                name: proposal,
                vote_count: 0,
            });
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,payer=user,space=64*64)]
    pub base_account: Account<'info, AccountDetails>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Voter {
    pub weight: i8,
    pub voted: bool,
    pub vote: i8,
    pub delegate: String,
}

#[account]
pub struct Proposal {
    pub name: String,
    pub vote_count: i8,
}

#[account]
pub struct AccountDetails {
    pub chairperson: String,
    pub voters: HashMap<String, Voter>,
    pub proposals: Vec<Proposal>,
}
