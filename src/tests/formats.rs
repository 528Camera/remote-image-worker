//! Test runs for image conversion between supported formats

use crate::encod::{ 
    imdecode, imencode,
    formats::{ ImgFormat, Png, Jpg, Webp }
};

fn format_png() -> Png {
    Png::with_compression(
        4, 
        false
    ).unwrap()
}

fn format_jpg() -> Jpg {
    Jpg::new(
        60,
        false,
        true,
        0,
        -1,
        -1,
    ).unwrap()
}

fn format_webp() -> Webp {
    Webp::new(
        60,
    ).unwrap()
}

fn convert(name: &str, format: impl ImgFormat) {
    let input = super::read_sample(name);

    let img = imdecode(input).unwrap();

    let result_name = format!("{}{}", name, &format.ext());

    let imbytes = imencode(img, &format).unwrap();

    super::write_result(
        result_name.as_str(),
        &imbytes,
    );
}

#[test]
fn run_convert_png_to_png() {
    convert("image.png", format_png());
}

#[test]
fn run_convert_png_to_jpg() {
    convert("image.png", format_jpg());
}

#[test]
fn run_convert_png_to_webp() {
    convert("image.png", format_webp());
}

#[test]
fn run_convert_jpg_to_png() {
    convert("image.jpg", format_png());
}

#[test]
fn run_convert_jpg_to_jpg() {
    convert("image.jpg", format_jpg());
}

#[test]
fn run_convert_jpg_to_webp() {
    convert("image.jpg", format_webp());
}

#[test]
fn run_convert_webp_to_png() {
    convert("image.webp", format_png());
}

#[test]
fn run_convert_webp_to_jpg() {
    convert("image.webp", format_jpg());
}

#[test]
fn run_convert_webp_to_webp() {
    convert("image.webp", format_webp());
}
