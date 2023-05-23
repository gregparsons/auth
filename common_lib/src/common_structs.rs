//! common_structs.rs

use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

pub static SESSION_USER_ID:&str = "session_user_id";
pub static SESSION_USERNAME:&str = "session_username";

#[derive(Serialize, Deserialize, Debug)]
pub struct FormData{
    pub username:String,
    pub password:String,
}
