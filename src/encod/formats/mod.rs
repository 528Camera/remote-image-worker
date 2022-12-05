pub mod png;
pub mod jpg;
pub mod webp;

pub trait ImgFormat {
    fn ext() -> &'static str;
    fn params(&self) -> Vec<i32>;
}

/// Supported image formats.
pub enum Formats {
    Png(png::Png),
    Jpg(jpg::Jpg),
    Webp(webp::Webp),
}

impl Formats {
    pub fn ext(&self) -> &'static str {
        match self {
            Formats::Png(_) => png::Png::ext(),
            Formats::Jpg(_) => jpg::Jpg::ext(),
            Formats::Webp(_) => webp::Webp::ext(),
        }
    }

    pub fn params(&self) -> Vec<i32> {
        match self {
            Formats::Png(png) => png.params(),
            Formats::Jpg(jpg) => jpg.params(),
            Formats::Webp(webp) => webp.params(),
        }
    }
}
