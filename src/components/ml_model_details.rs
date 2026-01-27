use web_sys::HtmlInputElement;
use crate::models::ml_model::MlModel;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MlModelDetailsProps {
    pub ml_model: MlModel,
    #[prop_or_default]
    pub on_change: Callback<MlModel>,
}

#[component]
pub fn MlModelDetails(props: &MlModelDetailsProps) -> Html {
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

    html! {
        <div>
            <hr />
            <h3>{ &props.ml_model.name }</h3>
            <div>
                <label>{ "Version" }</label>
                <select>
                    {
                        props.ml_model.version.iter().map(|v| {
                            html! {
                                <option value={v.clone()}>{ v }</option>
                            }
                        }).collect::<Html>()
                    }
                </select>
            </div>
            <div>
                { for props.ml_model.parameters.iter().map(|param| {
                    let on_input_change = on_input_change.clone();
                    let name = param.name.clone();
                    let oninput = move |e: InputEvent| {
                        let value = e.target_unchecked_into::<HtmlInputElement>().value();
                        on_input_change.emit((name.clone(), value));
                    };
                    html! {
                        <div>
                            <label>{ &param.name }</label>
                            <input type="text" value={param.value.clone()}
                                {oninput}
                            />
                        </div>
                    }
                }) }
            </div>
            <button> {"Predict"} </button>

            <hr />

            <article class="prediction">
                <header>
                    <strong>{ "Prediction" }</strong>
                </header>
                <output>
                    { prediction.as_deref().unwrap_or("—") }
                </output>
            </article>
        </div>
    }
}
