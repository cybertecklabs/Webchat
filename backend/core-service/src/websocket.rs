use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use redis::AsyncCommands;

use axum::extract::Query;
use std::collections::HashMap;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use crate::auth::Claims;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<HashMap<String, String>>,
    State((_db, redis_client)): State<(mongodb::Database, redis::Client)>,
) -> impl IntoResponse {
    let token = params.get("token").cloned();
    let mut user_id = None;

    if let Some(token_str) = token {
        if let Ok(secret) = std::env::var("JWT_SECRET") {
            if let Ok(token_data) = decode::<Claims>(
                &token_str,
                &DecodingKey::from_secret(secret.as_ref()),
                &Validation::new(Algorithm::HS256),
            ) {
                user_id = Some(token_data.claims.sub);
            }
        }
    }

    if user_id.is_none() {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    ws.on_upgrade(move |socket| handle_socket(socket, redis_client, user_id.unwrap()))
}

async fn handle_socket(socket: WebSocket, redis_client: redis::Client, user_id: String) {
    let (mut sender, mut receiver) = socket.split();

    // Subscribe to Redis channel for real-time messages
    let mut pubsub_conn = redis_client
        .get_async_connection()
        .await
        .unwrap()
        .into_pubsub();
    pubsub_conn.subscribe("chat:messages").await.unwrap();
    let mut pubsub_stream = pubsub_conn.on_message();

    // Task to forward Redis messages to WebSocket
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = pubsub_stream.next().await {
            let payload: String = msg.get_payload().unwrap();
            if sender.send(Message::Text(payload)).await.is_err() {
                break;
            }
        }
    });

    // Task to receive WebSocket messages and publish to Redis
    let mut recv_task = tokio::spawn(async move {
        let mut conn = redis_client.get_async_connection().await.unwrap();
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            // Wrap the incoming text in a proper message format
            let chat_msg = serde_json::json!({
                "user_id": user_id,
                "content": text,
                "created_at": chrono::Utc::now(),
            });
            
            if let Ok(msg_json) = serde_json::to_string(&chat_msg) {
                let _: () = conn.publish("chat:messages", msg_json).await.unwrap();
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }
}
