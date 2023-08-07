type TheError = lalrpop_util::ParseError<usize, crate::lexer::Token, ()>;

pub fn handle_string_literal(string: &str) -> Result<String, TheError> {
	let err = Err(TheError::User { error: () });
	let mut input = String::from(string);
	if input.len() < 3 { return err }
	if input.chars().nth(0).unwrap() != '\"' { return err }
	if input.chars().last().unwrap() != '\"' { return err }
	input.remove(0);
	input.pop();

	let mut chars = input.chars();
	while let Some(c0) = chars.next() {
		if c0 != '\\' { continue }
		let Some(c1) = chars.next() else { return err };
		match c1 {
			'a'|'b'|'e'|'f'|'n'|'r'|'t'|'v'|'\\'|'"'|'0' => {},
			'x' => {
				let Some(c2) = chars.next() else { return err };
				let Some(c3) = chars.next() else { return err };
				if !c2.is_ascii_hexdigit() { return err }
				if !c3.is_ascii_hexdigit() { return err }
			},
			_ => return err,
		}
	}
	return Ok(input);
}

pub fn handle_unsigned_literal(string: &str) -> Result<u16, TheError> {
	let mut sanitised = string.replace("_", "");
	let number = match sanitised.chars().nth(1) {
		Some('h') | Some('H') => {
			sanitised.remove(1);
			u16::from_str_radix(&sanitised, 16)
		},
		Some('b') | Some('B') => {
			sanitised.remove(1);
			u16::from_str_radix(&sanitised, 2)
		},
		Some(_) | None => u16::from_str_radix(&sanitised, 10),
	};
	return number.map_err(|_| TheError::User { error: () });
}

pub fn handle_signed_literal(string: &str) -> Result<i16, TheError> {
	let mut sanitised = string.replace("_", "");
	let number = match sanitised.chars().nth(2) {
		Some('h') | Some('H') => {
			sanitised.remove(2);
			i16::from_str_radix(&sanitised, 16)
		},
		Some('b') | Some('B') => {
			sanitised.remove(2);
			i16::from_str_radix(&sanitised, 2)
		},
		Some(_) | None => i16::from_str_radix(&sanitised, 10),
	};
	return number.map_err(|_| TheError::User { error: () });
}
