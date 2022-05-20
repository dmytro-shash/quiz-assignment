use crate::{QuestionId, QuestionOptionId, QuizId};
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct QuestionByQuiz {
    pub(crate) quiz_id: QuizId,
    pub(crate) question_id: QuestionId,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct QuestionOptionByQuiz {
    pub(crate) quiz_id: QuizId,
    pub(crate) question_id: QuestionId,
    pub(crate) question_option_id: QuestionOptionId,
}
