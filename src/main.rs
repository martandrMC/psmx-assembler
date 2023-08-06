#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(psmx);

#[macro_use]
mod lexer;
use crate::lexer::{Lexer, Token};

mod ast;

use std::io::{Result, Read, Write};
use std::fs::File;

fn main() -> Result<()> {
	let src_path: &'static str = "input.psmx";
	let dst_path: &'static str = "output.txt";
	let mut content = get_file(src_path)?;
	content.push('\n');
	let lexer = Lexer::new(&content);
	//return run_parser(dst_path, lexer);
	return dump_tokens(dst_path, lexer);
}

#[allow(dead_code)]
fn dump_tokens(path: &str, lexer: Lexer) -> Result<()> {
	let mut dst_file = File::create(path)?;
	for x in lexer {
		let Ok((_,t,_)) = x else { writeln!(dst_file, "Err")?; break };
		write!(dst_file, "{:?} ", t)?;
		if let Token::EOL = t { write!(dst_file, "\n")?; }
	}
	return Ok(());
}

#[allow(dead_code)]
fn run_parser(path: &str, lexer: Lexer) -> Result<()> {
	let mut dst_file = File::create(path)?;
	let parser = psmx::AsmFileParser::new();
	let result = parser.parse(lexer);
	writeln!(dst_file, "{:?}", result)?;
	return Ok(());
}

fn get_file(path: &str) -> Result<String> {
	let mut file = File::open(path)?;
	let mut content = String::new();
	file.read_to_string(&mut content)?;
	return Ok(content);
}
