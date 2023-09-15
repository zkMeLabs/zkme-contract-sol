use crate::prelude::*;

#[account]
#[derive(Debug)]
pub struct CooperatorConf {
    pub owner: Pubkey,
    pub approved: u64,
    pub approved_lite: u64,
    pub approved_user: u64,
    pub approved_lite_user: u64,
    pub data: String,
    pub valid_questions: Vec<String>,
}

impl CooperatorConf {
    pub const LEN: usize = 32 + // owner
        8 + 8 +
        8 +  // approved users
        8 +  // approved lite users
        4 + 1000 +  // data
        4 + 6000; // cooperator's questions
    pub const SEEDS: &'static str = "zkme_conf";

    pub fn save_valid_questions(&mut self, qs: Vec<String>) {
        self.valid_questions = qs
            .iter()
            .filter_map(|s| AnchorDeserialize::deserialize(&mut s.as_bytes()).ok())
            .collect();
    }
}
