use crate::proc::color::reduce_colors;
use crate::encod::{ imdecode, imencode, formats::Png };

const IMAGE_NAME: &str = "image.png";

/// Reduce color density for *image.png* sample to 64 colors \
/// (two most significant bits of each channel)
/// 
/// The result is saved as *image.png*. 
#[test]
fn run_reduce_colors() {
    let imbytes = super::read_sample(IMAGE_NAME);
    let image = imdecode(imbytes).unwrap();
    let img = reduce_colors(image).unwrap();
    let contents = imencode(img, Png::default()).unwrap();
    super::write_result(IMAGE_NAME, &contents);
}
