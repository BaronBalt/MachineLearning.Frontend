use std::string::String;
use crate::models::ml_model::MlModel;
use yew::prelude::*;
use crate::models::parameter::Parameter;

/// Fetches ML models from the given URL and updates the provided state.
/// Currently, hardcoded for demo purposes.
pub fn fetch_ml_models(ml_models_state: UseStateHandle<Vec<MlModel>>, _url: &str) {
    // Uncomment the below code if you want to fetch from a real API
    /*
    use gloo_net::http::Request;
    use wasm_bindgen_futures::spawn_local;
    use serde_json::json;
    use crate::services::config::API_BASE_URL;

    let ml_models_state = ml_models_state.clone();
    let url = format!("{}{}", API_BASE_URL, _url);

    spawn_local(async move {
        let fetched_ml_models: Vec<MlModel> = Request::get(&url)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        ml_models_state.set(fetched_ml_models);
    });
    */

    // Hardcoded demo data
    let hardcoded_models = vec![
        MlModel {
            id: 1,
            name: "Demo Model A".into(),
            description: "A sample ML model for demonstration.".into(),
            version: vec![
                "1.0".into(),
                "2.0".into(),
                "3.0".into(),
            ],
            parameters: vec![
                Parameter { name: "learning_rate".into(), value: "0.01".into() },
                Parameter { name: "max_depth".into(), value: "5".into() },
            ],
            url: "<URL>".into()
        },
        MlModel {
            id: 2,
            name: "Demo Model B".into(),
            description: "Another sample ML model.".into(),
            version: vec![
                "1.0".into(),
                "2.0".into(),
                "3.0".into(),
            ],
            parameters: vec![
                Parameter { name: "learning_rate".into(), value: "0.01".into() },
                Parameter { name: "max_depth".into(), value: "5".into() },
            ],
            url: "<URL>".into()
        },
        MlModel {
            id: 3,
            name: "Demo Model C".into(),
            description: "Yet another demo ML model.".into(),
            version: vec![
                "1.0".into(),
                "2.0".into(),
                "3.0".into(),
            ],
            parameters: vec![
                Parameter { name: "learning_rate".into(), value: "0.01".into() },
                Parameter { name: "max_depth".into(), value: "5".into() },
            ],
            url: "<URL>".into()
        },
    ];

    ml_models_state.set(hardcoded_models);
}

/// This is also hardcoded for now until backend API gets set up
pub fn predict(predict_text_state: UseStateHandle<Option<String>>, ml_model: MlModel, params: Vec<Parameter>, _url: &str) {
    let hardcoded_output = match ml_model.id {
        1 => Some("3".to_string()),
        2 => Some("2".to_string()),
        3 => Some("9".to_string()),
        _ => Some("69".to_string()),
    };
    predict_text_state.set(hardcoded_output);
}
