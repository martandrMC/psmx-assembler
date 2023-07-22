#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(psmx);

mod lexer;
use crate::lexer::Lexer;

use std::io::{Result, Read};
use std::fs::File;

fn main() -> Result<()> {
	let src_path: &'static str = "input.psmx";
	//let dst_path: &'static str = "output.txt";
	let content = get_file(src_path)?;
	let lexer = Lexer::new(&content);
	let parser = psmx::ThingParser::new();
	let result = parser.parse(lexer);
	println!("{:?}", result);
	return Ok(());
}

fn get_file(path: &str) -> Result<String> {
	let mut file = File::open(path)?;
	let mut content = String::new();
	file.read_to_string(&mut content)?;
	return Ok(content);
}
