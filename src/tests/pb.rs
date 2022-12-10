//! Test runs for protobuf serialization/deserialization
use std::time::SystemTime;

use crate::pb::{ deserialize_frame_data, serialize_frame_data };

const IMAGE_FILE: &str = "image.jpg";
const BINARY_FILE: &str = "image.dat";
const EXPECTED_VERSION: i32 = 1;

/// Pack the image into protobuf bytes.
/// 
/// This test expects *image.jpg* in the samples directory.
/// 
/// The result will be saved into an *image.dat* file. 
#[test]
fn run_pb_pack() {
    let image = super::read_sample(IMAGE_FILE);

    let time = Some(
        prost_types::Timestamp::from(
            SystemTime::now()
        )
    );

    // Protobuf struct:
    let frame_data = crate::pb::frame_data::FrameData { 
        version: EXPECTED_VERSION,
        image, 
        time,
        frame_index: 1,
    };

    // Write the message into byte array:
    let contents = serialize_frame_data(&frame_data);
    super::write_result(BINARY_FILE, &contents);
}

/// Extract the image from protobuf bytes.
/// 
/// This test expects *image.jpg* and the corresponding 
/// *image.dat* in the samples directory.
/// 
/// If bytes are equal between image and protobuf, 
/// the test succeeds.
#[test]
fn test_pb_unpack() {
    let buf = super::read_sample(BINARY_FILE);

    // Parse protobuf from byte array:
    let frame_data = deserialize_frame_data(&buf).unwrap();
    
    assert_eq!(frame_data.version, EXPECTED_VERSION);
    println!("Frame ID: {}", frame_data.frame_index);
    
    println!(
        "Timestamp: {} ms", 
        SystemTime::try_from(
            frame_data.time.unwrap()
        )
        .unwrap()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis());

    // Read the original image:
    let imbytes = super::read_sample(IMAGE_FILE);

    assert_eq!(frame_data.image.len(), imbytes.len());
    // Find the first unequal byte:
    let cmp = imbytes.iter()
        .zip(frame_data.image.iter())
        .find(|(a, b)| b != a);
    // No bytes are unequal === All bytes are equal
    assert!(cmp.is_none());
}
