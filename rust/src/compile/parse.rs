use std::{iter::Peekable, slice::Iter};

use itertools::Itertools;

use crate::*;

use super::{errors::*, types::*};

fn look_ahead(iter: &Iter<Token>) -> Result<Token, ParseError> {
	let mut lait = iter.clone();
	if let Some(tk) = lait.next() {
		Ok(tk.clone())
	} else {
		Err(ParseError::EndOfToken)
	}
}

fn take_next_token(iter: &mut Iter<Token>) -> Result<Token, ParseError> {
	if let Some(tk) = iter.next() {
		Ok(tk.clone())
	} else {
		Err(ParseError::EndOfToken)
	}
}

fn expect_identifier(iter: &mut Iter<Token>) -> Result<String, ParseError> {
	if let Token::Id(id) = take_next_token(iter)? {
		Ok(id)
	} else {
		Err(ParseError::TokenNotIdentifier)
	}
}

fn expect_punct(iter: &mut Iter<Token>, l: &[Punct]) -> Result<Punct, ParseError> {
	if let Token::Punct(punct) = take_next_token(iter)? {
		if l.contains(&punct) {
			Ok(punct)
		} else {
			Err(ParseError::expecting_str_but(
				&mut l.iter().map(|&x| x.to_string()).collect_vec(),
				punct.to_string().as_str(),
			))
		}
	} else {
		Err(ParseError::TokenNotPunct)
	}
}

fn expect_const(iter: &mut Iter<Token>) -> Result<Const, ParseError> {
	match take_next_token(iter)? {
		Token::Const(c) => Ok(c),
		Token::StringLiteral(_) => Err(ParseError::TypeMismatch),
		_ => Err(ParseError::UnexpectedToken("not const".into())),
	}
}

fn parse_stmt(iter: &mut Iter<Token>, ntk: Token) -> Result<Statement, ParseError> {
	match ntk {
		Token::Keyword(Keyword::Return) => {
			let cst = expect_const(iter)?;
			expect_punct(iter, &[Punct::Semicolon])?;
			Ok(Statement::Return(ReturnStmt { expr: Expr::Const(cst) }))
		}
		// TODO
		_ => Err(ParseError::TypeMismatch),
	}
}

fn parse_fn_definition(iter: &mut Iter<Token>, keyword: Keyword, id_name: String) -> Result<Declaration, ParseError> {
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
					return Err(ParseError::UnexpectedToken("not )".into()));
				}
			}
			Token::Keyword(pkw) => {
				if pkw == Keyword::Char || pkw == Keyword::Int {
					let param_name = expect_identifier(iter)?;
					params.push(Parameter { ctype: CType::BaseType(pkw), name: param_name });
					let next_punct = expect_punct(iter, &[Punct::Comma, Punct::ParentheseR])?;
					if next_punct == Punct::ParentheseR {
						break;
					}
				} else {
					return Err(ParseError::UnexpectedToken("not type".into()));
				}
			}
			_ => {
				return Err(ParseError::TokenNotKeyword);
			}
		}
	}

	expect_punct(iter, &[Punct::BracesL])?;
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

fn parse_variable_definition(
	iter: &mut Iter<Token>,
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
		let next_punct = expect_punct(iter, &[Punct::Comma, Punct::Semicolon])?;
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
		let next_punct = expect_punct(iter, &[Punct::Assign, Punct::Comma, Punct::Semicolon])?;
		if next_punct == Punct::Assign {
			let val = expect_const(iter)?.check_type_match(&keyword)?;
			il.push(Declarator { name: id_name, value: val });
			let next_punct = expect_punct(iter, &[Punct::Comma, Punct::Semicolon])?;
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
fn parse_declaration(iter: &mut Iter<Token>, tk: &Token) -> Result<Declaration, ParseError> {
	// 1. 函数定义 变量声明共通部分: 解析类型
	// 2. 解析标识符(包括* 指针)
	// 3. 判断是变量定义还是函数声明,分开处理
	// 4. 得到一个声明/定义
	let keyword = tk.try_basetype_keyword().ok_or_else(|| ParseError::UnexpectedToken("char int required".into()))?;

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
		_ => return Err(ParseError::TokenNotIdentifier),
	};
	// 标准兼容 end

	// 更合适的方案
	// let id_name = expect_identifier(iter)?;

	let punct = expect_punct(iter, &[Punct::Comma, Punct::Semicolon, Punct::Assign, Punct::ParentheseL])?;

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
	match TokenApi::parse_all(input) {
		Ok(token_list) => parse(token_list),
		Err(err) => Err(ParseError::LexError(err)),
	}
}

type EvalResult = Result<i64, ParseError>;

fn calc(op: &Punct, a: i64, b: i64) -> i64 {
	match op {
		Punct::Add => a + b,
		Punct::Sub => a - b,
		Punct::Mul => a * b,
		Punct::Div => a / b,
		Punct::Xor => pow(a, b),
		_ => 0,
	}
}

fn pow(a: i64, b: i64) -> i64 {
	let mut r = 1;
	for _ in 0..b {
		r *= a
	}
	r
}

fn op_info(op: &Punct) -> i8 {
	match op {
		Punct::Add => 1,
		Punct::Sub => 1,
		Punct::Mul => 2,
		Punct::Div => 2,
		Punct::Xor => 3,
		_ => panic!("bad input"),
	}
}

fn compute_atom(iter: &mut Iter<Token>) -> EvalResult {
	if let Some(lhs_tk) = iter.next() {
		match lhs_tk {
			Token::Const(Const::Integer(lhs)) => Ok(*lhs as i64),
			Token::Punct(Punct::ParentheseL) => Ok(start_eval(iter)?),
			_ => Err(ParseError::UnexpectedToken("no more".into())),
		}
	} else {
		Err(ParseError::EndOfToken)
	}
}

pub fn eval(iter: &mut Iter<Token>, mut lhs: i64, lv: i8) -> EvalResult {
	// let ops = [Punct::Add, Punct::Sub, Punct::Mul, Punct::Div, Punct::Xor];

	let mut lookahead_p = match look_ahead(iter) {
		Ok(Token::Punct(p)) => p,
		_ => return Ok(lhs),
	};

	while op_info(&lookahead_p) >= lv {
		let op = lookahead_p;
		if iter.next().is_none() {
			break;
		}

		// 取第二个数
		let mut rhs = compute_atom(iter)?;

		lookahead_p = match look_ahead(iter) {
			Ok(Token::Punct(p)) => {
				if p == Punct::ParentheseR {
					iter.next();
					return Ok(calc(&op, lhs, rhs));
				} else {
					p
				}
			}
			_ => return Ok(calc(&op, lhs, rhs)),
		};

		while op_info(&lookahead_p) > op_info(&op)
			|| (lookahead_p == Punct::Xor && op_info(&lookahead_p) == op_info(&op))
		{
			rhs = eval(iter, rhs, op_info(&op) + if op_info(&lookahead_p) > op_info(&op) { 1 } else { 0 })?;

			lookahead_p = match look_ahead(iter) {
				Ok(Token::Punct(p)) => p,
				_ => break,
			};
		}
		lhs = calc(&op, lhs, rhs);
	}
	Ok(lhs)
}

pub fn start_eval(iter: &mut Iter<Token>) -> EvalResult {
	// 取第一个数
	let lhs = compute_atom(iter)?;
	eval(iter, lhs, 0)
}

fn compute_atom2(iter: &mut Peekable<Iter<Token>>, cop: &mut Option<Punct>) -> EvalResult {
	if let Some(tk) = iter.next() {
		match tk {
			Token::Const(Const::Integer(lhs)) => Ok(*lhs as i64),
			Token::Punct(Punct::ParentheseL) => Ok(eval2(iter, 1, cop)?),
			_ => Err(ParseError::UnexpectedToken("".into())),
		}
	} else {
		Err(ParseError::EndOfToken)
	}
}

fn eval2(iter: &mut Peekable<Iter<Token>>, mp: i8, cop: &mut Option<Punct>) -> EvalResult {
	let mut lhs = compute_atom2(iter, cop)?;

	*cop = match iter.next() {
		Some(Token::Punct(mop)) => Some(*mop),
		None => None,
		_ => unreachable!(),
	};

	loop {
		match *cop {
			Some(Punct::ParentheseR) => break,
			Some(op) => {
				let lv = op_info(&op);
				// println!("comparing {} lv: {}, mp: {} == {}", op, lv, mp, lv >= mp);
				if lv >= mp {
					// println!("calc: {} {} {}", lhs, op, rhs);
					lhs = calc(&op, lhs, eval2(iter, lv + if op == Punct::Xor { 0 } else { 1 }, cop)?);
				} else {
					break;
				}
			}
			None => break,
		}
	}

	Ok(lhs)
}

pub fn t2(input: &str) -> EvalResult {
	match TokenApi::parse_all(input) {
		Ok(token_list) => {
			let mut iter = token_list.iter().peekable();
			let mut cop = None;
			eval2(&mut iter, 1, &mut cop)
		}
		Err(err) => Err(ParseError::LexError(err)),
	}
}

#[test]
fn test_peekable_eval() {
	assert_eq!(t2("1 + 2"), Ok(3));
	assert_eq!(t2("(1 + 2) * 3"), Ok(9));
	assert_eq!(t2("1 + 2 + 3"), Ok(6));
	assert_eq!(t2("1 + 2 * 3"), Ok(7));
	assert_eq!(t2("1 + 2 * 3 ^ 2 + 2 * 6"), Ok(31));
	assert_eq!(t2("(1 + 2) * ((3 - 5) * 2) ^ 2 + 2 * 6"), Ok(60));
	assert_eq!(t2("3 * ((3 - 5) * 2) ^ 2 + 2 * 6"), Ok(60));
	assert_eq!(t2("3 * (2 + 2) ^ 2 + 2 * 6"), Ok(60));
	assert_eq!(t2("(1 + 2) * ((3 - 5) * 2) ^ 2 ^ 2 + 2 * 6"), Ok(780));
}

pub fn t(input: &str) -> EvalResult {
	match TokenApi::parse_all(input) {
		Ok(token_list) => start_eval(&mut token_list.iter()),
		Err(err) => Err(ParseError::LexError(err)),
	}
}

#[test]
fn test_eval() {
	assert_eq!(t("1 + 2"), Ok(3));
	assert_eq!(t("1 + 2 + 3"), Ok(6));
	assert_eq!(t("1 + 2 * 3"), Ok(7));
	assert_eq!(t("(1 + 2) * 3"), Ok(9));
	assert_eq!(t("1 + 2 * 3 ^ 2"), Ok(19));
	assert_eq!(t("1 - 2 * 3 ^ 2"), Ok(-17));
	assert_eq!(t("1 - (2 * 3) ^ 2"), Ok(-35));

	assert_eq!(t("1 + 2 * 3 ^ 2 + 2 * 6"), Ok(31));

	assert_eq!(t("(1 + 2) * ((3 - 5) * 2) ^ 2 + 2 * 6"), Ok(60));

	assert_eq!(t("2 ^ 3 ^ 2"), Ok(512));
	assert_eq!(t("2 * 2 ^ 3 ^ 2"), Ok(1024));
	assert_eq!(t("2 * 2 ^ 3 ^ 2 * 2 / 2 + 1 * 2 ^ 2 * 20"), Ok(1104));
	assert_eq!(t("2 ^ 3 ^ 2 * 2"), Ok(1024));
}