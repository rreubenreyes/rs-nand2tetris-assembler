use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};

use crate::state::State;

pub fn parse(target_file: &String) {
    println!("{} from the parser", target_file);

    let path = Path::new(target_file);
    let path_s = path.display();

    let source_file = match File::open(&path) {
        Err(why) => panic!("Could not open {}: {}", path_s, why),
        Ok(file) => file
    };

    let reader = BufReader::new(source_file);
    for (line_num, line) in reader.lines().enumerate() {
        match line {
            Err(why) => panic!("Error reading line {}: {}", line_num, why),
            Ok(_) => ()
        }

        print!("{}: ", line_num + 1);

        for ch in line.unwrap().chars() {
            print!("{}", ch);
        }

        print!("\n");
    }

    let state = State::new("test state");

    println!("State: {}", state);
}
