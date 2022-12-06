use opencv::imgcodecs::ImwritePNGFlags;

pub mod png;
pub mod jpg;
pub mod webp;

pub trait ImgFormat {
    fn ext(&self) -> &'static str;
    fn params(&self) -> Vec<i32>;
}

pub struct Png {
    compression: i32,
    strategy: ImwritePNGFlags,
    is_bilevel: bool,
}

pub struct Jpg {
    quality: i32,
    is_progressive: bool,
    is_optimized: bool,
    restart_interval: i32,
    luma_quality: i32,
    chroma_quality: i32,
    // sampling_factor: not implemented yet
}

pub struct Webp {
    quality: i32,
}