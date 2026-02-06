use crate::models::ml_model::MlModel;
use crate::models::ml_result::MlResult;
use crate::models::parameter::Parameter;
use std::string::String;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;
use crate::services::config::API_BASE_URL;

/// Fetches ML models from the given URL and updates the provided state.
/// Currently, hardcoded for demo purposes.
pub fn fetch_ml_models(ml_models_state: UseStateHandle<Vec<MlModel>>, _url: &str) {
    // Uncomment the below code if you want to fetch from a real API
    use gloo_net::http::Request;
    use wasm_bindgen_futures::spawn_local;

    let url = format!("{}{}", API_BASE_URL, _url);

    spawn_local(async move {
        let result = Request::get(&url).send().await;

        match result {
            Ok(resp) => match resp.json::<Vec<MlModel>>().await {
                Ok(models) => ml_models_state.set(models),
                Err(e) => println!("JSON parse failed: {:?}", e),
            },
            Err(e) => println!("Request failed: {:?}", e),
        }
    });
    // Backend returns a list of these
    // MlModel {
    //     id: 1,
    //     name: "Demo Model A".into(),
    //     description: "A sample ML model for demonstration.".into(),
    //     version: vec!["1.0".into(), "2.0".into(), "3.0".into()],
    //     parameters: vec![
    //         Parameter {
    //             name: "learning_rate".into(),
    //             value: "0.01".into(),
    //         },
    //         Parameter {
    //             name: "max_depth".into(),
    //             value: "5".into(),
    //         },
    //     ],
    //     url: "<URL>".into(),
    // }

}

/// This is also hardcoded for now until backend API gets set up
pub fn predict(
    predict_text_state: UseStateHandle<Option<String>>,
    ml_model: MlModel,
    version: &str,
    _params: Vec<Parameter>,
    _url: &str,
) {
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
            Ok(resp) => match resp.json::<MlResult>().await {
                Ok(model_resp) => {
                    // return only the `result` field as JSON
                    let text = serde_json::to_string(&model_resp.result).unwrap_or_default();
                    predict_text_state.set(Some(text));
                }
                Err(e) => {
                    console::error_1(&JsValue::from_str(&format!("JSON parse failed: {:?}", e)))
                }
            },
            Err(e) => console::error_1(&JsValue::from_str(&format!("Request failed: {:?}", e))),
        }
    });
}
