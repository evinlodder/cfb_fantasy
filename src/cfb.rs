use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Conference {
    pub id: u32,
    pub name: String,
    pub short_name: String,
    pub abbreviation: Option<String>,
    pub classification: String,
}
