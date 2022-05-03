mod utils;
mod quiz;
mod user_profile;

use near_sdk::{ext_contract, Gas};
use near_sdk::collections::LookupMap;
use near_sdk::{AccountId, Balance, env, log, Promise, PromiseOrValue};
use near_sdk::env::signer_account_id;
use crate::utils::{Question, QuestionByQuiz, QuestionOption, QuestionOptionByQuiz};
use near_sdk::{near_bindgen, BorshStorageKey, PanicOnDefault};
use near_sdk::borsh::{BorshSerialize, BorshDeserialize};
use near_sdk::json_types::U128;
use crate::quiz::Quiz;
use near_contract_standards::fungible_token::FungibleToken;
use crate::user_profile::UserProfile;

type QuizId = u64;
type QuestionId = u16;
type QuestionOptionId = u16;
pub type WBalance = U128;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Controller {
    // quizzes: LookupMap<QuizId, Quiz>,
    //
    // questions: LookupMap<QuestionByQuiz, Question>,
    // question_options: LookupMap<QuestionOptionByQuiz, QuestionOption>,
    accounts: LookupMap<AccountId, UserProfile>,

}

pub const GAS_TO_MINT: Gas = Gas(5_000_000_000_000);

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Quizzes,
    Questions,
    QuestionOptions,
    Rewards,
    Accounts,
}


#[ext_contract(qtoken)]
trait QtokenInterface {
    fn mint(&mut self, account_id: AccountId, amount: U128);

    fn burn(&mut self, account_id: AccountId, amount: U128);
}


const QUIZ_TOKEN_RATE: Balance = 10_000;

#[near_bindgen]
impl Controller {
    // #[private]
    // pub fn mint(&mut self, account_id: AccountId, amount: WBalance) {
    //     if self.token.accounts.get(&account_id).is_none() {
    //         self.token.internal_register_account(&account_id);
    //     };
    //     self.token.internal_deposit(&account_id, amount.into());
    // }
    //
    // #[private]
    // pub fn burn(&mut self, account_id: &AccountId, amount: WBalance) {
    //     if !self.token.accounts.contains_key(&account_id.clone()) {
    //         panic!("User with account {} wasn't found", account_id.clone());
    //     }
    //     self.token.internal_withdraw(account_id, amount.into());
    // }


    #[init]
    pub fn new_contract() -> Self {
        Self {
            // quizzes: LookupMap::new(StorageKey::Quizzes),
            // questions: LookupMap::new(StorageKey::Questions),
            // question_options: LookupMap::new(StorageKey::QuestionOptions),
            accounts: LookupMap::new(StorageKey::Accounts),
        }
    }


    #[payable]
    pub fn purchase_quiz_token(&mut self) {
        // payable method which mint QUIZ token in
        // rate 1 Near = 10_000 QUIZ
        let account_id = signer_account_id();
        let amount: Balance = env::attached_deposit() * 10u128.pow(24) * QUIZ_TOKEN_RATE;

        log!("you have purchased {} QUIZ tokens", amount);

        let quiz_token_account: AccountId = "qtoken.dmytro-quiz.testnet".parse().unwrap();

        qtoken::mint(account_id, U128(amount),
                     quiz_token_account, // contract account id
                     0, // yocto NEAR to attach
                     GAS_TO_MINT,// gas to attach
        );

        self.update_balance(&account_id, amount);
    }

    pub fn update_balance(&mut self, account_id: &AccountId, amount: Balance) {
        // if there is no user profile yet we are initializing it
        if self.accounts.get(account_id).is_none() {
            let default_user_profile = UserProfile {
                quiz_balance: 0
            };
            self.accounts.insert(account_id, &default_user_profile);
        };

        // taking 100 % existent profile and updating quiz balance either it has 0 or already purchased some
        let mut user_profile = self.accounts.get(account_id).unwrap();
        user_profile.quiz_balance += amount;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut controller_contract = Controller::new_contract();


        controller_contract.purchase_quiz_token();


        dbg!(controller_contract.view_amount(), 0);
    }
}


