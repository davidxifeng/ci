#[cfg(test)]
use super::*;

#[test]
fn test_t0() {
	assert_eq!(
		SyntaxTree::compile("char ; int ;"),
		Ok(vec![
			Declaration::Variable { ci_type: (CiType::CiChar), list: vec![] },
			Declaration::Variable { ci_type: (CiType::CiInt), list: vec![] },
		])
	);
	assert_eq!(
		SyntaxTree::compile("char a = 'A', b, c = 'C'; int i = 1;"),
		Ok(vec![
			Declaration::Variable {
				ci_type: (CiType::CiChar),
				list: vec![
					Declarator { name: "a".into(), value: CiValue::CiChar('A') },
					Declarator { name: "b".into(), value: CiValue::CiChar('\0') },
					Declarator { name: "c".into(), value: CiValue::CiChar('C') },
				]
			},
			Declaration::Variable {
				ci_type: (CiType::CiInt),
				list: vec![Declarator { name: "i".into(), value: CiValue::CiInt(1) }]
			},
		])
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
