//! Test runs for base64 conversion
const IMB64_NAME: &str = "image.txt";
const IMAGE_NAME: &str = "image.webp";

#[test]
fn run_base64_encode() {
    let imbytes = super::read_sample(IMAGE_NAME);

    let strb64 = base64::encode(imbytes);
    
    let imbts64 = strb64.as_bytes();

    super::write_result(IMB64_NAME, imbts64);
}

#[test]
fn run_base64_decode() {
    let imbts64 = super::read_sample(IMB64_NAME);

    let imbytes = base64::decode(imbts64).unwrap();

    super::write_result(IMAGE_NAME, &imbytes);
}
