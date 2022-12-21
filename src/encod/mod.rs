pub mod formats;
use crate::error::{ Result, Error };
use opencv::prelude::*;
use opencv::core::{ Mat, Vector };

/// Decode an image from an array of bytes.
/// 
/// **On success:** image (`opencv::core::Mat`) \
/// **On failure:** ImDecodeError
pub fn imdecode(buf: Vec<u8>) -> Result<Mat> {
    use opencv::imgcodecs::{ 
        IMREAD_ANYCOLOR, IMREAD_COLOR,
    };
    let flags = IMREAD_ANYCOLOR | IMREAD_COLOR;
    let buf = &Vector::<u8>::from_slice(&buf);
    match opencv::imgcodecs::imdecode(buf, flags) {
        Err(err) => Err(Error::ImDecodeFailed(err.message)),
        Ok(img) => Ok(img),
    }
}

/// Encode an image into an array of bytes in a specified format.
/// 
/// **On success:** an array of bytes (`Vec<u8>`) \
/// **On failure:** ImEncodeError
pub fn imencode(img: Mat, format: &impl formats::ImgFormat) -> Result<Vec<u8>> {
    let ext = format.ext();
    let params = Vector::<i32>::from_slice(&format.params());

    let size = match img.size() {
        Err(err) => return Err(Error::ImEncodeFailed(err.message)),
        Ok(size) => size,
    };
    let size = img.depth() * size.width * size.height;
    let mut buf = Vector::<u8>::with_capacity(size as usize);

    match opencv::imgcodecs::imencode(ext, &img, &mut buf, &params) {
        Ok(true) => {
            let buf = buf.as_slice();
            Ok(Vec::from(buf))
        },
        Ok(false) => Err(Error::ImEncodeFailed(
            "Could not encode the image".to_string()
        )),
        Err(err) => Err(Error::ImEncodeFailed(err.message)),
    }
}