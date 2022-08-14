#[cfg(test)]
#[test]
fn t0() {
	use crate::{
		compile::{errors::*, parse::*, types::*},
		lex::*,
	};
	assert_eq!(
		compile("char ; int ;"),
		Ok(vec![
			Declaration::Variable(VariableDeclaration { ctype: CType::BaseType(Keyword::Char), list: vec![] }),
			Declaration::Variable(VariableDeclaration { ctype: CType::BaseType(Keyword::Int), list: vec![] }),
		]
		.into())
	);
	assert_eq!(
		compile("char a = 'A', b, c = 'C'; int i = 1;"),
		Ok(vec![
			Declaration::Variable(VariableDeclaration {
				ctype: CType::BaseType(Keyword::Char),
				list: vec![
					Declarator { name: "a".into(), value: ("A").into() },
					Declarator { name: "b".into(), value: ("").into() },
					Declarator { name: "c".into(), value: ("C").into() },
				]
			}),
			Declaration::Variable(VariableDeclaration {
				ctype: CType::BaseType(Keyword::Int),
				list: vec![Declarator { name: "i".into(), value: "1".into() }]
			}),
		]
		.into())
	);
	assert_eq!(compile(r###"char c = 'a'"###), Err(ParseError::EndOfToken));
	assert_eq!(compile(r###"char c = 'a' y "###), Err(ParseError::TokenNotPunct));
	assert_eq!(
		compile(r###"char c = 'a' = "###),
		Err(ParseError::expecting_str_but(&mut [Punct::Comma.to_string(), Punct::Semicolon.to_string()], "="))
	);
	assert_eq!(compile(r###"char c "###), Err(ParseError::EndOfToken));
	assert_eq!(compile(r###"int i = 'c';"###), Err(ParseError::TypeMismatch));
	assert_eq!(compile(r###"int i = "int";"###), Err(ParseError::TypeMismatch));
}

#[test]
fn t1() {}
