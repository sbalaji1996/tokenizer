fn main() {

    let test_str : String = "Through his portrayal in Plato's dialogues, Socrates has become renowned for his contribution to the field of ethics, and it is this Platonic Socrates who lends his name to the concepts of Socratic irony and the Socratic method, or elenchus. The latter remains a commonly used tool in a wide range of discussions, and is a type of pedagogy in which a series of questions is asked not only to draw individual answers, but also to encourage fundamental insight into the issue at hand. Plato's Socrates also made important and lasting contributions to the field of epistemology, and his ideologies and approach have proven a strong foundation for much Western philosophy that has followed.".to_string();

    let test_str_slice = test_str.as_str();
    let seperators : &[char] = &[' ', '.', '?', '!', ',', ';', ':', '(', ')', '[', ']']; 

    let tokens = test_str_slice.split(seperators);

    for token in tokens {
        println!("{}", token);
    }
}
