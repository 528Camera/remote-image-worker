use crate::encod::{ imdecode, imencode, formats::Png };
use crate::proc::{
    color::reduce_colors,
    edge::detect_edges,
    proc,
};

const IMAGE_NAME: &str = "image.png";

/// Reduce color density for *image.png* sample to 64 colors \
/// (two most significant bits of each channel)
/// 
/// The result is saved as *image.png*. 
#[test]
fn run_reduce_colors() {
    let imbytes = super::read_sample(IMAGE_NAME);
    // Decode: ~120 ms
    let image = imdecode(imbytes).unwrap();
    // Reduce: ~200 ms (~500 ms without rayon)
    let img = reduce_colors(image).unwrap();
    // Encode: ~120 ms
    let contents = imencode(img, &Png::default()).unwrap();
    super::write_result(IMAGE_NAME, &contents);
}

#[test]
fn run_detect_edges() {
    let imbytes = super::read_sample(IMAGE_NAME);
    let image = imdecode(imbytes).unwrap();
    let img = detect_edges(image).unwrap();
    let contents = imencode(img, &Png::default()).unwrap();
    super::write_result(IMAGE_NAME, &contents);
}

#[test]
fn run_proc() {
    let imbytes = super::read_sample(IMAGE_NAME);
    let image = imdecode(imbytes).unwrap();
    let img = proc(image).unwrap();
    let format = crate::encod::formats::Jpg::new(20, false, true, 0, -1, -1).unwrap();
    let contents = imencode(img, &format).unwrap();
    super::write_result("image.jpg", &contents);
}
