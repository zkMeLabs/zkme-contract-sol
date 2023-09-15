use crate::prelude::*;

#[account]
#[derive(Debug)]
pub struct Admin {
    pub owner: Pubkey,
    pub operators: [Pubkey; 20],
    pub next_counter: u64,
}

impl Admin {
    pub const LEN: usize = 32 + 32 * 20 + 8;
    pub const SEEDS: &'static str = "zkme_admin";

    pub fn is_operator(&self, target: &Pubkey) -> bool {
        self.operators.iter().any(|x| x == target)
    }

    pub fn add_operator(&mut self, operator: Pubkey) -> Option<usize> {
        if self.is_operator(&operator) {
            return None;
        }
        if let Some(index) = self.operators.iter().position(|x| x == &Pubkey::default()) {
            self.operators[index] = operator;
            Some(index)
        } else {
            None
        }
    }

    pub fn remove_operator(&mut self, operator: &Pubkey) -> bool {
        if let Some(index) = self.operators.iter().position(|x| x == operator) {
            let count = self
                .operators
                .iter()
                .filter(|x| x != &&Pubkey::default())
                .count();
            self.operators.swap(index, count - 1);
            self.operators[count - 1] = Pubkey::default();
            true
        } else {
            false
        }
    }
}
