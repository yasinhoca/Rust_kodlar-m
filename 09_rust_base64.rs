use base64::encode;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{

let string = b"rust dili";

let encoded = encode(string);

println!("Base64: {}", encoded);

Ok(())

}
