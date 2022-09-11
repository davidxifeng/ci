use crate::compile::{
	parse::*,
	types::{Object, Type},
};

use super::errors::{LexError, ParseError};

fn test_declaration(input: &str) {
	println!("\n------");
	match Parser::from_str(input).and_then(|mut p| {
		let r = p.declaration();
		p.show_parse_state(0);
		r
	}) {
		Ok(r) => println!("{}: {}", r.name.unwrap_or_default(), r.ctype),
		Err(e) => println!("\t[error]\t{}", e),
	}
}

#[test]
#[ignore = "done"]
fn test_types() {
	test_declaration("int i, j");
	test_declaration("int *i");
	test_declaration("int **i");
	test_declaration("int i[1]");
	test_declaration("int i[1][2]");
	test_declaration("int i[]");
	test_declaration("int *i[8]");
	test_declaration("int **i[8][2]");
	test_declaration("int i(void)");
	test_declaration("int *i(void)");
	test_declaration("int *i[2](void)");
	test_declaration("int i(int i, int j)");

	// int i(int , int (*)(void))
	// declare i as function (int, pointer to function (void) returning int) returning int
	test_declaration("int i(int i, int j(void))");

	// declare i as function (pointer to function (pointer to function (void) returning int) returning int) returning int
	test_declaration("int i(int (*)(int (*)(void)))");

	test_declaration("int (*x())()");
	test_declaration("int (*x[])()");
	test_declaration("int *x[][]");
	test_declaration("int x[1][2][3]");

	test_declaration("void (*signal(int, void (*)(int)))(int);");
}

fn test_expr(input: &str) {
	println!("------\n{}", input);
	match Parser::from_str(input).and_then(|mut x| {
		let r = x.parse_expr(crate::compile::token::Precedence::P1Comma);
		x.show_parse_state(0);
		r
	}) {
		Ok(Some(expr)) => println!("{}", expr),
		Ok(None) => println!("none"),
		Err(e) => println!("\t[error]\n{}", e),
	}
}

#[test]
#[ignore = "done"]
fn test_expr_parse() {
	test_expr("");
	test_expr("i");
	test_expr("i = 1");
	test_expr("(i) = 1");
	test_expr("(i) = (1)");
	test_expr("i = 1 + 2 + 3");
	test_expr("i = 1 + 2 * 3 + 4 * 5");
	test_expr("i = (1 + 2) * 3 + 4 * 5");
	test_expr("i = j = k = 1 + 2 * 3 || 1 + 2");
	test_expr("2, 3, i = 1 + 2, c = 3");
	test_expr("a ? t : f ");
	test_expr("a ? t + 1: f + 2");
	test_expr("a ? t : f = 2 ");
	test_expr("a ? t : (f = 2) ");
	test_expr("a ? t ? x : y : (f = 2) ");
	test_expr("1 + 2 == 3 || 1 + 2 > 3");
	test_expr("a ? c : a >= 1 && b <= 2 || c && d");
	test_expr("++i");
	test_expr("-1 + ++i");
	test_expr("sizeof i");
	test_expr("- - ! ~ 1  ");
	test_expr("i++ + ++i");
	test_expr("i[1]");
	test_expr("i[1 + 2]");
	test_expr("i[1 + 2] + 2");
	test_expr("i[a.b + c.d] + 2");
	test_expr("x &= i[a.b + c.d] + 2");
	test_expr("y >>= i[a.b + c.d] + 2");
	test_expr("&x + *p + sizeof c.d.e, y >>= i[a.b + c->d] + 2, c ? 1 + 2 : 2 + 5");
	test_expr("f()");
	test_expr("f(a,b,c)");
	test_expr("t.f(a,b,c = 2)");
	test_expr("s->t.f(a,(b ? 1 : 2),c) + 2");

	// example from N1256, 说明f1-f4的求值顺序可以任意,标准没有规定;
	// 所有副作用在函数调用前完成
	test_expr("(*pf[f1()]) (f2(), f3() + f4())");
}

fn parse_error(input: &str, ee: ParseError) {
	assert!(match Parser::from_str(input).and_then(|mut p| p.parse()) {
		Ok(_) => false,
		Err(e) => match e {
			ParseError::General(_) => matches!(ee, ParseError::General(_)),
			ParseError::LexError(_) => matches!(ee, ParseError::LexError(_)),
			ParseError::Unexpected(_) => matches!(ee, ParseError::Unexpected(_)),
			_ => e == ee,
		},
	})
}

#[test]
#[ignore]
fn test_parse_error() {
	parse_error("int ; ", ParseError::General(""));
	parse_error("int ; @", ParseError::LexError(LexError::InvalidChar('@')));
}

fn parse(input: &str) {
	println!("\n------");
	match Parser::from_str(input).and_then(|mut p| {
		let r = p.parse();
		p.show_parse_state(0);
		r
	}) {
		Ok(r) => {
			for obj in r {
				match obj {
					Object::Variable(var) => {
						println!("{}: {}", var.name, var.ctype);
						var.init_value.map(|e| println!(" = \n{}", e));
					}
					Object::Function(func) => {
						println!("name: {}\t\ttype: {}", func.name, Type::Func(func.ctype));
					}
				}
			}
		}

		Err(e) => println!("\t[error]\t{}", e),
	}
}

#[test]
fn test_parse() {
	parse("int i; int j , k = i + 3, l = 2; char c, d;");
	parse("int id(int arg) { return arg; }");
	parse("int add_one(int arg) { arg = arg + 1; return arg; }");
}
