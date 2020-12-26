use std::fmt;

use url::{ParseError, Url};

mod media_metadata;
pub use media_metadata::MediaMetadata;

#[derive(PartialEq, Debug)]
pub enum MediaLink {
    Post {
        metadata: MediaMetadata,
    },
    Story {
        metadata: MediaMetadata,
        owner_username: String,
    },
    StoryHighlight {
        metadata: MediaMetadata,
        highlight_id: Option<u64>,
    },
    IGTVVideo {
        metadata: MediaMetadata,
    },
    Reel {
        metadata: MediaMetadata,
    },
}

#[derive(PartialEq, Debug)]
pub enum MediaLinkParseError {
    UrlParseError(ParseError),
    Invalid(String),
    Unsupported,
}

impl fmt::Display for MediaLinkParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UrlParseError(err) => write!(f, "UrlParseError {}", err),
            Self::Invalid(msg) => write!(f, "Invalid {}", msg),
            Self::Unsupported => write!(f, "Unsupported"),
        }
    }
}

impl MediaLink {
    pub fn parse(url: impl AsRef<str>) -> Result<Self, MediaLinkParseError> {
        let url = url.as_ref();
        let url = Url::parse(url).map_err(MediaLinkParseError::UrlParseError)?;

        if url.scheme() != "https" {
            return Err(MediaLinkParseError::Invalid("scheme mismatch".to_owned()));
        }

        if url.host_str() != Some("www.instagram.com") && url.host_str() != Some("instagram.com") {
            return Err(MediaLinkParseError::Invalid("host mismatch".to_owned()));
        }

        let mut s = url.path().to_owned();
        s.remove(0);
        let offset = s
            .find(|c: char| c == '/')
            .ok_or(MediaLinkParseError::Unsupported)?;
        let path_l1: String = s.drain(..offset).collect();

        match path_l1.as_str() {
            "p" | "tv" | "reel" => {
                s.remove(0);
                let shortcode = if let Some(offset) = s.find(|c: char| c == '/') {
                    s.drain(..offset).collect::<String>()
                } else {
                    s.to_owned()
                };

                let metadata = MediaMetadata::with_shortcode(shortcode)
                    .map_err(|_| MediaLinkParseError::Invalid("shortcode invalid".to_owned()))?;

                match path_l1.as_str() {
                    "p" => Ok(Self::Post { metadata: metadata }),
                    "tv" => Ok(Self::IGTVVideo { metadata: metadata }),
                    "reel" => Ok(Self::Reel { metadata: metadata }),
                    _ => unreachable!(),
                }
            }
            "stories" => {
                s.remove(0);
                let offset = s
                    .find(|c: char| c == '/')
                    .ok_or(MediaLinkParseError::Invalid(
                        "owner_username not found".to_owned(),
                    ))?;
                let owner_username: String = s.drain(..offset).collect();

                s.remove(0);
                let ig_id = if let Some(offset) = s.find(|c: char| c == '/') {
                    s.drain(..offset).collect::<String>()
                } else {
                    s.to_owned()
                };
                let ig_id: u64 = ig_id
                    .parse()
                    .map_err(|_| MediaLinkParseError::Invalid("ig_id invalid".to_owned()))?;

                let metadata = MediaMetadata::with_ig_id(ig_id);

                Ok(Self::Story {
                    metadata: metadata,
                    owner_username: owner_username,
                })
            }
            "s" => {
                s.remove(0);
                let highlight_b64_encoded = if let Some(offset) = s.find(|c: char| c == '/') {
                    s.drain(..offset).collect::<String>()
                } else {
                    s.to_owned()
                };

                let mut highlight_id: Option<u64> = None;
                if let Ok(Ok(s)) = base64::decode(highlight_b64_encoded).map(String::from_utf8) {
                    let mut split = s.split(":");
                    if split.next() == Some("highlight") {
                        if let Some(Ok(id)) = split.next().map(|x| x.parse::<u64>()) {
                            highlight_id = Some(id);
                        }
                    }
                }

                let (_, ig_id) = url
                    .query_pairs()
                    .filter(|(k, _)| k == "story_media_id")
                    .next()
                    .ok_or(MediaLinkParseError::Invalid("ig_id not found".to_owned()))?;

                let ig_id: u64 = ig_id
                    .parse()
                    .map_err(|_| MediaLinkParseError::Invalid("ig_id invalid".to_owned()))?;

                let metadata = MediaMetadata::with_ig_id(ig_id);

                Ok(Self::StoryHighlight {
                    metadata: metadata,
                    highlight_id: highlight_id,
                })
            }
            _ => Err(MediaLinkParseError::Unsupported),
        }
    }
}

impl MediaLink {
    pub fn get_metadata(&self) -> &MediaMetadata {
        match self {
            Self::Post { metadata } => metadata,
            Self::Story {
                metadata,
                owner_username: _,
            } => metadata,
            Self::StoryHighlight {
                metadata,
                highlight_id: _,
            } => metadata,
            Self::IGTVVideo { metadata } => metadata,
            Self::Reel { metadata } => metadata,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_post() -> Result<(), String> {
        let link = MediaLink::parse("https://www.instagram.com/p/CJBsZ11MYha/?igshid=ffffffffffff")
            .map_err(|err| err.to_string())?;
        assert_eq!(
            link,
            MediaLink::Post {
                metadata: MediaMetadata {
                    ig_id: 2468449360609904730,
                    shortcode: "CJBsZ11MYha".to_owned(),
                    is_public_shortcode: Some(true)
                }
            }
        );

        let link = MediaLink::parse("https://www.instagram.com/p/CJBsZ11MYha?igshid=ffffffffffff")
            .map_err(|err| err.to_string())?;
        assert_eq!(
            link,
            MediaLink::Post {
                metadata: MediaMetadata {
                    ig_id: 2468449360609904730,
                    shortcode: "CJBsZ11MYha".to_owned(),
                    is_public_shortcode: Some(true)
                }
            }
        );

        let link = MediaLink::parse("https://www.instagram.com/p/CH5LLEGnhWDZpMs--h6rwCecLT3So9_ZOwTKCk0/?igshid=ffffffffffff")
            .map_err(|err| err.to_string())?;
        assert_eq!(
            link,
            MediaLink::Post {
                metadata: MediaMetadata {
                    ig_id: 2448037011284432259,
                    shortcode: "CH5LLEGnhWDZpMs--h6rwCecLT3So9_ZOwTKCk0".to_owned(),
                    is_public_shortcode: Some(false)
                }
            }
        );

        Ok(())
    }

    #[test]
    fn test_parse_tv() -> Result<(), String> {
        let link =
            MediaLink::parse("https://www.instagram.com/tv/CJEivokDjPR/?igshid=ffffffffffff")
                .map_err(|err| err.to_string())?;
        assert_eq!(
            link,
            MediaLink::IGTVVideo {
                metadata: MediaMetadata {
                    ig_id: 2469251302657242065,
                    shortcode: "CJEivokDjPR".to_owned(),
                    is_public_shortcode: Some(true)
                }
            }
        );

        Ok(())
    }

    #[test]
    fn test_parse_reel() -> Result<(), String> {
        let link =
            MediaLink::parse("https://www.instagram.com/reel/CH-__hxDV7T/?igshid=ffffffffffff")
                .map_err(|err| err.to_string())?;
        assert_eq!(
            link,
            MediaLink::Reel {
                metadata: MediaMetadata {
                    ig_id: 2449676689849802451,
                    shortcode: "CH-__hxDV7T".to_owned(),
                    is_public_shortcode: Some(true)
                }
            }
        );

        Ok(())
    }

    #[test]
    fn test_parse_story() -> Result<(), String> {
        let link =
            MediaLink::parse("https://instagram.com/stories/foo/1/?utm_source=ig_story_item_share&igshid=ffffffffffff")
                .map_err(|err| err.to_string())?;
        assert_eq!(
            link,
            MediaLink::Story {
                metadata: MediaMetadata {
                    ig_id: 1,
                    shortcode: "B".to_owned(),
                    is_public_shortcode: None,
                },
                owner_username: "foo".to_owned()
            }
        );

        let link =
            MediaLink::parse("https://instagram.com/stories/foo/1?utm_source=ig_story_item_share&igshid=ffffffffffff")
                .map_err(|err| err.to_string())?;
        assert_eq!(
            link,
            MediaLink::Story {
                metadata: MediaMetadata {
                    ig_id: 1,
                    shortcode: "B".to_owned(),
                    is_public_shortcode: None,
                },
                owner_username: "foo".to_owned()
            }
        );

        Ok(())
    }

    #[test]
    fn test_parse_story_highlight() -> Result<(), String> {
        let link =
            MediaLink::parse("https://www.instagram.com/s/aGlnaGxpZ2h0OjE4MDY2MTI4ODAzMTg4MjY3/?igshid=ffffffffffff&story_media_id=1")
                .map_err(|err| err.to_string())?;
        assert_eq!(
            link,
            MediaLink::StoryHighlight {
                metadata: MediaMetadata {
                    ig_id: 1,
                    shortcode: "B".to_owned(),
                    is_public_shortcode: None,
                },
                highlight_id: Some(18066128803188267)
            }
        );

        let link =
            MediaLink::parse("https://www.instagram.com/s/aGlnaGxpZ2h0OjE4MDY2MTI4ODAzMTg4MjY3?igshid=ffffffffffff&story_media_id=1")
                .map_err(|err| err.to_string())?;
        assert_eq!(
            link,
            MediaLink::StoryHighlight {
                metadata: MediaMetadata {
                    ig_id: 1,
                    shortcode: "B".to_owned(),
                    is_public_shortcode: None,
                },
                highlight_id: Some(18066128803188267)
            }
        );

        Ok(())
    }
}
