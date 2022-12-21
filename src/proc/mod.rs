pub mod color;
pub mod edge;
use opencv::prelude::*;
use opencv::core::{ Mat, CV_32SC3, add_weighted };
use crate::error::{ Result, Error };

pub fn proc(image: Mat) -> Result<Mat> {
    let color = match color::reduce_colors(image.clone()) {
        Ok(img) => img,
        err => return err,
    };
    let edges = match edge::detect_edges(image.clone()) {
        Ok(img) => img,
        err => return err,
    };
    let mut dst = Mat::zeros(image.rows(), image.cols(), CV_32SC3).unwrap().to_mat().unwrap();
    match add_weighted(&color, 1.0, &edges, -1.0, 0.0, &mut dst, CV_32SC3) {
        Err(err) => Err(Error::ProcResultsAddFailed(err.message)),
        _ => Ok(dst),
    }
}