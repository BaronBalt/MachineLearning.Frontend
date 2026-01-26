use yew::prelude::*;

use crate::components::{ml_model_details::MlModelDetails, ml_models_list::MlModelsList};
use crate::models::ml_model::MlModel;
use crate::services::api::fetch_ml_models;

#[component]
pub fn App() -> Html {
    let ml_models = use_state(|| vec![]);

    {
       let ml_models = ml_models.clone();
       use_effect_with((), move |_| {
           fetch_ml_models(ml_models, "/tutorial/data.json");
           || ()
       });
    }

    let selected_model = use_state(|| None);

    let on_model_select = {
        let selected_model = selected_model.clone();
        Callback::from(move |ml_model: MlModel| {
            selected_model.set(Some(ml_model));
        })
    };

    html! {
        <>
            <h1>{ "Machine Learning Frontend" }</h1>

            <div>
                <h3>{ "Models" }</h3>
                <MlModelsList ml_models={(*ml_models).clone()} on_click={on_model_select} />
            </div>

            if let Some(ml_model) = &*selected_model {
                <MlModelDetails ml_model={ml_model.clone()} />
            }
        </>
    }
}
