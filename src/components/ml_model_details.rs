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

    html! {
        <div>
            <h3>{ &props.ml_model.name }</h3>
            <div>
                { for props.ml_model.parameters.iter().map(|param| {
                    let on_input_change = on_input_change.clone();
                    let name = param.name.clone();
                    let oninput = move |e: InputEvent| {
                        let value = e.target_unchecked_into::<HtmlInputElement>().value();
                        on_input_change.emit((name.clone(), value));
                    };
                    html! {
                        <div key={param.name.clone()} class="parameter-row">
                            <label> { &param.name } </label>
                            <input type="text" value={param.value.clone()} {oninput} />
                        </div>
                    }
                }) }
            </div>
        </div>
    }
}
