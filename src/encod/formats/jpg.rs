use opencv::imgcodecs::ImwriteFlags;

const DISABLE_RST_INTERVAL: i32 = 0;
const DISABLE_LUMA_QUALITY: i32 = -1;
const DISABLE_CHROMA_QUALITY: i32 = -1;

impl Default for super::Jpg {
    fn default() -> Self {
        Self { 
            quality: 95,
            is_progressive: false,
            is_optimized: false,
            restart_interval: DISABLE_RST_INTERVAL,
            luma_quality: DISABLE_LUMA_QUALITY,
            chroma_quality: DISABLE_CHROMA_QUALITY,
        }
    }
}

impl super::Jpg {
    pub fn new(
        quality: impl Into<i32>,
        is_progressive: bool,
        is_optimized: bool,
        restart_interval: impl Into<i32>,
        luma_quality: impl Into<i32>,
        chroma_quality: impl Into<i32>,
    ) -> Option<Self> {
        let quality: i32 = quality.into();
        let restart_interval: i32 = restart_interval.into();
        let luma_quality: i32 = luma_quality.into();
        let chroma_quality: i32 = chroma_quality.into();
        if quality >= 0 && quality <= 100
        && restart_interval >= 0 && restart_interval <= 65535
        && luma_quality >= -1 && luma_quality <= 100
        && chroma_quality >= -1 && chroma_quality <= 100 
        {
            Some(Self {
                quality,
                is_progressive,
                is_optimized,
                restart_interval,
                luma_quality,
                chroma_quality,
            })
        } else {
            None
        }
    }
}

impl super::ImgFormat for super::Jpg {
    fn ext(&self) -> &'static str { ".jpg" }

    fn params(&self) -> Vec<i32> {
        let mut params = vec![
            ImwriteFlags::IMWRITE_JPEG_QUALITY as i32,
            self.quality,
        ];
        if self.is_progressive {
            params.push(ImwriteFlags::IMWRITE_JPEG_PROGRESSIVE as i32);
            params.push(1);
        }
        if self.is_optimized {
            params.push(ImwriteFlags::IMWRITE_JPEG_OPTIMIZE as i32);
            params.push(1);
        }
        if self.restart_interval != DISABLE_RST_INTERVAL {
            params.push(ImwriteFlags::IMWRITE_JPEG_RST_INTERVAL as i32);
            params.push(self.restart_interval);
        }
        if self.luma_quality != DISABLE_LUMA_QUALITY {
            params.push(ImwriteFlags::IMWRITE_JPEG_LUMA_QUALITY as i32);
            params.push(self.luma_quality);
        }
        if self.chroma_quality != DISABLE_CHROMA_QUALITY {
            params.push(ImwriteFlags::IMWRITE_JPEG_CHROMA_QUALITY as i32);
            params.push(self.chroma_quality);
        }
        params
    }
}
