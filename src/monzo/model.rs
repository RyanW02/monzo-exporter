use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug)]
pub struct GetBalance {
    // I'm assuming overdrafts are negative, can't find any docs for it
    pub balance: i64,
    pub total_balance: i64,
    pub balance_including_flexible_savings: i64,
    pub currency: Box<str>,
    pub spend_today: u64,
    pub local_currency: Box<str>,
    pub local_exchange_rate: u64,
    pub local_spend: Vec<LocalSpend>,
}

#[derive(Deserialize, Debug)]
pub struct LocalSpend {
    pub spend_today: u64,
    pub currency: Box<str>,
}

#[derive(Serialize, Debug)]
pub struct RefreshBody {
    pub grant_type: Box<str>,
    pub client_id: Box<str>,
    pub client_secret: Box<str>,
    pub refresh_token: Box<str>,
}

impl RefreshBody {
    pub fn new(client_id: Box<str>, client_secret: Box<str>, refresh_token: Box<str>) -> RefreshBody {
        RefreshBody {
            grant_type: "refresh_token".to_owned().into_boxed_str(),
            client_id,
            client_secret,
            refresh_token,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    pub access_token: Box<str>,
    pub client_id: Box<str>,
    pub expires_in: usize,
    pub refresh_token: Box<str>,
    pub token_type: Box<str>,
    pub user_id: Box<str>,
}

#[derive(Serialize, Debug)]
pub struct ExchangeBody {
    pub grant_type: Box<str>,
    pub client_id: Box<str>,
    pub client_secret: Box<str>,
    pub redirect_uri: Box<str>,
    pub code: Box<str>,
}

impl ExchangeBody {
    pub fn new(client_id: Box<str>, client_secret: Box<str>, redirect_uri: Box<str>, code: Box<str>) -> ExchangeBody {
        ExchangeBody {
            grant_type: "authorization_code".to_owned().into_boxed_str(),
            client_id,
            client_secret,
            redirect_uri,
            code,
        }
    }
}