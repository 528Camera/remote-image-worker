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
    dump_path: Option<String>,
}

fn args() -> Args {
    let mut args = std::env::args();
    let _name = args.next().unwrap();
    let pull_ep = args.next().unwrap();
    let push_ep = args.next().unwrap();
    let dump_path = args.next();
    Args {
        pull_ep,
        push_ep,
        dump_path,
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
    let Args { pull_ep, push_ep, dump_path } = args();
    let puller = sockets::PullSocket::new(pull_ep.as_str()).unwrap();
    let pusher = sockets::PushSocket::new().unwrap();
    pusher.connect(push_ep.as_str()).unwrap();
    // TODO: wait for sockets to initialize
    // PUSH must detect a receiver first
    let format = encod::formats::Jpg::new(20, false, true, 0, -1, -1).unwrap();
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
        let sw = std::time::Instant::now();
        let img = try_let!(
            proc::proc(image),
            "Failed to process the image"
        );
        println!(
            "Took {}s to process frame #{}", 
            sw.elapsed().as_secs_f32(), 
            frame_data.frame_index,
        );
        let data = try_let!(
            encod::imencode(img, &format),
            "Failed to encode the image"
        );
        if let Some(ref path) = dump_path {
            std::fs::write(
                format!(
                    "{}/frame_{}.jpg",
                    path,
                    frame_data.frame_index,
                ), 
                data,
            ).unwrap();
            continue;
        }
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
