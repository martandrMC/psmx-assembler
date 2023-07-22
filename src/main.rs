#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(psmx);

fn main() {
	let parser = psmx::ThingParser::new();
	let result = parser.parse("This string is 34 characters long!");
	println!("{:?}", result);
}
