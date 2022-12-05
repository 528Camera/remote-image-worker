pub mod io;

#[cfg(test)]
mod tests;

// Usage: 
// remote-image-worker encode [input-png] [output-base64]
// remote-image-worker decode [input-base64] [output-png]
fn args() -> Result<(bool, String, String), String> {
    let mut args = std::env::args();
    let _name = args.next().unwrap();
    let use_base64 = match args.next().unwrap().as_str() {
        "encode" => false,
        "decode" => true,
        _ => return Err("Unknown processing mode!".to_string())
    };
    Ok((
        use_base64,
        args.next().unwrap(), 
        args.next().unwrap(),
    ))
}

fn main() {
    let args = args();
    if let Err(err) = args {
        println!("{err}");
        return;
    }
    let (use_base64, input_path, output_path) = args.unwrap();
    if let Some(err) = match io::imread(input_path, use_base64) {
        Ok(img) => {
            let use_base64 = !use_base64;
            match io::imwrite(output_path.as_str(), img, use_base64) {
                Ok(_) => {
                    println!("Image was saved to {}", output_path);
                    None
                }
                Err(err) => Some(err),
            }
        }
        Err(err) => Some(err),
    } {
        println!("{err}");
    }
}
