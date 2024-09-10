// use std::io::{self, Read};
use std::io;
use serde_json::Value;

fn main() {
    // println!("Hello, world!");
    let stdin = io::stdin();
    let handle = stdin.lock();
    // let mut input = String::new();
    // let _ = handle.read_to_string(&mut input);
    // println!("input:\n{}\n", input);

    let stream = serde_json::Deserializer::from_reader(handle).into_iter::<Value>();
    for value in stream {
        match value {
            Ok(v) => {
                println!("input:\n{}\n", v);
            }
            Err(_) => todo!()
        }
    }
}
