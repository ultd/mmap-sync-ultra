use serde::{Deserialize, Serialize};

/// Example data-structure shared between writer and reader(s)
#[derive(Serialize, Deserialize)]
pub struct HelloWorld {
    pub version: u32,
    pub messages: Vec<String>,
}
