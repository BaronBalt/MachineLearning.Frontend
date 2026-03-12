use crate::models::{
    ml_model::MlModel,
    training_algorithm_model::Algorithm,
    training_files_model::TrainingFile,
};
use crate::models::ml_result::MlResult;
use crate::models::parameter::Parameter;
use std::string::String;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::{FormData, console};
use yew::prelude::*;
use crate::services::config::API_BASE_URL;

async fn parse_response<T: for<'de> serde::Deserialize<'de>>(
    resp: gloo_net::http::Response,
) -> Result<T, String> {
    if !resp.ok() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        let msg = serde_json::from_str::<serde_json::Value>(&body)
            .ok()
            .and_then(|v| v["error"].as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| format!("Request failed with status {}", status));
        return Err(msg);
    }
    resp.json::<T>().await.map_err(|e| e.to_string())
}

/// Fetches ML models from the given URL and updates the provided state.
/// Currently, hardcoded for demo purposes.
pub async fn fetch_ml_models() -> Result<Vec<MlModel>, String> {
    use gloo_net::http::Request;

    let url = format!("{}/{}", API_BASE_URL, "models");
    let resp = Request::get(&url).send().await.map_err(|e| e.to_string())?;
    parse_response(resp).await
}

/// This is also hardcoded for now until backend API gets set up
pub fn predict(
    predict_text_state: UseStateHandle<Option<String>>,
    ml_model: MlModel,
    version: &str,
    _params: Vec<Parameter>,
    _url: &str,
    loading: UseStateHandle<bool>,
    on_error: Callback<String>,
) {
    loading.set(true);
    let mut url = format!("{}/predict?model={}", API_BASE_URL, ml_model.name);
    if !version.is_empty() {
        url.push_str(&format!("&version={}", version));
    }
    let inputs: Vec<serde_json::Value> = _params
        .into_iter()
        .map(|p| match p.value.parse::<f64>() {
            Ok(n) => serde_json::Value::Number(serde_json::Number::from_f64(n).unwrap()),
            Err(_) => serde_json::Value::String(p.value),
        })
        .collect();

    let body = serde_json::json!({ "input": inputs });

    spawn_local(async move {
        let result: Result<gloo_net::http::Response, gloo_net::Error> =
            gloo_net::http::Request::post(&url)
                .header("Content-Type", "application/json")
                .body(body.to_string()).expect("REASON")
                .send()
                .await;

        match result {
            Ok(resp) => match parse_response::<MlResult>(resp).await {
                Ok(model_resp) => {
                    let text = serde_json::to_string(&model_resp.result).unwrap_or_default();
                    predict_text_state.set(Some(text));
                }
                Err(msg) => {
                    console::error_1(&JsValue::from_str(&msg));
                    on_error.emit(msg);
                }
            },
            Err(e) => {
                let msg = e.to_string();
                console::error_1(&JsValue::from_str(&msg));
                on_error.emit(msg);
            }
        }
        loading.set(false);
    });
}

pub async fn train_model(
    form_data: FormData,
) -> Result<(), String> {
    use gloo_net::http::Request;

    let url = format!("{}/{}", API_BASE_URL, "train");

    let result = Request::post(&url)
        .body(form_data).expect("REASON")
        .send()
        .await;

    match result {
        Ok(resp) => {
            if resp.ok() {
                console::log_1(&JsValue::from_str("Model training initiated successfully."));
                Ok(())
            } else {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_default();
                let msg = serde_json::from_str::<serde_json::Value>(&body)
                    .ok()
                    .and_then(|v| v["error"].as_str().map(|s| s.to_string()))
                    .unwrap_or_else(|| format!("Training failed with status {}", status));
                console::error_1(&JsValue::from_str(&msg));
                Err(msg)
            }
        }
        Err(e) => {
            let msg = e.to_string();
            console::error_1(&JsValue::from_str(&msg));
            Err(msg)
        }
    }
}

pub async fn fetch_training_files() -> Result<Vec<TrainingFile>, String> {
    use gloo_net::http::Request;
    let url = format!("{}/{}", API_BASE_URL, "training-files");
    let resp = Request::get(&url).send().await.map_err(|e| e.to_string())?;
    parse_response(resp).await
}

pub async fn fetch_training_algorithms() -> Result<Vec<Algorithm>, String> {
    use gloo_net::http::Request;
    let url = format!("{}/{}", API_BASE_URL, "train");
    let resp = Request::get(&url).send().await.map_err(|e| e.to_string())?;
    parse_response(resp).await
}
