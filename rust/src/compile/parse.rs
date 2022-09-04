use itertools::Itertools;

use super::{
	errors::*,
	token::{Const, Keyword, Punct, Token, TokenList},
	types::*,
};

fn take_next_token<'a>(iter: &mut impl Iterator<Item = &'a Token>) -> Result<Token, ParseError> {
	if let Some(tk) = iter.next() {
		Ok(tk.clone())
	} else {
		Err(ParseError::EndOfToken)
	}
}

fn expect_identifier<'a>(iter: &mut impl Iterator<Item = &'a Token>) -> Result<String, ParseError> {
	if let Token::Id(id) = take_next_token(iter)? {
		Ok(id)
	} else {
		Err(ParseError::NotIdentifier)
	}
}

fn expect_punct_list<'a>(iter: &mut impl Iterator<Item = &'a Token>, l: &[Punct]) -> Result<Punct, ParseError> {
	if let Token::Punct(punct) = take_next_token(iter)? {
		if l.contains(&punct) {
			Ok(punct)
		} else {
			Err(ParseError::expecting_but(
				&mut l.iter().map(|&x| x.to_string()).collect_vec(),
				punct.to_string().as_str(),
			))
		}
	} else {
		Err(ParseError::NotPunct)
	}
}

fn expect_const<'a>(iter: &mut impl Iterator<Item = &'a Token>) -> Result<Const, ParseError> {
	match take_next_token(iter)? {
		Token::Const(c) => Ok(c),
		Token::StringLiteral(_) => Err(ParseError::TypeMismatch),
		_ => Err(ParseError::Unexpected("not const".into())),
	}
}

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

			Punct::Inc | Punct::Dec | Punct::BrakL | Punct::Dot | Punct::Arrow => Precedence::P15Post,

			_ => Precedence::P0Min,
		},
	}
}

pub struct Parser {
	token_list: Vec<Token>,
	index: usize,
}

impl Parser {
	fn new(token_list: TokenList) -> Self {
		Parser { token_list: token_list.data, index: 0 }
	}

	fn parse_expr_list(&mut self) -> Result<Vec<Expr>, ParseError> {
		let mut expr_list = vec![];
		while let Some(expr) = self.parse_expr(Precedence::P1Comma)? {
			expr_list.push(expr);
		}
		Ok(expr_list)
	}

	#[inline]
	fn peek(&self) -> Option<Token> {
		self.token_list.get(self.index).cloned()
	}

	#[inline]
	fn advance(&mut self) {
		self.index += 1;
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
				Token::Id(id) => {
					if let Some(tk) = self.peek() {
						match tk {
							_ => Ok(Some(Expr::Id(id))),
						}
					} else {
						Ok(Some(Expr::Id(id)))
					}
				}
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

pub fn parse_expr_test(input: &str) -> Result<Vec<Expr>, ParseError> {
	match input.parse() {
		Ok(token_list) => Parser::new(token_list).parse_expr_list(),
		Err(err) => Err(ParseError::LexError(err)),
	}
}

fn parse_fn_definition<'a>(
	iter: &mut impl Iterator<Item = &'a Token>,
	keyword: Keyword,
	id_name: String,
) -> Result<Declaration, ParseError> {
	// int fn (int x, char c) {}
	// int fn () {}
	//        ^
	//        |
	//        |
	//    curr pos
	let mut params = vec![];

	// 解析参数列表
	loop {
		let next_token = take_next_token(iter)?;
		match next_token {
			Token::Punct(punct) => {
				if punct == Punct::ParentheseR {
					break;
				} else {
					return Err(ParseError::Unexpected("not )".into()));
				}
			}
			Token::Keyword(pkw) => {
				if pkw == Keyword::Char || pkw == Keyword::Int {
					let param_name = expect_identifier(iter)?;
					params.push(Parameter { ctype: CType::BaseType(pkw), name: param_name });
					let next_punct = expect_punct_list(iter, &[Punct::Comma, Punct::ParentheseR])?;
					if next_punct == Punct::ParentheseR {
						break;
					}
				} else {
					return Err(ParseError::Unexpected("not type".into()));
				}
			}
			_ => {
				return Err(ParseError::NotKeyword);
			}
		}
	}

	expect_punct_list(iter, &[Punct::BracesL])?;
	// 解析语句列表
	while let Ok(ntk) = take_next_token(iter) {
		if ntk == Token::Punct(Punct::BracesR) {
			break;
		}
	}

	Ok(Declaration::Function(FunctionDefinition {
		ctype: (CType::BaseType(keyword)),
		name: (id_name),
		params,
		stmts: vec![],
	}))
}

fn parse_variable_definition<'a>(
	iter: &mut impl Iterator<Item = &'a Token>,
	keyword: Keyword,
	id_name: String,
	punct: Punct,
) -> Result<Declaration, ParseError> {
	let mut il = vec![];

	// char c = 'A';
	// char c , d    ;
	// char c ;
	//        ^
	//        |
	//        |
	//    curr pos

	if punct == Punct::Assign {
		let val = expect_const(iter)?.check_type_match(&keyword)?;
		il.push(Declarator { name: id_name, value: val });
		// 下一个是 , 或 ;
		let next_punct = expect_punct_list(iter, &[Punct::Comma, Punct::Semicolon])?;
		if next_punct == Punct::Semicolon {
			return Ok(Declaration::Variable(VariableDeclaration { ctype: (CType::BaseType(keyword)), list: (il) }));
		}
	// 下一个是 标识符
	} else if punct == Punct::Comma {
		il.push(Declarator { name: id_name, value: Default::default() });
	// 下一个是 标识符
	} else if punct == Punct::Semicolon {
		il.push(Declarator { name: id_name, value: Default::default() });
		return Ok(Declaration::Variable(VariableDeclaration { ctype: (CType::BaseType(keyword)), list: (il) }));
	}

	// 当前声明数量大于 1, 循环处理
	loop {
		// 下一个是 标识符
		let id_name = expect_identifier(iter)?;
		let next_punct = expect_punct_list(iter, &[Punct::Assign, Punct::Comma, Punct::Semicolon])?;
		if next_punct == Punct::Assign {
			let val = expect_const(iter)?.check_type_match(&keyword)?;
			il.push(Declarator { name: id_name, value: val });
			let next_punct = expect_punct_list(iter, &[Punct::Comma, Punct::Semicolon])?;
			if next_punct == Punct::Semicolon {
				break;
			}
		} else if next_punct == Punct::Comma {
			il.push(Declarator { name: id_name, value: Default::default() });
		} else if next_punct == Punct::Semicolon {
			il.push(Declarator { name: id_name, value: Default::default() });
			break;
		}
	}

	Ok(Declaration::Variable(VariableDeclaration { ctype: (CType::BaseType(keyword)), list: (il) }))
}

/// 解析top level 全局变量声明, 函数定义
fn parse_declaration<'a>(iter: &mut impl Iterator<Item = &'a Token>, tk: &Token) -> Result<Declaration, ParseError> {
	// 1. 函数定义 变量声明共通部分: 解析类型
	// 2. 解析标识符(包括* 指针)
	// 3. 判断是变量定义还是函数声明,分开处理
	// 4. 得到一个声明/定义
	let keyword = tk.try_basetype_keyword().ok_or_else(|| ParseError::Unexpected("char int required".into()))?;

	// TODO 解析 * 间接引用

	// 标准兼容 begin

	// rust果然可以根据流程分析出此值会初始化
	let id_name = match take_next_token(iter)? {
		Token::Punct(Punct::Semicolon) => {
			// warning: useless type name in empty declarationx86-64 gcc 12.1
			// 虽然是合法的,但因为没有用处,gcc会产生警告,
			// 所以这里特殊处理一下也不报错了
			return Ok(Declaration::Variable(VariableDeclaration {
				ctype: (CType::BaseType(keyword)),
				list: (vec![]),
			}));
		}
		Token::Id(id) => id,
		_ => return Err(ParseError::NotIdentifier),
	};
	// 标准兼容 end

	// 更合适的方案
	// let id_name = expect_identifier(iter)?;

	let punct = expect_punct_list(iter, &[Punct::Comma, Punct::Semicolon, Punct::Assign, Punct::ParentheseL])?;

	if punct == Punct::ParentheseL {
		parse_fn_definition(iter, keyword, id_name)
	} else {
		parse_variable_definition(iter, keyword, id_name, punct)
	}
}

fn parse(token_list: Vec<Token>) -> Result<DeclarationList, ParseError> {
	let mut iter = token_list.iter();

	let mut dl = vec![];
	while let Some(tk) = iter.next() {
		dl.push(parse_declaration(&mut iter, tk)?);
	}
	Ok(dl.into())
}

pub fn compile(input: &str) -> Result<DeclarationList, ParseError> {
	match input.parse::<TokenList>() {
		Ok(token_list) => {
			println!("{}", token_list);
			parse(token_list.data)
		}
		Err(err) => Err(ParseError::LexError(err)),
	}
}
