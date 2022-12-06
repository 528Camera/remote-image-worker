use opencv::imgcodecs::ImwriteFlags;

const LOSSLESS_QUALITY: i32 = 100;

impl Default for super::Webp {
    fn default() -> Self {
        Self {
            quality: LOSSLESS_QUALITY,
        }
    }
}

impl super::Webp {
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

impl super::ImgFormat for super::Webp {
    fn ext(&self) -> &'static str { ".webp" }

    fn params(&self) -> Vec<i32> {
        vec![
            ImwriteFlags::IMWRITE_WEBP_QUALITY as i32,
            self.quality,
        ]
    }
}