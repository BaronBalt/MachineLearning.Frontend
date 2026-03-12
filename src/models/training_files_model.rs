
use serde::Deserialize;
use yew::AttrValue;

#[derive(Clone, PartialEq, Deserialize)]
pub struct TrainingFile {
    pub name: AttrValue,
    pub filename: AttrValue,
}
