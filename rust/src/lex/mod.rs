mod tests;

use std::fmt::Write;

use crate::*;
use itertools::Itertools;

#[inline]
fn is_digit(c: &char) -> bool {
	*c >= '0' && *c <= '9'
}

#[inline]
fn is_id_initial_char(c: &char) -> bool {
	let c = *c;
	c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_'
}

#[inline]
fn is_id_char(c: &char) -> bool {
	is_id_initial_char(c) || is_digit(c)
}

#[inline]
fn is_not_new_line(c: &char) -> bool {
	*c != '\r' && *c != '\n'
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keyword {
	Char,
	Int,
	Enum,

	If,
	Else,
	While,
	Return,
}

#[derive(Debug, PartialEq)]
pub enum Punct {
	Assign,
	Cond,
	Lor,
	Lan,
	Or,
	Xor,
	And,
	Eq,
	Ne,
	Lt,
	Gt,
	Le,
	Ge,
	Shl,
	Shr,
	Add,
	Sub,
	Mul,
	Div,
	Mod,
	Inc,
	Dec,
	Brak,

	Not,
	Semicolon,
	Comma,
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
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
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
			Self::Brak => "[",
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

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Const {
	#[default]
	Empty,
	Integer(i128),
	Character(char),
}

impl std::fmt::Display for Const {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Empty => Ok(()),
			Self::Character(c) => {
				f.write_char('\'');
				f.write_char(*c);
				f.write_char('\'')
			}
			Self::Integer(i) => f.write_str(i.to_string().as_str()),
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum Token {
	Const(Const),
	StringLiteral(String),
	Keyword(Keyword),
	Id(String),
	Punct(Punct),
	Todo(char),
}

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

	pub fn is_enum_type(&self) -> bool {
		match self {
			Token::Keyword(kw) => *kw == Keyword::Enum,
			_ => false,
		}
	}
}
#[derive(Debug, PartialEq)]
pub enum LexError {
	InvalidChar(char),
	UnexpectedEof,
	EmptyChar,
	ConstOverflow,
	MoreThanOneChar,
	ExpectingBut(char, char),
	UnknownEscape(char),
}

#[derive(Debug)]
pub struct TokenApi {
	/// 当前行号
	line: isize,
	token_count: isize,
}

type LexResult = Option<Result<Token, LexError>>;

impl TokenApi {
	/// 处理广义上的标识符, 应该包括关键字和enum 常量
	fn try_id(&mut self, iter: &mut std::str::Chars, c: char) -> LexResult {
		let mut ids = String::from(c);
		while let Some(idc) = iter.peeking_take_while(is_id_char).next() {
			ids.push(idc);
		}
		Some(Ok(match ids.as_str() {
			"if" => Token::Keyword(Keyword::If),
			"else" => Token::Keyword(Keyword::Else),
			"char" => Token::Keyword(Keyword::Char),
			"int" => Token::Keyword(Keyword::Int),
			"enum" => Token::Keyword(Keyword::Enum),
			"return" => Token::Keyword(Keyword::Return),
			"while" => Token::Keyword(Keyword::While),
			_ => Token::Id(ids),
		}))
	}

	/// const处理, 应该包含int, float, char
	fn try_decimal(&mut self, iter: &mut std::str::Chars, c: char) -> LexResult {
		let mut str = String::from(c);
		while let Some(nc) = iter.peeking_take_while(is_digit).next() {
			str.push(nc);
		}
		if let Ok(n) = str.parse() {
			Some(Ok(Token::Const(Const::Integer(n))))
		} else {
			Some(Err(LexError::ConstOverflow))
		}
	}

	fn simple_escape_seq(c: char) -> Option<char> {
		// Rust中的转义:
		// https://doc.rust-lang.org/reference/tokens.html
		// (6.4.4.4) simple-escape-sequence:
		// one of \' \" \? \\ \a \b \f \n \r \t \v
		match c {
			'\'' => Some('\''),
			'"' => Some('"'),
			'?' => Some('\x3F'),
			'\\' => Some('\\'),
			'a' => Some('\x07'), // aleat, bell
			'b' => Some('\x08'), // backspace
			'f' => Some('\x0C'), // formfeed page break
			'n' => Some('\n'),   // 0a
			'r' => Some('\r'),   // 0d
			't' => Some('\t'),   // 09 horizontal Tab
			'v' => Some('\x0b'), // vertical tab
			_ => None,
		}
	}

	fn escape(iter: &mut std::str::Chars) -> Result<char, LexError> {
		if let Some(c) = iter.next() {
			if let Some(r) = Self::simple_escape_seq(c) {
				Ok(r)
			} else {
				// TODO 八进制 十六进制 转义
				Err(LexError::UnknownEscape(c))
			}
		} else {
			Err(LexError::UnexpectedEof)
		}
	}

	fn try_string_literal(&mut self, iter: &mut std::str::Chars) -> LexResult {
		// 找到匹配的 " 之前, 匹配任何内容,并放入字符串常量; 需要处理转义,和 输入提前结束的异常
		let mut val = String::new();
		while let Some(nc) = iter.peeking_take_while(|&c| c != '"' && is_not_new_line(&c)).next() {
			if nc == '\\' {
				match Self::escape(iter) {
					Ok(ec) => {
						val.push(ec);
					}
					Err(e) => {
						return Some(Err(e));
					}
				}
			} else {
				val.push(nc);
			}
		}
		if let Some(err) = self.skip_next(iter, '"') {
			return Some(Err(err));
		}
		return Some(Ok(Token::StringLiteral(val)));
	}

	fn try_char(&mut self, iter: &mut std::str::Chars) -> LexResult {
		let mut val = String::new();
		// C标准规定字符串字面量中不能有换行
		while let Some(nc) = iter.peeking_take_while(|&c| c != '\'' && is_not_new_line(&c)).next() {
			if nc == '\\' {
				match Self::escape(iter) {
					Ok(ec) => {
						val.push(ec);
					}
					Err(e) => {
						return Some(Err(e));
					}
				}
			} else {
				val.push(nc);
			}
		}
		if let Some(err) = self.skip_next(iter, '\'') {
			return Some(Err(err));
		}
		let mut cs = val.chars();
		if let Some(c) = cs.next() {
			if let None = cs.next() {
				return Some(Ok(Token::Const(Const::Character(c))));
			} else {
				return Some(Err(LexError::MoreThanOneChar));
			}
		} else {
			return Some(Err(LexError::EmptyChar));
		}
	}

	fn skip_next(&mut self, iter: &mut std::str::Chars, c: char) -> Option<LexError> {
		if let Some(nnc) = iter.next() {
			if nnc == c {
				return None;
			} else {
				return Some(LexError::ExpectingBut(c, nnc));
			}
		} else {
			return Some(LexError::UnexpectedEof);
		}
	}
}

impl TokenApi {
	/// 标识符
	/// c4中的做法: 提前准备好符号表,把关键字 还有库函数添加到符号表中,
	/// 然后next函数只识别标识符, 并不区分 关键字 还是库函数,或者普通变量
	/// 文档上看到说go语言解析可以不用符号表,不知道是什么意思.
	/// 只有25个关键字, 不知是不是和词法解析有关系. 关键字和预定义标识符等内在关系上面
	fn try_next_token(&mut self, iter: &mut std::str::Chars) -> LexResult {
		// 不可以使用for in, into iter 会move走迭代器,就不能手动控制了
		while let Some(c) = iter.next() {
			match c {
				' ' | '\t' => {} // skip 空白
				'\r' => {
					// 处理换行
					iter.peeking_take_while(|&x| x == '\n').next();
					self.line += 1;
				}
				'\n' => self.line += 1,
				// 跳过 # 和换行之间的内容,预处理.
				'#' => while let Some(_) = iter.peeking_take_while(is_not_new_line).next() {},
				'/' => {
					if let Some(_) = iter.peeking_take_while(|&x| x == '/').next() {
						// 跳过 // 注释
						while let Some(_) = iter.peeking_take_while(is_not_new_line).next() {}
					} else {
						return Some(Ok(Token::Punct(Punct::Div)));
					}
				}
				'=' => {
					if let Some(_) = iter.peeking_take_while(|&x| x == '=').next() {
						return Some(Ok(Token::Punct(Punct::Eq)));
					} else {
						return Some(Ok(Token::Punct(Punct::Assign)));
					}
				}
				'!' => {
					if let Some(_) = iter.peeking_take_while(|&x| x == '=').next() {
						return Some(Ok(Token::Punct(Punct::Ne)));
					} else {
						return Some(Ok(Token::Punct(Punct::Not)));
					}
				}
				'+' => {
					if let Some(_) = iter.peeking_take_while(|&x| x == '+').next() {
						return Some(Ok(Token::Punct(Punct::Inc)));
					} else {
						return Some(Ok(Token::Punct(Punct::Add)));
					}
				}
				'-' => {
					if let Some(_) = iter.peeking_take_while(|&x| x == '-').next() {
						return Some(Ok(Token::Punct(Punct::Dec)));
					} else {
						return Some(Ok(Token::Punct(Punct::Sub)));
					}
				}

				'<' => {
					if let Some(nc) = iter.clone().next() {
						if nc == '=' {
							iter.next();
							return Some(Ok(Token::Punct(Punct::Le)));
						} else if nc == '<' {
							iter.next();
							return Some(Ok(Token::Punct(Punct::Shl)));
						} else {
							return Some(Ok(Token::Punct(Punct::Lt)));
						}
					} else {
						return Some(Ok(Token::Punct(Punct::Lt)));
					}
				}
				'>' => {
					if let Some(nc) = iter.clone().next() {
						if nc == '=' {
							iter.next();
							return Some(Ok(Token::Punct(Punct::Ge)));
						} else if nc == '>' {
							iter.next();
							return Some(Ok(Token::Punct(Punct::Shr)));
						} else {
							return Some(Ok(Token::Punct(Punct::Gt)));
						}
					} else {
						return Some(Ok(Token::Punct(Punct::Gt)));
					}
				}
				'|' => {
					if let Some(nc) = iter.clone().next() {
						if nc == '|' {
							iter.next();
							return Some(Ok(Token::Punct(Punct::Lor)));
						} else {
							return Some(Ok(Token::Punct(Punct::Or)));
						}
					} else {
						return Some(Ok(Token::Punct(Punct::Or)));
					}
				}
				'&' => {
					if let Some(nc) = iter.clone().next() {
						if nc == '&' {
							iter.next();
							return Some(Ok(Token::Punct(Punct::Lan)));
						} else {
							return Some(Ok(Token::Punct(Punct::And)));
						}
					} else {
						return Some(Ok(Token::Punct(Punct::And)));
					}
				}
				'^' => return Some(Ok(Token::Punct(Punct::Xor))),
				'%' => return Some(Ok(Token::Punct(Punct::Mod))),
				'*' => return Some(Ok(Token::Punct(Punct::Mul))),
				'[' => return Some(Ok(Token::Punct(Punct::Brak))),
				'?' => return Some(Ok(Token::Punct(Punct::Cond))),
				';' => return Some(Ok(Token::Punct(Punct::Semicolon))),
				',' => return Some(Ok(Token::Punct(Punct::Comma))),

				'"' => return self.try_string_literal(iter),
				'\'' => return self.try_char(iter),
				_ if is_id_initial_char(&c) => return self.try_id(iter, c),
				_ if is_digit(&c) => return self.try_decimal(iter, c),
				// TODO punctuators
				'~' | '{' | '}' | '(' | ')' | ']' | ':' => return Some(Ok(Token::Todo(c))),

				_ => return Some(Err(LexError::InvalidChar(c))),
			}
		}
		return None;
	}

	/// 对输入字符串进行词法解析,得到一组token list,或者错误信息
	/// TODO 修改接口,把迭代器放到结构体中
	pub fn parse_all(input: &str) -> Result<Vec<Token>, LexError> {
		let mut token_list = vec![];
		let mut lex_state = TokenApi { line: 1, token_count: 0 };
		let mut iter = input.chars();
		while let Some(result) = lex_state.try_next_token(&mut iter) {
			match result {
				Ok(token) => {
					lex_state.token_count += 1;
					token_list.push(token);
				}
				Err(err) => return Err(err),
			}
		}
		Ok(token_list)
	}
}
