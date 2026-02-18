use crate::services::api::train_model;
use gloo_console::log;
use web_sys::{HtmlInputElement, HtmlSelectElement, console::log};
use yew::prelude::*;

#[component]
pub fn MlTrainForm() -> Html {
    let use_file = use_state(|| false);
    let on_toggle = {
        let use_file = use_file.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            use_file.set(input.checked());
        })
    };

    // form
    let name_field = use_state(String::new);
    let version_field = use_state(String::new);
    let file_name_field = use_state(String::new);
    let file_ref = use_node_ref();

    let on_submit = {
        let use_file = use_file.clone();
        let file_ref = file_ref.clone();
        let name_field = name_field.clone();
        let version_field = version_field.clone();
        let file_name_field = file_name_field.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let form_data = web_sys::FormData::new().unwrap();

            if *use_file {
                web_sys::console::log_1(&"Using file".to_string().into());

                let input = file_ref.cast::<web_sys::HtmlInputElement>();
                if input.is_none() {
                    web_sys::console::log_1(&"file_ref is not a file input".to_string().into());
                }
                if let Some(input) = input {
                    let files = input.files();
                    if files.is_none() {
                        web_sys::console::log_1(&"files property is None".to_string().into());
                    } else {
                        web_sys::console::log_1(
                            &format!("files.length: {}", files.unwrap().length()).into(),
                        );
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
            form_data
                .append_with_str("version", &version_field)
                .unwrap();

            wasm_bindgen_futures::spawn_local(async move {
                train_model(form_data);
            });

            // Callback::from(move |_| {
            //     train_model(form_data);
            // })
        })
    };

    let file_name_field_onchange = {
        let file_name_field = file_name_field.clone();
        Callback::from(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            file_name_field.set(select.value());
        })
    };
    let name_field_onchange = {
        let name_field = name_field.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            name_field.set(input.value());
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
                    <input type="file" accept=".csv,text/csv" ref={file_ref}/>
                    }
                    } else {
                    html! {
                    // get files in the database from the backend
                    <select onchange={file_name_field_onchange} value={(*file_name_field).clone()}>
                        <option style="display:none"> { "-- Select a file --" } </option>
                        <option value="train.csv">{"Train"}</option>
                        <option value="train.csv">{"Train"}</option>
                    </select>
                    }
                    }
                    }
                </div>
                <div>
                    <label>
                        {"Version"}
                    </label>
                    <input type="number"
                        value={(*version_field).clone()}
                        oninput={Callback::from(move |e: InputEvent| {
                            let value = e.target_unchecked_into::<HtmlInputElement>().value();
                            version_field.set(value);
                        })}
                        />

                </div>
                <button type="submit" >{ "Train Model" }</button>
            </form>
        </div>
    }
}
