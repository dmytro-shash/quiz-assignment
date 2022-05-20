use crate::*;
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::near_bindgen;
use crate::Controller;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug, Default)]
pub struct UserProfile {
    pub quiz_balance: Balance,
}

#[near_bindgen]
impl Controller {
    pub fn view_quiz_balance(self, account_id: AccountId) -> Balance {
        self.accounts.get(&account_id).unwrap_or_default().quiz_balance
    }
}
