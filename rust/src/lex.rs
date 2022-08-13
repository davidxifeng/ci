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

#[derive(Debug, PartialEq)]
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

// 6.4 Lexical elements
// token:
//      keyword
//      identifier
//      constant: int, float, enum, char
//      string-literal
//      punctuator

#[derive(Debug, PartialEq)]
pub enum Token {
	IntegerConst(String),
	CharacterConst(char),
	StringLiteral(String),
	Keyword(Keyword),
	Id(String),
	Punct(Punct),
	Todo(char),
}

#[derive(Debug, PartialEq)]
pub enum LexError {
	InvalidChar(char),
	UnexpectedEof,
	EmptyChar,
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
		return Some(Ok(Token::IntegerConst(str)));
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
				return Some(Ok(Token::CharacterConst(c)));
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
					if let Some(nc) = iter.peekable().peek() {
						if *nc == '=' {
							iter.next();
							return Some(Ok(Token::Punct(Punct::Le)));
						} else if *nc == '<' {
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
					if let Some(nc) = iter.peekable().peek() {
						if *nc == '=' {
							iter.next();
							return Some(Ok(Token::Punct(Punct::Ge)));
						} else if *nc == '>' {
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
					if let Some(nc) = iter.peekable().peek() {
						if *nc == '|' {
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
					if let Some(nc) = iter.peekable().peek() {
						if *nc == '&' {
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

// 只在test的时候编译,build时候不编译
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_lex_keyword() {
		assert_eq!(TokenApi::parse_all("char"), Ok(vec![Token::Keyword(Keyword::Char)]));
		assert_eq!(TokenApi::parse_all("int"), Ok(vec![Token::Keyword(Keyword::Int)]));
		assert_eq!(TokenApi::parse_all("enum"), Ok(vec![Token::Keyword(Keyword::Enum)]));
		assert_eq!(TokenApi::parse_all("if"), Ok(vec![Token::Keyword(Keyword::If)]));
		assert_eq!(TokenApi::parse_all("else"), Ok(vec![Token::Keyword(Keyword::Else)]));
		assert_eq!(TokenApi::parse_all("while"), Ok(vec![Token::Keyword(Keyword::While)]));
		assert_eq!(TokenApi::parse_all("return"), Ok(vec![Token::Keyword(Keyword::Return)]));
	}

	#[test]
	fn test_identifier() {
		assert_eq!(TokenApi::parse_all("fn"), Ok(vec![Token::Id("fn".into())]));
	}

	#[test]
	fn test_const() {
		assert_eq!(TokenApi::parse_all("123"), Ok(vec![Token::IntegerConst("123".into())]));
		assert_eq!(
			TokenApi::parse_all("1 23"),
			Ok(vec![Token::IntegerConst("1".into()), Token::IntegerConst("23".into())])
		);
	}
	#[test]
	fn test_string_char() {
		assert_eq!(
			TokenApi::parse_all(r##""I am a C string""##),
			Ok(vec![Token::StringLiteral("I am a C string".into())])
		);
		assert_eq!(
			TokenApi::parse_all(r##""I am a C string\nline 2""##),
			Ok(vec![Token::StringLiteral("I am a C string\nline 2".into())])
		);
		assert_eq!(TokenApi::parse_all(r##""I am a C string"##), Err(LexError::UnexpectedEof));
		assert_eq!(TokenApi::parse_all(r##""I am a \C string"##), Err(LexError::UnknownEscape('C')));
		assert_eq!(
			TokenApi::parse_all(r##"123 fn "I am a C string""##),
			Ok(vec![
				Token::IntegerConst("123".into()),
				Token::Id("fn".into()),
				Token::StringLiteral("I am a C string".into())
			])
		);

		assert_eq!(TokenApi::parse_all("\"abc\n\""), Err(LexError::ExpectingBut('\"', '\n')));
		assert_eq!(TokenApi::parse_all("\'abc\'"), Err(LexError::MoreThanOneChar));
		assert_eq!(TokenApi::parse_all("\'\'"), Err(LexError::EmptyChar));

		assert_eq!(TokenApi::parse_all("\'a\'"), Ok(vec![Token::CharacterConst('a')]));
		assert_eq!(TokenApi::parse_all("\'\\n\'"), Ok(vec![Token::CharacterConst('\n')]));
	}

	#[test]
	fn test_comment_preprocessor() {
		assert_eq!(
			TokenApi::parse_all(
				r##"#include <stdio.h>
		x 123
		#if 1
		#endif
		c
		"##
			),
			Ok(vec![Token::Id("x".into()), Token::IntegerConst("123".into()), Token::Id("c".into()),])
		);
		assert_eq!(TokenApi::parse_all(r##"#include <stdio.h>"##), Ok(vec![]));
		assert_eq!(TokenApi::parse_all(r##"1#include <stdio.h>"##), Ok(vec![Token::IntegerConst("1".into())]));
		assert_eq!(TokenApi::parse_all(r##"// hi"##), Ok(vec![]));
		assert_eq!(TokenApi::parse_all(r##"1// hi"##), Ok(vec![Token::IntegerConst("1".into())]));
		assert_eq!(
			TokenApi::parse_all(
				r##"1// hi
		2
		"##
			),
			Ok(vec![Token::IntegerConst("1".into()), Token::IntegerConst("2".into())])
		);
	}

	#[test]
	fn test_punct() {
		assert_eq!(
			TokenApi::parse_all(r##"1/2"##),
			Ok(vec![Token::IntegerConst("1".into()), Token::Punct(Punct::Div), Token::IntegerConst("2".into())])
		);
		assert_eq!(TokenApi::parse_all(r##"1//2"##), Ok(vec![Token::IntegerConst("1".into())]));
		assert_eq!(TokenApi::parse_all("="), Ok(vec![Token::Punct(Punct::Assign)]));
		assert_eq!(TokenApi::parse_all("=="), Ok(vec![Token::Punct(Punct::Eq)]));
		assert_eq!(TokenApi::parse_all("==="), Ok(vec![Token::Punct(Punct::Eq), Token::Punct(Punct::Assign)]));
		assert_eq!(TokenApi::parse_all("===="), Ok(vec![Token::Punct(Punct::Eq), Token::Punct(Punct::Eq)]));

		assert_eq!(TokenApi::parse_all("+"), Ok(vec![Token::Punct(Punct::Add)]));
		assert_eq!(TokenApi::parse_all("++"), Ok(vec![Token::Punct(Punct::Inc)]));
		assert_eq!(TokenApi::parse_all("+++"), Ok(vec![Token::Punct(Punct::Inc), Token::Punct(Punct::Add)]));
		assert_eq!(TokenApi::parse_all("++++"), Ok(vec![Token::Punct(Punct::Inc), Token::Punct(Punct::Inc)]));

		assert_eq!(TokenApi::parse_all("-"), Ok(vec![Token::Punct(Punct::Sub)]));
		assert_eq!(TokenApi::parse_all("--"), Ok(vec![Token::Punct(Punct::Dec)]));

		assert_eq!(TokenApi::parse_all("!"), Ok(vec![Token::Punct(Punct::Not)]));
		assert_eq!(TokenApi::parse_all("!="), Ok(vec![Token::Punct(Punct::Ne)]));

		assert_eq!(TokenApi::parse_all("<"), Ok(vec![Token::Punct(Punct::Lt)]));
		assert_eq!(TokenApi::parse_all("< "), Ok(vec![Token::Punct(Punct::Lt)]));
		assert_eq!(TokenApi::parse_all("<="), Ok(vec![Token::Punct(Punct::Le)]));
		assert_eq!(TokenApi::parse_all("<<"), Ok(vec![Token::Punct(Punct::Shl)]));

		assert_eq!(TokenApi::parse_all(">"), Ok(vec![Token::Punct(Punct::Gt)]));
		assert_eq!(TokenApi::parse_all("> "), Ok(vec![Token::Punct(Punct::Gt)]));
		assert_eq!(TokenApi::parse_all(">="), Ok(vec![Token::Punct(Punct::Ge)]));
		assert_eq!(TokenApi::parse_all(">>"), Ok(vec![Token::Punct(Punct::Shr)]));

		assert_eq!(TokenApi::parse_all("|"), Ok(vec![Token::Punct(Punct::Or)]));
		assert_eq!(TokenApi::parse_all("||"), Ok(vec![Token::Punct(Punct::Lor)]));
		assert_eq!(TokenApi::parse_all("&"), Ok(vec![Token::Punct(Punct::And)]));
		assert_eq!(TokenApi::parse_all("&&"), Ok(vec![Token::Punct(Punct::Lan)]));
		assert_eq!(TokenApi::parse_all("^"), Ok(vec![Token::Punct(Punct::Xor)]));

		assert_eq!(TokenApi::parse_all("%"), Ok(vec![Token::Punct(Punct::Mod)]));
		assert_eq!(TokenApi::parse_all("*"), Ok(vec![Token::Punct(Punct::Mul)]));
		assert_eq!(TokenApi::parse_all("["), Ok(vec![Token::Punct(Punct::Brak)]));
		assert_eq!(TokenApi::parse_all("?"), Ok(vec![Token::Punct(Punct::Cond)]));
		assert_eq!(TokenApi::parse_all(";"), Ok(vec![Token::Punct(Punct::Semicolon)]));
	}
}
