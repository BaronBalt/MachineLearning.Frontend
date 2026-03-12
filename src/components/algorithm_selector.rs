use crate::models::training_algorithm_model::{Algorithm, AlgorithmSelection};
use std::collections::HashMap;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AlgorithmSelectorProps {
    pub algorithms: Vec<Algorithm>,
    pub on_change: Callback<Option<AlgorithmSelection>>,
}

#[function_component]
pub fn AlgorithmSelector(props: &AlgorithmSelectorProps) -> Html {
    let selected_id = use_state(String::new);
    let param_values: UseStateHandle<HashMap<String, String>> = use_state(HashMap::new);

    let selected_algorithm = props
        .algorithms
        .iter()
        .find(|a| a.id == *selected_id)
        .cloned();

    let on_algorithm_change = {
        let selected_id = selected_id.clone();
        let param_values = param_values.clone();
        let on_change = props.on_change.clone();
        let algorithms = props.algorithms.clone();
        Callback::from(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            let id = select.value();

            let mut defaults = HashMap::new();
            if let Some(algo) = algorithms.iter().find(|a| a.id == id) {
                for p in &algo.parameters {
                    defaults.insert(p.name.clone(), p.default_value.clone());
                }
                on_change.emit(Some(AlgorithmSelection {
                    algorithm_id: id.clone(),
                    params: defaults.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
                }));
            } else {
                on_change.emit(None);
            }

            param_values.set(defaults);
            selected_id.set(id);
        })
    };

    html! {
        <div>
            <label>{"Algorithm"}</label>
            <select onchange={on_algorithm_change} value={(*selected_id).clone()} required=true>
                <option value="" disabled={true} selected={true}>{"-- Select an algorithm --"}</option>
                { for props.algorithms.iter().map(|algo| html! {
                    <option value={algo.id.clone()}>{ &algo.name }</option>
                })}
            </select>

            { if let Some(algo) = selected_algorithm {
                let params_html = algo.parameters.iter().map(|param| {
                    let param_name = param.name.clone();
                    let selected_id_val = (*selected_id).clone();
                    let param_values_state = param_values.clone();
                    let on_change = props.on_change.clone();

                    let on_param_change = {
                        let param_name = param_name.clone();
                        let param_values_state = param_values_state.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            let mut new_values = (*param_values_state).clone();
                            new_values.insert(param_name.clone(), input.value());
                            let params_vec: Vec<(String, String)> =
                                new_values.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                            on_change.emit(Some(AlgorithmSelection {
                                algorithm_id: selected_id_val.clone(),
                                params: params_vec,
                            }));
                            param_values_state.set(new_values);
                        })
                    };

                    let current_value = param_values_state
                        .get(&param_name)
                        .cloned()
                        .unwrap_or_else(|| param.default_value.clone());

                    html! {
                        <div key={param.name.clone()}>
                            <label>{ &param.label }</label>
                            <input
                                type={param.param_type.clone()}
                                value={current_value}
                                oninput={on_param_change}
                            />
                        </div>
                    }
                }).collect::<Html>();

                html! { <div>{ params_html }</div> }
            } else {
                html! {}
            }}
        </div>
    }
}
