use std::path::Path;
use opencv::prelude::*;
use opencv::core::{ Mat, Vector };
use opencv::imgcodecs::{
    imdecode, ImreadModes,
    imencode, ImwriteFlags,
};

pub fn imread<P>(path: P, use_base64: bool) -> Result<Mat, String>
where P: AsRef<Path> {
    match std::fs::read(path) {
        Ok(input) => {
            let buf = if use_base64 {
                match base64::decode(input) {
                    Ok(buf) => Ok(buf),
                    Err(err) => Err(
                        err.to_string()
                    ),
                }
            } else {
                Ok(input)
            };
            match buf {
                Ok(buf) => {
                    let buf = &Vector::<u8>::from_slice(&buf);
                    let flags = 
                        ImreadModes::IMREAD_ANYCOLOR as i32
                        | ImreadModes::IMREAD_COLOR as i32;
                    match imdecode(buf, flags) {
                        Ok(img) => Ok(img),
                        Err(err) => Err(err.message),
                    }
                }
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(
            err.to_string()
        )
    }
}

pub fn imwrite(path: &str, img: Mat, use_base64: bool) -> Result<(), String> {
    let size = img.size().unwrap();
    let size = img.depth() * size.width * size.height;
    let mut buf = Vector::<u8>::with_capacity(size as usize);
    match imencode(
        ".jpg", 
        &img, 
        &mut buf, 
        &Vector::<i32>::new()
    ) {
        Ok(true) => {
            let imb64 = if use_base64 {
                Some(base64::encode(buf.as_slice()))
            } else {
                None
            };
            let contents = match &imb64 {
                Some(imb64) => imb64.as_bytes(),
                None => buf.as_slice(),
            };
            if let Err(err) = std::fs::write(path, contents) {
                Err(err.to_string())
            } else { 
                Ok(())
            }
        }
        Ok(false) => Ok(()),
        Err(err) => Err(err.message),
    }
}
