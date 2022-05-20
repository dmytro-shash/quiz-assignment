use crate::{
    Controller, QuestionByQuiz, QuestionId, QuestionOptionByQuiz, QuestionOptionId, QuizId,
    QUIZ_TOKEN_RATE,
};
use near_sdk::base64::encode_config_slice;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId, Balance};
use near_sdk::{log, BorshStorageKey};
use near_sdk::{near_bindgen, require};
use std::borrow::Borrow;

pub const DECIMAL: u128 = 10u128.pow(24);
pub type qBalance = u128;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct Quiz {
    title: String,
    description: String,

    // the account that created quiz and so set up the reward
    owner_id: AccountId,
    total_questions: u16,
    funded_amount: Option<qBalance>,
    correct_answers: Vec<String>,
}

#[derive(BorshDeserialize, BorshSerialize, BorshStorageKey)]
enum StorageKeys {
    Metadata { sub_key: String },
}

impl Default for Controller {
    fn default() -> Self {
        env::panic_str("Token contract should be initialized before usage")
    }
}

#[near_bindgen]
impl Controller {
    #[payable]
    pub fn create_quiz(
        &mut self,
        title: String,
        description: String,
        questions: Vec<String>,
        all_question_options: Vec<Vec<String>>,
    ) -> QuizId {
        let deposit: qBalance = env::attached_deposit() * QUIZ_TOKEN_RATE / DECIMAL;
        let owner_id = env::predecessor_account_id();

        let mut quiz_id = self.quizzes.len() + 1;

        for (question_id_index, all_question_option) in all_question_options.iter().enumerate() {
            for (question_option_index, question_option) in all_question_option.iter().enumerate() {
                self.question_options.insert(
                    &QuestionOptionByQuiz {
                        quiz_id,
                        question_id: question_id_index as QuestionId,
                        question_option_id: question_option_index as QuestionOptionId,
                    },
                    question_option,
                );
            }
        }

        for (index, question) in questions.iter().enumerate() {
            self.questions.insert(
                &QuestionByQuiz {
                    quiz_id,
                    question_id: index as QuestionId,
                },
                question,
            );
        }

        let quiz = Quiz {
            title,
            description,
            owner_id: owner_id.clone(),
            total_questions: questions.len() as u16,
            funded_amount: Some(deposit),
            correct_answers: vec![],
        };

        self.register_quiz(quiz_id, quiz);

        quiz_id
    }

    pub fn register_quiz(&mut self, id: QuizId, quiz: Quiz) {
        self.quizzes.insert(id.borrow(), &quiz);
    }

    pub fn get_all_quizzes(self) -> Vec<Quiz> {
        let mut result = vec![];
        for (_, quiz) in self.quizzes.iter() {
            let quiz_to_display = Quiz {
                title: quiz.title,
                description: quiz.description,
                owner_id: quiz.owner_id,
                total_questions: quiz.total_questions,
                funded_amount: quiz.funded_amount,
                correct_answers: vec!["try to guess it, mate :) ".to_string()],
            };

            result.push(quiz_to_display);
        }

        result
    }

    pub fn get_quiz_by_id(self, quiz_id: QuizId) -> Option<Quiz> {
        if self.quizzes.get(quiz_id.borrow()).is_some() {
            let quiz = self.quizzes.get(quiz_id.borrow()).unwrap();

            Some(Quiz {
                title: quiz.title,
                description: quiz.description,
                owner_id: quiz.owner_id,
                total_questions: quiz.total_questions,
                funded_amount: quiz.funded_amount,
                correct_answers: vec!["try to guess it, mate :) ".to_string()],
            })
        } else {
            None
        }
    }

    pub fn set_correct_answers(self, quiz_id: QuizId, correct_answer: Vec<String>) {
        // can be set up by owner only
        require!(
            self.quizzes.get(quiz_id.borrow()).is_some(),
            "Quiz by this id doesnt exist"
        );

        let mut quiz = self.quizzes.get(quiz_id.borrow()).unwrap();

        require!(
            env::signer_account_id() == quiz.owner_id,
            "Can be set up by owner only"
        );

        // // iterating throw all the combination of answers to ensure that correct answers is in question options
        // let mut is_answers_absent_in_options: bool = true;
        //
        // for answer in correct_answer.clone() {
        //     for i in 1..quiz.total_questions {
        //         for j in 1..quiz.total_questions {
        //             is_answers_absent_in_options &= self
        //                 .question_options
        //                 .get(&QuestionOptionByQuiz {
        //                     quiz_id,
        //                     question_id: i as QuestionId,
        //                     question_option_id: j as QuestionId,
        //                 })
        //                 .filter(|option| *option != answer)
        //                 .is_some()
        //         }
        //     }
        // }

        for answers in correct_answer {
            quiz.correct_answers.push(answers);
        }
    }
}
