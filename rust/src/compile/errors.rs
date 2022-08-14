use crate::lex::LexError;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
	LexError(LexError),
	EndOfToken,
	UnexpectedToken(String),
	TypeMismatch,
	TokenNotPunct,
}

impl std::fmt::Display for ParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			ParseError::LexError(_) => "lex error",
			ParseError::EndOfToken => "EndOfToken",
			ParseError::UnexpectedToken(s) => s.as_str(),
			ParseError::TypeMismatch => "TypeMismatch",
			ParseError::TokenNotPunct => "TokenNotPunct",
		})
	}
}

impl std::error::Error for ParseError {}

impl ParseError {
	pub fn expecting_str_but(es: &mut [String], g: &str) -> ParseError {
		es.sort();
		ParseError::UnexpectedToken(format!("expecting: {} got: {}", es.join(" "), g))
	}
}
