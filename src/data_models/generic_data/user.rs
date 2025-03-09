use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMeta {
    pub demat_consent: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub access_token: String,
    pub api_key: String,
    pub avatar_url: Option<String>,
    pub broker: String,
    pub email: String,
    pub enctoken: String,
    pub exchanges: Vec<String>,
    pub login_time: PrimitiveDateTime,
    pub meta: UserMeta,
    pub order_types: Vec<String>,
    pub products: Vec<String>,
    pub public_token: String,
    pub refresh_token: String,
    pub user_id: String,
    pub user_name: String,
    pub user_shortname: String,
    pub user_type: String,
}

