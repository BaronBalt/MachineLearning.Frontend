use yew::prelude::*;
use crate::models::ml_model::MlModel;

#[derive(Properties, PartialEq)]
pub struct MlModelDetailsProps {
    pub ml_model: MlModel
}

#[component]
pub fn MlModelDetails(MlModelDetailsProps { ml_model }: &MlModelDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ &*ml_model.name }</h3>
            <p>{ &*ml_model.version }</p>
        </div>
    }
}
