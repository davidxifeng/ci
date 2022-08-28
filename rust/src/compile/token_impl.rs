use std::fmt::{Formatter, Write};

use super::{
	errors::ParseError,
	token::{Const, Keyword, Punct, Token},
};

impl Token {
	pub fn try_basetype_keyword(&self) -> Option<Keyword> {
		match self {
			Token::Keyword(kw) => match kw {
				Keyword::Char | Keyword::Int => Some(*kw),
				_ => None,
			},
			_ => None,
		}
	}
}

impl std::str::FromStr for Punct {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"+" => Ok(Self::Add),
			"-" => Ok(Self::Sub),
			"*" => Ok(Self::Mul),
			"/" => Ok(Self::Div),
			"%" => Ok(Self::Mod),
			"==" => Ok(Self::Eq),
			"!=" => Ok(Self::Ne),
			_ => Err(()),
		}
	}
}

impl std::fmt::Display for Punct {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.pad(match self {
			Self::Add => "+",
			Self::Assign => "=",
			Self::Comma => ",",
			Self::Semicolon => ";",
			Self::Not => "!",
			Self::Cond => "?",
			Self::Lor => "||",
			Self::Lan => "&&",
			Self::Or => "|",
			Self::Xor => "^",
			Self::And => "&",
			Self::Eq => "==",
			Self::Ne => "!=",
			Self::Lt => "<",
			Self::Gt => ">",
			Self::Le => "<=",
			Self::Ge => ">=",
			Self::Shl => ">>",
			Self::Shr => "<<",
			Self::Sub => "-",
			Self::Mul => "*",
			Self::Div => "/",
			Self::Mod => "%",
			Self::Inc => "++",
			Self::Dec => "--",
			Self::BrakL => "[",
			Self::BrakR => "]",
			Self::BracesL => "{",
			Self::BracesR => "}",
			Self::ParentheseL => "(",
			Self::ParentheseR => ")",
			Self::Tilde => "~",
			Self::Colon => ":",
		})
	}
}

// 6.4 Lexical elements
// token:
//      keyword
//      identifier
//      constant: int, float, enum, char
//      string-literal
//      punctuator
impl Const {
	pub fn check_type_match(self, kw: &Keyword) -> Result<Self, ParseError> {
		match self {
			Self::Empty => Ok(self),
			Self::Character(_) => {
				if *kw == Keyword::Char {
					Ok(self)
				} else {
					Err(ParseError::TypeMismatch)
				}
			}
			Self::Integer(_) => {
				if *kw == Keyword::Int {
					Ok(self)
				} else {
					Err(ParseError::TypeMismatch)
				}
			}
		}
	}
}

impl std::fmt::Display for Const {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Empty => Ok(()),
			Self::Character(c) => {
				f.write_char('\'')?;
				if let Some(s) = simple_unescape(c) {
					f.write_str(s)?
				} else {
					f.write_char(*c)?
				}
				f.write_char('\'')
			}
			Self::Integer(i) => f.write_str(i.as_str()),
		}
	}
}

#[inline]
fn simple_unescape(c: &char) -> Option<&'static str> {
	// Rust中的转义:
	// https://doc.rust-lang.org/reference/tokens.html
	// (6.4.4.4) simple-escape-sequence:
	// one of \' \" \? \\ \a \b \f \n \r \t \v
	match *c {
		'\'' => Some("\\\''"),
		'"' => Some("\\\""),
		'\x3f' => Some("\\?"),
		'\\' => Some("\\\\"),
		'\x07' => Some("\\a"), // aleat, bell
		'\x08' => Some("\\b"), // backspace
		'\x0C' => Some("\\f"), // formfeed page break
		'\n' => Some("\\n"),   // 0a
		'\r' => Some("\\r"),   // 0d
		'\t' => Some("\\t"),   // 09 horizontal Tab
		'\x0b' => Some("\\v"), // vertical tab
		_ => None,
	}
}
