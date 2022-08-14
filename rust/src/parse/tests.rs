#[cfg(test)]
use crate::*;

#[test]
fn test_t0() {
	assert_eq!(
		SyntaxTree::compile("char ; int ;"),
		Ok(vec![
			Declaration::Variable { ci_type: (CiType::BaseType(Keyword::Char)), list: vec![] },
			Declaration::Variable { ci_type: (CiType::BaseType(Keyword::Int)), list: vec![] },
		]
		.into())
	);
	assert_eq!(
		SyntaxTree::compile("char a = 'A', b, c = 'C'; int i = 1;"),
		Ok(vec![
			Declaration::Variable {
				ci_type: (CiType::BaseType(Keyword::Char)),
				list: vec![
					Declarator { name: "a".into(), value: ("A").into() },
					Declarator { name: "b".into(), value: ("").into() },
					Declarator { name: "c".into(), value: ("C").into() },
				]
			},
			Declaration::Variable {
				ci_type: (CiType::BaseType(Keyword::Int)),
				list: vec![Declarator { name: "i".into(), value: "1".into() }]
			},
		]
		.into())
	);
	assert_eq!(SyntaxTree::compile(r###"char c = 'a'"###), Err(ParseError::EndOfToken));
	assert_eq!(SyntaxTree::compile(r###"char c = 'a' y "###), Err(ParseError::TokenNotPunct));
	assert_eq!(SyntaxTree::compile(r###"char c = 'a' = "###), Err(ParseError::expecting_but(&[",", ";"], "=")));
	assert_eq!(SyntaxTree::compile(r###"char c "###), Err(ParseError::EndOfToken));
	assert_eq!(SyntaxTree::compile(r###"int i = 'c';"###), Err(ParseError::TypeMismatch));
	assert_eq!(SyntaxTree::compile(r###"int i = "int";"###), Err(ParseError::TypeMismatch));
}

#[test]
fn test_t1() {}
