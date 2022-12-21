# remote-image-worker
A little educational program for remote distributed image processing - Go branch!

## Requirements
### opencv
Program uses [opencv](https://opencv.org/) ([bindings](https://gocv.io/)) to process the image.

To set up **gocv** easily, use the tutorial for [Windows](https://gocv.io/getting-started/windows/) or [Linux](https://gocv.io/getting-started/linux/).

You may also install OpenCV independently. Use OpenCV instructions for [Windows](https://docs.opencv.org/4.x/d3/d52/tutorial_windows_install.html) or [Linux](https://docs.opencv.org/4.x/d7/d9f/tutorial_linux_install.html) to do that. Alternatively, use the package manager:
* Chocolatey (Windows): `choco install opencv`
* Ubuntu: `sudo apt install libopencv-dev`
* Fedora: `sudo dnf install opencv`
* Arch Linux: `sudo pacman -S opencv` or `sudo pacman -S opencv-cuda`

### zeromq
Program uses [zeromq](https://zeromq.org/) ([bindings](https://crates.io/crates/zmq)) to receive input images and send out results.

You might need to install `libzmq` library for this to work.

## Install
1. Install [Go](https://go.dev/)
2. Install [protoc](#protobuf)
3. Clone this repository
4. Update submodules: `git submodule update --init`
5. In the project directory:
  * `go get` to download dependencies;
  * `go build` to build the executable; it will appear as *remote-image-worker* file;
  * `go test {test-file}` to run a specific test (see [Tests](#test));
  * `go run . {arguments}` to run the executable;

## Develop
### IDE
It is advised to use [VS Code](https://vscodium.com/) or a comparable editor/IDE.

If using VS Code, install the **Go** extension for syntax highlighting, debugging and other good stuff.

### protobuf
Program uses Google's [protobuf](https://developers.google.com/protocol-buffers) to serialize and deserialize data packets.

The protobuf code should already be generated and placed in the repository. Otherwise, you may need the protobuf compiler ([protoc](https://github.com/protocolbuffers/protobuf/releases)). Once you have it, run the following [command](https://developers.google.com/protocol-buffers/docs/gotutorial):

`protoc -I=internal/pb/ --go_out=internal/pb/ internal/pb/frame_data.proto`

## Test
To run a specific test, use `go test {test-file}`.
 
Included tests:
* *socket*: for better effect (parallel testing) open the terminal in *test/socket* directory and run `go test` with no arguments.
  * *pull_test.go*
  * *push_test.go*
* *proc*: image processing tests; require an *image.jpg* file to be present in *test/proc* directory.
  * *color_test.go* - color reduction; 
  * *edge_test.go* - edge detection;
  * *proc_test.go* - composition of results.
