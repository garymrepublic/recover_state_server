use std::fmt::{Display, Formatter};
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use sqlx::types::BigDecimal;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StoredExitProof {
    pub chain_id: i16,
    pub account_id: i64,
    pub sub_account_id: i16,
    pub l1_target_token: i32,
    pub l2_source_token: i32,
    pub proof: Option<Value>,
    pub amount: Option<BigDecimal>,
    pub created_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
}

impl Display for StoredExitProof {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "[account_id({}), sub_account_id({}), l2_source_token({})] => [chain_id({}), l1_target_token({})],",
            self.account_id, self.sub_account_id, self.l2_source_token, self.chain_id, self.l1_target_token,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StoredExitInfo {
    pub chain_id: i16,
    pub account_id: i64,
    pub sub_account_id: i16,
    pub l1_target_token: i32,
    pub l2_source_token: i32,
}

impl Display for StoredExitInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "[account_id({}), sub_account_id({}), l2_source_token({})] => [chain_id({}), l1_target_token({})],",
            self.account_id, self.sub_account_id, self.l2_source_token, self.chain_id, self.l1_target_token,
        )
    }
}

impl From<&StoredExitProof> for StoredExitInfo {
    fn from(value: &StoredExitProof) -> Self {
        Self{
            chain_id: value.chain_id,
            account_id: value.account_id,
            sub_account_id: value.sub_account_id,
            l1_target_token: value.l1_target_token,
            l2_source_token: value.l2_source_token,
        }
    }
}
