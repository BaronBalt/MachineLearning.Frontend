use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct AlgorithmParameterDef {
    pub name: String,
    pub label: String,
    pub default_value: String,
    /// HTML input type, e.g. "number" or "text"
    pub param_type: String,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct Algorithm {
    pub id: String,
    pub name: String,
    pub parameters: Vec<AlgorithmParameterDef>,
}

/// Carries the selected algorithm id + current parameter values out of AlgorithmSelector.
#[derive(Clone, PartialEq)]
pub struct AlgorithmSelection {
    pub algorithm_id: String,
    pub params: Vec<(String, String)>,
}
