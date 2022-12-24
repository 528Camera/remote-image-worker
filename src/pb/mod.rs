use std::io::Cursor;
use prost::Message;
use frame_data::FrameData;
use crate::error::{ Result, Error };

pub mod frame_data;

pub fn serialize_frame_data(frame_data: &FrameData) -> Result<Vec<u8>> {
    let mut buf = vec![];
    buf.reserve(frame_data.encoded_len());
    match frame_data.encode(&mut buf) {
        Ok(_) => Ok(buf),
        Err(err) => Err(Error::PbEncodeFailed(err.to_string())),
    }
}

pub fn deserialize_frame_data(buf: &[u8]) -> Result<FrameData> {
    match FrameData::decode(&mut Cursor::new(buf)) {
        Ok(frame_data) => Ok(frame_data),
        Err(err) => Err(Error::PbDecodeFailed(err.to_string())),
    }
}
