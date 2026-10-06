use serde::{Serialize, Deserialize};
use crate::enums::typesafe_state::TypesafeState;
use crate::enums::typesafe_question::Question;
use std::collections::HashMap;


#[derive(Debug, Serialize, Deserialize)]
pub struct TypesafeRequest {
    pub state: TypesafeState,
    pub model: String,
    pub questions: HashMap<String, Question>,
}