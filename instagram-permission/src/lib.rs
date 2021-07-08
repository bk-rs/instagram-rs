use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum InstagramPermission {
    // echo 'instagram_graph_user_media' | sed -r 's/(^|_)([a-z])/\U\2/g'
    InstagramGraphUserMedia,
    InstagramGraphUserProfile,
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
