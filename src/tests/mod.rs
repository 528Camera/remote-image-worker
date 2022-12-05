mod io;
use opencv::prelude::*;
use opencv::core::Mat;

/// Equality of two matrices.
fn mat_eq(mat1: Mat, mat2: Mat) -> bool {
    if mat1.size().unwrap() != mat2.size().unwrap() {
        return false;
    }
    let mut diff = Mat::new_size_with_default(
        mat1.size().unwrap(),
        opencv::core::CV_32S,
        opencv::core::Scalar::default(),
    ).unwrap();

    opencv::core::compare(
        &mat1, 
        &mat2,
        &mut diff,
        opencv::core::CMP_NE
    ).unwrap();
    
    let nz = opencv::core::count_non_zero(&diff).unwrap();
    return nz == 0;
}

/// Read test sample from file in *target/test_samples/* subdirectory. 
fn read_sample(name: &str) -> Vec<u8> {
    std::fs::read(
        format!(
            "{}/target/test_samples/{}",
            env!("CARGO_MANIFEST_DIR"),
            name,
        )
    ).unwrap()
}
