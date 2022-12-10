use std::io::Cursor;
use prost::Message;

pub mod frame_data {
    include!(concat!(env!("OUT_DIR"), "/frame_data.rs"));
}

pub fn serialize_frame_data(frame_data: &frame_data::FrameData) -> Vec<u8> {
    let mut buf = vec![];
    buf.reserve(frame_data.encoded_len());
    frame_data.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_frame_data(buf: &[u8]) 
-> Result<frame_data::FrameData, prost::DecodeError> {
    frame_data::FrameData::decode(&mut Cursor::new(buf))
}
