use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct ShortLink {
    pub id: u32,
    pub url: String,
}
