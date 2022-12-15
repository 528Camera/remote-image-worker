//! Run tests on ZMQ push/pull sockets.
//! 
//! **IMPORTANT:** the two tests MUST be run simultaneously. \
//! PUSH socket will block the thread indefinitely until a receiver appears. \
//! PULL socket will time out if there is no transmitter.

use crate::pb::frame_data::FrameData;
use crate::sockets::{ PullSocket, PushSocket };
const FRAME_COUNT: i32 = 10;
const EXPECTED_VERSION: i32 = 1;
 
#[test]
fn test_push_socket() {
    let pusher = PushSocket::new().unwrap();
    pusher.bind("tcp://*:5557").unwrap();
    for i in 0..FRAME_COUNT {
        let frame_data = FrameData {
            version: EXPECTED_VERSION,
            frame_index: i,
            ..FrameData::default()
        };
        pusher.push(frame_data).unwrap();
        println!("Pushed frame #{i}");
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    // drop(pusher);
}

#[test]
fn test_pull_socket() {
    let puller = PullSocket::new("tcp://127.0.0.1:5557").unwrap();
    for i in 0..FRAME_COUNT {
        let frame_data = puller.pull().unwrap();
        println!("Pulled frame #{i}:");
        println!("- Version: {}", frame_data.version);
        if frame_data.version != EXPECTED_VERSION {
            println!("WARNING: unexpected version!");
            println!("Expected {}", EXPECTED_VERSION);
        }
        println!("- Frame index: {}", frame_data.frame_index);
    }
    // drop(puller);
}
