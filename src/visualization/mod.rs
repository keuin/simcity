use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use futures::{SinkExt, StreamExt};
use serde_json::json;
use std::net::SocketAddr;
use std::sync::{atomic, Arc};
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

use crate::simulation::simulation::Simulation;

#[derive(Clone)]
pub struct AppState {
    pub simulation: Arc<Mutex<Simulation>>,
    pub running: Arc<atomic::AtomicBool>,
}

pub async fn start_server(
    simulation: Arc<Mutex<Simulation>>,
    addr: SocketAddr,
) -> Result<(), anyhow::Error> {
    let state = AppState {
        simulation,
        running: Arc::new(atomic::AtomicBool::new(false)),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route("/api/city", get(get_city))
        .route("/api/start", get(start_simulation))
        .route("/api/stop", get(stop_simulation))
        .fallback_service(ServeDir::new("frontend/dist"))
        .layer(cors)
        .with_state(state);

    log::info!("starting visualization server on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn ws_handler<'a>(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

/// real-time city grid display
/// send updates through websocket
async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    tokio::spawn(async move { while let Some(message) = receiver.next().await {} });

    // subscribe to simulation updates
    let mut rx = {
        let sim = state.simulation.lock().await;
        sim.subscribe()
    };

    // fan-out updates to client
    'receive_updates: loop {
        match rx.recv().await {
            Ok(update) => {
                let json = serde_json::to_string(&update).unwrap();
                if let Err(why) = sender.send(Message::Text(json.into())).await {
                    log::error!("failed to send update: {:?}", why);
                    break 'receive_updates;
                }
            }
            Err(_) => break 'receive_updates,
        }
    }
}

async fn get_city(State(state): State<AppState>) -> Json<serde_json::Value> {
    let city = {
        let sim = state.simulation.lock().await;
        sim.city.clone()
    };

    Json(json!({
        "width": city.width,
        "height": city.height,
        "cells": city.cells,
    }))
}

async fn start_simulation(State(state): State<AppState>) -> Json<serde_json::Value> {
    if let Err(_) = state.running.clone().compare_exchange(
        false,
        true,
        atomic::Ordering::SeqCst,
        atomic::Ordering::SeqCst,
    ) {
        return Json(json!({ "status": "already_running" }));
    }
    let new_running = Arc::clone(&state.running);
    tokio::spawn(async move {
        Simulation::run(Arc::clone(&state.simulation), new_running).await;
    });
    Json(json!({ "status": "started" }))
}

async fn stop_simulation(State(state): State<AppState>) -> Json<serde_json::Value> {
    state
        .running
        .clone()
        .store(false, atomic::Ordering::Relaxed);
    Json(json!({ "status": "stopped" }))
}
