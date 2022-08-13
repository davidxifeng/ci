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

impl Token {
	pub fn is_int_type(&self) -> bool {
		match self {
			Token::Keyword(Keyword::Int) => true,
			_ => false,
		}
	}
	pub fn get_punct(&self) -> Result<&Punct, ParseError> {
		match self {
			Token::Punct(p) => Ok(p),
			_ => Err(ParseError::TokenNotPunct),
		}
	}

	pub fn is_not_semicolon(&self) -> bool {
		match self {
			Token::Punct(Punct::Semicolon) => false,
			_ => true,
		}
	}

	pub fn is_keyword_char(&self) -> bool {
		match self {
			Token::Keyword(Keyword::Char) => true,
			_ => false,
		}
	}

	pub fn is_enum_type(&self) -> bool {
		match self {
			Token::Keyword(kw) => *kw == Keyword::Enum,
			_ => false,
		}
	}
}

impl SyntaxTree {
	fn next_token<'a>(iter: &'a mut core::slice::Iter<Token>) -> Result<&'a Token, ParseError> {
		match iter.next() {
			None => Err(ParseError::EndOfToken),
			Some(tk) => Ok(tk),
		}
	}

	fn next_id(iter: &mut core::slice::Iter<Token>) -> Result<String, ParseError> {
		let tk = Self::next_token(iter)?;
		match tk {
			Token::Id(id) => Ok(id.clone()),
			_ => Err(ParseError::UnexpectedToken("not id".into())),
		}
	}

	fn next_int_const(iter: &mut core::slice::Iter<Token>) -> Result<i32, ParseError> {
		let tk = Self::next_token(iter)?;
		match tk {
			Token::IntegerConst(id) => Ok(id.parse().unwrap()),
			Token::CharacterConst(_) => Err(ParseError::TypeMismatch),
			Token::StringLiteral(_) => Err(ParseError::TypeMismatch),
			_ => Err(ParseError::UnexpectedToken("not int const".into())),
		}
	}

	fn next_char_const(iter: &mut core::slice::Iter<Token>) -> Result<char, ParseError> {
		let tk = Self::next_token(iter)?;
		match tk {
			Token::CharacterConst(id) => Ok(*id),
			Token::IntegerConst(_) => Err(ParseError::TypeMismatch),
			Token::StringLiteral(_) => Err(ParseError::TypeMismatch),
			_ => Err(ParseError::UnexpectedToken("not char const".into())),
		}
	}

	fn peek_next_token<'a>(iter: &'a mut std::slice::Iter<Token>) -> Result<&'a Token, ParseError> {
		// 会advance 迭代器的写法, 这样的话这个peekable很有问题呀
		// if let Some(nt) = iter.peekable().peek() {
		// 这样迭代可以,去掉clone之后,就会advance迭代器了
		// if let Some(nt) = iter.clone().peekable().peek() {
		if let Some(nt) = iter.clone().next() {
			Ok(nt)
		} else {
			Err(ParseError::EndOfToken)
		}
	}

	pub fn parse(&mut self) -> Result<DeclarationList, ParseError> {
		let mut dl = vec![];

		let mut iter = self.token_list.iter();
		while let Some(tk) = iter.next() {
			if tk.is_keyword_char() {
				let mut il = vec![];
				while Self::peek_next_token(&mut iter)?.is_not_semicolon() {
					let id_name = Self::next_id(&mut iter)?;
					let next_punct = Self::peek_next_token(&mut iter)?.get_punct()?;
					if *next_punct == Punct::Assign {
						iter.next();
						let val = Self::next_char_const(&mut iter)?;
						il.push(Declarator { name: id_name, value: CiValue::CiChar(val) });

						let next_punct = Self::peek_next_token(&mut iter)?.get_punct()?;
						if *next_punct == Punct::Comma {
							iter.next();
						} else if *next_punct == Punct::Semicolon {
							iter.next();
							break;
						} else {
							return Err(ParseError::expecting_but(&[",", ";"], next_punct.to_string().as_str()));
						}
					} else if *next_punct == Punct::Comma {
						iter.next();
						il.push(Declarator { name: id_name, value: CiValue::CiChar('\0') });
					} else if *next_punct == Punct::Semicolon {
						iter.next();
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
					let next_punct = Self::peek_next_token(&mut iter)?.get_punct()?;
					if *next_punct == Punct::Assign {
						iter.next();
						let val = Self::next_int_const(&mut iter)?;
						il.push(Declarator { name: id_name, value: CiValue::CiInt(val) });

						let next_punct = Self::peek_next_token(&mut iter)?.get_punct()?;
						if *next_punct == Punct::Comma {
							iter.next();
						} else if *next_punct == Punct::Semicolon {
							iter.next();
							break;
						} else {
							return Err(ParseError::UnexpectedToken("expecting: ,;".into()));
						}
					} else if *next_punct == Punct::Comma {
						il.push(Declarator { name: id_name, value: CiValue::CiInt(0) });
						iter.next();
					} else if *next_punct == Punct::Semicolon {
						iter.next();
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

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_t0() {
		assert_eq!(
			SyntaxTree::compile("char ; int ;"),
			Ok(vec![
				Declaration::Variable { ci_type: (CiType::CiChar), list: vec![] },
				Declaration::Variable { ci_type: (CiType::CiInt), list: vec![] },
			])
		);
		assert_eq!(
			SyntaxTree::compile("char a = 'A', b, c = 'C'; int i = 1;"),
			Ok(vec![
				Declaration::Variable {
					ci_type: (CiType::CiChar),
					list: vec![
						Declarator { name: "a".into(), value: CiValue::CiChar('A') },
						Declarator { name: "b".into(), value: CiValue::CiChar('\0') },
						Declarator { name: "c".into(), value: CiValue::CiChar('C') },
					]
				},
				Declaration::Variable {
					ci_type: (CiType::CiInt),
					list: vec![Declarator { name: "i".into(), value: CiValue::CiInt(1) }]
				},
			])
		);
		assert_eq!(SyntaxTree::compile(r###"char c = 'a'"###), Err(ParseError::EndOfToken));
		assert_eq!(SyntaxTree::compile(r###"char c = 'a' y "###), Err(ParseError::TokenNotPunct));
		assert_eq!(SyntaxTree::compile(r###"char c = 'a' = "###), Err(ParseError::expecting_but(&[",", ";"], "=")));
		assert_eq!(SyntaxTree::compile(r###"char c "###), Err(ParseError::EndOfToken));
		assert_eq!(SyntaxTree::compile(r###"int i = 'c';"###), Err(ParseError::TypeMismatch));
		assert_eq!(SyntaxTree::compile(r###"int i = "int";"###), Err(ParseError::TypeMismatch));
	}

	#[test]
	fn test_t1() {}
}
