extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use serde_json::Error;
use std::collections::HashMap;

fn main() {
    tokenize();
}

#[derive(Serialize, Deserialize, Default)]
struct Tokens {
    map: HashMap<String, i32>
}

fn tokenize() -> Result<(), Error>{

    let test_str : String = "Through his portrayal in Plato's dialogues, Socrates has become renowned for his contribution to the field of ethics, and it is this Platonic Socrates who lends his name to the concepts of Socratic irony and the Socratic method, or elenchus. The latter remains a commonly used tool in a wide range of discussions, and is a type of pedagogy in which a series of questions is asked not only to draw individual answers, but also to encourage fundamental insight into the issue at hand. Plato's Socrates also made important and lasting contributions to the field of epistemology, and his ideologies and approach have proven a strong foundation for much Western philosophy that has followed.".to_string();

    let test_str_slice = test_str.as_str();
    let separators : &[char] = &[' ', '.', '?', '!', ',', ';', ':', '(', ')', '[', ']'];

    let tokens = test_str_slice.split(separators);

    let mut tks = Tokens::default(); //HashMap::new();

    for token in tokens {
        let count = tks.map.entry(token.to_owned()).or_insert(0);
        *count += 1;
    }

    println!("{:?}", tks.map);

    let j = serde_json::to_string(&tks)?;

    println!("{}", j);

    Ok(())
}
