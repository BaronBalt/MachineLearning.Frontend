use serde::Deserialize;
use yew::AttrValue;

#[derive(Clone, PartialEq, Deserialize)]
pub struct MlModel {
    pub id: usize,
    pub name: AttrValue,
    pub description: AttrValue,
    pub version: AttrValue,
    pub url: AttrValue,
}