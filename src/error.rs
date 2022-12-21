pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    // Encoding
    ImEncodeFailed(String),
    ImDecodeFailed(String),

    // Processing
    ProcReduceColorFailed(String),
    ProcDetectEdgesFailed(String),
    ProcResultsAddFailed(String),

    // Protobuf
    PbEncodeFailed(String),
    PbDecodeFailed(String),

    // Sockets
    SocketCreationFailed(String),
    SocketPullFailed(String),
    SocketPushFailed(String),
}
