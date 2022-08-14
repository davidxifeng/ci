mod tests;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CiType {
	BaseType(Keyword),
	// CiEnum(String),
}

impl std::fmt::Display for CiType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			Self::BaseType(kw) => match kw {
				Keyword::Char => "char ",
				Keyword::Int => "int ",
				_ => "<error>",
			},
		})
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Declarator {
	name: String,
	value: Const,
	// idr: i32,
}

impl std::fmt::Display for Declarator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{} = {}, ", self.name, self.value))
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
	Variable { ci_type: CiType, list: Vec<Declarator> },
	// Function { ci_type: CiType, name: String },
}

impl std::fmt::Display for Declaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Variable { ci_type, list } => {
				f.write_fmt(format_args!("{}", ci_type));

				if list.len() > 0 {
					let v = &list[0];
					if v.value == Const::Empty {
						f.write_fmt(format_args!("{} ", v.name));
					} else {
						f.write_fmt(format_args!("{} = {}", v.name, v.value));
					}
				}
				for v in list.iter().skip(1) {
					if v.value == Const::Empty {
						f.write_fmt(format_args!(", {} ", v.name));
					} else {
						f.write_fmt(format_args!(", {} = {}", v.name, v.value));
					}
				}
				Ok(())
			}
		}
	}
}

#[derive(Debug, PartialEq)]
pub struct DeclarationList {
	list: Vec<Declaration>,
}

impl std::convert::From<Vec<Declaration>> for DeclarationList {
	fn from(l: Vec<Declaration>) -> Self {
		DeclarationList { list: (l) }
	}
}

impl std::fmt::Display for DeclarationList {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for v in &self.list {
			f.write_fmt(format_args!("{}", v));
			f.write_str(";\n");
		}
		Ok(())
	}
}

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

type ParseResult = Result<DeclarationList, ParseError>;

impl SyntaxTree {
	fn peek_next_token<'a>(iter: &'a std::slice::Iter<Token>) -> Result<&'a Token, ParseError> {
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

	fn next_const(iter: &mut core::slice::Iter<Token>) -> Result<Const, ParseError> {
		let tk = Self::take_next_token(iter)?;
		match tk {
			Token::Const(c) => Ok(*c),
			Token::StringLiteral(_) => Err(ParseError::TypeMismatch),
			_ => Err(ParseError::UnexpectedToken("not char const".into())),
		}
	}
}

impl SyntaxTree {
	fn parse_declaration(keyword: Keyword, iter: &mut core::slice::Iter<Token>) -> Result<Declaration, ParseError> {
		let mut il = vec![];

		while Self::peek_next_token(iter)?.is_not_semicolon() {
			let id_name = Self::next_id(iter)?;
			let next_punct = Self::take_next_token(iter)?.get_punct()?;
			if *next_punct == Punct::Assign {
				let val = Self::next_const(iter)?.check_type_match(&keyword)?;
				il.push(Declarator { name: id_name, value: val });

				let next_punct = Self::take_next_token(iter)?.get_punct()?;
				if *next_punct == Punct::Comma {
				} else if *next_punct == Punct::Semicolon {
					break;
				} else {
					return Err(ParseError::expecting_but(&[",", ";"], next_punct.to_string().as_str()));
				}
			} else if *next_punct == Punct::Comma {
				il.push(Declarator { name: id_name, value: Default::default() });
			} else if *next_punct == Punct::Semicolon {
				break;
			} else {
				return Err(ParseError::expecting_but(&[",", ";", "="], next_punct.to_string().as_str()));
			}
		}

		Ok(Declaration::Variable { ci_type: (CiType::BaseType(keyword)), list: (il) })
	}
}

impl SyntaxTree {
	pub fn parse(&mut self) -> ParseResult {
		let mut dl = vec![];
		let mut iter = self.token_list.iter();

		while let Some(tk) = iter.next() {
			if let Some(kw) = tk.try_basetype_keyword() {
				dl.push(Self::parse_declaration(kw, &mut iter)?);
			} else if tk.is_enum_type() {
			}
		}

		Ok(DeclarationList { list: dl })
	}

	pub fn compile(input: &str) -> Result<DeclarationList, ParseError> {
		match TokenApi::parse_all(input) {
			Ok(token_list) => (SyntaxTree { token_list }).parse(),
			Err(err) => Err(ParseError::LexError(err)),
		}
	}
}
