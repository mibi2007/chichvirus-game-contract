use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, Balance, Timestamp};

pub type MatchId = String;

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum GameStatus {
    Init,
    Done,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GameMatch {
    pub players: (AccountId, AccountId),
    pub balance: Balance,
    pub start_ts: Timestamp,
    pub status: GameStatus,
    pub end_ts: Option<Timestamp>,
    pub winner: Option<AccountId>,
}
