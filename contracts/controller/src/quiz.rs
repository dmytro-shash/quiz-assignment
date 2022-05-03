use near_sdk::{AccountId, Balance};
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshDeserialize, BorshSerialize)]
pub struct Quiz {
    title: Option<String>,
    description: Option<String>,

    owner_id: AccountId,
    // status: QuizStatus,

    total_questions: u16,

    // the account that created quiz and so set up the reward
    sponsor_account_id: Option<AccountId>,
    funded_amount: Option<Balance>,
}