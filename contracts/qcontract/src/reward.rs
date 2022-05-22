use crate::utils::Qbalance;
use crate::QuizId;
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct RewardInfo {
    pub reward_by_user: UnorderedMap<QuizId, Qbalance>,
}
