use opencv::prelude::*;
use opencv::core::{ Vec3b, Mat };
use rayon::prelude::*;
use crate::error::{ Result, Error };

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
fn reduce_color(v3: &Vec3b) -> Vec3b {
    let v3 = v3.0;
    Vec3b::from([
        reduce(v3[0]),
        reduce(v3[1]),
        reduce(v3[2]),
    ])
}

macro_rules! try_let {
    ( $x:expr ) => {
        match $x {
            Ok(value) => value,
            Err(err) => return Err(
                Error::ProcReduceColorFailed(err.message)
            ),
        }
    };
}

pub fn reduce_colors(image: Mat) -> Result<Mat> {
    let pixels = try_let!(image.data_typed::<Vec3b>());
    let pixels: Vec<Vec3b> = pixels.par_iter()
        .map(reduce_color)
        .collect();
    let mat = try_let!(Mat::from_slice(&pixels));
    let mat = try_let!(mat.reshape(3, image.rows()));
    Ok(mat)
}
