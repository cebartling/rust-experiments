use crate::proxy::ProxyState;
use crate::toxic::{Toxic, ToxicConfig};
use crate::toxics::corrupt::CorruptToxic;
use crate::toxics::latency::LatencyToxic;
use crate::toxics::slow_close::SlowCloseToxic;
use axum::{
    extract::State,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
struct ProxyConfig {
    name: String,
    listen: String,
    upstream: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateToxicRequest {
    proxy: String,
    config: ToxicConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct ToxicResponse {
    proxy: String,
    toxic_type: String,
    config: ToxicConfig,
}

// REST API handlers
async fn list_toxics(
    State(state): State<ProxyState>,
) -> Json<Vec<ToxicResponse>> {
    let state = state.lock().unwrap();
    let mut responses = Vec::new();

    for (proxy_name, toxics) in state.iter() {
        for toxic in toxics {
            responses.push(ToxicResponse {
                proxy: proxy_name.clone(),
                toxic_type: toxic.get_type(),
                config: toxic.get_config(),
            });
        }
    }

    Json(responses)
}

async fn add_toxic(
    State(state): State<ProxyState>,
    Json(request): Json<CreateToxicRequest>,
) -> Json<ToxicResponse> {
    let toxic: Arc<dyn Toxic> = match request.config {
        ToxicConfig::Latency { latency_ms } => Arc::new(LatencyToxic {
            latency: Duration::from_millis(latency_ms),
        }),
        ToxicConfig::Corrupt { probability } => Arc::new(CorruptToxic { probability }),
        ToxicConfig::SlowClose { delay_ms } => Arc::new(SlowCloseToxic {
            delay: Duration::from_millis(delay_ms),
        }),
    };

    let mut state = state.lock().unwrap();
    let toxics = state
        .entry(request.proxy.clone())
        .or_insert_with(Vec::new);

    let response = ToxicResponse {
        proxy: request.proxy,
        toxic_type: toxic.get_type(),
        config: toxic.get_config(),
    };

    toxics.push(toxic);
    Json(response)
}

