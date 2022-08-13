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
	UnexpectedToken,
	TypeMismatch,
	TokenNotPunct,
}

#[derive(Debug, PartialEq)]
pub struct SyntaxTree {
	token_list: Vec<Token>,
}

impl Token {
	pub fn tk_assign() -> &'static Token {
		static ASSIGN: Token = Token::Punct(Punct::Assign);
		&ASSIGN
	}

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

	pub fn is_char_type(&self) -> bool {
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
			_ => Err(ParseError::UnexpectedToken),
		}
	}

	fn next_int_const(iter: &mut core::slice::Iter<Token>) -> Result<i32, ParseError> {
		let tk = Self::next_token(iter)?;
		match tk {
			Token::IntegerConst(id) => Ok(id.parse().expect("error in lex")),
			Token::CharacterConst(_) => Err(ParseError::TypeMismatch),
			Token::StringLiteral(_) => Err(ParseError::TypeMismatch),
			_ => Err(ParseError::UnexpectedToken),
		}
	}

	fn next_char_const(iter: &mut core::slice::Iter<Token>) -> Result<char, ParseError> {
		let tk = Self::next_token(iter)?;
		match tk {
			Token::CharacterConst(id) => Ok(*id),
			_ => Err(ParseError::UnexpectedToken),
		}
	}

	fn expect_token(iter: &mut core::slice::Iter<Token>, etk: &Token) -> Result<(), ParseError> {
		let tk = Self::next_token(iter)?;
		if tk == etk {
			Ok(())
		} else {
			Err(ParseError::UnexpectedToken)
		}
	}

	fn peek_next_token<'a>(iter: &'a mut core::slice::Iter<Token>) -> Result<&'a Token, ParseError> {
		let mnt = iter.peekable().next();
		match mnt {
			None => Err(ParseError::EndOfToken),
			Some(nt) => Ok(nt),
		}
	}

	pub fn parse(&mut self) -> Result<DeclarationList, ParseError> {
		let mut dl = vec![];

		let mut iter = self.token_list.iter();
		while let Some(tk) = iter.next() {
			if tk.is_char_type() {
				// char b = 'B', c, d = 'D';
				while Self::peek_next_token(&mut iter)?.is_not_semicolon() {
					let id_name = Self::next_id(&mut iter)?;
					let next_punct = Self::peek_next_token(&mut iter)?.get_punct()?;
					if *next_punct == Punct::Assign {
						iter.next();
						let val = Self::next_char_const(&mut iter)?;
						dl.push(Declaration::Variable {
							ci_type: (CiType::CiChar),
							list: (vec![Declarator { name: id_name, value: CiValue::CiChar(val) }]),
						})
					} else if *next_punct == Punct::Comma {
						iter.next();
						dl.push(Declaration::Variable {
							ci_type: (CiType::CiChar),
							list: (vec![Declarator { name: id_name, value: CiValue::CiChar('\0') }]),
						})
					} else if *next_punct == Punct::Semicolon {
					} else {
						return Err(ParseError::UnexpectedToken);
					}
				}
			} else if tk.is_int_type() {
				let id_name = Self::next_id(&mut iter)?;
				Self::expect_token(&mut iter, Token::tk_assign())?;
				let val = Self::next_int_const(&mut iter)?;
				Self::expect_token(&mut iter, &Token::Todo(';'))?;
				dl.push(Declaration::Variable {
					ci_type: (CiType::CiChar),
					list: (vec![Declarator { name: id_name, value: CiValue::CiInt(val) }]),
				})
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
		let src = include_str!("../data/t0.c");
		assert_eq!(
			SyntaxTree::compile(src),
			Ok(vec![
				Declaration::Variable {
					ci_type: (CiType::CiChar),
					list: vec![Declarator { name: "c".into(), value: CiValue::CiChar('A') }]
				},
				Declaration::Variable {
					ci_type: (CiType::CiInt),
					list: vec![Declarator { name: "i".into(), value: CiValue::CiInt(1) }]
				},
			])
		);
		assert_eq!(SyntaxTree::compile(r###"char c = 'a'"###), Err(ParseError::EndOfToken));
		assert_eq!(SyntaxTree::compile(r###"char c = 'a' y "###), Err(ParseError::UnexpectedToken));
		assert_eq!(SyntaxTree::compile(r###"char c "###), Err(ParseError::UnexpectedToken));
		assert_eq!(SyntaxTree::compile(r###"int i = 'c';"###), Err(ParseError::TypeMismatch));
		assert_eq!(SyntaxTree::compile(r###"int i = "int";"###), Err(ParseError::TypeMismatch));
	}

	#[test]
	fn test_t1() {
		let src = r###"char; int;"###;
		assert_eq!(
			SyntaxTree::compile(src),
			Ok(vec![
				Declaration::Variable { ci_type: (CiType::CiChar), list: vec![] },
				Declaration::Variable { ci_type: (CiType::CiInt), list: vec![] },
			])
		);
		let src = r###"char b = 'B', c, d = 'D';"###;
		assert_eq!(
			SyntaxTree::compile(src),
			Ok(vec![Declaration::Variable {
				ci_type: (CiType::CiChar),
				list: vec![
					Declarator { name: "b".into(), value: CiValue::CiChar('B') },
					Declarator { name: "c".into(), value: CiValue::CiChar('\0') },
					Declarator { name: "d".into(), value: CiValue::CiChar('D') },
				]
			},])
		);
	}
}
