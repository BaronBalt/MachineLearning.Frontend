use yew::prelude::*;
use web_sys::HtmlSelectElement;
use crate::models::ml_model::MlModel;

#[derive(Properties, PartialEq)]
pub struct MlModelListProps {
    pub ml_models: Vec<MlModel>,
    pub selected_id: Option<usize>,
    pub on_change: Callback<MlModel>,
}

#[component]
pub fn MlModelsList(props: &MlModelListProps) -> Html {
    let onchange = {
        let ml_models = props.ml_models.clone();
        let on_change = props.on_change.clone();

        Callback::from(move |e: Event| {
            let select = e.target_unchecked_into::<HtmlSelectElement>();
            let value = select.value();

            if let Some(model) = ml_models.iter().find(|m| m.id.to_string() == value) {
                on_change.emit(model.clone());
            }
        })
    };

    html! {
        <select {onchange}>
            <option style="display:none"> { "-- Select a model --" } </option>
            {
                props.ml_models.iter().map(|ml_model| {
                    html! {
                        <option
                            key={ml_model.id.clone()}
                            value={ml_model.id.clone().to_string()}
                            selected={Some(ml_model.id.clone()) == props.selected_id}
                        >
                            { &ml_model.name }
                        </option>
                    }
                }).collect::<Html>()
            }
        </select>
    }
}
