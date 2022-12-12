use crate::pb::{frame_data::FrameData, deserialize_frame_data};

impl super::PullSocket {
    pub fn new(endpoint: &str) -> Result<Self, String> {
        let context = zmq::Context::new();
        let socket = context.socket(zmq::PULL).unwrap();
        socket.connect(endpoint).unwrap();
        Ok(Self { context, socket })
    }

    pub fn pull(&self) -> Result<FrameData, String> {
        match self.socket.poll(zmq::POLLIN, 3000) {
            Ok(poll) => {
                if !zmq::PollEvents::from_bits(poll as i16).unwrap().contains(zmq::POLLIN) {
                    return Err("Pull: timeout".to_string());
                }
                let mut msg = zmq::Message::new();
                if let Err(err) = self.socket.recv(&mut msg, 0) {
                    return Err(err.to_string())
                }
                let buf = msg.as_ref();
                match deserialize_frame_data(buf) {
                    Err(err) => Err(err.to_string()),
                    Ok(frame_data) => Ok(frame_data),
                }
            }
            Err(err) => Err(err.to_string()),
        }

        
    }
}

impl Drop for super::PullSocket {
    fn drop(&mut self) {
        self.context.destroy().unwrap();
    }
}
