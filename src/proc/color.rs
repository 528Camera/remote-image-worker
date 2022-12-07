use opencv::prelude::*;
use opencv::core::{ Vec3b, Mat };

#[inline]
fn reduce(v: u8) -> u8 {
    // Mask LSB (11000000):
    (v & 0xC0) + 32
    // match v {
    //     0..=63 => 0,
    //     64..=127 => 64,
    //     128..=191 => 128,
    //     _ => 192,
    // }
}

#[inline]
fn reduce_color(v3: &[u8]) -> [u8; 3] {
    [
        reduce(v3[0]),
        reduce(v3[1]),
        reduce(v3[2]),
    ]
}

pub fn reduce_colors(image: Mat) -> Result<Mat, String> {
    let pixels = image.data_typed::<Vec3b>();
    if let Err(err) = pixels {
        return Err(err.message);
    }
    let pixels: Vec<Vec3b> = pixels.unwrap().iter()
        .map(|p| Vec3b::from(reduce_color(&p.0)))
        .collect();
    match Mat::from_slice(&pixels) {
        Ok(mat) => Ok(mat.reshape(3, image.rows()).unwrap()),
        Err(err) => Err(err.message),
    }
}
