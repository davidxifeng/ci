use crate::compile::{
	errors::LexError,
	lex::TokenApi,
	token::{Const, Keyword, Punct, Token},
};

#[test]
fn lex_keyword() {
	assert_eq!(TokenApi::parse_all("char"), Ok(vec![Token::Keyword(Keyword::Char)]));
	assert_eq!(TokenApi::parse_all("int"), Ok(vec![Token::Keyword(Keyword::Int)]));
	assert_eq!(TokenApi::parse_all("enum"), Ok(vec![Token::Keyword(Keyword::Enum)]));
	assert_eq!(TokenApi::parse_all("if"), Ok(vec![Token::Keyword(Keyword::If)]));
	assert_eq!(TokenApi::parse_all("else"), Ok(vec![Token::Keyword(Keyword::Else)]));
	assert_eq!(TokenApi::parse_all("while"), Ok(vec![Token::Keyword(Keyword::While)]));
	assert_eq!(TokenApi::parse_all("return"), Ok(vec![Token::Keyword(Keyword::Return)]));
}

#[test]
fn identifier() {
	assert_eq!(TokenApi::parse_all("fn"), Ok(vec![Token::Id("fn".into())]));
	assert_eq!(TokenApi::parse_all("fn id2"), Ok(vec![Token::Id("fn".into()), Token::Id("id2".into())]));
	assert_eq!(
		TokenApi::parse_all("fn+id2"),
		Ok(vec![Token::Id("fn".into()), Token::Punct(Punct::Add), Token::Id("id2".into())])
	);
}

#[test]
fn const_value() {
	assert_eq!(TokenApi::parse_all("123"), Ok(vec![Token::Const(Const::Integer("123".to_owned()))]));
	assert_eq!(
		TokenApi::parse_all("1 23"),
		Ok(vec![Token::Const(Const::Integer("1".to_owned())), Token::Const(Const::Integer("23".to_owned()))])
	);
}
#[test]
fn string_char() {
	assert_eq!(TokenApi::parse_all(r##""I am a C string""##), Ok(vec![Token::StringLiteral("I am a C string".into())]));
	assert_eq!(
		TokenApi::parse_all(r##""I am a C string\nline 2""##),
		Ok(vec![Token::StringLiteral("I am a C string\nline 2".into())])
	);
	assert_eq!(TokenApi::parse_all(r##""I am a C string"##), Err(LexError::UnexpectedEof));
	assert_eq!(TokenApi::parse_all(r##""I am a \C string"##), Err(LexError::UnknownEscape('C')));
	assert_eq!(
		TokenApi::parse_all(r##"123 fn "I am a C string""##),
		Ok(vec![
			Token::Const(Const::Integer("123".to_owned())),
			Token::Id("fn".into()),
			Token::StringLiteral("I am a C string".into())
		])
	);

	assert_eq!(TokenApi::parse_all("\"abc\n\""), Err(LexError::ExpectingBut('\"', '\n')));
	assert_eq!(TokenApi::parse_all("\'abc\'"), Err(LexError::MoreThanOneChar));
	assert_eq!(TokenApi::parse_all("\'\'"), Err(LexError::EmptyChar));

	assert_eq!(TokenApi::parse_all("\'a\'"), Ok(vec![Token::Const(Const::Character('a'))]));
	assert_eq!(TokenApi::parse_all("\'\\n\'"), Ok(vec![Token::Const(Const::Character('\n'))]));
}

#[test]
fn comment_preprocessor() {
	assert_eq!(
		TokenApi::parse_all(
			r##"#include <stdio.h>
		x 123
		#if 1
		#endif
		c
		"##
		),
		Ok(vec![Token::Id("x".into()), Token::Const(Const::Integer("123".to_owned())), Token::Id("c".into()),])
	);
	assert_eq!(TokenApi::parse_all(r##"#include <stdio.h>"##), Ok(vec![]));
	assert_eq!(TokenApi::parse_all(r##"1#include <stdio.h>"##), Ok(vec![Token::Const(Const::Integer("1".to_owned()))]));
	assert_eq!(TokenApi::parse_all(r##"// hi"##), Ok(vec![]));
	assert_eq!(TokenApi::parse_all(r##"1// hi"##), Ok(vec![Token::Const(Const::Integer("1".to_owned()))]));
	assert_eq!(
		TokenApi::parse_all(
			r##"1// hi
		2
		"##
		),
		Ok(vec![Token::Const(Const::Integer("1".to_owned())), Token::Const(Const::Integer("2".to_owned()))])
	);
}

#[test]
fn punct_and_ordering() {
	assert_eq!(
		TokenApi::parse_all(r##"1/2"##),
		Ok(vec![
			Token::Const(Const::Integer("1".to_owned())),
			Token::Punct(Punct::Div),
			Token::Const(Const::Integer("2".to_owned()))
		])
	);
	assert_eq!(TokenApi::parse_all(r##"1//2"##), Ok(vec![Token::Const(Const::Integer("1".to_owned()))]));
	assert_eq!(TokenApi::parse_all("="), Ok(vec![Token::Punct(Punct::Assign)]));
	assert_eq!(TokenApi::parse_all("=="), Ok(vec![Token::Punct(Punct::Eq)]));
	assert_eq!(TokenApi::parse_all("==="), Ok(vec![Token::Punct(Punct::Eq), Token::Punct(Punct::Assign)]));
	assert_eq!(TokenApi::parse_all("===="), Ok(vec![Token::Punct(Punct::Eq), Token::Punct(Punct::Eq)]));

	assert_eq!(TokenApi::parse_all("+"), Ok(vec![Token::Punct(Punct::Add)]));
	assert_eq!(TokenApi::parse_all("++"), Ok(vec![Token::Punct(Punct::Inc)]));
	assert_eq!(TokenApi::parse_all("+++"), Ok(vec![Token::Punct(Punct::Inc), Token::Punct(Punct::Add)]));
	assert_eq!(TokenApi::parse_all("++++"), Ok(vec![Token::Punct(Punct::Inc), Token::Punct(Punct::Inc)]));

	assert_eq!(TokenApi::parse_all("-"), Ok(vec![Token::Punct(Punct::Sub)]));
	assert_eq!(TokenApi::parse_all("--"), Ok(vec![Token::Punct(Punct::Dec)]));

	assert_eq!(TokenApi::parse_all("!"), Ok(vec![Token::Punct(Punct::Not)]));
	assert_eq!(TokenApi::parse_all("!="), Ok(vec![Token::Punct(Punct::Ne)]));

	assert_eq!(TokenApi::parse_all("<"), Ok(vec![Token::Punct(Punct::Lt)]));
	assert_eq!(TokenApi::parse_all("< "), Ok(vec![Token::Punct(Punct::Lt)]));
	assert_eq!(TokenApi::parse_all("<="), Ok(vec![Token::Punct(Punct::Le)]));
	assert_eq!(TokenApi::parse_all("<=<="), Ok(vec![Token::Punct(Punct::Le), Token::Punct(Punct::Le)]));
	assert_eq!(TokenApi::parse_all("<= <="), Ok(vec![Token::Punct(Punct::Le), Token::Punct(Punct::Le)]));
	assert_eq!(TokenApi::parse_all("<<"), Ok(vec![Token::Punct(Punct::Shl)]));

	assert_eq!(TokenApi::parse_all(">"), Ok(vec![Token::Punct(Punct::Gt)]));
	assert_eq!(TokenApi::parse_all("> "), Ok(vec![Token::Punct(Punct::Gt)]));
	assert_eq!(TokenApi::parse_all(">="), Ok(vec![Token::Punct(Punct::Ge)]));
	assert_eq!(TokenApi::parse_all(">=>="), Ok(vec![Token::Punct(Punct::Ge), Token::Punct(Punct::Ge)]));
	assert_eq!(TokenApi::parse_all(">>"), Ok(vec![Token::Punct(Punct::Shr)]));

	assert_eq!(TokenApi::parse_all(">=<="), Ok(vec![Token::Punct(Punct::Ge), Token::Punct(Punct::Le)]));
	assert_eq!(
		TokenApi::parse_all(">=1"),
		Ok(vec![Token::Punct(Punct::Ge), Token::Const(Const::Integer("1".to_owned()))])
	);
	assert_eq!(
		TokenApi::parse_all(">1"),
		Ok(vec![Token::Punct(Punct::Gt), Token::Const(Const::Integer("1".to_owned()))])
	);

	assert_eq!(TokenApi::parse_all("|"), Ok(vec![Token::Punct(Punct::Or)]));
	assert_eq!(TokenApi::parse_all("||"), Ok(vec![Token::Punct(Punct::Lor)]));
	assert_eq!(TokenApi::parse_all("&"), Ok(vec![Token::Punct(Punct::And)]));
	assert_eq!(TokenApi::parse_all("&&"), Ok(vec![Token::Punct(Punct::Lan)]));
	assert_eq!(TokenApi::parse_all("^"), Ok(vec![Token::Punct(Punct::Xor)]));

	assert_eq!(TokenApi::parse_all("%"), Ok(vec![Token::Punct(Punct::Mod)]));
	assert_eq!(TokenApi::parse_all("*"), Ok(vec![Token::Punct(Punct::Mul)]));
	assert_eq!(TokenApi::parse_all("["), Ok(vec![Token::Punct(Punct::BrakL)]));
	assert_eq!(TokenApi::parse_all("?"), Ok(vec![Token::Punct(Punct::Cond)]));
	assert_eq!(TokenApi::parse_all(";"), Ok(vec![Token::Punct(Punct::Semicolon)]));
	assert_eq!(TokenApi::parse_all(","), Ok(vec![Token::Punct(Punct::Comma)]));
	assert_eq!(TokenApi::parse_all("."), Ok(vec![Token::Punct(Punct::Dot)]));
	assert_eq!(TokenApi::parse_all(".."), Ok(vec![Token::Punct(Punct::Dot), Token::Punct(Punct::Dot)]));
	assert_eq!(TokenApi::parse_all("..."), Ok(vec![Token::Punct(Punct::VarArg)]));
	assert_eq!(TokenApi::parse_all("->"), Ok(vec![Token::Punct(Punct::Arrow)]));
	assert_eq!(TokenApi::parse_all(":"), Ok(vec![Token::Punct(Punct::Colon)]));
	assert_eq!(TokenApi::parse_all("<:"), Ok(vec![Token::Punct(Punct::BrakL)]));
	assert_eq!(TokenApi::parse_all(":>"), Ok(vec![Token::Punct(Punct::BrakR)]));
	assert_eq!(TokenApi::parse_all("<%"), Ok(vec![Token::Punct(Punct::BracesL)]));
	assert_eq!(TokenApi::parse_all("%>"), Ok(vec![Token::Punct(Punct::BracesR)]));

	assert_eq!(TokenApi::parse_all(">>="), Ok(vec![Token::Punct(Punct::AssignShr)]));
	assert_eq!(TokenApi::parse_all("<<="), Ok(vec![Token::Punct(Punct::AssignShl)]));
	assert_eq!(TokenApi::parse_all("+="), Ok(vec![Token::Punct(Punct::AssignAdd)]));
	assert_eq!(TokenApi::parse_all("-="), Ok(vec![Token::Punct(Punct::AssignSub)]));
	assert_eq!(TokenApi::parse_all("*="), Ok(vec![Token::Punct(Punct::AssignMul)]));
	assert_eq!(TokenApi::parse_all("/="), Ok(vec![Token::Punct(Punct::AssignDiv)]));
	assert_eq!(TokenApi::parse_all("%="), Ok(vec![Token::Punct(Punct::AssignMod)]));
	assert_eq!(TokenApi::parse_all("&="), Ok(vec![Token::Punct(Punct::AssignBAnd)]));
	assert_eq!(TokenApi::parse_all("|="), Ok(vec![Token::Punct(Punct::AssignBOr)]));
	assert_eq!(TokenApi::parse_all("^="), Ok(vec![Token::Punct(Punct::AssignBXor)]));
}
