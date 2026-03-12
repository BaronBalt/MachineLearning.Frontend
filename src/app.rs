use yew::prelude::*;

use crate::components::{
    banner::{Banner, BannerVariant},
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
    let ml_models = use_state(std::vec::Vec::new);
    let selected_model = use_state(|| None::<ModelSelection>);
    let banner = use_state(|| None::<(String, BannerVariant)>);

    let on_error = {
        let banner = banner.clone();
        Callback::from(move |msg: String| {
            banner.set(Some((msg, BannerVariant::Error)));
        })
    };

    let on_success = {
        let banner = banner.clone();
        Callback::from(move |msg: String| {
            banner.set(Some((msg, BannerVariant::Success)));
        })
    };

    let on_dismiss = {
        let banner = banner.clone();
        Callback::from(move |_| {
            banner.set(None);
        })
    };

    {
        let ml_models = ml_models.clone();
        let on_error = on_error.clone();
        use_effect_with((), move |_| {
            let ml_models = ml_models.clone();
            let on_error = on_error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match fetch_ml_models().await {
                    Ok(models) => ml_models.set(models),
                    Err(msg) => on_error.emit(msg),
                }
            });
            || ()
        });
    }

    let on_model_trained = {
        let selected_model = selected_model.clone();
        Callback::from(move |model: MlModel| {
            selected_model.set(Some(ModelSelection::Model(model)));
        })
    };

    let on_ml_models_change = {
        let ml_models = ml_models.clone();
        Callback::from(move |updated_models: Vec<MlModel>| {
            ml_models.set(updated_models);
        })
    };

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

    let (banner_msg, banner_variant) = match &*banner {
        Some((msg, variant)) => (Some(msg.clone()), variant.clone()),
        None => (None, BannerVariant::Error),
    };

    html! {
        <body>
            <main>
                <h1>{ "Machine Learning Frontend" }</h1>

                <Banner message={banner_msg} on_dismiss={on_dismiss} variant={banner_variant} />

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
                            <MlTrainForm
                                on_ml_models_change={on_ml_models_change.clone()}
                                on_error={on_error.clone()}
                                on_model_trained={on_model_trained.clone()}
                                on_success={on_success.clone()}
                            />
                        },
                        ModelSelection::Model(model) => html! {
                            <MlModelDetails
                                key={model.id.clone()}
                                ml_model={model.clone()}
                                on_ml_models_change={on_ml_models_change.clone()}
                                on_change={on_model_save.clone()}
                                on_error={on_error.clone()}
                                on_model_trained={on_model_trained.clone()}
                                on_success={on_success.clone()}
                            />
                        }
                    }
                }) }
            </main>
        </body>
    }
}
