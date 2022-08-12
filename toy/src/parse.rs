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
	Function { ci_type: CiType, name: String },
}

pub type DeclarationList = Vec<Declaration>;

#[derive(Debug, PartialEq)]
pub enum ParseError {
	LexError(LexError),
}

#[derive(Debug, PartialEq)]
pub struct SyntaxTree {
	token_list: Vec<Token>,
}

impl SyntaxTree {
	pub fn parse(&mut self) -> Result<DeclarationList, ParseError> {
		Ok(vec![
			Declaration::Variable { ci_type: (CiType::CiChar), name: ("c".into()), value: (CiValue::CiChar('A')) },
			Declaration::Variable { ci_type: (CiType::CiInt), name: ("i".into()), value: (CiValue::CiInt(1)) },
			Declaration::Function { ci_type: (CiType::CiInt), name: ("main".into()) },
		])
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
	fn test_simple() {
		let src = include_str!("../data/simple.c");
		println!("{}\n{:?}", src, SyntaxTree::compile(src));
	}
}
