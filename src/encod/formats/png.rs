use opencv::imgcodecs::{ 
    ImwriteFlags, ImwritePNGFlags
};

const DEFAULT_COMPRESSION: i32 = 1;
const DEFAULT_STRATEGY: ImwritePNGFlags = ImwritePNGFlags::IMWRITE_PNG_STRATEGY_RLE;

impl Default for super::Png {
    fn default() -> Self {
        Self { 
            compression: DEFAULT_COMPRESSION,
            strategy: DEFAULT_STRATEGY,
            is_bilevel: false,
        }
    }
}

impl super::Png {
    pub fn with_compression(
        compression: impl Into<i32>,
        is_bilevel: bool,
    ) -> Option<Self> {
        let compression: i32 = compression.into();
        if compression >= 0 && compression <= 9 {
            Some(Self {
                compression,
                strategy: ImwritePNGFlags::IMWRITE_PNG_STRATEGY_DEFAULT,
                is_bilevel,
            })
        } else {
            None
        }
    }

    pub fn with_strategy(
        strategy: ImwritePNGFlags,
        is_bilevel: bool,
    ) -> Self {
        Self {
            compression: DEFAULT_COMPRESSION,
            strategy,
            is_bilevel,
        }
    }
}

impl super::ImgFormat for super::Png {
    fn ext(&self) -> &'static str { ".png" }

    fn params(&self) -> Vec<i32> {
        let mut params: Vec<i32> = vec![];
        if self.compression != DEFAULT_COMPRESSION {
            params.push(ImwriteFlags::IMWRITE_PNG_COMPRESSION as i32);
            params.push(self.compression);
        }
        if self.strategy != DEFAULT_STRATEGY {
            params.push(ImwriteFlags::IMWRITE_PNG_STRATEGY as i32);
            params.push(self.strategy as i32);
        }
        if self.is_bilevel {
            params.push(ImwriteFlags::IMWRITE_PNG_BILEVEL as i32);
            params.push(1);
        }
        params
    }
}
