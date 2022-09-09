use crate::compile::parse::*;

#[test]
fn test_types() {
	test_declspec("int");
}

fn test_declspec(input: &str) {
	println!("{}\n", input);
	match Parser::from_str(input).and_then(|mut x| x.declspec()) {
		Ok(Some(expr)) => println!("------\n{}", expr),
		Ok(None) => println!("none"),
		Err(e) => println!("\t[error]\n{}", e),
	}
}

fn test_expr(input: &str) {
	println!("\t[ok]\n{}", input);
	match Parser::from_str(input).and_then(|mut x| x.parse()) {
		Ok(Some(expr)) => println!("------\n{}", expr),
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
