#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("JDMKvhV3gafANaWaLjsUTrevDbG7kXNUm6R9UgzP1ixy");

#[program]
pub mod journal {
    use super::*;

  //instructions function
  pub fn create_journal_entry( ctx: Context<CreateEntry>, title: String, message: String) -> Result<()> {
    let journal_entry = &mut ctx.accounts.journal_entry;
    journal_entry.owner = *ctx.accounts.owner.key;
    journal_entry.title = title;
    journal_entry.message = message;

    msg!("Message created and state saced in to the Journal Entry State account");

    Ok(())
  }

  pub fn update_journal_entry( ctx: Context<UpdateEntry>, _title: String, message: String) -> Result<()> {
    let journal_entry = &mut ctx.accounts.journal_entry;  
    journal_entry.message = message;
    let msg = &journal_entry.message;

    msg!("Message edited and state saced in to the Journal Entry State account");
    msg!("New message is: {}", msg);

    Ok(())
  }

  pub fn delete_journal_entry(_ctx: Context<DeleteEntry>,  _title: String) -> Result<()> {
   //the account delete occurs inside the Account Struct as all the accounts handling must happens inside the data/Account structure
   msg!("Your journal entry was deleted!");
    Ok(())
  }
  
}

//define all the accounts that are going through the context
#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateEntry<'info> {
  #[account(
    init, 
    payer = owner, 
    seeds = [title.as_bytes(), owner.key.as_ref()],
    bump,
    space = 8 + JournalEntryState::INIT_SPACE)
    ]
  pub journal_entry: Account <'info, JournalEntryState>,

  // this account must be mutable because owner will pay to initializer the create entry account and therefore it will change the Owner state too, so onwer needs to be mut
  #[account(mut)]
  pub owner: Signer<'info>,

  pub system_program: Program<'info, System>,
}

//in update account because we are giving the seed, the state will saved to the Account with the corresponding seed
#[derive(Accounts)]
#[instruction(title: String)]
pub struct UpdateEntry<'info> {
  #[account(
    mut,
    seeds = [title.as_bytes(), owner.key.as_ref()],
    bump,
    //this way only the owner can close the account
    close = owner,
   )]
  pub journal_entry: Account <'info, JournalEntryState>,

 
  #[account(mut)]
  pub owner: Signer<'info>,

  pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteEntry<'info> {
  #[account(
    mut,
    seeds = [title.as_bytes(), owner.key.as_ref()],
    bump,
    //since the message string size can change with the update we use realloc to realocate the lamports debiting or crediting the lamports to the owner as needed
    realloc = 8 + JournalEntryState::INIT_SPACE,
    realloc::payer = owner,
    //this sets the original space back to zero and recalculate everything
    realloc::zero = true,
   )]
  pub journal_entry: Account <'info, JournalEntryState>,

 
  #[account(mut)]
  pub owner: Signer<'info>,

  pub system_program: Program<'info, System>,
}

//State is where tou are going to save all your data and stated is stored inside the program accounts
#[account]
#[derive(InitSpace)]
pub struct JournalEntryState {
  pub owner: Pubkey,
  #[max_len(50)]
  pub title: String,
  #[max_len(1000)]
  pub message: String,
}