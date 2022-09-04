use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum LexError {
	InvalidChar(char),
	UnexpectedEof,
	EmptyChar,
	MoreThanOneChar,
	ExpectingBut(char, char),
	UnknownEscape(char),
}

impl Display for LexError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s;
		f.write_str(match self {
			LexError::ExpectingBut(e, g) => {
				s = format!("expect: {}, got: {}", e, g);
				&s
			}
			LexError::InvalidChar(c) => {
				s = format!("invalid char: {}", c);
				&s
			}
			LexError::UnknownEscape(c) => {
				s = format!("unknown escape: {}", c);
				&s
			}
			LexError::MoreThanOneChar => "MoreThanOneChar",
			LexError::EmptyChar => "EmptyChar",
			LexError::UnexpectedEof => "UnexpectedEof",
		})
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
	LexError(LexError),
	Unexpected(String),
	General(&'static str),
	EndOfToken,
	NoMoreExpr,
	TypeMismatch,
	NotPunct,
	NotKeyword,
	NotIdentifier,
}

impl Display for ParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s;
		f.write_str(match self {
			ParseError::LexError(e) => {
				s = format!("Lex Error: {}", e);
				&s
			}
			ParseError::Unexpected(s) => s.as_str(),
			ParseError::General(s) => s,
			ParseError::EndOfToken => "EndOfToken",
			ParseError::NoMoreExpr => "NoMoreExpr",
			ParseError::TypeMismatch => "TypeMismatch",
			ParseError::NotPunct => "NotPunct",
			ParseError::NotIdentifier => "NotIdentifier",
			ParseError::NotKeyword => "NotKeyword",
		})
	}
}

impl std::error::Error for ParseError {}

impl ParseError {
	pub fn expecting_but(es: &mut [String], g: &str) -> ParseError {
		es.sort();
		ParseError::Unexpected(format!("expecting: {} got: {}", es.join(" "), g))
	}
}
