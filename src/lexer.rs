use std::collections::{HashMap, HashSet};
use std::str::CharIndices;
use std::iter::Peekable;

pub type Spanned<T, L, E> = Result<(L, T, L), E>;
pub type LexItem = Spanned<Token, usize, ()>;

#[derive(Debug, Clone)]
pub enum Token {
	_Err, EOL, SEP,
	
	Identifier(String), LitNumeric(String), LitString(String),
	
	KWPragma, KWDefine, KWReloc, KWIncl, KWMeta, KWSect, KWCode, KWData,
	KWEnd, KWMulti, KWArray, KWReserve, KWString, KWOnly,
	
	LSqParen, RSqParen, LParen, RParen,
	At, Bang, Percent, Colon, Dot,
	Plus, Minus, Mult, Dollar, Hash,
}

pub struct Lexer<'input> {
	symbols: HashMap<char, Token>,
	keywords: HashMap<&'input str, Token>,
	chars: Peekable<CharIndices<'input>>,
}

macro_rules! mkhash {
	($name:ident, $keytype:ty, $valtype:ty; $($key:expr => $value:expr;)+) => {
		let mut $name: HashMap<$keytype, $valtype> = HashMap::new();
		$($name.insert($key, $value));+
	};
	($name:ident, $keytype:ty; $($key:expr;)+) => {
		let mut $name: HashSet<$keytype> = HashSet::new();
		$($name.insert($key));+
	};
}

impl<'input> Lexer<'input> {
	pub fn new(input: &'input str) -> Self {
		mkhash!(keywords, &'input str, Token;
			"reserve"	=>	Token::KWReserve;
			"define"	=>	Token::KWDefine;
			"string"	=>	Token::KWString;
			"pragma"	=>	Token::KWPragma;
			"multi"		=>	Token::KWMulti;
			"array"		=>	Token::KWArray;
			"reloc"		=>	Token::KWReloc;
			"incl"		=>	Token::KWIncl;
			"meta"		=>	Token::KWMeta;
			"sect"		=>	Token::KWSect;
			"code"		=>	Token::KWCode;
			"data"		=>	Token::KWData;
			"only"		=>	Token::KWOnly;
			"end"		=>	Token::KWEnd;
		);
		mkhash!(symbols, char, Token;
			'@'	=>	Token::At;
			'('	=>	Token::LParen;
			')'	=>	Token::RParen;
			'['	=>	Token::LSqParen;
			']'	=>	Token::RSqParen;
			'!'	=>	Token::Bang;
			'%'	=>	Token::Percent;
			':'	=>	Token::Colon;
			'.'	=>	Token::Dot;
			'+'	=>	Token::Plus;
 			'-'	=>	Token::Minus;
 			'*'	=>	Token::Mult;
 			'$'	=>	Token::Dollar;
			'#'	=>	Token::Hash;
		);
		Lexer { symbols, keywords, chars: input.char_indices().peekable() }
	}
}

impl<'input> Iterator for Lexer<'input> {
	type Item = LexItem;
	fn next(&mut self) -> Option<Self::Item> {
		let (start, c) = self.chars.next()?;
		let mut end = start;
		let tok = match c {
			';' | ' ' | '\t' | '\r' | '\n' => {
				let mut comment = c == ';';
				let mut eol = c == '\r' || c == '\n';
				loop {
					match self.chars.peek() {
						Some((_, ';')) => { comment = true; end += 1 },
						Some((_, ' ')) | Some((_, '\t')) => end += 1,
						Some((_, '\r')) | Some((_, '\n')) => { eol = true; comment = false; end += 1 },
						Some((_, _)) => if !comment { break } else { end += 1 },
						None => break,
					}
					self.chars.next();
				}
				if start == 0 { return self.next() }
				if eol { Token::EOL } else { Token::SEP }
			},
			'"' => {
				let mut buf = String::from(c);
				let mut escape = false;
				let mut exit = false;
				loop {
					let x = self.chars.peek();
					if let None = x { break }
					let (_, c_) = x.unwrap();
					match c_ {
						'\r' | '\n' => break,
						'\\' => if !escape { escape = true },
						'"' => if !escape { exit = true },
						_ => escape = false,
					}
					buf.push(*c_);
					end += 1;
					self.chars.next();
					if exit { break }
				}
				Token::LitString(buf)
			},
			_ => {
				if let Some(x) = self.symbols.get(&c) { x.clone() }
				else if c.is_ascii_digit() {
					let mut buf = String::from(c);
					loop {
						let x = self.chars.peek();
						if let None = x { break }
						let (_, c_) = x.unwrap();
						if *c_ != '_' {
							if !c_.is_ascii_alphanumeric() { break }
							buf.push(*c_);
						}
						end += 1;
						self.chars.next();
					}
					match self.keywords.get(buf.as_str()) {
						Some(x) => x.clone(),
						None => Token::LitNumeric(buf),
					}
				} else if c.is_ascii_alphabetic() {
					let mut buf = String::from(c);
					loop {
						let x = self.chars.peek();
						if let None = x { break }
						let (_, c_) = x.unwrap();
						if !c_.is_ascii_alphanumeric() { break }
						buf.push(*c_);
						end += 1;
						self.chars.next();
					}
					match self.keywords.get(buf.as_str()) {
						Some(x) => x.clone(),
						None => Token::Identifier(buf),
					}
				} else { Token::_Err }
			},
		};
		if let Token::_Err = tok { return Some(Err(())); }
		else { return Some(Ok((start, tok, end))); }
	}
}
