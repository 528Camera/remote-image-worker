pub mod encod;
pub mod io;
use encod::formats::{ 
    Formats,
    png::Png,
    jpg::Jpg,
    webp::Webp,
};

#[cfg(test)]
mod tests;

struct ImageWorkerArgs {
    use_base64: bool,
    input_path: String,
    output_path: String,
}

/// Read and parse command line arguments.
fn args() -> Result<ImageWorkerArgs, String> {
    let mut args = std::env::args();
    let _name = args.next().unwrap();
    let use_base64 = match args.next().unwrap().as_str() {
        "encode" => false,
        "decode" => true,
        _ => return Err("Unknown processing mode!".to_string())
    };
    Ok(ImageWorkerArgs {
        use_base64,
        input_path: args.next().unwrap(), 
        output_path: args.next().unwrap(),
    })
}

fn main() {
    let args = args();
    if let Err(err) = args {
        println!("{err}");
        println!("Usage:");
        println!("remote-image-worker encode [input-png] [output-base64]");
        println!("remote-image-worker decode [input-base64] [output-png]");
        return;
    }

    let ImageWorkerArgs { 
        use_base64, 
        input_path,
        output_path 
    } = args.unwrap();

    // Try reading data from the input file:
    if let Some(err) = match io::imread(input_path, use_base64) {
        Ok(img) => {
            let use_base64 = !use_base64;
            // Try writing data to the output file:
            match io::imwrite(
                output_path.as_str(), 
                img, 
                use_base64,
                Formats::Webp(
                    Webp::new(45).unwrap()
                ),
            ) {
                Ok(_) => {
                    println!("Image was saved to {}", output_path);
                    None
                }
                // "Throw" an error down the stack:
                Err(err) => Some(err),
            }
        }
        // "Throw" an error down the stack:
        Err(err) => Some(err),
    } {
        // "Catch" and print the error:
        println!("{err}");
    }
}
