pub mod error;
pub mod encod;
pub mod pb;
pub mod proc;
pub mod sockets;

#[cfg(test)]
mod tests;

struct Args {
    pull_ep: String,
    push_ep: String,
}

fn args() -> Args {
    let mut args = std::env::args();
    let _name = args.next().unwrap();
    let pull_ep = args.next().unwrap();
    let push_ep = args.next().unwrap();
    Args {
        pull_ep,
        push_ep,
    }
}

macro_rules! try_let {
    ( $x:expr, $message:expr ) => {
        match $x {
            Ok(value) => value,
            Err(err) => {
                println!($message);
                dbg!(err);
                continue;
            }
        }
    }
}

const MAX_RETRY_COUNT: i32 = 10;

fn main() {
    let Args { pull_ep, push_ep } = args();
    let puller = sockets::PullSocket::new(pull_ep.as_str()).unwrap();
    let pusher = sockets::PushSocket::new(push_ep.as_str()).unwrap();
    // TODO: wait for sockets to initialize
    // PUSH must detect a receiver first
    let mut retry_count = 0;
    while retry_count < MAX_RETRY_COUNT {
        let mut frame_data = match puller.pull() {
            Err(_) => {
                retry_count += 1;
                println!("Pull failed");
                continue;
            }
            Ok(frame_data) => frame_data,
        };
        let image = try_let!(
            encod::imdecode(frame_data.image),
            "Failed to decode the image"
        );
        let img = try_let!(
            proc::color::reduce_colors(image),
            "Failed to process the image"
        );
        let data = try_let!(
            encod::imencode(img, encod::formats::Jpg::default()),
            "Failed to encode the image"
        );
        frame_data.image = data;
        try_let!(
            pusher.push(frame_data),
            "Failed to push the new frame"
        );
    }
    if retry_count >= MAX_RETRY_COUNT {
        println!("Maximum tries exceeded.");
    }
    println!("Shutting down.");
}
