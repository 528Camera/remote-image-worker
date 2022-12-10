extern crate prost_build;

fn main() {
    prost_build::compile_protos(
        &["src/pb/frame_data.proto"],
        &["src/"]
    ).unwrap();
}