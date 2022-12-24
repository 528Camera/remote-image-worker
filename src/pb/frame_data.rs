#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrameData {
    #[prost(int32, tag = "1")]
    pub version: i32,
    #[prost(bytes = "vec", tag = "2")]
    pub image: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(int32, tag = "4")]
    pub frame_index: i32,
}
