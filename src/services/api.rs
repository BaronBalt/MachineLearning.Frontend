use crate::models::{
    ml_model::MlModel,
    training_algorithm_model::Algorithm,
    training_files_model::TrainingFile,
};
use crate::models::ml_result::MlResult;
use crate::models::parameter::Parameter;
use std::string::String;
use gloo_net::Error;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::{FormData, console};
use yew::prelude::*;
use crate::services::config::API_BASE_URL;

/// Fetches ML models from the given URL and updates the provided state.
/// Currently, hardcoded for demo purposes.
pub async fn fetch_ml_models() -> Result<Vec<MlModel>, Error> {
    // Uncomment the below code if you want to fetch from a real API
    use gloo_net::http::Request;

    let url = format!("{}/{}", API_BASE_URL, "models");

    let result = Request::get(&url).send().await?;
    let models = result.json::<Vec<MlModel>>().await?;
    Ok(models)
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
    loading: UseStateHandle<bool>,
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
        loading.set(false);
    });
}

pub fn train_model(
    form_data: FormData,
) {
    let url = format!("{}/{}", API_BASE_URL, "train");


    use gloo_net::http::Request;
    use wasm_bindgen_futures::spawn_local;


    spawn_local(async move {
        let result = Request::post(&url)
            .body(form_data).expect("REASON")
            .send()
            .await;

        match result {
            Ok(resp) => {
                if resp.ok() {
                    console::log_1(&JsValue::from_str("Model training initiated successfully."));
                } else {
                    console::error_1(&JsValue::from_str(&format!("Training failed: HTTP {}", resp.status())));
                }
            }
            Err(e) => console::error_1(&JsValue::from_str(&format!("Request failed: {:?}", e))),
        }


    });

}

pub async fn fetch_training_files() -> Result<Vec<TrainingFile>, Error> {
    use gloo_net::http::Request;
    let url = format!("{}/{}", API_BASE_URL, "training-files");
    
    let resp = Request::get(&url).send().await?;
    let training_files = resp.json::<Vec<TrainingFile>>().await?;
    Ok(training_files)
    
    // Ok(vec![
    //     TrainingFile {
    //         name: "training_data_1".into(),
    //         filename: "training_data_1.csv".into(),
    //     },
    //     TrainingFile {
    //         name: "training_data_2".into(),
    //         filename: "training_data_2.csv".into(),
    //     },
    // ])
}

pub async fn fetch_training_algorithms() -> Result<Vec<Algorithm>, Error> {
    use gloo_net::http::Request;
    let url = format!("{}/{}", API_BASE_URL, "train");

    let resp = Request::get(&url).send().await?;
    let algorithms = resp.json::<Vec<Algorithm>>().await?;
    Ok(algorithms)

    // Ok(vec![
    //     Algorithm {
    //         id: "RANDOM_FORREST".into(),
    //         name: "Random Forest".into(),
    //         parameters: vec![
    //         ],
    //     },
    //     Algorithm {
    //         id: "LOGISTIC_REGRESSION".into(),
    //         name: "Logistic Regression".into(),
    //         parameters: vec![
    //             AlgorithmParameterDef { name: "classes".into(), label: "Classes (List)".into(), default_value: "[0,1,2]".into(), param_type: "text".into() },
    //         ],
    //     },
    // ])
}
