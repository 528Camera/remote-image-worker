//! Test runs for protobuf serialization/deserialization
use std::borrow::Cow;
use std::time::{ SystemTime, UNIX_EPOCH };

use crate::pb::sample::Sample;
use quick_protobuf::{ 
    MessageRead, MessageWrite, 
    BytesReader, Writer,
};

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
    let imbytes = super::read_sample(IMAGE_FILE);
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
            as i64;

    // Protobuf struct:
    let message = Sample {
        version: EXPECTED_VERSION,
        frame_id: 1,
        timestamp,
        image_data: Cow::from(imbytes),
    };

    // Write the message into byte array:
    let mut r = vec![];
    let mut writer = Writer::new(&mut r);
    message.write_message(&mut writer).unwrap();

    super::write_result(BINARY_FILE, &r);
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
    let bytes = super::read_sample(BINARY_FILE);

    // Parse protobuf from byte array:
    let mut r = BytesReader::from_bytes(&bytes);
    let message = Sample::from_reader(&mut r, &bytes).unwrap();

    assert_eq!(message.version, EXPECTED_VERSION);
    println!("Frame ID: {}", message.frame_id);
    
    println!("Timestamp from Unix epoch: {} ms", message.timestamp);

    // Read the original image:
    let imbytes = super::read_sample(IMAGE_FILE);

    assert_eq!(message.image_data.len(), imbytes.len());
    // Find the first unequal byte:
    let cmp = imbytes.iter()
        .zip(message.image_data.iter())
        .find(|(a, b)| b != a);
    // No bytes are unequal === All bytes are equal
    assert!(cmp.is_none());
}
