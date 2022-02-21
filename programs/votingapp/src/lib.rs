use anchor_lang::prelude::*;
use std::collections::HashMap;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod votingapp {
    use super::*;
    pub fn initialize(
        ctx: Context<Initialize>,
        authority: Pubkey,
        proposal_names: Vec<String>,
    ) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.chairperson = authority; //replace with the one who aclled the function
        base_account.voters.insert(
            authority,
            Voter {
                weight: 1,
                voted: false,
                vote: 0,
                delegate: authority,
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

    pub fn give_right_to_vote(
        ctx: Context<GiveRightToVote>,
        voter_address: Pubkey,
    ) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        if base_account.voters[&voter_address].voted {
            println!("Voter has already voted!");
            return Err(ProgramError::InvalidArgument);
        }
        if base_account.voters[&voter_address].weight != 0 {
            println!("Voter weight is too low!");
            return Err(ProgramError::InvalidArgument);
        }
        base_account.voters.get_mut(&voter_address).unwrap().weight = 1;
        Ok(())
    }

    pub fn delegate(ctx: Context<Delegate>, from: Pubkey, to: Pubkey) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let sender_weight: i8;
        {
            let sender = base_account.voters.get_mut(&from).unwrap();
            sender_weight = sender.weight;
            if (sender.voted) {
                println!("Voter has already voted!");
                return Err(ProgramError::InvalidArgument);
            }
            if (from == to) {
                println!("Voter can't delegate to himself!");
                return Err(ProgramError::InvalidArgument);
            }
            sender.voted = true;
            sender.delegate = to;
        }
        let delegate_vote: i8;
        let delegate_voted: bool;
        let delegate_weight: &mut i8;
        {
            let delegate_ = base_account.voters.get_mut(&to).unwrap();
            delegate_vote = delegate_.vote;
            delegate_voted = delegate_.voted;
            delegate_weight = &mut delegate_.weight;
        }
        if delegate_voted {
            base_account.proposals[delegate_vote as usize].vote_count += sender_weight;
        } else {
            *delegate_weight += sender_weight;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Delegate<'info> {
    #[account(mut)]
    pub base_account: Account<'info, AccountDetails>,
}
#[derive(Accounts)]
pub struct GiveRightToVote<'info> {
    #[account(mut,has_one = chairperson)]
    pub base_account: Account<'info, AccountDetails>,
    pub chairperson: Signer<'info>,
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
    pub delegate: Pubkey,
}

#[account]
pub struct Proposal {
    pub name: String,
    pub vote_count: i8,
}

#[account]
pub struct AccountDetails {
    pub chairperson: Pubkey,
    pub voters: HashMap<Pubkey, Voter>,
    pub proposals: Vec<Proposal>,
}
