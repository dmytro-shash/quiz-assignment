mod quiz;
mod ft;
mod utils;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, ext_contract, AccountId};
use near_sdk::json_types::U128;


type AnswerId = u16;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct QuizContract {

}


#[near_bindgen]
impl QuizContract {
    pub fn answer_quiz() {}

    pub fn get_score() {}
}


#[ext_contract(controller)]
trait ControllerInterface {
    fn purchase_quiz_token(self);
    fn create_quiz(self);
    // claim rewards
    // get all quiz's
}











