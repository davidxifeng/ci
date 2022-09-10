use std::{
	fmt::{Display, Formatter, Write},
	str::FromStr,
};

use console::style;

use super::{
	errors::LexError,
	lex::TokenApi,
	token::{Const, Keyword, Precedence, Punct, Token, TokenList},
};

impl Punct {
	pub fn is_binary_op(&self) -> bool {
		matches!(
			self,
			Punct::Add
				| Punct::Sub | Punct::Mul
				| Punct::Div | Punct::Mod
				| Punct::Shl | Punct::Shr
				| Punct::And | Punct::Or
				| Punct::Xor | Punct::Lan
				| Punct::Lor | Punct::Lt
				| Punct::Le | Punct::Ge
				| Punct::Gt | Punct::Eq
				| Punct::Ne
		)
	}

	pub fn is_assign(&self) -> bool {
		matches!(
			self,
			Punct::Assign
				| Punct::AssignAdd
				| Punct::AssignSub
				| Punct::AssignMul
				| Punct::AssignDiv
				| Punct::AssignMod
				| Punct::AssignShl
				| Punct::AssignShr
				| Punct::AssignBAnd
				| Punct::AssignBOr
				| Punct::AssignBXor
		)
	}
}

impl Display for Punct {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let s = match self {
			Self::Add => "+",
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
			Self::Assign => "=",
			Self::AssignAdd => "+=",
			Self::AssignSub => "-=",
			Self::AssignMul => "*=",
			Self::AssignDiv => "/=",
			Self::AssignMod => "%=",
			Self::AssignShl => "<<=",
			Self::AssignShr => ">>=",
			Self::AssignBAnd => "&=",
			Self::AssignBOr => "|=",
			Self::AssignBXor => "^=",
			Self::Dot => ".",
			Self::Arrow => "->",
			Self::VARARG => "...",
		};
		if f.alternate() {
			f.write_str(&style(s).blue().bold().to_string())
		} else {
			f.write_str(s)
		}
	}
}

// 6.4 Lexical elements
// token:
//      keyword
//      identifier
//      constant: int, float, enum, char
//      string-literal
//      punctuator

impl Display for Const {
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

impl Display for Keyword {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let s = match self {
			Keyword::Complex => "complex",
			Keyword::Imaginary => "imaginary",
			Keyword::Bool => "bool",
			// Keyword::Bool => "_Bool",
			// Keyword::Complex => "_Complex",
			// Keyword::Imaginary => "_Imaginary",
			Keyword::Auto => "auto",
			Keyword::Break => "break",
			Keyword::Case => "case",
			Keyword::Char => "char",
			Keyword::Const => "const",
			Keyword::Continue => "continue",
			Keyword::Default => "default",
			Keyword::Do => "do",
			Keyword::Double => "double",
			Keyword::Else => "else",
			Keyword::Enum => "enum",
			Keyword::Extern => "extern",
			Keyword::Float => "float",
			Keyword::For => "for",
			Keyword::Goto => "goto",
			Keyword::If => "if",
			Keyword::Inline => "inline",
			Keyword::Int => "int",
			Keyword::Long => "long",
			Keyword::Register => "register",
			Keyword::Restrict => "restrict",
			Keyword::Return => "return",
			Keyword::Short => "short",
			Keyword::Signed => "signed",
			Keyword::SizeOf => "sizeof",
			Keyword::Static => "static",
			Keyword::Struct => "struct",
			Keyword::Switch => "switch",
			Keyword::Typedef => "typedef",
			Keyword::Union => "union",
			Keyword::Unsigned => "unsigned",
			Keyword::Void => "void",
			Keyword::Volatile => "volatile",
			Keyword::While => "while",
		};
		if f.alternate() {
			f.write_str(&style(s).bright().green().to_string())
		} else {
			f.write_str(s)
		}
	}
}

impl Display for Token {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Token::Const(c) => write!(f, "{}", c),
			Token::Id(id) => write!(f, "{}", id),
			Token::Keyword(kw) => write!(f, "{:#}", kw),
			Token::Punct(p) => write!(f, "{:#}", p),
			Token::StringLiteral(s) => f.write_str(s.as_str()),
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

impl FromStr for TokenList {
	type Err = LexError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match TokenApi::parse_all(s) {
			Ok(data) => Ok(TokenList { data }),
			Err(e) => Err(e),
		}
	}
}

impl Display for TokenList {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if f.alternate() {
			if let Some((first, elems)) = self.data.split_first() {
				write!(f, "{:?}", first)?;
				for tk in elems {
					f.write_str(&style(" ◦ ").dim().to_string())?;
					write!(f, "{:?}", tk)?
				}
				f.write_char('\n')
			} else {
				Ok(())
			}
		} else if let Some((first, elems)) = self.data.split_first() {
			write!(f, "{}", first)?;
			for tk in elems {
				f.write_str(&style(" ◦ ").dim().to_string())?;
				write!(f, "{}", tk)?
			}
			f.write_char('\n')
		} else {
			Ok(())
		}
	}
}

impl Precedence {
	pub fn next_level(&self) -> Self {
		match self {
			Self::P0Min => Self::P0Min,
			Self::P1Comma => Self::P2Assign,
			Self::P2Assign => Self::P2Assign, // right to left
			Self::P3Cond => Self::P3Cond,     // right to left
			Self::P4LOr => Self::P5LAnd,
			Self::P5LAnd => Self::P6BOr,
			Self::P6BOr => Self::P7BXor,
			Self::P7BXor => Self::P8BAnd,
			Self::P8BAnd => Self::P9Eq,
			Self::P9Eq => Self::P10Cmp,
			Self::P10Cmp => Self::P11BShift,
			Self::P11BShift => Self::P12Add,
			Self::P12Add => Self::P13Mul,
			Self::P13Mul => Self::P14Unary,
			Self::P14Unary => Self::P14Unary, // right to left
			Self::P15Post => Self::P15Post,
		}
	}
}

impl Token {
	pub fn to_identifier(self) -> Option<String> {
		match self {
			Token::Id(id) => Some(id),
			_ => None,
		}
	}

	pub fn precedence(&self) -> Precedence {
		match self {
			Token::Const(_) => Precedence::P0Min,
			Token::Id(_) => Precedence::P0Min,
			Token::StringLiteral(_) => Precedence::P0Min,

			Token::Keyword(Keyword::SizeOf) => Precedence::P14Unary,
			Token::Keyword(_) => Precedence::P0Min,

			Token::Punct(punct) => match punct {
				Punct::Comma => Precedence::P1Comma,

				// right to left
				Punct::Assign
				| Punct::AssignAdd
				| Punct::AssignSub
				| Punct::AssignMul
				| Punct::AssignDiv
				| Punct::AssignMod
				| Punct::AssignBAnd
				| Punct::AssignBOr
				| Punct::AssignBXor
				| Punct::AssignShl
				| Punct::AssignShr => Precedence::P2Assign,

				// right to left
				Punct::Cond => Precedence::P3Cond,

				Punct::Lor => Precedence::P4LOr,
				Punct::Lan => Precedence::P5LAnd,
				Punct::Or => Precedence::P6BOr,
				Punct::Xor => Precedence::P7BXor,
				Punct::And => Precedence::P8BAnd,

				Punct::Eq | Punct::Ne => Precedence::P9Eq,

				Punct::Lt | Punct::Le | Punct::Gt | Punct::Ge => Precedence::P10Cmp,

				Punct::Shl | Punct::Shr => Precedence::P11BShift,

				Punct::Add | Punct::Sub => Precedence::P12Add,

				Punct::Mul | Punct::Div | Punct::Mod => Precedence::P13Mul,

				// 一元运算符不会调用此函数获取优先级
				Punct::Not | Punct::Tilde => unreachable!("unary"),

				Punct::Inc | Punct::Dec | Punct::ParentheseL | Punct::BrakL | Punct::Dot | Punct::Arrow => {
					Precedence::P15Post
				}

				_ => Precedence::P0Min,
			},
		}
	}
}
