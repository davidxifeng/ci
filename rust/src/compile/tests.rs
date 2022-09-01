use std::{iter::Peekable, slice::Iter};

use crate::compile::{
	errors::*,
	parse::*,
	token::{Const, Keyword, Punct, Token},
	types::*,
};

struct V<'a, VT> {
	iter: &'a mut Peekable<Iter<'a, VT>>,
	cur: Option<Option<&'a VT>>,
}

#[test]
#[ignore]
fn test_nc() {
	let v = vec![Token::Punct(Punct::Add), Token::Punct(Punct::Sub), Token::Punct(Punct::Mul)];
	let mut vi = v.iter().peekable();
	let mut v = V { iter: &mut vi, cur: None };
	println!("{:?}", v.get_peek());
	println!("{:?}", v.get_next());
	println!("{:?}", v.curr());
	println!("{:?}", v.peek_curr());
}

impl<VT> V<'_, VT> {
	fn peek_curr(&mut self) -> Option<&VT> {
		match self.cur {
			None => {
				let v = self.iter.next();
				self.cur = Some(v);
				v
			}
			Some(v) => v,
		}
	}

	fn curr(&mut self) -> Option<&VT> {
		match self.cur {
			None => None,
			Some(v) => v,
		}
	}

	fn get_next(&mut self) -> Option<&VT> {
		let v = self.iter.next();
		self.cur = Some(v);
		v
	}

	fn get_peek(&mut self) -> Option<&&VT> {
		self.iter.peek()
	}
}

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
#[ignore]
fn test_expr_tree_display() {
	let expr = Expr::AssignExpr(AssignExpr {
		assign: Punct::Assign,
		left: Box::new(Expr::Id("demo".into())),
		right: Box::new(Expr::BinOp(BinOp {
			left: Box::new(Expr::Const(Const::Integer("123".into()))),
			op: Punct::Add,
			right: Box::new(Expr::Const(Const::Integer("456".into()))),
		})),
	});

	let expr2 = Expr::CondExpr(CondExpr {
		cond: Box::new(Expr::BinOp(BinOp {
			left: Box::new(Expr::Id("value".into())),
			op: Punct::Eq,
			right: Box::new(Expr::Const(Const::Integer("21".into()))),
		})),
		left: Box::new(Expr::CommaExpr(CommaExpr {
			expr: vec![
				Expr::Id("v1".into()),
				Expr::Id("v2".into()),
				Expr::BinOp(BinOp {
					left: Box::new(Expr::Const(Const::Integer("123".into()))),
					op: Punct::Add,
					right: Box::new(Expr::Const(Const::Integer("456".into()))),
				}),
				Expr::Id("ok".into()),
			],
		})),
		right: Box::new(Expr::BinOp(BinOp {
			left: Box::new(Expr::Const(Const::Integer("123".into()))),
			op: Punct::Add,
			right: Box::new(Expr::Const(Const::Integer("456".into()))),
		})),
	});

	let tree: DeclarationList = vec![Declaration::Function(FunctionDefinition {
		ctype: CType::BaseType(Keyword::Int),
		name: "id".into(),
		params: vec![
			Parameter { ctype: CType::BaseType(Keyword::Char), name: "c".into() },
			Parameter { ctype: CType::BaseType(Keyword::Int), name: "i".into() },
		],
		stmts: vec![Statement::ExprStmt(expr), Statement::ExprStmt(expr2)],
	})]
	.into();
	println!("{}", tree)
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

#[test]
#[ignore]
fn test_variable_declaration() {
	compile_test("int i = 2, j = 1, k; char c = 'c', d;", false, None);
}

#[test]
fn test_func_declaration() {
	compile_test("int id(char c,int i) { i = 1; return 'a'; }", true, None);
}
