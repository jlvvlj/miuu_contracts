
use anchor_lang::prelude::*;

declare_id!("48tMte8iJvvYowxzVZdjYXKN19982b1vjxfs8aWFThJG");

#[program]
pub mod music_track {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, track_id: String, track_name: String, participants: Vec<Participant>) -> ProgramResult {
        let music_track = &mut ctx.accounts.music_track;
        // music_track.track_id = track_id;
        // music_track.track_name = track_name;
        // music_track.participants = participants;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32 + 32 + (40 * 8))]
    pub music_track: ProgramAccount<'info, MusicTrack>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MusicTrack {
    pub track_id: String,
    pub track_name: String,
    pub participants: Vec<Participant>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Participant {
    pub id: String,
    pub name: String,
    pub share_percentage: u8,
    pub is_master_owner: bool,
}


// #[program]
// pub mod music_contracts {
//     use super::*;
    
//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize {}

