use std::{fs::OpenOptions, io::Write};

use base64::Engine;
fn main() {
    const DATA: &'static [u8] = include_bytes!("times.csv");
    let s: String = base64::prelude::BASE64_URL_SAFE.encode(DATA);
    let out = s.as_bytes();
    let mut options = OpenOptions::new();
    options.create(true).write(true);
    let mut file = options.open("./times.b64").unwrap();
    file.write(out).unwrap();
}
