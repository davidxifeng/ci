use crate::*;

use super::{errors::*, types::*};

type ParseResult = Result<DeclarationList, ParseError>;

fn peek_next_token<'a>(iter: &'a std::slice::Iter<Token>) -> Result<&'a Token, ParseError> {
	if let Some(nt) = iter.clone().next() {
		Ok(nt)
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

fn next_id(iter: &mut core::slice::Iter<Token>) -> Result<String, ParseError> {
	let tk = take_next_token(iter)?;
	match tk {
		Token::Id(id) => Ok(id.clone()),
		_ => Err(ParseError::UnexpectedToken("not id".into())),
	}
}

fn next_const(iter: &mut core::slice::Iter<Token>) -> Result<Const, ParseError> {
	let tk = take_next_token(iter)?;
	match tk {
		Token::Const(c) => Ok(*c),
		Token::StringLiteral(_) => Err(ParseError::TypeMismatch),
		_ => Err(ParseError::UnexpectedToken("not char const".into())),
	}
}

fn parse_declaration(keyword: Keyword, iter: &mut core::slice::Iter<Token>) -> Result<Declaration, ParseError> {
	let mut il = vec![];

	while peek_next_token(iter)?.is_not_semicolon() {
		let id_name = next_id(iter)?;
		let next_punct = take_next_token(iter)?.expect_punct(&[Punct::Comma, Punct::Semicolon, Punct::Assign])?;
		if *next_punct == Punct::Assign {
			let val = next_const(iter)?.check_type_match(&keyword)?;
			il.push(Declarator { name: id_name, value: val });

			let next_punct = take_next_token(iter)?.expect_punct(&[Punct::Comma, Punct::Semicolon])?;
			if *next_punct == Punct::Semicolon {
				break;
			}
		} else if *next_punct == Punct::Comma {
			il.push(Declarator { name: id_name, value: Default::default() });
		} else if *next_punct == Punct::Semicolon {
			il.push(Declarator { name: id_name, value: Default::default() });
			break;
		}
	}

	Ok(Declaration::Variable { ci_type: (CiType::BaseType(keyword)), list: (il) })
}

pub fn parse(token_list: Vec<Token>) -> ParseResult {
	let mut dl = vec![];
	let mut iter = token_list.iter();

	while let Some(tk) = iter.next() {
		if let Some(kw) = tk.try_basetype_keyword() {
			dl.push(parse_declaration(kw, &mut iter)?);
		} else if tk.is_enum_type() {
		}
	}

	Ok(DeclarationList { list: dl })
}

pub fn compile(input: &str) -> Result<DeclarationList, ParseError> {
	match TokenApi::parse_all(input) {
		Ok(token_list) => parse(token_list),
		Err(err) => Err(ParseError::LexError(err)),
	}
}
