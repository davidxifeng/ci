use super::{
	errors::*,
	token::{Keyword, Punct, Token, TokenList},
	types::*,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
	P0Min,
	P1Comma,
	P2Assign,
	P3Cond,
	P4LOr,
	P5LAnd,
	P6BOr,
	P7BXor,
	P8BAnd,
	P9Eq,
	P10Cmp,
	P11BShift,
	P12Add,
	P13Mul,
	P14Unary,
	P15Post,
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

fn token_info(token: &Token) -> Precedence {
	match token {
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

pub struct Parser {
	token_list: Vec<Token>,
	index: usize,
}

impl Parser {
	pub fn new(token_list: TokenList) -> Self {
		Parser { token_list: token_list.data, index: 0 }
	}

	pub fn from_str(input: &str) -> Result<Self, ParseError> {
		input.parse().map_err(ParseError::LexError).map(Self::new)
	}

	pub fn parse(&mut self) -> Result<Option<Expr>, ParseError> {
		self.parse_expr(Precedence::P1Comma)
	}

	pub fn test(input: &str) -> Result<Expr, ParseError> {
		match Self::from_str(input)?.parse()? {
			Some(expr) => Ok(expr),
			None => Err(ParseError::EndOfToken),
		}
	}

	#[inline]
	fn peek(&self) -> Option<Token> {
		self.token_list.get(self.index).cloned()
	}

	#[inline]
	fn next(&mut self) -> Result<Token, ParseError> {
		let r = self.token_list.get(self.index);
		self.index += 1;
		r.map_or(Err(ParseError::EndOfToken), |x| Ok(x.clone()))
	}

	#[inline]
	fn advance(&mut self) {
		self.index += 1;
	}

	fn expect_punct(&mut self, punct: Punct) -> Result<(), ParseError> {
		match self.next()? {
			Token::Punct(p) if p == punct => Ok(()),
			_ => Err(ParseError::Unexpected(format!("expecting {}", punct))),
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
		if let Ok(tk) = self.next() {
			match tk {
				Token::Const(c) => Ok(Some(Expr::Const(c))),
				Token::StringLiteral(str) => Ok(Some(Expr::StringLiteral(str))),
				Token::Id(id) => Ok(Some(Expr::Id(id))),
				Token::Punct(punct) => match punct {
					Punct::ParentheseL => match self.parse_expr(Precedence::P1Comma)? {
						Some(expr) => {
							self.expect_punct(Punct::ParentheseR)?;
							Ok(Some(expr))
						}
						None => Err(ParseError::General("(expr) failed")),
					},
					Punct::Inc
					| Punct::Dec
					| Punct::Add
					| Punct::Sub
					| Punct::Not
					| Punct::Tilde
					| Punct::And
					| Punct::Mul => {
						let expr = self.expect_expr(Precedence::P14Unary)?;
						Ok(Some(Expr::new_unary(punct, expr)))
					}
					_ => Ok(None),
				},
				Token::Keyword(keyword) => match keyword {
					Keyword::SizeOf => {
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

		while let Some(ntk) = self.peek() {
			let ntk_precedence = token_info(&ntk);
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
							while let Some(tk) = self.peek() {
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
