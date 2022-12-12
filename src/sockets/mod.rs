pub mod pull;
pub mod push;
use zmq::{ Context, Socket };

pub struct PushSocket {
    context: Context,
    socket: Socket,
}

pub struct PullSocket {
    context: Context,
    socket: Socket,
}
