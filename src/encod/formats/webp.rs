use opencv::imgcodecs::ImwriteFlags;
use super::ImgFormat;

pub struct Webp {
    quality: i32,
}

const LOSSLESS_QUALITY: i32 = 100;

impl Default for Webp {
    fn default() -> Self {
        Self {
            quality: LOSSLESS_QUALITY,
        }
    }
}

impl Webp {
    pub fn new(
        quality: impl Into<i32>,
    ) -> Option<Self> {
        let quality: i32 = quality.into();
        if quality >= 1 {
            Some(Self {
                quality,
            })
        } else {
            None
        }
    }
}

impl ImgFormat for Webp {
    fn ext() -> &'static str { ".webp" }

    fn params(&self) -> Vec<i32> {
        vec![
            ImwriteFlags::IMWRITE_WEBP_QUALITY as i32,
            self.quality,
        ]
    }
}