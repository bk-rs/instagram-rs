use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Deserialize_enum_str, Serialize_enum_str, PartialEq, Eq, Hash, Debug, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum InstagramPermission {
    // echo 'instagram_graph_user_media' | sed -r 's/(^|_)([a-z])/\U\2/g'
    InstagramGraphUserMedia,
    InstagramGraphUserProfile,
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde::Deserialize;

    #[test]
    fn test_de() {
        #[derive(Deserialize)]
        struct Foo {
            permission: InstagramPermission,
        }

        assert_eq!(
            serde_json::from_str::<Foo>(r#"{"permission": "instagram_graph_user_media"}"#)
                .unwrap()
                .permission,
            InstagramPermission::InstagramGraphUserMedia
        );

        assert_eq!(
            serde_json::from_str::<Foo>(r#"{"permission": "instagram_graph_user_profile"}"#)
                .unwrap()
                .permission,
            InstagramPermission::InstagramGraphUserProfile
        );
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            InstagramPermission::InstagramGraphUserMedia.to_string(),
            "instagram_graph_user_media"
        );
    }
}
