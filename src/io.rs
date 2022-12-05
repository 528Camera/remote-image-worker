use std::path::Path;
use opencv::core::Mat;

/// Read an image from a file on the specified path. 
/// 
/// In `use_base64` mode reads a text/binary and 
/// decodes the Base64 string within. 
/// Otherwise reads a valid image file. 
/// 
/// **On success:** image (`opencv::core::Mat`) \
/// **On failure:** an error message (`String`)
pub fn imread<P>(path: P, use_base64: bool) -> Result<Mat, String>
where P: AsRef<Path> {
    // Try reading data from file:
    match std::fs::read(path) {
        Ok(input) => {
            // Try decoding Base64, if needed:
            let buf = if use_base64 {
                match base64::decode(input) {
                    Ok(buf) => Ok(buf),
                    // "Throw" an error down the stack:
                    Err(err) => Err(
                        err.to_string()
                    ),
                }
            } else {
                Ok(input)
            };
            match buf {
                // Try decoding the received bytes into Mat:
                Ok(buf) => crate::encod::imdecode(buf),
                // "Throw" an error down the stack:
                Err(err) => Err(err),
            }
        }
        // "Catch" and return the error message:
        Err(err) => Err(
            err.to_string()
        )
    }
}

/// Save an image into a file on the specified path. 
/// 
/// In `use_base64` mode encodes image bytes as Base64
/// string and writes it in a text/binary.
/// Otherwise writes in any supported image format. 
/// 
/// **On success:** void \
/// **On failure:** an error message (`String`)
pub fn imwrite(
    path: &str,
    img: Mat,
    use_base64: bool,
    format: crate::encod::formats::Formats,
) -> Result<(), String> {
    // Try encoding a Mat into an array of bytes:
    match crate::encod::imencode(img, format) {
        Ok(buf) => {
            // Encode Base64, if needed:
            let imb64 = if use_base64 {
                Some(base64::encode(&buf))
            } else {
                None
            };
            // Convert data into bytes:
            let contents = match &imb64 {
                Some(imb64) => imb64.as_bytes(),
                None => &buf,
            };
            // Try writing bytes into file:
            if let Err(err) = std::fs::write(path, contents) {
                // "Throw" an error down the stack:
                Err(err.to_string())
            } else { 
                Ok(())
            }
        }
        // "Catch" and return the error message:
        Err(err) => Err(err),
    }
}
