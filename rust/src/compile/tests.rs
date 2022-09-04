use crate::compile::{
	errors::*,
	parse::*,
	token::{Const, Keyword, Punct},
	types::*,
};

#[test]
#[ignore]
fn parse_basic_test() {
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
					Declarator { name: "a".into(), value: Const::Character('A') },
					Declarator { name: "b".into(), value: Const::Empty },
					Declarator { name: "c".into(), value: Const::Character('C') },
				]
			}),
			Declaration::Variable(VariableDeclaration {
				ctype: CType::BaseType(Keyword::Int),
				list: vec![Declarator { name: "i".into(), value: Const::Integer("1".to_string()) }]
			}),
		]
		.into())
	);
	assert_eq!(compile(r###"char 2 = 'a'"###), Err(ParseError::NotIdentifier));
	assert_eq!(compile(r###"char a = 'a', 2 = 'c'"###), Err(ParseError::NotIdentifier));

	assert_eq!(compile(r###"char c = 'a'"###), Err(ParseError::EndOfToken));

	assert_eq!(compile(r###"char c = 'a' y "###), Err(ParseError::NotPunct));
	assert_eq!(
		compile(r###"char c = 'a' = "###),
		Err(ParseError::expecting_but(&mut [Punct::Comma.to_string(), Punct::Semicolon.to_string()], "="))
	);
	assert_eq!(compile(r###"char c "###), Err(ParseError::EndOfToken));
	assert_eq!(compile(r###"int i = 'c';"###), Err(ParseError::TypeMismatch));
	assert_eq!(compile(r###"int i = "int";"###), Err(ParseError::TypeMismatch));
}

fn compile_test(input: &str, print: bool, expected: Option<DeclarationList>) {
	println!("{}\n------", input);
	let r = compile(input);
	match r {
		Ok(d) => {
			if let Some(expected) = expected {
				assert_eq!(d, expected)
			}
			if print {
				println!("{}", d)
			}
		}
		Err(e) => println!("compile error: {}", e),
	}
}

fn test_expr(input: &str) {
	match parse_expr_test(input) {
		Ok(el) => {
			println!("\t[ok]\n{}", input);
			for (i, e) in el.iter().enumerate() {
				println!("------\n  {}:\n{}", i, e);
			}
		}
		Err(e) => println!("\t[error]\n{}", e),
	}
}

#[test]
#[ignore]
fn test_variable_declaration() {
	compile_test("int i = 2, j = 1, k; char c = 'c', d;", false, None);
}

#[test]
fn test_expr_parse() {
	// compile_test("int id(char c,int i) { i = 1; return 'a'; }", true, None);
	test_expr("");
	test_expr("i");
	test_expr("i = 1");
	test_expr("(i) = 1");
	test_expr("(i) = (1)");
	test_expr("i = 1 + 2 + 3");
	test_expr("i = 1 + 2 * 3 + 4 * 5");
	test_expr("i = (1 + 2) * 3 + 4 * 5");
	test_expr("2, 3, i = 1 + 2, c = 3");
	test_expr("a ? t : f ");
	test_expr("a ? t + 1: f + 2");
	test_expr("a ? t : f = 2 ");
	test_expr("a ? t : (f = 2) ");
	test_expr("a ? t ? x : y : (f = 2) ");
	// test_expr("i = 1; j = 1 + 2;");
}
