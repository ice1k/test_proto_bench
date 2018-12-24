#[derive(Clone, PartialEq, Message)]
pub struct SwitchModeRequest {
    #[prost(enumeration="SwitchMode", tag="1")]
    pub mode: i32,
}
#[derive(Clone, PartialEq, Message)]
pub struct SwitchModeResponse {
}
#[derive(Clone, PartialEq, Message)]
pub struct Range {
    #[prost(bytes, tag="1")]
    pub start: Vec<u8>,
    #[prost(bytes, tag="2")]
    pub end: Vec<u8>,
}
#[derive(Clone, PartialEq, Message)]
pub struct SstMeta {
    #[prost(bytes, tag="1")]
    pub uuid: Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub range: ::std::option::Option<Range>,
    #[prost(uint32, tag="3")]
    pub crc32: u32,
    #[prost(uint64, tag="4")]
    pub length: u64,
    #[prost(string, tag="5")]
    pub cf_name: String,
    #[prost(uint64, tag="6")]
    pub region_id: u64,
    #[prost(message, optional, tag="7")]
    pub region_epoch: ::std::option::Option<super::metapb::RegionEpoch>,
}
#[derive(Clone, PartialEq, Message)]
pub struct UploadRequest {
    #[prost(oneof="upload_request::Chunk", tags="1, 2")]
    pub chunk: ::std::option::Option<upload_request::Chunk>,
}
pub mod upload_request {
    #[derive(Clone, Oneof, PartialEq)]
    pub enum Chunk {
        #[prost(message, tag="1")]
        Meta(super::SstMeta),
        #[prost(bytes, tag="2")]
        Data(Vec<u8>),
    }
}
#[derive(Clone, PartialEq, Message)]
pub struct UploadResponse {
}
#[derive(Clone, PartialEq, Message)]
pub struct IngestRequest {
    #[prost(message, optional, tag="1")]
    pub context: ::std::option::Option<super::kvrpcpb::Context>,
    #[prost(message, optional, tag="2")]
    pub sst: ::std::option::Option<SstMeta>,
}
#[derive(Clone, PartialEq, Message)]
pub struct IngestResponse {
    #[prost(message, optional, tag="1")]
    pub error: ::std::option::Option<super::errorpb::Error>,
}
#[derive(Clone, PartialEq, Message)]
pub struct CompactRequest {
    /// Compact files in the range and above the output level.
    /// Compact all files if the range is not specified.
    /// Compact all files to the bottommost level if the output level is -1.
    #[prost(message, optional, tag="1")]
    pub range: ::std::option::Option<Range>,
    #[prost(int32, tag="2")]
    pub output_level: i32,
}
#[derive(Clone, PartialEq, Message)]
pub struct CompactResponse {
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
pub enum SwitchMode {
    Normal = 0,
    Import = 1,
}
