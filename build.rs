extern crate prost_build;

fn main() {
    std::env::set_var(
        "PROTOC",
        format!(
            "{}/.local/bin/protoc",
            env!("HOME"),
        )
    );
    prost_build::compile_protos(
        &["src/pb/frame_data.proto"],
        &["src/"]
    ).unwrap();
}