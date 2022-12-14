use crate::pb::{frame_data::FrameData, deserialize_frame_data};
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

impl super::PullSocket {
    pub fn new(endpoint: &str) -> Result<Self> {
        let context = zmq::Context::new();
        let socket = try_let!(context.socket(zmq::PULL));
        try_let!(socket.set_linger(super::SOCKET_LINGER));
        try_let!(socket.connect(endpoint));
        Ok(Self { socket })
    }

    pub fn pull(&self) -> Result<FrameData> {
        let poll = match self.socket.poll(zmq::POLLIN, 3000) {
            Err(err) => return Err(Error::SocketPullFailed(err.to_string())),
            Ok(poll) => zmq::PollEvents::from_bits(poll as i16).unwrap(),
        };
        if !poll.contains(zmq::POLLIN) {
            return Err(Error::SocketPullFailed("Timeout".to_string()));
        }
        let mut msg = zmq::Message::new();
        if let Err(err) = self.socket.recv(&mut msg, 0) {
            return Err(Error::SocketPullFailed(err.to_string()));
        }
        let buf = msg.as_ref();
        deserialize_frame_data(buf)

        
    }
}

impl Drop for super::PullSocket {
    fn drop(&mut self) {
        dbg!("Dropping pull socket...");
        // self.context.destroy().unwrap();
        drop(&self.socket);
        dbg!("Pull socket is dropped.");
    }
}
