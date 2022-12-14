use crate::pb::{frame_data::FrameData, serialize_frame_data};
use crate::error::{ Result, Error };

macro_rules! try_let {
    ( $x:expr ) => {
        match $x {
            Ok(value) => value,
            Err(err) => return Err(
                Error::SocketCreationFailed(err.message().to_string())
            ),
        }
    };
}

impl super::PushSocket {
    pub fn new(endpoint: &str) -> Result<Self> {
        let context = zmq::Context::new();
        let socket = try_let!(context.socket(zmq::PUSH));
        try_let!(socket.set_linger(super::SOCKET_LINGER));
        try_let!(socket.bind(endpoint));
        Ok(Self { socket })
    }

    pub fn push(&self, frame_data: FrameData) -> Result<()> {
        let data = match serialize_frame_data(&frame_data) {
            Err(err) => return Err(err),
            Ok(data) => data,
        };
        match self.socket.send(data, 0) {
            Err(err) => Err(Error::SocketPushFailed(err.to_string())),
            Ok(_) => Ok(()),
        }
    }
}

impl Drop for super::PushSocket {
    fn drop(&mut self) {
        dbg!("Dropping push socket...");
        // self.context.destroy().unwrap();
        drop(&self.socket);
        dbg!("Push socket is dropped.");
    }
}
