use crate::pb::{frame_data::FrameData, serialize_frame_data};

impl super::PushSocket {
    pub fn new(endpoint: &str) -> Result<Self, String> {
        let context = zmq::Context::new();
        let socket = context.socket(zmq::PUSH).unwrap();
        socket.bind(endpoint).unwrap();
        Ok(Self { context, socket })
    }

    pub fn push(&self, frame_data: FrameData) 
    -> Result<(), String> {
        let data = serialize_frame_data(&frame_data);
        if let Err(err) = self.socket.send(data, zmq::DONTWAIT) {
            Err(err.to_string())
        } else {
            Ok(())
        }
    }
}

impl Drop for super::PushSocket {
    fn drop(&mut self) {
        self.context.destroy().unwrap();
    }
}
