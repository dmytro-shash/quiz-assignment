use crate::{QuestionId, QuestionOptionId, QuizId};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::borsh::{BorshSerialize, BorshDeserialize};

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct QuestionByQuiz {
    quiz_id: QuizId,
    question_id: QuestionId,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Question {
    content: String,
    options_quantity: u16,
}


#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct QuestionOptionByQuiz {
    quiz_id: QuizId,
    question_id: QuestionId,
    question_option_id: QuestionOptionId,
}


#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct QuestionOption {
    content: String,
}
