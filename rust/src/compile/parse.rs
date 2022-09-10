use super::{
	errors::*,
	token::{Keyword, Precedence, Punct, Token, TokenList},
	types::*,
};

pub struct Parser {
	token_list: Vec<Token>,
	index: usize,
	globals: Option<Object>,
}

impl Parser {
	pub fn new(token_list: TokenList) -> Self {
		Parser { token_list: token_list.data, index: 0, globals: None }
	}

	pub fn from_str(input: &str) -> Result<Self, ParseError> {
		input.parse().map_err(ParseError::LexError).map(Self::new)
	}

	pub fn parse(&mut self) -> Result<Option<Expr>, ParseError> {
		self.parse_expr(Precedence::P1Comma)
	}

	pub fn declaration(&mut self) -> Result<Type, ParseError> {
		let base_type = self.declspec()?;
		let result = self.declarator(base_type)?;
		Ok(result)
	}

	pub fn test(input: &str) -> Result<Expr, ParseError> {
		match Self::from_str(input)?.parse()? {
			Some(expr) => Ok(expr),
			None => Err(ParseError::EndOfToken),
		}
	}

	fn declspec(&mut self) -> Result<Type, ParseError> {
		match self.next()? {
			Token::Keyword(Keyword::Void) => Ok(TYPE_VOID),
			Token::Keyword(Keyword::Bool) => Ok(TYPE_BOOL),
			Token::Keyword(Keyword::Char) => Ok(TYPE_CHAR),
			Token::Keyword(Keyword::Int) => Ok(TYPE_INT),
			_ => Err(ParseError::NotType),
		}
	}

	fn func_params(&mut self, base_type: Type) -> Result<Type, ParseError> {
		if let Some(Token::Keyword(Keyword::Void)) = self.peek_next() {
			if let Some(Token::Punct(Punct::ParentheseR)) = self.peek_next_n(1) {
				self.advance_by_n(2);
				return Ok(base_type.into_function());
			}
		}

		let mut params = vec![];
		while let Some(token) = self.peek_next() {
			if token == Token::Punct(Punct::ParentheseR) {
				self.advance();
				break;
			}
			let param_type = self.declspec()?;
			params.push(self.declarator(param_type)?);

			if let Some(Token::Punct(Punct::Comma)) = self.peek_next() {
				self.advance();
			}
		}

		Ok(base_type.into_function_with_param(params))
	}

	fn array_dimensions(&mut self, mut base_type: Type) -> Result<Type, ParseError> {
		let maybe_expr = self.parse_expr(Precedence::P2Assign)?;
		self.expect_punct(Punct::BrakR)?;
		base_type = self.type_suffix(base_type)?;
		Ok(base_type.into_array(maybe_expr))
	}

	fn type_suffix(&mut self, base_type: Type) -> Result<Type, ParseError> {
		if let Some(token) = self.peek_next() {
			match token {
				Token::Punct(Punct::ParentheseL) => {
					self.advance();
					self.func_params(base_type)
				}
				Token::Punct(Punct::BrakL) => {
					self.advance();
					self.array_dimensions(base_type)
				}
				_ => Ok(base_type),
			}
		} else {
			Ok(base_type)
		}
	}

	fn declarator(&mut self, mut base_type: Type) -> Result<Type, ParseError> {

		while let Some(Token::Punct(Punct::Mul)) = self.peek_next() {
			self.advance();
			base_type = base_type.into_pointer();
		}

		if let Some(token) = self.peek_next() {
			if token == Token::Punct(Punct::ParentheseL) {
				let pos1 = self.advance();

				self.skip_after_matching();

				base_type = self.type_suffix(base_type)?;
				let pos2 = self.index;

				self.seek_to(pos1);
				base_type = self.declarator(base_type)?;
				self.seek_to(pos2);

				Ok(base_type)
			} else {
				let name = token.into_identifier();
				if name.is_some() {
					self.advance();
				}

				base_type = self.type_suffix(base_type)?;
				Ok(base_type)
			}
		} else {
			Err(ParseError::EndOfToken)
		}
	}

	fn skip_after_matching(&mut self) {
		let mut level = 1;
		while let Some(token) = self.peek_next() {
			self.advance();
			match token {
				Token::Punct(Punct::ParentheseL) => {
					level += 1;
				}
				Token::Punct(Punct::ParentheseR) => {
					level -= 1;
					if level == 0 {
						break;
					}
				}
				_ => {}
			}
		}
	}

	#[inline]
	fn peek_next_n(&self, n: usize) -> Option<Token> {
		self.token_list.get(self.index + n).cloned()
	}

	#[inline]
	fn advance_by_n(&mut self, n: usize) {
		self.index += n;
	}

	#[inline]
	fn seek_to(&mut self, n: usize) {
		self.index = n;
	}

	#[inline]
	fn peek_next(&self) -> Option<Token> {
		self.token_list.get(self.index).cloned()
	}

	#[inline]
	fn advance(&mut self) -> usize {
		self.index += 1;
		self.index
	}

	#[inline]
	fn next(&mut self) -> Result<Token, ParseError> {
		let r = self.token_list.get(self.index);
		self.index += 1;
		r.map_or(Err(ParseError::EndOfToken), |x| Ok(x.clone()))
	}

	fn expect_punct(&mut self, punct: Punct) -> Result<(), ParseError> {
		match self.next()? {
			Token::Punct(p) if p == punct => Ok(()),
			other => Err(ParseError::Unexpected(format!("expecting {}, but {}", punct, other))),
		}
	}

	fn expect_identifier(&mut self) -> Result<String, ParseError> {
		match self.next()? {
			Token::Id(id) => Ok(id),
			_ => Err(ParseError::NotIdentifier),
		}
	}

	fn expect_expr(&mut self, precedence: Precedence) -> Result<Expr, ParseError> {
		match self.parse_expr(precedence)? {
			Some(expr) => Ok(expr),
			None => Err(ParseError::NoMoreExpr),
		}
	}

	fn parse_leaf(&mut self) -> Result<Option<Expr>, ParseError> {
		if let Some(tk) = self.peek_next() {
			match tk {
				Token::Const(c) => {
					self.advance();
					Ok(Some(Expr::Const(c)))
				}
				Token::StringLiteral(str) => {
					self.advance();
					Ok(Some(Expr::StringLiteral(str)))
				}
				Token::Id(id) => {
					self.advance();
					Ok(Some(Expr::Id(id)))
				}
				Token::Punct(punct) => match punct {
					Punct::ParentheseL => {
						self.advance();
						match self.parse_expr(Precedence::P1Comma)? {
							Some(expr) => {
								self.expect_punct(Punct::ParentheseR)?;
								Ok(Some(expr))
							}
							None => Err(ParseError::General("(expr) failed")),
						}
					}
					Punct::Inc
					| Punct::Dec
					| Punct::Add
					| Punct::Sub
					| Punct::Not
					| Punct::Tilde
					| Punct::And
					| Punct::Mul => {
						self.advance();
						let expr = self.expect_expr(Precedence::P14Unary)?;
						Ok(Some(Expr::new_unary(punct, expr)))
					}
					_ => Ok(None),
				},
				Token::Keyword(keyword) => match keyword {
					Keyword::SizeOf => {
						self.advance();
						let expr = self.expect_expr(Precedence::P14Unary)?;
						// sizeof -> ?
						Ok(Some(Expr::new_unary(Punct::Cond, expr)))
					}
					_ => Ok(None),
				},
			}
		} else {
			Ok(None)
		}
	}

	fn parse_expr(&mut self, precedence: Precedence) -> Result<Option<Expr>, ParseError> {
		let mut first = match self.parse_leaf()? {
			Some(leaf) => leaf,
			None => return Ok(None),
		};

		while let Some(ntk) = self.peek_next() {
			let ntk_precedence = ntk.precedence();
			if ntk_precedence >= precedence {
				self.advance();
				match ntk {
					Token::Punct(p) => match p {
						Punct::Inc | Punct::Dec => first = Expr::new_postfix(p, first),
						Punct::Dot => {
							let id = self.expect_identifier()?;
							first = Expr::new_member_access(first, id)
						}
						Punct::Arrow => {
							let id = self.expect_identifier()?;
							first = Expr::new_member_access_p(first, id)
						}
						Punct::BrakL => match self.parse_expr(Precedence::P1Comma)? {
							Some(second) => {
								self.expect_punct(Punct::BrakR)?;
								first = Expr::new_binary(first, p, second)
							}
							None => return Err(ParseError::NoMoreExpr),
						},
						Punct::ParentheseL => {
							let mut argument_expr_list = vec![];
							while let Some(tk) = self.peek_next() {
								if tk == Token::Punct(Punct::ParentheseR) {
									self.advance();
									break;
								} else if tk == Token::Punct(Punct::Comma) {
									self.advance();
								}
								argument_expr_list.push(self.expect_expr(Precedence::P2Assign)?);
							}
							first = Expr::new_func_call(first, argument_expr_list)
						}
						_ if p.is_binary_op() => match self.parse_expr(ntk_precedence.next_level())? {
							Some(second) => first = Expr::new_binary(first, p, second),
							None => return Err(ParseError::NoMoreExpr),
						},
						_ if p.is_assign() => match self.parse_expr(ntk_precedence.next_level())? {
							Some(second) => first = Expr::new_assign(first, p, second),
							None => return Err(ParseError::NoMoreExpr),
						},
						Punct::Comma => match self.parse_expr(ntk_precedence.next_level())? {
							Some(second) => first = Expr::new_comma(first, second),
							None => return Err(ParseError::NoMoreExpr),
						},
						Punct::Cond => match self.parse_expr(Precedence::P1Comma)? {
							Some(left) => {
								self.expect_punct(Punct::Colon)?;
								match self.parse_expr(ntk_precedence.next_level())? {
									Some(right) => {
										first = Expr::new_cond(first, left, right);
									}
									None => return Err(ParseError::General("cond: missing false expr")),
								}
							}
							None => return Err(ParseError::General("cond: missing true expr")),
						},
						_ => unimplemented!(),
					},
					_ => unimplemented!(),
				}
			} else {
				break;
			}
		}

		Ok(Some(first))
	}
}
