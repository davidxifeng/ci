use crate::compile::errors::*;
use itertools::Itertools;
use std::str::Chars;

use super::token::{Const, Keyword, Punct, Token};

#[derive(Debug)]
pub struct TokenApi {
	/// 当前行号
	line: isize,
	token_count: isize,
}

type LexResult = Option<Result<Token, LexError>>;

// TODO 简化类型,Token中增加一个特殊的EOF,表示结束
//type LexResult = Result<Token, LexError>;

impl TokenApi {
	/// 处理广义上的标识符, 应该包括关键字和enum 常量
	fn try_id(&mut self, iter: &mut Chars, c: char) -> LexResult {
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
	fn try_decimal(&mut self, iter: &mut Chars, c: char) -> LexResult {
		let mut str = String::from(c);
		while let Some(nc) = iter.peeking_take_while(is_digit).next() {
			str.push(nc);
		}
		Some(Ok(Token::Const(Const::Integer(str))))
	}

	fn escape(iter: &mut Chars) -> Result<char, LexError> {
		if let Some(c) = iter.next() {
			if let Some(r) = simple_escape_seq(c) {
				Ok(r)
			} else {
				// TODO 八进制 十六进制 转义
				Err(LexError::UnknownEscape(c))
			}
		} else {
			Err(LexError::UnexpectedEof)
		}
	}

	fn try_string_literal(&mut self, iter: &mut Chars) -> LexResult {
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
			Some(Err(err))
		} else {
			Some(Ok(Token::StringLiteral(val)))
		}
	}

	fn try_char(&mut self, iter: &mut Chars) -> LexResult {
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
			if cs.next().is_none() {
				Some(Ok(Token::Const(Const::Character(c))))
			} else {
				Some(Err(LexError::MoreThanOneChar))
			}
		} else {
			Some(Err(LexError::EmptyChar))
		}
	}

	fn skip_next(&mut self, iter: &mut Chars, c: char) -> Option<LexError> {
		if let Some(nnc) = iter.next() {
			if nnc == c {
				None
			} else {
				Some(LexError::ExpectingBut(c, nnc))
			}
		} else {
			Some(LexError::UnexpectedEof)
		}
	}
}

impl TokenApi {
	/// 标识符
	fn try_next_token(&mut self, iter: &mut Chars) -> LexResult {
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
				'#' => while iter.peeking_take_while(is_not_new_line).next().is_some() {},
				'/' => {
					if iter.peeking_take_while(|&x| x == '/').next().is_some() {
						// 跳过 // 注释
						while iter.peeking_take_while(is_not_new_line).next().is_some() {}
					} else {
						return Some(Ok(Token::Punct(Punct::Div)));
					}
				}
				'=' => {
					if iter.peeking_take_while(|&x| x == '=').next().is_some() {
						return Some(Ok(Token::Punct(Punct::Eq)));
					} else {
						return Some(Ok(Token::Punct(Punct::Assign)));
					}
				}
				'!' => {
					if iter.peeking_take_while(|&x| x == '=').next().is_some() {
						return Some(Ok(Token::Punct(Punct::Ne)));
					} else {
						return Some(Ok(Token::Punct(Punct::Not)));
					}
				}
				'+' => {
					if iter.peeking_take_while(|&x| x == '+').next().is_some() {
						return Some(Ok(Token::Punct(Punct::Inc)));
					} else {
						return Some(Ok(Token::Punct(Punct::Add)));
					}
				}
				'-' => {
					if iter.peeking_take_while(|&x| x == '-').next().is_some() {
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
				'[' => return Some(Ok(Token::Punct(Punct::BrakL))),
				'?' => return Some(Ok(Token::Punct(Punct::Cond))),
				';' => return Some(Ok(Token::Punct(Punct::Semicolon))),
				',' => return Some(Ok(Token::Punct(Punct::Comma))),
				'{' => return Some(Ok(Token::Punct(Punct::BracesL))),
				'}' => return Some(Ok(Token::Punct(Punct::BracesR))),
				']' => return Some(Ok(Token::Punct(Punct::BrakR))),
				'(' => return Some(Ok(Token::Punct(Punct::ParentheseL))),
				')' => return Some(Ok(Token::Punct(Punct::ParentheseR))),
				':' => return Some(Ok(Token::Punct(Punct::Colon))),
				'~' => return Some(Ok(Token::Punct(Punct::Tilde))),

				'"' => return self.try_string_literal(iter),
				'\'' => return self.try_char(iter),
				_ if is_id_initial_char(&c) => return self.try_id(iter, c),
				_ if is_digit(&c) => return self.try_decimal(iter, c),

				_ => return Some(Err(LexError::InvalidChar(c))),
			}
		}
		None
	}

	/// 对输入字符串进行词法解析,得到一组token list,或者错误信息
	/// TODO 修改接口,把迭代器放到结构体中
	pub fn parse_all(input: &str) -> Result<Vec<Token>, LexError> {
		let mut token_list = vec![];
		let mut lex_state = TokenApi { line: 1, token_count: 0 };
		let mut iter = input.chars();
		while let Some(result) = lex_state.try_next_token(&mut iter) {
			token_list.push(result?);
			lex_state.token_count += 1;
		}
		Ok(token_list)
	}
}

#[inline]
fn is_digit(c: &char) -> bool {
	('0'..='9').contains(c)
}

#[inline]
fn is_id_initial_char(c: &char) -> bool {
	('a'..='z').contains(c) || ('A'..='Z').contains(c) || *c == '_'
}

#[inline]
fn is_id_char(c: &char) -> bool {
	is_id_initial_char(c) || is_digit(c)
}

#[inline]
fn is_not_new_line(c: &char) -> bool {
	*c != '\r' && *c != '\n'
}

#[inline]
/// 完整的C语言中的转义
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
