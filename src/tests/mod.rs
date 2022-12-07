//! This is a collection of tests for various functions in this "library". \
//! To run a specific test, use `cargo test {test-name}`. \
//! 
//! These tests require additional resources (images) to be placed in *target/test_samples* directory.
//! 
//! Included tests:
//! * base64:
//!     * `run_base64_encode`
//!     * `run_base64_decode`
//! * formats: `convert_x_to_y`, where x and y are:
//!     * png
//!     * jpg
//!     * webp
//! * protobuf (pb):
//!     * `run_pb_pack`
//!     * `test_pb_unpack`

mod mat_eq;
mod base64;
mod formats;
mod pb;

/// Get test sample filename in *target/test_samples/* subdirectory. 
fn read_sample(name: &str) -> Vec<u8> {
    std::fs::read(
        format!(
            "{}/target/test_samples/{}",
            env!("CARGO_MANIFEST_DIR"),
            name,
        )
    ).unwrap()
}

fn write_result(name: &str, contents: &[u8]) {
    std::fs::write(
        format!(
            "{}/target/test_results/{}",
            env!("CARGO_MANIFEST_DIR"),
            name,
        ),
        contents,
    ).unwrap();
}
