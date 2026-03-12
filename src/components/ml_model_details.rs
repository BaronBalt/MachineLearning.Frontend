use crate::models::ml_model::MlModel;
use crate::services::api::{fetch_ml_models, poll_training_status, predict, train_model};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MlModelDetailsProps {
    pub ml_model: MlModel,
    pub on_ml_models_change: Callback<Vec<MlModel>>,
    #[prop_or_default]
    pub on_change: Callback<MlModel>,
    pub on_error: Callback<String>,
    pub on_model_trained: Callback<MlModel>,
    pub on_success: Callback<String>,
}

#[component]
pub fn MlModelDetails(props: &MlModelDetailsProps) -> Html {
    let selected_version = use_state(|| props.ml_model.version.last().cloned());

    // When the version list grows (e.g. after Train Further), jump to the new latest version.
    {
        let selected_version = selected_version.clone();
        let latest = props.ml_model.version.last().cloned();
        use_effect_with(props.ml_model.version.clone(), move |_| {
            selected_version.set(latest);
            || ()
        });
    }

    let on_selected_version_changed = {
        let selected_version = selected_version.clone();
        Callback::from(move |e: Event| {
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            selected_version.set(Some(value.into()));
        })
    };

    let on_input_change = {
        let on_change_prop = props.on_change.clone();
        let current_model = props.ml_model.clone(); // Clone from props

        Callback::from(move |(name, value): (String, String)| {
            let mut updated_model = current_model.clone();
            if let Some(param) = updated_model.parameters.iter_mut().find(|p| p.name == name) {
                param.value = value;
            }
            on_change_prop.emit(updated_model);
        })
    };
    let prediction = use_state(|| None::<String>);
    let is_loading = use_state(|| false);

    let on_click = {
        let prediction = prediction.clone();
        let ml_model = props.ml_model.clone();
        let params = props.ml_model.parameters.clone();
        let selected_version = selected_version.clone();
        let is_loading = is_loading.clone();
        let on_error = props.on_error.clone();

        Callback::from(move |_| {
            is_loading.set(true);
            if let Some(version) = &*selected_version {
                predict(
                    prediction.clone(),
                    ml_model.clone(),
                    version.as_ref(),
                    params.clone(),
                    "/predict", // i am overriding this -- Theodor
                    is_loading.clone(),
                    on_error.clone(),
                );
            }
        })
    };

    let selected_version_value: AttrValue = selected_version.as_ref().cloned().unwrap_or_default();

    let is_new_version = selected_version_value == "new_version";
    let on_ml_models_change = props.on_ml_models_change.clone();
    let on_train_further = {
        let is_loading = is_loading.clone();
        let model_name = props.ml_model.name.clone();
        let on_ml_models_change = on_ml_models_change.clone();
        let on_error = props.on_error.clone();
        let on_model_trained = props.on_model_trained.clone();
        let on_success = props.on_success.clone();

        Callback::from(move |_| {
            let is_loading = is_loading.clone();
            is_loading.set(true);
            let on_ml_models_change = on_ml_models_change.clone();
            let on_error = on_error.clone();
            let on_model_trained = on_model_trained.clone();
            let on_success = on_success.clone();
            let model_name = model_name.clone();

            let form_data = web_sys::FormData::new().unwrap();

            form_data.append_with_str("model", &model_name).unwrap();
            wasm_bindgen_futures::spawn_local(async move {
                match train_model(form_data).await {
                    Ok(job_id) => {
                        loop {
                            gloo_timers::future::TimeoutFuture::new(2000).await;
                            match poll_training_status(&job_id).await {
                                Ok(job) if job.status == "complete" => {
                                    match fetch_ml_models().await {
                                        Ok(models) => {
                                            let trained = models.iter()
                                                .find(|m| m.name == model_name)
                                                .cloned();
                                            on_ml_models_change.emit(models);
                                            if let Some(model) = trained {
                                                on_success.emit(format!("\"{}\" trained further successfully.", model.name));
                                                on_model_trained.emit(model);
                                            }
                                        }
                                        Err(msg) => on_error.emit(msg),
                                    }
                                    break;
                                }
                                Ok(job) if job.status == "failed" => {
                                    let msg = job.error.unwrap_or_else(|| "Training failed".to_string());
                                    on_error.emit(msg);
                                    break;
                                }
                                Ok(_) => {}
                                Err(msg) => {
                                    on_error.emit(msg);
                                    break;
                                }
                            }
                        }
                    }
                    Err(msg) => on_error.emit(msg),
                }
                is_loading.set(false);
            });
        })
    };

    html! {
        <form>
            <hr />

            <h3>{ &props.ml_model.name }</h3>

            <fieldset>
                <h4>{ "Version" }</h4>
                <label>
                    <select
                        onchange={on_selected_version_changed}
                        value={selected_version_value}
                    >
                        {
                            props.ml_model.version.iter().map(|v| {
                                html! {
                                    <option selected={Some(v.clone()) == selected_version.as_ref().cloned()} value={v.clone()}>{ v }</option>
                                }
                            }).collect::<Html>()
                        }
                        <option selected={false} value="new_version">{ "--- New Version ---" }</option>
                    </select>
                </label>
            </fieldset>

            {
                if is_new_version {
                    let is_logistic = props.ml_model.algorithm.as_deref()
                        .map(|a| a.eq_ignore_ascii_case("LOGISTIC_REGRESSION"))
                        .unwrap_or(false);
                    html! {
                        <div>
                            <button
                                aria-busy={if *is_loading {"true"} else {"false"}}
                                type="button"
                                onclick={on_train_further}
                                disabled={is_logistic || *is_loading}
                            >
                                { if *is_loading { "Training…" } else { "Train Further" } }
                            </button>
                            if is_logistic {
                                <small style="display: block; margin-top: 0.25rem; opacity: 0.7;">
                                    { "Logistic Regression models cannot be trained further." }
                                </small>
                            }
                        </div>
                    }
                } else {
                    html! {
                        <>
                            <fieldset>
                                <h4>{ "Parameters" }</h4>

                                {
                                    for props.ml_model.parameters.iter().map(|param| {
                                        let on_input_change = on_input_change.clone();
                                        let name = param.name.clone();

                                        let oninput = move |e: InputEvent| {
                                            let value = e.target_unchecked_into::<HtmlInputElement>().value();
                                            on_input_change.emit((name.clone(), value));
                                        };

                                        html! {
                                            <label>
                                                { &param.name }
                                                <input
                                                    type="text"
                                                    value={param.value.clone()}
                                                    {oninput}
                                                />
                                            </label>
                                        }
                                    })
                                }
                            </fieldset>
                            <button
                                aria-busy={if *is_loading {"true"} else {"false"}}
                                disabled={*is_loading}
                                type="button"
                                onclick={on_click}
                            >
                                { if *is_loading { "Predicting…" } else { "Predict" } }
                            </button>

                            <hr />

                            <article>
                                <header>
                                    <strong>{ "Prediction" }</strong>
                                </header>

                                <output>
                                    { prediction.as_deref().unwrap_or("—") }
                                </output>
                            </article>
                        </>
                    }
                }
            }
        </form>
    }
}
