use std::{fs::File, io::{self, BufRead}, path::Path};

use dial::Dial;

mod dial;

fn main() {
    let input_path = Path::new("input/day01.in");

    let input_file = File::open(input_path).expect("Couldn't open input");

    let lines = io::BufReader::new(input_file).lines();

    let mut dial = Dial::new();
    for rotation in lines.map_while(Result::ok) {
        dial.apply(&rotation);
    }

    println!("{}", dial.password());
    println!("{}", dial.real_password());
}
