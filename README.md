# tokenizer

A command-line-based tokenizer written in Rust. Takes in a file and tokenizes each word, while also counting the number of times that word appears in the text file.

## usage

`/target/debug/tokenizer $TEXT_FILE.txt$`

## todo

- [x] convert hardcoded text into CL input
- [ ] configure tool to print output json file to the location of your choosing
- [ ] configure tool to allow user to configure delimeters 
- [ ] refactor such that the code meets the [style guide](https://doc.rust-lang.org/book/second-edition/ch12-03-improving-error-handling-and-modularity.html#separation-of-concerns-for-binary-projects)
- [ ] implement benchmark tests and optimize performance accordingly
