pub mod pull;
pub mod push;
// use zmq::{ Context, Socket };

pub struct PushSocket {
    // context: Context,
    socket: zmq::Socket,
}

pub struct PullSocket {
    // context: Context,
    socket: zmq::Socket,
}

const SOCKET_LINGER: i32 = 100;
