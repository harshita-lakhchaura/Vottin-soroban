
#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, Address,
    ConversionError, Env, Map, Symbol,
};

#[contracterror]
#[derive(Clone, Debug, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    Conversion = 2,
    KeyExpected = 3,
    AlreadyVoted = 4,
    Overflow = 5,
    VotingClosed = 6,
}

impl From<ConversionError> for Error {
    fn from(_: ConversionError) -> Self {
        Error::Conversion
    }
}


#[contracttype]
#[derive(Clone, Copy)]
pub enum DataKey {
    AlreadyInitialized = 0,
    Proposal = 1,
}

#[contract]
pub struct SimpleVoting;

#[contractimpl]
impl SimpleVoting {
    
    pub fn init_voting(
        env: Env,
        voting_period_secs: u64,
        target_approval_rate_bps: u32,
        total_voters: u32,
    ) -> Result<(), Error> {
        let storage = env.storage().persistent();

        if storage
            .get::<_, bool>(&DataKey::AlreadyInitialized)
            .is_some()
        {
            panic_with_error!(&env, Error::AlreadyInitialized);
        }

        storage.set(&DataKey::AlreadyInitialized, &true);
        Self::create_proposal_voting(
            env,
            voting_period_secs,
            target_approval_rate_bps,
            total_voters,
        )
    }

    fn create_proposal_voting(
        env: Env,
        voting_period_secs: u64,
        target_approval_rate_bps: u32,
        total_voters: u32,
    ) -> Result<(), Error> {
        let storage = env.storage().persistent();

        storage.set(
            &DataKey::Proposal,
            &Proposal {
                voting_end_time: env
                    .ledger()
                    .timestamp()
                    .checked_add(voting_period_secs)
                    .unwrap(),
                target_approval_rate_bps,
                votes: 0,
                voters: Map::<Address, bool>::new(&env),
                total_voters,
            },
        );
        Ok(())
    }

   
    pub fn vote(env: Env, voter: Address) -> Result<(), Error> {
        voter.require_auth();

        let storage = env.storage().persistent();

        let mut proposal = storage
            .get::<_, Proposal>(&DataKey::Proposal)
            .ok_or(Error::KeyExpected)?;

        proposal.vote(env.ledger().timestamp(), voter.clone())?;
        let updated_approval_rate = proposal.approval_rate_bps();

        storage.set(&DataKey::Proposal, &proposal);

        env.events().publish(
            (Symbol::new(&env, "proposal_voted"), voter.clone()),
            updated_approval_rate,
        );
        Ok(())
    }
}


#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Proposal {
    
    voting_end_time: u64,
   
    votes: u32,
   
    target_approval_rate_bps: u32,
    
    total_voters: u32,
 
    voters: Map<Address, bool>,
}

impl Proposal {
   
    pub fn vote(&mut self, current_time: u64, voter: Address) -> Result<(), Error> {
        if self.is_closed(current_time) {
            return Err(Error::VotingClosed);
        }

        if self.voters.get(voter.clone()).is_some() {
            return Err(Error::AlreadyVoted);
        }

        self.votes = self.votes.checked_add(1).ok_or(Error::Overflow)?;
        self.voters.set(voter, true);
        Ok(())
    }

    pub fn is_closed(&self, current_time: u64) -> bool {
        current_time >= self.voting_end_time || self.voters.len() == self.total_voters
    }

   
    pub fn approval_rate_bps(&self) -> Result<u32, Error> {
        if self.votes == 0 {
            return Ok(0);
        }
        self.votes
            .checked_mul(10_000)
            .ok_or(Error::Overflow)?
            .checked_div(self.total_voters)
            .ok_or(Error::Overflow)
    }

    pub fn is_approved(&self) -> bool {
        self.approval_rate_bps().unwrap() >= self.target_approval_rate_bps
    }
}

