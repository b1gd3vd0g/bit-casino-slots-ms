use chrono::Utc;
use reqwest::{Client, StatusCode, header::HeaderMap};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const API_ROOT: &str = "http://currency-ms:3000";

#[derive(Serialize)]
pub struct TransactionRequestBody {
    amount: i128,
    reason: String,
}

impl TransactionRequestBody {
    fn wager(multiplier: u32, spin_id: Uuid) -> Self {
        Self {
            amount: -128 * multiplier as i128,
            reason: format!(
                "BYTE_BUILDER_WAGER DATE={} SPIN_ID={}",
                Utc::now().to_rfc3339(),
                spin_id
            ),
        }
    }
    fn payout(multiplier: u32, payout: i128, spin_id: Uuid) -> Self {
        Self {
            amount: 128 * multiplier as i128 + payout,
            reason: format!(
                "BYTE_BUILDER_PAYOUT DATE={} SPIN_ID={}",
                Utc::now().to_rfc3339(),
                spin_id
            ),
        }
    }
}

pub enum WagerRequestFailure {
    ReqFailed,
    Unauthorized,
    Conflict,
    ServerSide,
}

#[derive(Deserialize, Debug)]
pub struct TransactionResponse {
    balance: u128,
}

pub async fn request_wager(
    token: &str,
    multiplier: u32,
    spin_id: Uuid,
) -> Result<u128, WagerRequestFailure> {
    let client = Client::new();

    let mut hm = HeaderMap::new();
    hm.insert(
        "Authorization",
        format!("Bearer {}", token).parse().unwrap(),
    );

    let body = TransactionRequestBody::wager(multiplier, spin_id);

    let response = client
        .post(format!("{}/transaction", API_ROOT))
        .headers(hm)
        .json(&body)
        .send()
        .await;

    let Ok(response) = response else {
        return Err(WagerRequestFailure::ReqFailed);
    };
    println!("{:?}", response);
    Err(match response.status() {
        StatusCode::OK => {
            let balance: TransactionResponse = response.json().await.unwrap();
            return Ok(balance.balance);
        }

        StatusCode::CONFLICT => WagerRequestFailure::Conflict,
        StatusCode::UNAUTHORIZED => WagerRequestFailure::Unauthorized,
        _ => WagerRequestFailure::ServerSide,
    })
}

pub async fn request_payout(
    token: &str,
    multiplier: u32,
    payout: i128,
    spin_id: Uuid,
) -> Result<u128, WagerRequestFailure> {
    let client = Client::new();

    let mut hm = HeaderMap::new();
    hm.insert(
        "Authorization",
        format!("Bearer {}", token).parse().unwrap(),
    );

    let body = TransactionRequestBody::payout(multiplier, payout, spin_id);

    let response = client
        .post(format!("{}/transaction", API_ROOT))
        .headers(hm)
        .json(&body)
        .send()
        .await;

    let Ok(response) = response else {
        return Err(WagerRequestFailure::ReqFailed);
    };

    Err(match response.status() {
        StatusCode::OK => {
            let balance: TransactionResponse = response.json().await.unwrap();
            return Ok(balance.balance);
        }
        StatusCode::CONFLICT => WagerRequestFailure::Conflict,
        StatusCode::UNAUTHORIZED => WagerRequestFailure::Unauthorized,
        _ => WagerRequestFailure::ServerSide,
    })
}
