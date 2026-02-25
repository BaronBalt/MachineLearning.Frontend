use crate::{
    components::algorithm_selector::AlgorithmSelector,
    models::{
        ml_model::MlModel,
        training_algorithm_model::{Algorithm, AlgorithmSelection},
        training_files_model::TrainingFile,
    },
    services::api::{
        fetch_ml_models, fetch_training_algorithms, fetch_training_files, train_model,
    },
};
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MlModelListProps {
    pub on_ml_models_change: Callback<Vec<MlModel>>,
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
        use_effect_with((), move |_| {
            let files = files.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let training_files = fetch_training_files().await;

                match training_files {
                    Ok(training) => {
                        files.set(training);
                    }
                    Err(err) => web_sys::console::error_1(
                        &format!("Error fetching models: {:?}", err).into(),
                    ),
                }
            });
            || ()
        });
    }

    {
        let algorithms = algorithms.clone();
        use_effect_with((), move |_| {
            let algorithms = algorithms.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let training_algorithms = fetch_training_algorithms().await;

                match training_algorithms {
                    Ok(training) => algorithms.set(training),
                    Err(err) => web_sys::console::error_1(
                        &format!("Error fetching algorithms: {:?}", err).into(),
                    ),
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
        let algorithm_selection = algorithm_selection.clone();
        let target_column = target_column.clone();
        Callback::from(move |e: SubmitEvent| {
            let is_loading = is_loading.clone();
            is_loading.set(true);
            let on_ml_models_change = on_ml_models_change.clone();
            e.prevent_default();

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
                form_data
                    .append_with_str("algorithm", &selection.algorithm_id)
                    .unwrap();
                for (key, value) in &selection.params {
                    form_data.append_with_str(key, value).unwrap();
                }
            }

            wasm_bindgen_futures::spawn_local(async move {
                train_model(form_data);
                gloo_timers::future::TimeoutFuture::new(1000).await;

                let models = fetch_ml_models().await;
                match models {
                    Ok(models) => {
                        for model in &models {
                            web_sys::console::log_1(
                                &format!("Model: {} (ID: {})", model.name, model.id).into(),
                            );
                        }
                        web_sys::console::log_1(
                            &"Fetched updated models list after training"
                                .to_string()
                                .into(),
                        );
                        on_ml_models_change.emit(models);
                        web_sys::console::log_1(
                            &"Model created and models list updated".to_string().into(),
                        );
                    }
                    Err(err) => web_sys::console::error_1(
                        &format!("Error fetching models: {:?}", err).into(),
                    ),
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
                    <input type="text" value={(*name_field).clone()} oninput={name_field_onchange}/>
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
                <button aria-busy={if (*is_loading).clone() {"true"} else {"false"} } type="submit" >{ "Train Model" }</button>
            </form>
        </div>
    }
}
