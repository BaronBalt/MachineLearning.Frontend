use crate::models::ml_model::MlModel;
use crate::services::api::{fetch_ml_models, predict, train_model};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MlModelDetailsProps {
    pub ml_model: MlModel,
    pub on_ml_models_change: Callback<Vec<MlModel>>,
    #[prop_or_default]
    pub on_change: Callback<MlModel>,
}

#[component]
pub fn MlModelDetails(props: &MlModelDetailsProps) -> Html {
    let selected_version = use_state(|| props.ml_model.version.last().cloned());
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

        Callback::from(move |_| {
            // Send to training but without version and the file for training.
            // I will use the previous training data.
            let is_loading = is_loading.clone();
            is_loading.set(true);
            let on_ml_models_change = on_ml_models_change.clone();

            let form_data = web_sys::FormData::new().unwrap();

            form_data.append_with_str("model", &model_name).unwrap();
            wasm_bindgen_futures::spawn_local(async move {
                train_model(form_data);

                let models = fetch_ml_models().await;
                match models {
                    Ok(models) => {
                        for model in &models {
                            web_sys::console::log_1(
                                &format!("Model: {} (ID: {})", model.name, model.id).into(),
                            );
                        }
                        web_sys::console::log_1(
                            &"Fetched updated models list after training".to_string().into(),
                        );
                        on_ml_models_change.emit(models);
                        web_sys::console::log_1(
                            &"Model trained further and models list updated"
                                .to_string()
                                .into(),
                        );
                    }
                    Err(err) => web_sys::console::error_1(
                        &format!("Error fetching models: {:?}", err).into(),
                    ),
                }
            });

            is_loading.set(false);
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
                    html! {
                        <div>
                            <button aria-busy={if *is_loading {"true"} else {"false"} } type="button" onclick={on_train_further}>
                                { "Train Further" }
                            </button>
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
                            <button aria-busy={if (*is_loading).clone() {"true"} else {"false"} } type="button" onclick={on_click}>
                                { "Predict" }
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
