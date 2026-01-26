use yew::prelude::*;
use crate::models::ml_model::MlModel;

#[derive(Properties, PartialEq)]
pub struct MlModelListProps {
    pub ml_models: Vec<MlModel>,
    pub on_click: Callback<MlModel>,
}

#[component]
pub fn MlModelsList(MlModelListProps { ml_models, on_click }: &MlModelListProps) -> Html {
    let on_select = |ml_model: &MlModel| {
        let on_click = on_click.clone();
        let ml_model = ml_model.clone();
        Callback::from(move |_| on_click.emit(ml_model.clone()))
    };

    html! {
        for ml_model in ml_models {
            <p key={ml_model.id} onclick={on_select(ml_model)}>
                {format!("{}, Version: {}", ml_model.name, ml_model.version)}
            </p>
        }
    }
}
