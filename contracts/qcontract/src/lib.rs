mod ft;
mod reward;
mod score;
mod utils;

use crate::reward::RewardInfo;
use crate::score::{Score, ScoreByQuestionInfo, ScoreInfo, ScoreType};
use crate::utils::{QuestionByQuiz, QuestionId, QuizId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::env::signer_account_id;

use crate::QuizContractStorageKey::ScoreByUser;
use near_sdk::{
    env, ext_contract, near_bindgen, require, AccountId, BorshStorageKey, Gas, PromiseOrValue,
    PromiseResult,
};
use std::borrow::Borrow;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct QuizContract {
    rewards: UnorderedMap<AccountId, RewardInfo>,
    scores: UnorderedMap<AccountId, ScoreInfo>,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum QuizContractStorageKey {
    Rewards,
    Scores,
    ScoreByUser {
        quiz_id: QuizId,
        answer_1: String,
        answer_2: String,
        signer: AccountId,
    },
}

impl Default for QuizContract {
    fn default() -> Self {
        env::panic_str("Token contract should be initialized before usage")
    }
}

#[near_bindgen]
impl QuizContract {
    #[init]
    pub fn new_contract() -> Self {
        Self {
            rewards: UnorderedMap::new(QuizContractStorageKey::Rewards),
            scores: UnorderedMap::new(QuizContractStorageKey::Scores),
        }
    }

    pub fn answer_quiz(mut self, quiz_id: QuizId, answers: Vec<String>) -> PromiseOrValue<u8> {
        let controller_account: AccountId = "controller.dmytro-quiz.testnet".parse().unwrap();

        // initializing
        let mut score_by_quiz: UnorderedMap<QuizId, Score> = UnorderedMap::new(ScoreByUser {
            quiz_id,
            answer_1: answers[0].clone(),
            answer_2: answers[1].clone(),
            signer: signer_account_id(),
        });

        self.scores
            .insert(&signer_account_id(), &ScoreInfo { score_by_quiz });

        controller::answer_quiz(
            quiz_id,
            answers,
            controller_account,     // controller contract account id
            0,                      // yocto NEAR to attach
            Gas(5_000_000_000_000), // gas to attach
        )
        .then(ext_self::answer_quiz_callback(
            quiz_id,
            env::current_account_id(), // controller contract account id
            0,                         // yocto NEAR to attach
            Gas(5_000_000_000_000),    // gas to attach
        ))
        .into()
    }

    #[private]
    pub fn answer_quiz_callback(mut self, quiz_id: QuizId) {
        let score: u8 = match env::promise_result(0) {
            PromiseResult::NotReady => 0,
            PromiseResult::Failed => 0,
            PromiseResult::Successful(result) => near_sdk::serde_json::from_slice::<u8>(&result)
                .unwrap()
                .into(),
        };

        let mut score_by_quiz = self.scores.get(&signer_account_id()).unwrap().score_by_quiz;

        score_by_quiz.insert(quiz_id.borrow(), score.borrow());
    }

    pub fn get_score(self, quiz_id: QuizId) -> Score {
        require!(self.scores.get(&signer_account_id()).is_some());

        let score_bu_quiz = self.scores.get(&signer_account_id()).unwrap().score_by_quiz;
        let score = score_bu_quiz.get(&quiz_id).unwrap();

        score
    }
}
#[ext_contract(controller)]
trait ControllerInterface {
    fn purchase_quiz_token(self);
    fn create_quiz(self);
    fn answer_quiz(self, quiz_id: QuizId, answers: Vec<String>) -> PromiseOrValue<u8>;
    // claim rewards
}

#[ext_contract(ext_self)]
trait QuizContractInternal {
    fn answer_quiz_callback(mut self, quiz_id: QuizId);
}
