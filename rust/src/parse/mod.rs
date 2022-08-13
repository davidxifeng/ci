mod tests;

use crate::*;

#[derive(Debug, PartialEq)]
pub enum CiType {
	CiInt,
	CiChar,
	// CiEnum(String),
}

#[derive(Debug, PartialEq)]
pub enum CiValue {
	CiInt(i32),
	CiChar(char),
	// CiEnum(i32),
}

#[derive(Debug, PartialEq)]
pub struct Declarator {
	name: String,
	value: CiValue,
	// idr: i32,
}

#[derive(Debug, PartialEq)]
pub enum Declaration {
	Variable { ci_type: CiType, list: Vec<Declarator> },
	// Function { ci_type: CiType, name: String },
}

pub type DeclarationList = Vec<Declaration>;

#[derive(Debug, PartialEq)]
pub enum ParseError {
	LexError(LexError),
	EndOfToken,
	UnexpectedToken(String),
	TypeMismatch,
	TokenNotPunct,
}

impl ParseError {
	pub fn expecting_but(es: &[&str], g: &str) -> ParseError {
		let mut ds = es.to_vec();
		ds.sort();
		ParseError::UnexpectedToken(format!("expecting: {} got: {}", ds.join(" "), g))
	}
}

#[derive(Debug, PartialEq)]
pub struct SyntaxTree {
	token_list: Vec<Token>,
}


impl SyntaxTree {
	fn peek_next_token<'a>(iter: &'a mut std::slice::Iter<Token>) -> Result<&'a Token, ParseError> {
		if let Some(nt) = iter.clone().next() {
			Ok(nt)
		} else {
			Err(ParseError::EndOfToken)
		}
	}

	fn take_next_token<'a>(iter: &'a mut core::slice::Iter<Token>) -> Result<&'a Token, ParseError> {
		if let Some(tk) = iter.next() {
			Ok(tk)
		} else {
			Err(ParseError::EndOfToken)
		}
	}

	fn next_id(iter: &mut core::slice::Iter<Token>) -> Result<String, ParseError> {
		let tk = Self::take_next_token(iter)?;
		match tk {
			Token::Id(id) => Ok(id.clone()),
			_ => Err(ParseError::UnexpectedToken("not id".into())),
		}
	}

	fn next_int_const(iter: &mut core::slice::Iter<Token>) -> Result<i32, ParseError> {
		let tk = Self::take_next_token(iter)?;
		match tk {
			Token::IntegerConst(id) => Ok(id.parse().unwrap()),
			Token::CharacterConst(_) => Err(ParseError::TypeMismatch),
			Token::StringLiteral(_) => Err(ParseError::TypeMismatch),
			_ => Err(ParseError::UnexpectedToken("not int const".into())),
		}
	}

	fn next_char_const(iter: &mut core::slice::Iter<Token>) -> Result<char, ParseError> {
		let tk = Self::take_next_token(iter)?;
		match tk {
			Token::CharacterConst(id) => Ok(*id),
			Token::IntegerConst(_) => Err(ParseError::TypeMismatch),
			Token::StringLiteral(_) => Err(ParseError::TypeMismatch),
			_ => Err(ParseError::UnexpectedToken("not char const".into())),
		}
	}
}

impl SyntaxTree {
	pub fn parse(&mut self) -> Result<DeclarationList, ParseError> {
		let mut dl = vec![];
		let mut iter = self.token_list.iter();

		while let Some(tk) = iter.next() {
			if tk.is_keyword_char() {
				let mut il = vec![];
				while Self::peek_next_token(&mut iter)?.is_not_semicolon() {
					let id_name = Self::next_id(&mut iter)?;
					let next_punct = Self::take_next_token(&mut iter)?.get_punct()?;
					if *next_punct == Punct::Assign {
						let val = Self::next_char_const(&mut iter)?;
						il.push(Declarator { name: id_name, value: CiValue::CiChar(val) });

						let next_punct = Self::take_next_token(&mut iter)?.get_punct()?;
						if *next_punct == Punct::Comma {
						} else if *next_punct == Punct::Semicolon {
							break;
						} else {
							return Err(ParseError::expecting_but(&[",", ";"], next_punct.to_string().as_str()));
						}
					} else if *next_punct == Punct::Comma {
						il.push(Declarator { name: id_name, value: CiValue::CiChar('\0') });
					} else if *next_punct == Punct::Semicolon {
						break;
					} else {
						return Err(ParseError::UnexpectedToken("only = , ; allowed after id".into()));
					}
				}
				dl.push(Declaration::Variable { ci_type: (CiType::CiChar), list: (il) });
			} else if tk.is_int_type() {
				let mut il = vec![];

				while Self::peek_next_token(&mut iter)?.is_not_semicolon() {
					let id_name = Self::next_id(&mut iter)?;
					let next_punct = Self::take_next_token(&mut iter)?.get_punct()?;
					if *next_punct == Punct::Assign {
						let val = Self::next_int_const(&mut iter)?;
						il.push(Declarator { name: id_name, value: CiValue::CiInt(val) });

						let next_punct = Self::take_next_token(&mut iter)?.get_punct()?;
						if *next_punct == Punct::Comma {
						} else if *next_punct == Punct::Semicolon {
							break;
						} else {
							return Err(ParseError::UnexpectedToken("expecting: ,;".into()));
						}
					} else if *next_punct == Punct::Comma {
						il.push(Declarator { name: id_name, value: CiValue::CiInt(0) });
					} else if *next_punct == Punct::Semicolon {
						break;
					} else {
						return Err(ParseError::UnexpectedToken("only = , ; allowed after id".into()));
					}
				}
				dl.push(Declaration::Variable { ci_type: (CiType::CiInt), list: (il) });
			} else if tk.is_enum_type() {
			}
		}

		Ok(dl)
	}

	pub fn compile(input: &str) -> Result<DeclarationList, ParseError> {
		match TokenApi::parse_all(input) {
			Ok(token_list) => (SyntaxTree { token_list }).parse(),
			Err(err) => Err(ParseError::LexError(err)),
		}
	}
}
