use itertools::Itertools;

use crate::*;

use super::{errors::*, types::*};

fn peek_next_token(iter: &std::slice::Iter<Token>) -> Result<Token, ParseError> {
	if let Some(nt) = iter.clone().next() {
		Ok(nt.clone())
	} else {
		Err(ParseError::EndOfToken)
	}
}

fn take_next_token<'a>(iter: &'a mut core::slice::Iter<Token>) -> Result<&'a Token, ParseError> {
	if let Some(tk) = iter.next() {
		Ok(tk)
	} else {
		Err(ParseError::EndOfToken)
	}
}

fn expect_identifier(iter: &mut core::slice::Iter<Token>) -> Result<String, ParseError> {
	if let Token::Id(id) = take_next_token(iter)? {
		Ok(id.clone())
	} else {
		Err(ParseError::UnexpectedToken("not identifier".into()))
	}
}

fn expect_punct(iter: &mut core::slice::Iter<Token>, l: &[Punct]) -> Result<Punct, ParseError> {
	if let Token::Punct(punct) = take_next_token(iter)? {
		if l.contains(punct) {
			Ok(*punct)
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

fn expect_const(iter: &mut core::slice::Iter<Token>) -> Result<Const, ParseError> {
	match take_next_token(iter)? {
		Token::Const(c) => Ok(*c),
		Token::StringLiteral(_) => Err(ParseError::TypeMismatch),
		_ => Err(ParseError::UnexpectedToken("not const".into())),
	}
}

fn parse_fn_definition(
	iter: &mut core::slice::Iter<Token>,
	keyword: Keyword,
	id_name: String,
) -> Result<Declaration, ParseError> {
	iter.next();
	Err(ParseError::TypeMismatch)
}

fn parse_variable_definition(
	iter: &mut core::slice::Iter<Token>,
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

	// 当前声明数量大于 1, loop直到遇到 ;
	// 下一个是 标识符
	loop {
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
fn parse_declaration(iter: &mut core::slice::Iter<Token>, tk: &Token) -> Result<Declaration, ParseError> {
	// 1. 函数定义 变量声明共通部分: 解析类型
	// 2. 解析标识符(包括* 指针)
	// 3. 判断是变量定义还是函数声明,分开处理
	// 4. 得到一个声明/定义
	let keyword = tk.try_basetype_keyword().ok_or(ParseError::UnexpectedToken("char int required".into()))?;

	// TODO 解析 * 间接引用

	// 标准兼容 begin

	let id_name: String; // rust果然可以根据流程分析出此值会初始化
	match take_next_token(iter)? {
		Token::Punct(Punct::Semicolon) => {
			// warning: useless type name in empty declarationx86-64 gcc 12.1
			// 虽然是合法的,但因为没有用处,gcc会产生警告,
			// 所以这里特殊处理一下也不报错了
			return Ok(Declaration::Variable(VariableDeclaration {
				ctype: (CType::BaseType(keyword)),
				list: (vec![]),
			}));
		}
		Token::Id(id) => {
			id_name = id.clone();
		}
		_ => return Err(ParseError::UnexpectedToken("".into())),
	}
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
