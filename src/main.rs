extern crate clap;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use clap::{Arg,App};
use serde_json::Error;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let matches = App::new("tokenizer")
        .version("0.1.0")
        .author("Srinand Balaji <srinand@berkeley.edu>")
        .about("basic tokenizer written in rust.")
        .arg(Arg::with_name("file")
             .required(true)
             .takes_value(true)
             .index(1)
             .help("text file to tokenize"))
        .get_matches();
    let file = matches.value_of("file").unwrap();
    println!("{}", file);
    
    let mut file = File::open(file).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    //println!("{}", contents);
    tokenize(contents);
}

#[derive(Serialize, Deserialize, Default)]
struct Tokens {
    map: HashMap<String, i32>
}

fn tokenize(contents: String) -> Result<(), Error>{

    let test_str_slice = contents.as_str();
    let separators : &[char] = &[' ', '.', '?', '!', ',', ';', ':', '(', ')', '[', ']'];

    let tokens = test_str_slice.split(separators);

    let mut tks = Tokens::default(); //HashMap::new();

    for token in tokens {
        let count = tks.map.entry(token.to_owned()).or_insert(0);
        *count += 1;
    }

    //println!("{:?}", tks.map);

    let j = serde_json::to_string(&tks)?;

    println!("{}", j);

    Ok(())
}
