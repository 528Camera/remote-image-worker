// Automatically generated rust module for 'sample.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::borrow::Cow;
use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Sample<'a> {
    pub version: i32,
    pub frame_id: i32,
    pub timestamp: i64,
    pub image_data: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for Sample<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.version = r.read_int32(bytes)?,
                Ok(16) => msg.frame_id = r.read_int32(bytes)?,
                Ok(24) => msg.timestamp = r.read_int64(bytes)?,
                Ok(34) => msg.image_data = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Sample<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.version == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.version) as u64) }
        + if self.frame_id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.frame_id) as u64) }
        + if self.timestamp == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.timestamp) as u64) }
        + if self.image_data == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.image_data).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.version != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.version))?; }
        if self.frame_id != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.frame_id))?; }
        if self.timestamp != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.timestamp))?; }
        if self.image_data != Cow::Borrowed(b"") { w.write_with_tag(34, |w| w.write_bytes(&**&self.image_data))?; }
        Ok(())
    }
}

