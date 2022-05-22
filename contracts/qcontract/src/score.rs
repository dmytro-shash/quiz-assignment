use crate::utils::QuestionByQuiz;
use crate::QuizId;
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use serde::{Deserialize, Serialize};

pub type Score = u8;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub enum ScoreType {
    // score 0 or 1 for 1 question
    Answered { score: u8 },
    NotAnswered,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ScoreByQuestionInfo {
    pub score_by_user: UnorderedMap<QuestionByQuiz, ScoreType>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ScoreInfo {
    pub score_by_quiz: UnorderedMap<QuizId, Score>,
}
