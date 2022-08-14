#[cfg(test)]
use crate::{
	compile::{errors::*, parse::*, types::*},
	lex::*,
};

#[test]
fn t0() {
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
	assert_eq!(compile(r###"char 2 = 'a'"###), Err(ParseError::TokenNotIdentifier));
	assert_eq!(compile(r###"char a = 'a', 2 = 'c'"###), Err(ParseError::TokenNotIdentifier));

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
fn t1() {
	assert_eq!(
		compile("int id() {  }"),
		Ok(vec![Declaration::Function(FunctionDefinition {
			ctype: CType::BaseType(Keyword::Int),
			name: "id".into(),
			params: vec![],
			stmts: vec![]
		}),]
		.into())
	);
	assert_eq!(
		compile("int id(char c,int i) { return 1; return 'a'; }"),
		Ok(vec![Declaration::Function(FunctionDefinition {
			ctype: CType::BaseType(Keyword::Int),
			name: "id".into(),
			params: vec![
				Parameter { ctype: CType::BaseType(Keyword::Char), name: "c".into() },
				Parameter { ctype: CType::BaseType(Keyword::Int), name: "i".into() }
			],
			stmts: vec![
				Statement::Return(ReturnStmt { expr: Expr::Const(Const::Integer(1)) }),
				Statement::Return(ReturnStmt { expr: Expr::Const(Const::Character('a')) })
			]
		}),]
		.into())
	);
}
