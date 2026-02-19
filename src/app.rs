use yew::prelude::*;

use crate::components::{
    ml_model_details::MlModelDetails, ml_models_list::MlModelsList, ml_train_form::MlTrainForm,
};
use crate::models::ml_model::MlModel;
use crate::services::api::fetch_ml_models;

#[derive(Clone, PartialEq)]
pub enum ModelSelection {
    Model(MlModel),
    TrainNew,
}

#[component]
pub fn App() -> Html {
    let ml_models = use_state(|| vec![]);
    let selected_model = use_state(|| None::<ModelSelection>);

    {
        let ml_models = ml_models.clone();
        use_effect_with((), move |_| {
            fetch_ml_models(ml_models, "/models");
            || ()
        });
    }

    let on_model_select = {
        let selected_model = selected_model.clone();
        Callback::from(move |selection: ModelSelection| {
            selected_model.set(Some(selection));
        })
    };

    let on_model_save = {
        let ml_models = ml_models.clone();
        let selected_model = selected_model.clone();
        Callback::from(move |updated_model: MlModel| {
            let mut list = (*ml_models).clone();
            if let Some(pos) = list.iter().position(|m| m.id == updated_model.id) {
                list[pos] = updated_model.clone();
                ml_models.set(list);
            }

            selected_model.set(Some(ModelSelection::Model(updated_model)));
        })
    };

    html! {
        <body>
            <main>
                <h1>{ "Machine Learning Frontend" }</h1>
                <div>
                    <h3>{ "Models" }</h3>

                    <MlModelsList
                        ml_models={(*ml_models).clone()}
                        on_change={on_model_select}
                    />
                </div>

                { (*selected_model).as_ref().map(|selection| {
                    match selection {
                        ModelSelection::TrainNew => html! {
                            <MlTrainForm />
                        },
                        ModelSelection::Model(model) => html! {
                            <MlModelDetails
                                key={model.id.clone()}
                                ml_model={model.clone()}
                                on_change={on_model_save.clone()}
                            />
                        }
                    }
                }) }
            </main>
        </body>
    }
}
