use web_sys::HtmlInputElement;
use crate::models::ml_model::MlModel;
use yew::prelude::*;
use crate::services::api::predict;

#[derive(Properties, PartialEq)]
pub struct MlModelDetailsProps {
    pub ml_model: MlModel,
    #[prop_or_default]
    pub on_change: Callback<MlModel>,
}

#[component]
pub fn MlModelDetails(props: &MlModelDetailsProps) -> Html {
    let selected_version = use_state(|| props.ml_model.version.last().cloned());
    let on_selected_version_changed = {
        let selected_version = selected_version.clone();
        Callback::from(move |e: Event| {
            let value = e
                .target_unchecked_into::<HtmlInputElement>()
                .value();
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

    let on_click = {
        let prediction = prediction.clone();
        let ml_model = props.ml_model.clone();
        let params = props.ml_model.parameters.clone();
        let selected_version = selected_version.clone();

        Callback::from(move |_| {
            if let Some(version) = &*selected_version {
                predict(
                    prediction.clone(),
                    ml_model.clone(),
                    version.as_ref(),
                    params.clone(),
                    "/predict"
                );
            }
        })
    };

    let selected_version_value: AttrValue = selected_version
        .as_ref()
        .cloned()
        .unwrap_or_default();

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
                                    <option value={v.clone()}>{ v }</option>
                                }
                            }).collect::<Html>()
                        }
                    </select>
                </label>
            </fieldset>
            // <hr />
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
            <button type="button" onclick={on_click}>
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
        </form>
    }
}
