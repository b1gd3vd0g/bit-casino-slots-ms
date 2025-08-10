use axum::{
    Json,
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    handlers::{helper::authenticate_player, responses::MessageResponse},
    machines::{SlotMachine, byte_builder::ByteBuilder},
    requests::currency::{request_payout, request_wager},
};

#[derive(Deserialize)]
pub struct SpinByteBuilderReqBody {
    multiplier: u32,
}

#[derive(Serialize)]
pub struct SpinByteBuilderResponse {
    payout: i128,
    byte: String,
    event: Option<String>,
    balance: u128,
}

/// This handles a player's request to spin the Byte Builder slot machine.
/// - Authenticates the player via JWT Bearer authentication
/// - Pulls the `wager` (`128 * multiplier`) out of the player's bit wallet.
/// - Spins the slot machine and calculates the `payout`.
/// - Pays the player back `wager + payout` bits.
/// # Arguments
/// - `headers`: The HTTP request headers. Should include a token in the Authorization header.
/// - `body`: The HTTP request body.
pub async fn handle_spin_byte_builder(
    headers: HeaderMap,
    Json(body): Json<SpinByteBuilderReqBody>,
) -> Response {
    let Ok((id, token)) = authenticate_player(&headers).await else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(MessageResponse::new("Token authentication failed.")),
        )
            .into_response();
    };

    let spin_id = Uuid::new_v4();
    let Ok(bal) = request_wager(&token, body.multiplier, spin_id).await else {
        return (
            StatusCode::CONFLICT,
            Json(MessageResponse::new("You do not have enough money.")),
        )
            .into_response();
    };

    let mut machine = ByteBuilder::new();
    machine.set_mult(body.multiplier);
    machine.spin();
    let payout = machine.payout();

    let Ok(bal) = request_payout(&token, body.multiplier, payout, spin_id).await else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MessageResponse::new("The machine ate your bits! Oh no!")),
        )
            .into_response();
    };

    (
        StatusCode::OK,
        Json(SpinByteBuilderResponse {
            payout: payout,
            byte: machine.byte().to_bitstring(),
            event: machine.event(),
            balance: bal,
        }),
    )
        .into_response()
}
