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

fn expect_punct<'a>(iter: &mut impl Iterator<Item = &'a Token>, expect: &Punct) -> Result<(), ParseError> {
	if let Token::Punct(punct) = take_next_token(iter)? {
		if *expect == punct {
			Ok(())
		} else {
			let s = format!("expecting: {} ,got: {}", expect, punct);
			Err(ParseError::Unexpected(s))
		}
	} else {
		Err(ParseError::NotPunct)
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
	P0None,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Associativity {
	None,
	LeftToRight,
	RightToLeft,
}

fn token_info(token: &Token) -> (Precedence, Associativity) {
	match token {
		Token::Const(_) => (Precedence::P0None, Associativity::None),
		Token::Id(_) => (Precedence::P0None, Associativity::None),
		Token::StringLiteral(_) => (Precedence::P0None, Associativity::None),

		Token::Keyword(Keyword::SizeOf) => (Precedence::P14Unary, Associativity::RightToLeft),
		Token::Keyword(_) => (Precedence::P0None, Associativity::None),

		Token::Punct(Punct::Comma) => (Precedence::P1Comma, Associativity::LeftToRight),

		Token::Punct(Punct::Assign) => (Precedence::P2Assign, Associativity::RightToLeft),
		Token::Punct(Punct::AssignAdd) => (Precedence::P2Assign, Associativity::RightToLeft),
		Token::Punct(Punct::AssignSub) => (Precedence::P2Assign, Associativity::RightToLeft),
		Token::Punct(Punct::AssignMul) => (Precedence::P2Assign, Associativity::RightToLeft),
		Token::Punct(Punct::AssignDiv) => (Precedence::P2Assign, Associativity::RightToLeft),
		Token::Punct(Punct::AssignMod) => (Precedence::P2Assign, Associativity::RightToLeft),

		Token::Punct(Punct::Cond) => (Precedence::P3Cond, Associativity::RightToLeft),

		Token::Punct(Punct::Lor) => (Precedence::P4LOr, Associativity::LeftToRight),
		Token::Punct(Punct::Lan) => (Precedence::P5LAnd, Associativity::LeftToRight),
		Token::Punct(Punct::Or) => (Precedence::P6BOr, Associativity::LeftToRight),
		Token::Punct(Punct::Xor) => (Precedence::P7BXor, Associativity::LeftToRight),
		Token::Punct(Punct::And) => (Precedence::P8BAnd, Associativity::LeftToRight),

		Token::Punct(Punct::Eq) => (Precedence::P9Eq, Associativity::LeftToRight),
		Token::Punct(Punct::Ne) => (Precedence::P9Eq, Associativity::LeftToRight),

		Token::Punct(Punct::Lt) => (Precedence::P10Cmp, Associativity::LeftToRight),
		Token::Punct(Punct::Le) => (Precedence::P10Cmp, Associativity::LeftToRight),
		Token::Punct(Punct::Gt) => (Precedence::P10Cmp, Associativity::LeftToRight),
		Token::Punct(Punct::Ge) => (Precedence::P10Cmp, Associativity::LeftToRight),

		Token::Punct(Punct::Shl) => (Precedence::P11BShift, Associativity::LeftToRight),
		Token::Punct(Punct::Shr) => (Precedence::P11BShift, Associativity::LeftToRight),

		Token::Punct(Punct::Add) => (Precedence::P12Add, Associativity::LeftToRight),
		Token::Punct(Punct::Sub) => (Precedence::P12Add, Associativity::LeftToRight),

		Token::Punct(Punct::Mul) => (Precedence::P13Mul, Associativity::LeftToRight),
		Token::Punct(Punct::Div) => (Precedence::P13Mul, Associativity::LeftToRight),
		Token::Punct(Punct::Mod) => (Precedence::P13Mul, Associativity::LeftToRight),

		Token::Punct(Punct::Not) => (Precedence::P14Unary, Associativity::RightToLeft),

		Token::Punct(Punct::BrakL) => (Precedence::P15Post, Associativity::LeftToRight),

		// 一些运算符有多种含义, 比如 & 是 位运算与 和 取地址运算符
		// - + 既是 一元运算符,又是二元运算符, 这些需要放到语法中处理
		Token::Punct(_) => (Precedence::P0None, Associativity::None),
	}
}

fn single_expr_node<'a>(iter: &mut impl Iterator<Item = &'a Token>, ntk: Token) -> Result<Expr, ParseError> {
	match ntk {
		Token::Const(c) => Ok(Expr::Const(c)),
		Token::StringLiteral(str) => Ok(Expr::StringLiteral(str)),
		Token::Id(id) => {
			if let Some(tk) = iter.next() {
				if *tk == Token::Punct(Punct::Assign) {
					let ntk = take_next_token(iter)?;
					let right = parse_expr(iter, ntk, Punct::Comma)?;
					return Ok(Expr::AssignExpr(AssignExpr {
						left: Box::new(Expr::Id(id)),
						assign: Punct::Assign,
						right: Box::new(right),
					}));
				} else {
					unimplemented!()
				}
			} else {
				return Ok(Expr::Id(id));
			}
		}
		Token::Punct(Punct::Semicolon) => {
			unimplemented!()
		}
		Token::Punct(Punct::ParentheseL) => {
			let ntk = take_next_token(iter)?;
			let expr = parse_expr(iter, ntk, Punct::Comma)?;
			expect_punct_list(iter, &[Punct::ParentheseR])?;
			return Ok(expr);
		}
		Token::Keyword(Keyword::SizeOf) => {
			unimplemented!()
		}
		_ => unimplemented!(),
	}
}

fn parse_expr<'a>(iter: &mut impl Iterator<Item = &'a Token>, ntk: Token, level: Punct) -> Result<Expr, ParseError> {
	let expr = single_expr_node(iter, ntk)?;
	Ok(expr)
}

pub struct Parser {
	token_list: Vec<Token>,
	index: usize,
}

impl Parser {
	fn parse_expr_list(&mut self) -> Result<Vec<Expr>, ParseError> {
		let mut expr_list = vec![];
		while let Some(expr) = self.parse_expr_slice(Precedence::P1Comma)? {
			expr_list.push(expr);
		}
		Ok(expr_list)
	}

	fn lookahead(&mut self) -> Option<Token> {
		self.token_list.get(self.index + 1).map(|x| x.clone())
	}
	fn peek(&mut self) -> Option<Token> {
		self.token_list.get(self.index).map(|x| x.clone())
	}
	fn advance(&mut self) {
		self.index += 1;
	}

	fn next(&mut self) -> Result<Token, ParseError> {
		let r = self.token_list.get(self.index);
		self.index += 1;
		r.map_or(Err(ParseError::EndOfToken), |x| Ok(x.clone()))
	}

	fn parse_leaf(&mut self) -> Option<Expr> {
		if let Ok(tk) = self.next() {
			match tk {
				Token::Const(c) => Some(Expr::Const(c.to_owned())),
				Token::StringLiteral(str) => Some(Expr::StringLiteral(str.to_owned())),
				Token::Id(id) => {
					if let Some(tk) = self.peek() {
						match tk {
							_ => Some(Expr::Id(id.to_owned())),
						}
					} else {
						Some(Expr::Id(id.to_owned()))
					}
				}
				Token::Punct(Punct::Semicolon) => {
					unimplemented!()
				}
				Token::Punct(Punct::ParentheseL) => {
					unimplemented!()

					// let ntk = take_next_token(iter)?;
					// let expr = parse_expr(iter, ntk, Punct::Comma)?;
					// expect_punct_list(iter, &[Punct::ParentheseR])?;
					// return Ok(expr);
				}
				Token::Keyword(Keyword::SizeOf) => {
					unimplemented!()
				}
				_ => unimplemented!(),
			}
		} else {
			None
		}
	}

	fn parse_expr_slice(&mut self, precedence: Precedence) -> Result<Option<Expr>, ParseError> {
		let leaf = match self.parse_leaf() {
			Some(leaf) => leaf,
			None => return Ok(None),
		};

		match self.peek() {
			None => Ok(Some(leaf)),
			Some(ref ntk) => {
				let next_token_info = token_info(ntk);
				if next_token_info.0 >= precedence {
					self.advance();
					match ntk {
						Token::Punct(p) if p.is_assign() => {
							self.parse_expr_slice(Precedence::P2Assign)?.map_or(Err(ParseError::EndOfToken), |right| {
								Ok(Some(Expr::AssignExpr(AssignExpr {
									left: Box::new(leaf),
									assign: Punct::Assign,
									right: Box::new(right),
								})))
							})
						}
						_ => unimplemented!(),
					}
				} else {
					Ok(Some(leaf))
				}
			}
		}
	}
}

pub fn parse_expr_test(input: &str) -> Result<Vec<Expr>, ParseError> {
	match input.parse::<TokenList>() {
		Ok(r) => {
			let mut parser = Parser { token_list: r.token_list, index: 0 };
			parser.parse_expr_list()
		}
		Err(err) => Err(ParseError::LexError(err)),
	}
}

fn parse_stmt<'a>(iter: &mut impl Iterator<Item = &'a Token>, ntk: Token) -> Result<Statement, ParseError> {
	match ntk {
		Token::Keyword(Keyword::Return) => {
			let cst = expect_const(iter)?;
			expect_punct(iter, &Punct::Semicolon)?;
			Ok(Statement::ReturnStmt(Expr::Const(cst)))
		}
		Token::Keyword(Keyword::If) => unimplemented!("if stmt"),
		Token::Keyword(Keyword::While) => unimplemented!("while stmt"),
		Token::Punct(Punct::BracesL) => unimplemented!("compound statement"),
		// id, lookahead : => labeled stmt
		_ => {
			let expr = parse_expr(iter, ntk, Punct::Comma)?;
			expect_punct(iter, &Punct::Semicolon)?;
			Ok(Statement::ExprStmt(expr))
		}
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
	let mut stmts = vec![];

	// 解析语句列表
	while let Ok(ntk) = take_next_token(iter) {
		if ntk == Token::Punct(Punct::BracesR) {
			break;
		} else {
			stmts.push(parse_stmt(iter, ntk)?);
		}
	}

	Ok(Declaration::Function(FunctionDefinition { ctype: (CType::BaseType(keyword)), name: (id_name), params, stmts }))
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
			parse(token_list.token_list)
		}
		Err(err) => Err(ParseError::LexError(err)),
	}
}
