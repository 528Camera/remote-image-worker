# remote-image-worker
A little educational program for remote distributed image processing.

## Requirements
### opencv
Program uses [opencv](https://opencv.org/) ([bindings](https://crates.io/crates/opencv)) to process the image.

Use the instructions to install the library on [Windows](https://docs.opencv.org/4.x/d3/d52/tutorial_windows_install.html) or [Linux](https://docs.opencv.org/4.x/d7/d9f/tutorial_linux_install.html).

Alternatively, use the package manager:
* Chocolatey (Windows): `choco install opencv`
* Ubuntu: `sudo apt install libopencv-dev`
* Fedora: `sudo dnf install opencv`
* Arch Linux: `sudo pacman -S opencv` or `sudo pacman -S opencv-cuda`

### zeromq
Program uses [zeromq](https://zeromq.org/) ([bindings](https://crates.io/crates/zmq)) to receive input images and send out results.

You might need to install `libzmq` library for this to work.

## Install
1. Install [Rust](https://rustup.rs/)
2. Install [protoc](#protobuf)
3. Clone this repository
4. Update submodules: `git submodule update --init`
5. In the project directory:
    * `cargo build` to build the executable; it will appear as *target/debug/remote-image-worker* file;
    * `cargo build --release` to build the optimized executable; it will appear as *target/release/remote-image-worker* file;
    * `cargo test {test-name}` to run a specific test (see [Tests](#test) or *src/tests/mod.rs*);
    * `cargo run -- {arguments}` to run the executable in **debug** configuration;

## Develop
### IDE
It is advised to use [VS Code](https://vscodium.com/) or a comparable editor/IDE.

If using VS Code, install the following extensions:
* **rust-analyzer** language server - for syntax highlighting and other good stuff;
* **CodeLLDB** - for debugging the executable.

### protobuf
Program uses Google's [protobuf](https://developers.google.com/protocol-buffers) to serialize and deserialize data packets.

To build this program, you will need to [install](https://grpc.io/docs/protoc-installation/) the protobuf compiler ([protoc](https://github.com/protocolbuffers/protobuf#protocol-compiler-installation)).

## Test
To run a specific test, use `cargo test {test-name}`.

These tests require additional resources (images) 
to be placed in *target/test_samples* directory.
 
Included tests:
* base64:
  * `run_base64_encode`
  * `run_base64_decode`
* formats: `convert_x_to_y`, where x and y are:
  * png
  * jpg
  * webp
* protobuf (pb):
  * `run_pb_pack`
  * `test_pb_unpack`
* image processing (proc):
  * `run_reduce_colors`
