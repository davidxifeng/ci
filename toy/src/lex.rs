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

/*
#[derive(Debug, PartialEq)]
enum Keyword {
	Char,
	Int,
	Enum,
	If,
	Else,
	While,
	Return,
}

#[derive(Debug, PartialEq)]
enum Punct {
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
}

*/

#[derive(Debug, PartialEq)]
pub enum Token {
	IntegerConst(String),
	// CharacterConst(char),
	StringLiteral(String),
	// Keyword(Keyword),
	Id(String),
	// Punct(Punct),
}

#[derive(Debug, PartialEq)]
pub enum LexError {
	InvalidChar(char),
	UnexpectedEof,
	ExpectingCh(char, char),
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
	fn try_decimal(&mut self, iter: &mut std::str::Chars, c: char) -> LexResult {
		let mut str = String::from(c);
		while let Some(nc) = iter.peeking_take_while(is_digit).next() {
			str.push(nc);
		}
		return Some(Ok(Token::IntegerConst(str)));
	}

	fn try_string_literal(&mut self, iter: &mut std::str::Chars) -> LexResult {
		// 找到匹配的 " 之前, 匹配任何内容,并放入字符串常量; 需要处理转义,和 输入提前结束的异常
		let mut val = String::new();
		while let Some(nc) = iter.peeking_take_while(|c| *c != '"').next() {
			let mut v = nc;
			if nc == '\\' {
				if let Some(nnc) = iter.next() {
					match nnc {
						'\\' => (), // just skip
						'r' => {
							v = '\r';
						}
						'n' => {
							v = '\n';
						}
						't' => {
							v = '\t';
						}
						'"' => {
							v = '"';
						}
						uc => {
							return Some(Err(LexError::UnknownEscape(uc)));
						}
					}
				} else {
					return Some(Err(LexError::UnexpectedEof));
				}
			}
			val.push(v);
		}
		// better here?
		if let Some(err) = self.skip_next(iter, '"') {
			return Some(Err(err));
		}
		return Some(Ok(Token::StringLiteral(val)));
	}

	fn skip_next(&mut self, iter: &mut std::str::Chars, c: char) -> Option<LexError> {
		if let Some(nnc) = iter.next() {
			if nnc == c {
				return None;
			} else {
				return Some(LexError::ExpectingCh(c, nnc));
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
		loop {
			// 6.4 Lexical elements
			// token:
			//      keyword
			//      identifier
			//      constant: int, float, enum, char
			//      string-literal
			//      punctuator

			match iter.next() {
				None => {
					return None;
				}
				Some(c) => match c {
					// 处理换行和空白
					'\r' => {
						iter.peeking_take_while(|&x| x == '\n').next();
						self.line += 1;
					}
					'\n' => {
						self.line += 1;
					}
					// 跳过 # 和换行之间的内容,预处理.
					'#' => while let Some(_) = iter.peeking_take_while(is_not_new_line).next() {},
					// 跳过 // 注释
					'/' => {
						if let Some(_) = iter.peeking_take_while(|&x| x == '/').next() {
							while let Some(_) = iter.peeking_take_while(is_not_new_line).next() {}
						}
					}
					' ' | '\t' => {} // skip

					// 处理广义上的标识符, 应该包括关键字和enum 常量
					_ if is_id_initial_char(&c) => {
						let mut ids = String::from(c);
						while let Some(idc) = iter.peeking_take_while(is_id_char).next() {
							ids.push(idc);
						}
						return Some(Ok(Token::Id(ids)));
					}
					// const处理, 应该包含int, float, char
					_ if is_digit(&c) => {
						return self.try_decimal(iter, c);
					}
					// string literal
					'"' => {
						return self.try_string_literal(iter);
					}

					// punctuators
					// report error for unknown & unexpected input
					_ => return Some(Err(LexError::InvalidChar(c))),
				},
			};
		}
	}

	/// 对输入字符串进行词法解析,得到一组token list,或者错误信息
	pub fn parse_all(input: &str) -> Result<Vec<Token>, LexError> {
		println!("now parsing: {}", input);

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

		println!("state: {:#?}", lex_state);

		Ok(token_list)
	}
}

// 只在test的时候编译,build时候不编译
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_lex_1() {
		assert_eq!(TokenApi::parse_all("123"), Ok(vec![Token::IntegerConst("123".into())]));
		assert_eq!(
			TokenApi::parse_all("1 23"),
			Ok(vec![Token::IntegerConst("1".into()), Token::IntegerConst("23".into())])
		);
		assert_eq!(
			TokenApi::parse_all(r##""I am a C string""##),
			Ok(vec![Token::StringLiteral("I am a C string".to_string())])
		);
		assert_eq!(
			TokenApi::parse_all(r##""I am a C string\nline 2""##),
			Ok(vec![Token::StringLiteral("I am a C string\nline 2".to_string())])
		);
		assert_eq!(TokenApi::parse_all(r##""I am a C string"##), Err(LexError::UnexpectedEof));
		assert_eq!(TokenApi::parse_all(r##""I am a \C string"##), Err(LexError::UnknownEscape('C')));
		assert_eq!(
			TokenApi::parse_all(r##"123 fn "I am a C string""##),
			Ok(vec![
				Token::IntegerConst("123".into()),
				Token::Id("fn".into()),
				Token::StringLiteral("I am a C string".to_string())
			])
		);
	}

	#[test]
	fn run_lex_2() {
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
	fn test_put_back() {
		let mut c = itertools::put_back("hello".chars());
		c.put_back('X');
		c.put_back('Y'); // 会覆盖上一次,因为内部只有一个空间
		for v in c {
			println!("{}", v);
		}
		let mut pn = itertools::put_back_n("hello".chars());
		pn.put_back('Z');
		pn.put_back('Y');
		pn.put_back('X');
		for v in pn {
			println!("{}", v);
		}
	}
}
