
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct MlResult {
    pub result: Vec<f32>
}
