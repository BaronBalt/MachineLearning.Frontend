use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub value: String,
}