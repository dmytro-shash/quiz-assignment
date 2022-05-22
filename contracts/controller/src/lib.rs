pub mod quiz;
pub mod user_profile;
pub mod utils;

use crate::quiz::Quiz;
use crate::user_profile::UserProfile;
use crate::utils::{QuestionByQuiz, QuestionOptionByQuiz};
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::env::signer_account_id;
use near_sdk::json_types::U128;
use near_sdk::{env, log, AccountId, Balance};
use near_sdk::{ext_contract, Gas};
use near_sdk::{near_bindgen, BorshStorageKey};

pub type QuizId = u64;
pub type QuestionId = u16;
pub type QuestionOptionId = u16;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Controller {
    quizzes: UnorderedMap<QuizId, Quiz>,

    questions: UnorderedMap<QuestionByQuiz, String>,
    question_options: UnorderedMap<QuestionOptionByQuiz, String>,

    accounts: UnorderedMap<AccountId, UserProfile>,
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
    #[init]
    pub fn new_contract() -> Self {
        Self {
            quizzes: UnorderedMap::new(StorageKey::Quizzes),
            questions: UnorderedMap::new(StorageKey::Questions),
            question_options: UnorderedMap::new(StorageKey::QuestionOptions),
            accounts: UnorderedMap::new(StorageKey::Accounts),
        }
    }

    #[payable]
    pub fn purchase_quiz_token(&mut self) {
        // payable method which mint QUIZ token in
        // rate 1 Near = 10_000 QUIZ
        let account_id = signer_account_id();
        let amount: Balance = env::attached_deposit() * QUIZ_TOKEN_RATE;

        log!("you have purchased {} QUIZ tokens", amount);

        let quiz_token_account: AccountId = "qtoken.dmytro-quiz.testnet".parse().unwrap();

        qtoken::mint(
            account_id.clone(),
            U128(amount),
            quiz_token_account, // contract account id
            0,                  // yocto NEAR to attach
            GAS_TO_MINT,        // gas to attach
        );

        self.update_balance(&account_id.clone(), amount);
    }

    pub fn update_balance(&mut self, account_id: &AccountId, amount: Balance) {
        // if there is no user profile yet we are initializing it
        if self.accounts.get(account_id).is_none() {
            let default_user_profile = UserProfile { quiz_balance: 0 };
            self.accounts.insert(account_id, &default_user_profile);
        };

        // taking 100 % existent profile and updating quiz balance either it has 0 or already purchased some
        let mut user_profile = self.accounts.get(account_id).unwrap();
        user_profile.quiz_balance += amount;
    }
}
