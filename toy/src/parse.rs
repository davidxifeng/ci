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
pub enum Declaration {
	Variable { ci_type: CiType, name: String, value: CiValue },
	// Function { ci_type: CiType, name: String },
}

pub type DeclarationList = Vec<Declaration>;

#[derive(Debug, PartialEq)]
pub enum ParseError {
	LexError(LexError),
	EndOfToken,
	NotExpectedToken,
}

#[derive(Debug, PartialEq)]
pub struct SyntaxTree {
	token_list: Vec<Token>,
}

impl Token {
	pub fn tk_assign() -> Token {
		Token::Punct(Punct::Assign)
	}

	pub fn is_int_type(&self) -> Option<CiType> {
		match self {
			Token::Keyword(Keyword::Int) => Some(CiType::CiInt),
			_ => None,
		}
	}

	pub fn is_char_type(&self) -> Option<CiType> {
		match self {
			Token::Keyword(Keyword::Char) => Some(CiType::CiChar),
			_ => None,
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
			_ => Err(ParseError::NotExpectedToken),
		}
	}

	fn next_int_const(iter: &mut core::slice::Iter<Token>) -> Result<i32, ParseError> {
		let tk = Self::next_token(iter)?;
		match tk {
			Token::IntegerConst(id) => Ok(id.parse().expect("error in lex")),
			_ => Err(ParseError::NotExpectedToken),
		}
	}

	fn next_char_const(iter: &mut core::slice::Iter<Token>) -> Result<char, ParseError> {
		let tk = Self::next_token(iter)?;
		match tk {
			Token::CharacterConst(id) => Ok(*id),
			_ => Err(ParseError::NotExpectedToken),
		}
	}

	fn expect_token(iter: &mut core::slice::Iter<Token>, etk: &Token) -> Result<(), ParseError> {
		match iter.next() {
			None => Err(ParseError::EndOfToken),
			Some(tk) => {
				if tk == etk {
					Ok(())
				} else {
					Err(ParseError::NotExpectedToken)
				}
			}
		}
	}

	pub fn parse(&mut self) -> Result<DeclarationList, ParseError> {
		let mut dl = vec![];

		let mut iter = self.token_list.iter();
		while let Some(tk) = iter.next() {
			if let Some(cit) = tk.is_int_type() {
				let id_name = Self::next_id(&mut iter)?;
				Self::expect_token(&mut iter, &Token::tk_assign())?;
				let val = Self::next_int_const(&mut iter)?;
				Self::expect_token(&mut iter, &Token::Todo(';'))?;
				dl.push(Declaration::Variable { ci_type: (cit), name: (id_name), value: (CiValue::CiInt(val)) })
			} else if let Some(cit) = tk.is_char_type() {
				let id_name = Self::next_id(&mut iter)?;
				Self::expect_token(&mut iter, &Token::tk_assign())?;
				let val = Self::next_char_const(&mut iter)?;
				Self::expect_token(&mut iter, &Token::Todo(';'))?;
				dl.push(Declaration::Variable { ci_type: (cit), name: (id_name), value: (CiValue::CiChar(val)) })
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
				Declaration::Variable { ci_type: (CiType::CiChar), name: ("c".into()), value: (CiValue::CiChar('A')) },
				Declaration::Variable { ci_type: (CiType::CiInt), name: ("i".into()), value: (CiValue::CiInt(1)) },
				Declaration::Variable { ci_type: (CiType::CiInt), name: ("j".into()), value: (CiValue::CiInt(2)) },
			])
		);
		assert_eq!(SyntaxTree::compile(r###"char c = 'a'"###), Err(ParseError::EndOfToken));
		assert_eq!(SyntaxTree::compile(r###" char c = 'a' y "###), Err(ParseError::NotExpectedToken));
		assert_eq!(SyntaxTree::compile(r###" char c; "###), Err(ParseError::NotExpectedToken));
	}

	#[test]
	fn test_simple() {
		let src = include_str!("../data/simple.c");
		println!("{}\n{:?}", src, SyntaxTree::compile(src));
	}
}
