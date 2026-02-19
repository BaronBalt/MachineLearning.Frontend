use crate::app::ModelSelection;
use crate::models::ml_model::MlModel;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MlModelListProps {
    pub ml_models: Vec<MlModel>,
    pub on_change: Callback<ModelSelection>,
}

#[component]
pub fn MlModelsList(props: &MlModelListProps) -> Html {
    let select_value = use_state(|| None::<String>);
    let onchange = {
        let ml_models = props.ml_models.clone();
        let on_change = props.on_change.clone();
        let select_value = select_value.clone();

        Callback::from(move |e: Event| {
            let select = e.target_unchecked_into::<HtmlSelectElement>();
            let value = select.value();
            select_value.set(select.value().into());

            if value == "new" {
                on_change.emit(ModelSelection::TrainNew);
            } else if let Some(model) = ml_models.iter().find(|m| m.id.to_string() == value) {
                on_change.emit(ModelSelection::Model(model.clone()));
            }
        })
    };

    html! {
        <select {onchange} id={"train_form"} value={(*select_value).clone()}>
            <option disabled=true selected=true> { "-- Select a model --" } </option>
            {
                props.ml_models.iter().map(|ml_model| {
                    html! {
                        <option value={ml_model.id.to_string()}>
                            { &ml_model.name }
                        </option>
                    }
                }).collect::<Html>()
            }
            <option value="new"> { "-- Train New Model --" } </option>
        </select>
    }
}
