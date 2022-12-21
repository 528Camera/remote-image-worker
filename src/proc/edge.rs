use opencv::prelude::*;
use opencv::core::Mat;
use opencv::imgproc::{ canny, cvt_color, COLOR_GRAY2BGR };
use crate::error::{ Result, Error };

pub fn detect_edges(image: Mat) -> Result<Mat> {
    let mut edges = Mat::zeros(image.rows(), image.cols(), opencv::core::CV_32FC1).unwrap().to_mat().unwrap();
    if let Err(err) = canny(&image, &mut edges, 50.0, 200.0, 3, false) {
        return Err(Error::ProcDetectEdgesFailed(err.message));
    }
    let mut edges2 = Mat::zeros(image.rows(), image.cols(), opencv::core::CV_32SC3).unwrap().to_mat().unwrap();
    match cvt_color(&edges, &mut edges2, COLOR_GRAY2BGR, 3) {
        Err(err) => Err(Error::ProcDetectEdgesFailed(err.message)),
        _ => Ok(edges2),
    }
}