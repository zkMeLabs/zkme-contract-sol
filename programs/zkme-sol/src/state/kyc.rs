use crate::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct Question([u8; 60]);

impl AnchorSerialize for Question {
    #[inline]
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        if let Some(u8_slice) = u8::u8_slice(&self.0) {
            writer.write_all(u8_slice)?;
        } else {
            for el in self.0.iter() {
                el.serialize(writer)?;
            }
        }
        Ok(())
    }
}

impl AnchorDeserialize for Question {
    #[inline]
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut result = [u8::default(); 60];
        if !u8::copy_from_bytes(buf, &mut result)? {
            for item in result.iter_mut() {
                *item = u8::deserialize(buf)?;
            }
        }
        Ok(Question(result))
    }
}

#[non_exhaustive]
#[repr(u8)]
#[derive(Debug, Copy, Clone, AnchorDeserialize, AnchorSerialize)]
pub enum KycStatus {
    Uninitialized,
    Initialized,
    Approved,
    Revoked,
}

#[account]
#[derive(Debug)]
pub struct KycLite {
    pub status: KycStatus,
    pub owner: Pubkey,
    pub token_account: Pubkey,
    pub threshold_key: String,
    pub kyc: Pubkey,
}

impl KycLite {
    pub const LEN: usize = 1 + 32 + 32 + 4 + 500 + 32;
    pub const SEEDS: &'static str = "zkme_kyc_lite";
}

#[account]
#[derive(Debug)]
pub struct Kyc {
    pub status: KycStatus,
    /// There is two kind of Kyc, if owner is a wallet owner, then it is belongs to an user,
    /// otherwise it is belongs to cooperator's conf, meaning that is an approved copy
    pub owner: Pubkey,
    pub token_account: Pubkey,
    /// owner or cooperator's threshold key
    pub threshold_key: String,
    pub validity: u64,
    pub data: String,
    pub questions: Vec<Question>,
}

impl Kyc {
    pub const LEN: usize = 1 + // status
        32 +  // owner
        32 +  // token account
        4 + 500 +  // threshold key
        8 +  // validity
        4 + 500 +  // data
        4 + 600; // questions

    pub const SEEDS: &'static str = "zkme_kyc";

    pub fn update(
        &mut self,
        threshold_key: String,
        validity: u64,
        data: String,
        questions: Vec<String>,
    ) {
        self.threshold_key = threshold_key;
        self.validity = validity;
        self.data = data;
        self.questions = questions
            .iter()
            .filter_map(|s| AnchorDeserialize::deserialize(&mut s.as_bytes()).ok())
            .collect();
    }
}
