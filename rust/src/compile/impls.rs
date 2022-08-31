use std::fmt::Display;

use console::style;

use super::{
	token::{Const, Keyword},
	types::*,
};

impl From<Vec<Declaration>> for DeclarationList {
	fn from(l: Vec<Declaration>) -> Self {
		DeclarationList { list: (l) }
	}
}

impl Display for DeclarationList {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for v in &self.list {
			write!(f, "{}", v)?;
		}
		Ok(())
	}
}

impl Display for CType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			Self::BaseType(kw) => match kw {
				Keyword::Char => "char",
				Keyword::Int => "int",
				_ => "<error>",
			},
		})
	}
}

impl Display for Declarator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.value {
			Const::Empty => write!(f, "{}", self.name),
			_ => write!(f, "{} = {}", self.name, self.value),
		}
	}
}

impl Display for Declaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Variable(v) => {
				write!(f, "{}", v.ctype)?;

				if !v.list.is_empty() {
					write!(f, " {}", v.list[0])?;
				}
				for v in v.list.iter().skip(1) {
					f.write_str(", ")?;
					write!(f, "{}", v)?;
				}
				writeln!(f, ";")
			}
			Self::Function(func) => write!(f, "{}", func),
		}
	}
}

impl Display for Parameter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} {}", self.ctype, self.name)
	}
}

impl Display for FunctionDefinition {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} {} (", self.ctype, self.name)?;
		if !self.params.is_empty() {
			write!(f, "{}", self.params[0])?;
		}
		for param in self.params.iter().skip(1) {
			write!(f, ", {}", param)?;
		}
		writeln!(f, ");")?;
		for stmt in self.stmts.iter() {
			write!(f, "{}", stmt)?;
		}
		writeln!(f)
	}
}

impl Display for Statement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Empty => writeln!(f, ";"),
			Self::ExprStmt(expr) => writeln!(f, "{};", expr),
			Self::ReturnStmt(expr) => writeln!(f, "return\n{};", expr),
		}
	}
}

fn print_leaf(s: &mut String, p: &str, is_right: &bool, v: impl Display) {
	if p.is_empty() {
		s.push_str(&style(v.to_string().as_str()).green().to_string());
	} else {
		let pf = if *is_right { "┌───" } else { "└───" };

		s.push_str(&style(p).dim().to_string());
		s.push_str(&style(pf).dim().to_string());
		s.push_str(&style(v.to_string().as_str()).green().to_string());
		s.push('\n');
	}
}

fn print_binary_node(s: &mut String, prev: &str, is_left: &bool, v: impl Display, left: &Expr, right: &Expr) {
	let prefix_str = if *is_left || prev.is_empty() { "    " } else { "│   " };
	print_expr_tree(left, s, &(prev.to_owned() + prefix_str), &true);

	let op_prefix = if prev.is_empty() {
		"────"
	} else if *is_left {
		"┌───"
	} else {
		"└───"
	};

	s.push_str(&style(prev).dim().to_string());
	s.push_str(&style(op_prefix).dim().to_string());
	s.push_str(&style(v.to_string().as_str()).bold().blue().to_string());
	s.push('\n');

	print_expr_tree(right, s, &(prev.to_owned() + if *is_left { "│   " } else { "    " }), &false);
}

fn print_multi_node(s: &mut String, prev: &str, is_left: &bool, v: impl Display, expr: &Vec<Expr>) {
	if !expr.is_empty() {
		let prefix_str = if *is_left || prev.is_empty() { "    " } else { "│   " };
		print_expr_tree(&expr[0], s, &(prev.to_owned() + prefix_str), &true);

		let op_prefix = if prev.is_empty() {
			"────"
		} else if *is_left {
			"┌───"
		} else {
			"└───"
		};
		s.push_str(&style(prev).dim().to_string());
		s.push_str(&style(op_prefix).dim().to_string());
		s.push_str(&style(v.to_string().as_str()).bold().blue().to_string());
		s.push('\n');
	}

	for e in expr.iter().skip(1) {
		print_expr_tree(e, s, &(prev.to_owned() + if *is_left { "│   " } else { "    " }), &false);
	}
}

fn print_expr_tree(this: &Expr, s: &mut String, prev: &str, is_top: &bool) {
	match this {
		Expr::Const(v) => {
			print_leaf(s, prev, &is_top, v);
		}
		Expr::StringLiteral(v) => {
			print_leaf(s, prev, &is_top, v);
		}
		Expr::Id(v) => {
			print_leaf(s, prev, &is_top, v);
		}

		Expr::BinOp(BinOp { op, left, right }) => {
			print_binary_node(s, prev, is_top, op, left, right);
		}
		Expr::AssignExpr(AssignExpr { left, assign, right }) => {
			print_binary_node(s, prev, is_top, assign, left, right);
		}
		Expr::CommaExpr(CommaExpr { expr }) => print_multi_node(s, prev, is_top, ",", expr),
		Expr::CondExpr(CondExpr { cond, left, right }) => {
			let prefix_str = if *is_top || prev.is_empty() { "    " } else { "│   " };
			print_expr_tree(&cond, s, &(prev.to_owned() + prefix_str), &true);

			let op_prefix = if prev.is_empty() {
				"────"
			} else if *is_top {
				"┌───"
			} else {
				"└───"
			};

			s.push_str(&style(prev).dim().to_string());
			s.push_str(&style(op_prefix).dim().to_string());
			s.push_str(&style("? :").bold().blue().to_string());
			s.push('\n');

			print_expr_tree(&left, s, &(prev.to_owned() + if *is_top { "│   " } else { "    " }), &false);
			print_expr_tree(&right, s, &(prev.to_owned() + if *is_top { "│   " } else { "    " }), &false);
		}
		Expr::SimplePostfix(PostfixOP { op, expr }) => {
			let op_prefix = if prev.is_empty() {
				"────"
			} else if *is_top {
				"┌───"
			} else {
				"└───"
			};

			s.push_str(&style(prev).dim().to_string());
			s.push_str(&style(op_prefix).dim().to_string());
			s.push_str(&style(op.to_string().as_str()).bold().blue().to_string());
			s.push('\n');

			print_expr_tree(expr, s, &(prev.to_owned() + if *is_top { "│   " } else { "    " }), &false);
		}
		Expr::UnaryOp(UnaryOp { op, expr }) => {
			let op_prefix = if prev.is_empty() {
				"────"
			} else if *is_top {
				"┌───"
			} else {
				"└───"
			};

			s.push_str(&style(prev).dim().to_string());
			s.push_str(&style(op_prefix).dim().to_string());
			s.push_str(&style(op.to_string().as_str()).bold().blue().to_string());
			s.push('\n');

			print_expr_tree(&expr, s, &(prev.to_owned() + if *is_top { "│   " } else { "    " }), &false);
		}
	}
}

impl Display for Expr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if f.alternate() {
			f.write_str("expr")
		} else {
			let mut s = String::new();
			print_expr_tree(self, &mut s, "", &false);
			f.write_str(s.as_str())
		}
	}
}
