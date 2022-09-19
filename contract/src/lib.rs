/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

pub mod game_match;
use game_match::*;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{near_bindgen, AccountId, Balance, BorshStorageKey, Timestamp};

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    owner_id: AccountId,
    game_cost: Balance,
    matches: LookupMap<MatchId, GameMatch>,
    player_init_game: LookupMap<AccountId, MatchId>,
}

impl Default for Contract {
    fn default() -> Self {
        todo!()
    }
}

#[derive(BorshDeserialize, BorshSerialize, BorshStorageKey)]
enum StorageKey {
    MatchKey,
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        let m_matches: LookupMap<MatchId, GameMatch> = LookupMap::new(StorageKey::MatchKey);
        let m_player_init: LookupMap<AccountId, MatchId> = LookupMap::new(StorageKey::MatchKey);
        Self {
            owner_id,
            game_cost: 100,
            matches: m_matches,
            player_init_game: m_player_init,
        }
    }

    // Public method - returns the greeting saved, defaulting to DEFAULT_MESSAGE
    pub fn get_matches(&self, match_id: String) -> Option<GameMatch> {
        return self.matches.get(&match_id);
    }

    pub fn get_players_matches(&self) -> String {
        return format!(
            "games: {:?}",
            self.player_init_game
                .get(&"choptr.testnet".parse().unwrap())
        );
    }

    pub fn create_game_match(
        &mut self,
        match_id: MatchId,
        players: (AccountId, AccountId),
        balance: Balance,
        start_ts: Timestamp,
    ) -> Option<GameMatch> {
        if self.player_init_game.get(&players.0).is_some() {
            panic!("player_1 already in game");
        }
        if self.player_init_game.get(&players.1).is_some() {
            panic!("player_2 already in game");
        }
        let new_game = GameMatch {
            players: players.clone(),
            start_ts,
            status: GameStatus::Init,
            balance,
            end_ts: None,
            winner: None,
        };
        self.matches.insert(&match_id, &new_game);
        self.player_init_game.insert(&players.0, &match_id);
        self.player_init_game.insert(&players.1, &match_id);

        Some(new_game)
    }

    pub fn save_match_result(
        &mut self,
        match_id: MatchId,
        winner: AccountId,
        end_ts: Timestamp,
    ) -> Option<GameMatch> {
        let mut current_match = self.matches.get(&match_id).expect("MATCH_NOT_FOUND");
        current_match.status = GameStatus::Done;
        current_match.winner = Some(winner.clone());
        current_match.end_ts = Some(end_ts);

        // Replace the current match with new information
        self.matches.insert(&match_id, &current_match);

        Some(current_match)
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use near_sdk::{
        env::block_timestamp,
        test_utils::{accounts, VMContextBuilder},
        testing_env, Gas, VMContext,
    };

    use super::*;

    fn get_context(is_view: bool) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(accounts(0))
            .predecessor_account_id(accounts(0))
            .is_view(is_view);

        builder
    }

    fn get_sample_games() -> LookupMap<MatchId, GameMatch> {
        let mut m: LookupMap<MatchId, GameMatch> = LookupMap::new(StorageKey::MatchKey);
        m.insert(
            &"123456".to_string(),
            &GameMatch {
                players: (accounts(0), accounts(1)),
                start_ts: block_timestamp(),
                status: GameStatus::Init,
                balance: 12345678,
                end_ts: None,
                winner: None,
            },
        );
        m
    }

    #[test]
    fn test_get_matches() {
        let mut context = get_context(false);
        let alice: AccountId = accounts(0);

        context
            .account_balance(1000)
            .predecessor_account_id(alice.clone())
            .attached_deposit(1000)
            .signer_account_id(alice.clone());

        testing_env!(context.build());

        let contract = Contract {
            owner_id: alice.clone(),
            game_cost: 100,
            matches: get_sample_games(),
            player_init_game: LookupMap::new(StorageKey::MatchKey),
        };
        assert_eq!(
            contract.get_matches("123456".to_string()),
            Some(GameMatch {
                players: (accounts(0), accounts(1)),
                start_ts: block_timestamp(),
                status: GameStatus::Init,
                balance: 12345678,
                end_ts: None,
                winner: None,
            })
        );
    }

    #[test]
    fn create_game_match() {
        let mut context = get_context(false);
        let alice: AccountId = accounts(0);
        let bob: AccountId = accounts(1);
        let charlie: AccountId = accounts(1);

        context
            .account_balance(1000)
            .predecessor_account_id(alice.clone())
            .attached_deposit(1000)
            .signer_account_id(alice.clone());

        testing_env!(context.build());

        let mut contract = Contract::new(alice.clone());
        let start_time = block_timestamp();
        assert_eq!(
            contract.create_game_match(
                "match_1".to_owned(),
                (bob.clone(), charlie.clone()),
                100,
                start_time,
            ),
            Some(GameMatch {
                players: (bob.clone(), charlie.clone()),
                start_ts: start_time,
                status: GameStatus::Init,
                balance: 100,
                end_ts: None,
                winner: None,
            })
        )
    }

    #[test]
    fn save_match_result() {
        let mut context = get_context(false);
        let alice: AccountId = accounts(0);
        let bob: AccountId = accounts(1);
        let charlie: AccountId = accounts(2);

        context
            .account_balance(1000)
            .predecessor_account_id(alice.clone())
            .attached_deposit(1000)
            .signer_account_id(alice.clone());

        testing_env!(context.build());

        let mut contract = Contract::new(alice.clone());
        let start_time = block_timestamp();
        contract.create_game_match(
            "match_1".to_owned(),
            (bob.clone(), charlie.clone()),
            100,
            start_time,
        );

        let end_time = block_timestamp() + 1_000 * 60; // After 60 seconds
        contract.save_match_result("match_1".to_owned(), bob.clone(), end_time);

        let current_match = contract.get_matches("match_1".to_string()).unwrap();
        assert_eq!(current_match.winner, Some(bob.clone()));
    }
}
