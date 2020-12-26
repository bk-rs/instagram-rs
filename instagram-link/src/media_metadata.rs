use instagram_media_shortcode::{ig_id_to_shortcode, is_private_shortcode, shortcode_to_ig_id};

#[derive(PartialEq, Debug, Clone)]
pub struct MediaMetadata {
    pub ig_id: u64,
    pub shortcode: String,
    pub is_public_shortcode: Option<bool>,
}

impl MediaMetadata {
    pub fn with_ig_id(ig_id: u64) -> Self {
        Self {
            ig_id,
            shortcode: ig_id_to_shortcode(ig_id),
            is_public_shortcode: None,
        }
    }

    pub fn with_shortcode(shortcode: String) -> Result<Self, String> {
        let ig_id = shortcode_to_ig_id(&shortcode)?;
        let is_public_shortcode = Some(!is_private_shortcode(&shortcode));

        Ok(Self {
            ig_id,
            shortcode,
            is_public_shortcode,
        })
    }
}
