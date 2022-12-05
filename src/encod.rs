use opencv::prelude::*;
use opencv::core::{ Mat, Vector };
use opencv::imgcodecs::{
    imdecode as cv_imdecode, ImreadModes,
    imencode as cv_imencode, ImwriteFlags,
};

/// Decode an image from an array of bytes.
/// 
/// **On success:** image (`opencv::core::Mat`) \
/// **On failure:** an error message (`String`)
pub fn imdecode(buf: Vec<u8>) -> Result<Mat, String> {
    let buf = &Vector::<u8>::from_slice(&buf);
    let flags = 
        ImreadModes::IMREAD_ANYCOLOR as i32
        | ImreadModes::IMREAD_COLOR as i32;
    match cv_imdecode(buf, flags) {
        Ok(img) => Ok(img),
        // "Throw" an error down the stack:
        Err(err) => Err(err.message),
    }
}

/// Encode an image into an array of bytes in a specified format (.jpg, .png, etc.)
/// 
/// **On success:** an array of bytes (`Vec<u8>`) \
/// **On failure:** an error message (`String`)
pub fn imencode(img: Mat, ext: &str) -> Result<Vec<u8>, String> {
    let size = img.size().unwrap();
    let size = img.depth() * size.width * size.height;
    let mut buf = Vector::<u8>::with_capacity(size as usize);
    let params = Vector::<i32>::new();
    match cv_imencode(ext, &img, &mut buf, &params) {
        Ok(true) => {
            let buf = buf.as_slice();
            Ok(Vec::from(buf))
        },
        Ok(false) => Err("Could not encode the image".to_string()),
        Err(err) => Err(err.message),
    }
}