use crate::models::parameter::Parameter;
use serde::Deserialize;
use yew::AttrValue;

#[derive(Clone, PartialEq, Deserialize)]
pub struct MlModel {
    pub id: usize,
    pub name: AttrValue,
    pub description: AttrValue,
    pub version: Vec<AttrValue>,
    pub parameters: Vec<Parameter>,
    pub url: AttrValue,
}