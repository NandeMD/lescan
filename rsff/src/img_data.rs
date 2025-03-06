use crate::serde_overwrites::b64_img_overwrite;
use serde::{Deserialize, Serialize};

/// A simple image container
#[derive(Default, Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct BalloonImage {
    pub img_type: String,
    #[serde(with = "b64_img_overwrite")]
    pub img_data: Vec<u8>,
}

pub type DocumentImage = Option<Vec<String>>;
