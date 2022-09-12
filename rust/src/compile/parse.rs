use std::collections::HashMap;

use console::style;

use super::{
	errors::*,
	token::{Keyword, Precedence, Punct, Token, TokenList},
	types::*,
};

pub struct Parser {
	token_list: TokenList,
	index: usize,
	globals: HashMap<String, Object>,
}

impl Parser {
	fn skip_after_matching(&mut self) -> Result<(), ParseError> {
		let mut level = 1;
		while let Ok(token) = self.next() {
			match token {
				Token::Punct(Punct::ParentheseL) => level += 1,
				Token::Punct(Punct::ParentheseR) => {
					level -= 1;
					if level == 0 {
						return Ok(());
					}
				}
				_ => (),
			}
		}
		Err(ParseError::NoMatchFound)
	}

	#[inline]
	fn peek_next_n(&self, n: usize) -> Option<Token> {
		self.token_list.data.get(self.index + n).cloned()
	}

	#[inline]
	fn advance_by_n(&mut self, n: usize) {
		self.index += n;
		assert!(self.index <= self.token_list.data.len());
	}

	// #[inline] fn back_n(&mut self, n: usize) { self.index -= n; }

	// pub fn reset(&mut self) {
	// 	self.index = 0;
	// }

	#[inline]
	fn seek_to(&mut self, n: usize) {
		assert!(n <= self.token_list.data.len());
		self.index = n;
	}

	#[inline]
	fn peek_next(&self) -> Option<Token> {
		self.token_list.data.get(self.index).cloned()
	}

	#[inline]
	fn must_peek_next(&self) -> Result<Token, ParseError> {
		self.token_list.data.get(self.index).map_or(Err(ParseError::EndOfToken), |x| Ok(x.clone()))
	}

	#[inline]
	fn advance(&mut self) -> usize {
		self.index += 1;
		assert!(self.index <= self.token_list.data.len());
		self.index
	}

	#[inline]
	fn next(&mut self) -> Result<Token, ParseError> {
		self.token_list.data.get(self.index).map_or(Err(ParseError::EndOfToken), |x| {
			self.index += 1;
			Ok(x.clone())
		})
	}

	fn peek_next_punct(&self, punct: Punct) -> bool {
		if let Some(Token::Punct(p)) = self.peek_next() {
			p == punct
		} else {
			false
		}
	}

	fn next_punct(&mut self) -> Result<Punct, ParseError> {
		match self.next()? {
			Token::Punct(p) => Ok(p),
			_ => Err(ParseError::General("expecting punct")),
		}
	}

	fn expect_punct(&mut self, punct: Punct) -> Result<(), ParseError> {
		match self.next()? {
			Token::Punct(p) if p == punct => Ok(()),
			other => Err(ParseError::Unexpected(format!("expecting {}, but {}", punct, other))),
		}
	}

	fn is_not_eof(&self) -> bool {
		self.index < self.token_list.data.len()
	}

	fn expect_identifier(&mut self) -> Result<String, ParseError> {
		match self.next()? {
			Token::Id(id) => Ok(id),
			_ => Err(ParseError::NotIdentifier),
		}
	}
}

fn expect_string(str: Option<String>) -> Result<String, ParseError> {
	str.map_or(Err(ParseError::General("identifier should not be empty")), Ok)
}

impl Parser {
	fn new(token_list: TokenList) -> Self {
		Parser { token_list, index: 0, globals: HashMap::new() }
	}

	pub fn from_str(input: &str) -> Result<Self, ParseError> {
		input.parse().map_err(ParseError::LexError).map(Self::new)
	}

	// translation-unit: external-declaration *
	// ---
	// external-declaration:
	// 	function-definition
	// 		declaration-specifiers declarator compound-statement
	// 		// 去掉对早期C参数声明的支持
	// 	declaration
	// 		declaration-specifiers declarator [= initializer], ;
	// 		declaration-specifiers init-declarator-list opt ;
	// 			init-declarator-list: init-declarator,
	// 			init-declarator: declarator = initializer
	pub fn parse(&mut self) -> Result<Vec<Object>, ParseError> {
		self.globals.clear();
		while self.is_not_eof() {
			let base_type = self.declspec()?;

			// 不支持 可选的 declarator, 也就是说 `int ;`会报错.

			let declarator = self.declarator(base_type.clone())?;

			let is_compound_stmt_start = self.peek_next_punct(Punct::BracesL);

			let maybe_func = declarator.ctype.get_func();
			match maybe_func {
				Some(func) if is_compound_stmt_start => {
					let name = expect_string(declarator.name)?;
					let func = self.parse_function(name.clone(), func)?;
					self.globals.insert(name, func);
				}
				_ => {
					self.new_global_declaration(declarator)?;
					loop {
						let punct = self.next_punct()?;
						if punct == Punct::Semicolon {
							break;
						} else if punct == Punct::Comma {
							let var = self.declarator(base_type.clone())?;
							self.new_global_declaration(var)?;
						} else {
							return Err(ParseError::General("unexpected token"));
						}
					}
				}
			}
		}

		Ok(self.globals.values().cloned().collect())
	}

	fn parse_function(&mut self, name: String, return_type: Func) -> Result<Object, ParseError> {
		let stmts = self.parse_stmt()?;

		Ok(Object::Function(Function {
			name,
			ctype: return_type,
			locals: vec![],
			stmts,
			stack_size: 0,
			is_definition: true,
		}))
	}

	fn parse_stmt(&mut self) -> Result<Statement, ParseError> {
		Ok(match self.must_peek_next()? {
			Token::Punct(Punct::BracesL) => {
				self.advance();
				let mut stmts = vec![];
				while Token::Punct(Punct::BracesR) != self.must_peek_next()? {
					let stmt = self.parse_stmt()?;
					stmts.push(stmt);
				}
				self.advance();
				Statement::CompoundStmt(stmts)
			}
			Token::Keyword(Keyword::Return) => {
				self.advance(); // skip return

				let expr = self.expect_expr(Precedence::P1Comma)?;

				self.expect_punct(Punct::Semicolon)?;

				Statement::ReturnStmt(expr)
			}
			Token::Punct(Punct::Semicolon) => {
				self.advance();
				Statement::Empty
			}
			Token::Keyword(Keyword::If) => {
				self.advance();
				self.expect_punct(Punct::ParentheseL)?;
				let cond = self.expect_expr(Precedence::P1Comma)?;
				self.expect_punct(Punct::ParentheseR)?;
				let then_stmt = self.parse_stmt()?;
				let m_else_stmt = if let Some(next) = self.peek_next() {
					match next {
						Token::Keyword(Keyword::Else) => {
							self.advance();
							Some(Box::new(self.parse_stmt()?))
						}
						_ => None,
					}
				} else {
					None
				};
				Statement::IfStmt(cond, Box::new(then_stmt), m_else_stmt)
			}
			_ => {
				let expr = self.expect_expr(Precedence::P1Comma)?;
				self.expect_punct(Punct::Semicolon)?;
				Statement::ExprStmt(expr)
			}
		})
	}

	pub fn declaration(&mut self) -> Result<TypeIdentifier, ParseError> {
		let base_type = self.declspec()?;
		self.declarator(base_type)
	}

	fn get_optional_initializer(&mut self) -> Result<Option<Expr>, ParseError> {
		Ok(if self.peek_next_punct(Punct::Assign) {
			self.advance();
			Some(self.expect_expr(Precedence::P2Assign)?)
		} else {
			None
		})
	}

	fn new_global_declaration(&mut self, var: TypeIdentifier) -> Result<(), ParseError> {
		let name = expect_string(var.name)?;
		let gvar = Object::Variable(Variable {
			ctype: var.ctype,
			name: name.clone(),
			init_value: self.get_optional_initializer()?,
			is_local: false,
			is_tentative: false,
		});
		self.globals.insert(name, gvar);
		Ok(())
	}

	pub fn show_token_list(&self) {
		self.show_parse_state(0);
	}

	pub fn show_parse_state(&self, context: usize) {
		let (mut before, mut after) = self.token_list.data.split_at(self.index);
		if context != 0 {
			if before.len() > context {
				before = &before[before.len() - context..];
			}
			if context <= after.len() {
				after = &after[..context];
			}
		}

		if let Some((first, elems)) = before.split_first() {
			print!("{}", first);
			for tk in elems {
				print!("{}", style(" ◦ ").dim());
				print!("{}", tk);
			}
		}
		print!("{}", style(" ▵ ").red());
		if let Some((first, elems)) = after.split_first() {
			print!("{}", first);
			for tk in elems {
				print!("{}", style(" ◦ ").dim());
				print!("{}", tk);
			}
		}
		println!();
	}

	// fn enter_scope(&mut self) {} fn leave_scope(&mut self) {}

	fn declspec(&mut self) -> Result<Type, ParseError> {
		while let Some(token) = self.peek_next() {
			if token.is_type_keyword() {
				self.advance();
				return match token {
					Token::Keyword(Keyword::Void) => Ok(TYPE_VOID),
					Token::Keyword(Keyword::Bool) => Ok(TYPE_BOOL),
					Token::Keyword(Keyword::Char) => Ok(TYPE_CHAR),
					Token::Keyword(Keyword::Int) => Ok(TYPE_INT),
					_ => Err(ParseError::NotType),
				};
			}
		}
		Err(ParseError::NotType)
	}

	fn func_params(&mut self, base_type: Type) -> Result<Type, ParseError> {
		if let Some(Token::Keyword(Keyword::Void)) = self.peek_next() {
			if let Some(Token::Punct(Punct::ParentheseR)) = self.peek_next_n(1) {
				self.advance_by_n(2);
				return Ok(base_type.into_function());
			}
		}

		let mut params = vec![];
		loop {
			let token = self.must_peek_next()?;
			if token == Token::Punct(Punct::ParentheseR) {
				self.advance();
				break;
			} else if token == Token::Punct(Punct::Comma) {
				self.advance();
			}

			let param_type = self.declspec()?;
			params.push(self.declarator(param_type)?);
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

	fn declarator(&mut self, mut base_type: Type) -> Result<TypeIdentifier, ParseError> {
		while let Some(Token::Punct(Punct::Mul)) = self.peek_next() {
			self.advance();
			base_type = base_type.into_pointer();
		}

		if let Some(token) = self.peek_next() {
			if token == Token::Punct(Punct::ParentheseL) {
				let pos1 = self.advance();

				self.skip_after_matching()?;

				base_type = self.type_suffix(base_type)?;
				let pos2 = self.index;

				self.seek_to(pos1);
				let base_type = self.declarator(base_type)?;
				self.seek_to(pos2);

				Ok(base_type)
			} else {
				let name = token.into_identifier();
				if name.is_some() {
					self.advance();
				}

				base_type = self.type_suffix(base_type)?;

				Ok(TypeIdentifier::new(base_type, name))
			}
		} else {
			Err(ParseError::EndOfToken)
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

	pub fn parse_expr(&mut self, precedence: Precedence) -> Result<Option<Expr>, ParseError> {
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
						_ => unreachable!(),
					},
					_ => unreachable!(),
				}
			} else {
				break;
			}
		}

		Ok(Some(first))
	}
}
