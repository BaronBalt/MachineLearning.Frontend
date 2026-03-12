use crate::{
    components::algorithm_selector::AlgorithmSelector,
    models::{
        ml_model::MlModel,
        training_algorithm_model::{Algorithm, AlgorithmSelection},
        training_files_model::TrainingFile,
    },
    services::api::{
        fetch_ml_models, fetch_training_algorithms, fetch_training_files, poll_training_status,
        train_model,
    },
};
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MlModelListProps {
    pub on_ml_models_change: Callback<Vec<MlModel>>,
    pub on_error: Callback<String>,
    pub on_model_trained: Callback<MlModel>,
    pub on_success: Callback<String>,
}

#[component]
pub fn MlTrainForm(props: &MlModelListProps) -> Html {
    let use_file = use_state(|| false);
    let is_loading = use_state(|| false);
    let on_toggle = {
        let use_file = use_file.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            use_file.set(input.checked());
        })
    };

    let files: UseStateHandle<Vec<TrainingFile>> = use_state(std::vec::Vec::new);
    let algorithms: UseStateHandle<Vec<Algorithm>> = use_state(std::vec::Vec::new);
    let algorithm_selection: UseStateHandle<Option<AlgorithmSelection>> = use_state(|| None);
    let target_column = use_state(String::new);
    let name_field = use_state(String::new);

    // let version_field = use_state(String::new);
    let file_name_field = use_state(String::new);
    let file_ref = use_node_ref();

    {
        let files = files.clone();
        let on_error = props.on_error.clone();
        use_effect_with((), move |_| {
            let files = files.clone();
            let on_error = on_error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match fetch_training_files().await {
                    Ok(training) => files.set(training),
                    Err(msg) => on_error.emit(msg),
                }
            });
            || ()
        });
    }

    {
        let algorithms = algorithms.clone();
        let on_error = props.on_error.clone();
        use_effect_with((), move |_| {
            let algorithms = algorithms.clone();
            let on_error = on_error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match fetch_training_algorithms().await {
                    Ok(training) => algorithms.set(training),
                    Err(msg) => on_error.emit(msg),
                }
            });
            || ()
        });
    }

    // form

    let on_submit = {
        let use_file = use_file.clone();
        let file_ref = file_ref.clone();
        let name_field = name_field.clone();
        let version_field = "1";
        let file_name_field = file_name_field.clone();
        let is_loading = is_loading.clone();
        let on_ml_models_change = props.on_ml_models_change.clone();
        let on_error = props.on_error.clone();
        let on_model_trained = props.on_model_trained.clone();
        let on_success = props.on_success.clone();
        let algorithm_selection = algorithm_selection.clone();
        let target_column = target_column.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let is_loading = is_loading.clone();
            let on_ml_models_change = on_ml_models_change.clone();
            let on_error = on_error.clone();
            let on_model_trained = on_model_trained.clone();
            let on_success = on_success.clone();
            let trained_name = (*name_field).clone();

            if trained_name.trim().is_empty() {
                on_error.emit("Please enter a name for the model.".to_string());
                return;
            }

            is_loading.set(true);

            let form_data = web_sys::FormData::new().unwrap();

            if *use_file {
                web_sys::console::log_1(&"Using file".to_string().into());

                if target_column.is_empty() {
                    web_sys::window()
                        .unwrap()
                        .alert_with_message("Please enter the target column name.")
                        .unwrap();
                    is_loading.set(false);
                    return;
                } else {
                    form_data
                        .append_with_str("target_column", &(*target_column))
                        .unwrap();
                }

                let input = file_ref.cast::<web_sys::HtmlInputElement>();
                if input.is_none() {
                    web_sys::console::log_1(&"file_ref is not a file input".to_string().into());
                }
                if let Some(input) = input {
                    let files = input.files();
                    match files.is_none() {
                        true => {
                            web_sys::console::log_1(&"files property is None".to_string().into());
                        }
                        false => {
                            web_sys::console::log_1(
                                &format!("files.length: {}", files.unwrap().length()).into(),
                            );
                        }
                    }
                }

                let input = file_ref.cast::<web_sys::HtmlInputElement>();
                if let Some(files) = input.unwrap().files()
                    && let Some(file) = files.get(0)
                {
                    web_sys::console::log_1(&format!("Selected: {}", file.name()).into());
                    form_data.append_with_blob("file", &file).unwrap();
                    web_sys::console::log_1(&format!("file.name: {}", file.name()).into());
                    web_sys::console::log_1(&format!("file.size: {}", file.size()).into());
                    web_sys::console::log_1(&format!("file.type: {}", file.type_()).into());
                } else {
                    web_sys::console::log_1(&"No file selected".to_string().into());
                }
            } else {
                web_sys::console::log_1(&"NOT Using file".to_string().into());
                web_sys::console::log_1(&format!("Selected: {}", *file_name_field).into());
                let input = (*file_name_field).clone();
                if input.is_empty() {
                    web_sys::window()
                        .unwrap()
                        .alert_with_message("Please select a training data source.")
                        .unwrap();
                    return;
                }
                form_data.append_with_str("file_name", &input).unwrap();
            }

            form_data.append_with_str("model", &name_field).unwrap();
            form_data.append_with_str("version", version_field).unwrap();

            if let Some(ref selection) = *algorithm_selection {
                web_sys::console::log_1(&format!("Algorithm id: {}", &selection.algorithm_id).into());
                form_data
                    .append_with_str("algorithm", &selection.algorithm_id)
                    .unwrap();
                for (key, value) in &selection.params {
                    form_data.append_with_str(key, value).unwrap();
                }
            }

            wasm_bindgen_futures::spawn_local(async move {
                match train_model(form_data).await {
                    Ok(job_id) => {
                        // Poll until the training job completes or fails
                        loop {
                            gloo_timers::future::TimeoutFuture::new(2000).await;
                            match poll_training_status(&job_id).await {
                                Ok(job) if job.status == "complete" => {
                                    match fetch_ml_models().await {
                                        Ok(models) => {
                                            let trained = models.iter()
                                                .find(|m| m.name.as_ref() == trained_name.as_str())
                                                .cloned();
                                            on_ml_models_change.emit(models);
                                            if let Some(model) = trained {
                                                on_success.emit(format!("Model \"{}\" trained successfully.", model.name));
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
                                Ok(_) => {
                                    // still pending or running — keep polling
                                }
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

    let on_algorithm_change = {
        let algorithm_selection = algorithm_selection.clone();
        Callback::from(move |selection: Option<AlgorithmSelection>| {
            algorithm_selection.set(selection);
        })
    };

    let file_name_field_onchange = {
        let file_name_field = file_name_field.clone();
        Callback::from(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            file_name_field.set(select.value());
            web_sys::console::log_1(&format!("Selected file: {}", select.value()).into());
        })
    };
    let name_field_onchange = {
        let name_field = name_field.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            name_field.set(input.value());
        })
    };

    let target_column_onchange = {
        let target_column = target_column.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            target_column.set(input.value());
        })
    };

    html! {
        <div>
            <form onsubmit={on_submit}>
                <div>
                    <label>
                        {"What should the name be?"}
                    </label>
                    <input type="text" required=true value={(*name_field).clone()} oninput={name_field_onchange} placeholder="Model name"/>
                </div>
                <label>
                    {"Select Training Data Source"}
                </label>

                <div>

                    <input
                        type="checkbox"
                        role="switch"
                        checked={*use_file}
                        onchange={on_toggle}
                    />
                </div>

                <div>
                    <label>
                        {
                        if *use_file {
                        "Upload Training Data (.csv)"
                        } else {
                        "Existing Training Data"
                        }
                        }

                    </label>
                    {
                    if *use_file {
                    html! {
                    <>
                    <input type="file" accept=".csv,text/csv" ref={file_ref}/>
                    <input type="text" value={(*target_column).clone()} oninput={target_column_onchange} placeholder="Target Column Name"/>
                    </>
                    }
                    } else {
                    html! {
                    // get files in the database from the backend
                    <select onchange={file_name_field_onchange} value={(*file_name_field).clone()} required={true}>
                        <option value={""} selected={true} disabled={true}> { "-- Select a file --" } </option>
                        {files.iter().map(|file| {
                            html! {
                                <option value={file.filename.clone()}>
                                    { &file.name }
                                </option>
                            }
                        }).collect::<Html>()}
                    </select>
                    }
                    }
                    }
                </div>
                <AlgorithmSelector
                    algorithms={(*algorithms).clone()}
                    on_change={on_algorithm_change}
                />
                <button
                    aria-busy={if *is_loading {"true"} else {"false"}}
                    disabled={*is_loading}
                    type="submit"
                >
                    { if *is_loading { "Training…" } else { "Train Model" } }
                </button>
            </form>
        </div>
    }
}
