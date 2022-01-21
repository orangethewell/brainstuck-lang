use std::env;
use std::fs;
use std::process;

mod ast;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let path = &args[1];
    let content = fs::read_to_string(path)
        .expect("Something went wrong reading the file");

    let opcode_list = ast::decode_to_opcodes(content);
    let program = ast::build_tree(opcode_list);

    if program.is_err() {
        process::exit(program.err().unwrap());
    }

}
