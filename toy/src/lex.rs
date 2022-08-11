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

impl TokenApi {
	/// 当前结构果然思路上还有严重的问题,状态不足,不能识别出词法阶段的错误
	/// 词法识别阶段确实也可以检测出错误,比如 数字后面只能接空白 或者是运算符,不能是数字
	/// 识别关键字的时候,好像还需要回退: 比如识别的i后,如果后面不是f,必须当作其他关键字或普通
	/// 标识符
	/// c4中的做法: 提前准备好符号表,把关键字 还有库函数添加到符号表中,
	/// 然后next函数只识别标识符, 并不区分 关键字 还是库函数,或者普通变量
	/// 文档上看到说go语言解析可以不用符号表,不知道是什么意思.
	/// 只有25个关键字, 不知是不是和词法解析有关系. 关键字和预定义标识符等内在关系上面
	fn try_next_token(self: &mut Self, iter: &mut std::str::Chars) -> Option<Result<Token, LexError>> {
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

	fn try_decimal(self: &mut Self, iter: &mut std::str::Chars, c: char) -> Option<Result<Token, LexError>> {
		let mut str = String::from(c);
		while let Some(nc) = iter.peeking_take_while(is_digit).next() {
			str.push(nc);
		}
		return Some(Ok(Token::IntegerConst(str)));
	}

	fn try_string_literal(self: &mut Self, iter: &mut std::str::Chars) -> Option<Result<Token, LexError>> {
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

	fn skip_next(self: &mut Self, iter: &mut std::str::Chars, c: char) -> Option<LexError> {
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

	/// 对输入字符串进行词法解析,得到一组token list,或者错误信息
	pub fn parse(input: &str) -> Result<Vec<Token>, LexError> {
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_lex_1() {
		assert_eq!(TokenApi::parse("123"), Ok(vec![Token::IntegerConst("123".into())]));
		assert_eq!(
			TokenApi::parse("1 23"),
			Ok(vec![Token::IntegerConst("1".into()), Token::IntegerConst("23".into())])
		);
		// assert_eq!(
		//     TokenApi::parse("1x23"),
		//     vec![Token::Num(1), Token::Id("x23".to_string())]
		// );
	}

	#[test]
	fn run_lex_2() {
		assert_eq!(
			TokenApi::parse(r##""I am a C string""##),
			Ok(vec![Token::StringLiteral("I am a C string".to_string())])
		);
		assert_eq!(
			TokenApi::parse(r##""I am a C string\nline 2""##),
			Ok(vec![Token::StringLiteral("I am a C string\nline 2".to_string())])
		);
		assert_eq!(TokenApi::parse(r##""I am a C string"##), Err(LexError::UnexpectedEof));
		assert_eq!(TokenApi::parse(r##""I am a \C string"##), Err(LexError::UnknownEscape('C')));

		// assert_eq!(TokenApi::parse("ix"), vec![Token::Id("ix".to_string())]);
		// assert_eq!(
		//     TokenApi::parse("if ix"),
		//     vec![Token::Id("if".to_string()), Token::Id("ix".to_string())]
		// );
		// assert_eq!(
		//     TokenApi::parse("else 123"),
		//     vec![Token::Id("else".to_string()), Token::Num(123)]
		// );
		// assert_eq!(
		//     TokenApi::parse("if_123"),
		//     vec![Token::Id("if_123".to_string())]
		// );
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
