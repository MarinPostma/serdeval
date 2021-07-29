use serdeval::*;
use std::fs::File;
use std::env::args;
use std::io::BufReader;

fn main() {
    let mut args = args();
    args.next().unwrap();
    let input_file = args.next().unwrap();

    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    match serde_json::from_reader::<_, Any>(reader) {
        Ok(_) => println!("Valid json"),
        Err(e) => println!("Invalid json: {}", e),
    }
}
