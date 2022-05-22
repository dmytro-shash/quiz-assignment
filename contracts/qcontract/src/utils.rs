pub type QuizId = u64;
pub type Qbalance = u128;
pub type QuestionId = u16;

use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct QuestionByQuiz {
    pub(crate) quiz_id: QuizId,
    pub(crate) question_id: QuestionId,
}
