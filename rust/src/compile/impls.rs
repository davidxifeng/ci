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

#[derive(PartialEq, Eq)]
pub enum NodePos {
	Init,
	Top,
	Middle,
	Bottom,
}

impl NodePos {
	fn is_init(&self) -> bool {
		*self == NodePos::Init
	}
	fn is_top(&self) -> bool {
		*self == NodePos::Top
	}
	fn is_middle(&self) -> bool {
		*self == NodePos::Middle
	}
	fn is_bottom(&self) -> bool {
		*self == NodePos::Bottom
	}
}

fn print_leaf(s: &mut String, p: &str, pos: &NodePos, v: impl Display) {
	if p.is_empty() {
		s.push_str(&style(v.to_string().as_str()).green().to_string());
	} else {
		let pf = if pos.is_top() {
			"┌───"
		} else {
			if pos.is_middle() {
				"├───"
			} else if pos.is_bottom() {
				"└───"
			} else {
				"────"
			}
		};

		s.push_str(&style(p).dim().to_string());
		s.push_str(&style(pf).dim().to_string());
		s.push_str(&style(v.to_string().as_str()).green().to_string());
		s.push('\n');
	}
}

fn print_binary_node(s: &mut String, prev: &str, pos: &NodePos, v: impl Display, left: &Expr, right: &Expr) {
	let prefix_str = if pos.is_init() || pos.is_top() {
		"    "
	} else {
		"│   "
		// if pos.is_top() {
		// 	"    "
		// } else if pos.is_middle() {
		// 	"│   "
		// } else if pos.is_bottom() {
		// 	"│   "
		// } else {
		// 	"│   "
		// }
	};
	print_expr_tree(left, s, &(prev.to_owned() + prefix_str), &NodePos::Top);

	let op_prefix = if prev.is_empty() {
		"────"
	} else if pos.is_top() {
		"┌───"
	} else if pos.is_middle() {
		"├───"
	} else {
		"└───"
	};

	s.push_str(&style(prev).dim().to_string());
	s.push_str(&style(op_prefix).dim().to_string());
	s.push_str(&style(v.to_string().as_str()).bold().blue().to_string());
	s.push('\n');

	print_expr_tree(
		right,
		s,
		&(prev.to_owned() + if pos.is_top() || pos.is_middle() { "│   " } else { "    " }),
		&NodePos::Bottom,
	);
}

fn print_multi_node(s: &mut String, prev: &str, top: &NodePos, v: impl Display, expr: &Vec<Expr>) {
	if !expr.is_empty() {
		let prefix_str = if top.is_top() || prev.is_empty() { "    " } else { "│   " };
		print_expr_tree(&expr[0], s, &(prev.to_owned() + prefix_str), &NodePos::Top);

		let op_prefix = if prev.is_empty() {
			"────"
		} else if top.is_top() {
			"┌───"
		} else if top.is_middle() {
			"├───"
		} else {
			"└───"
		};
		s.push_str(&style(prev).dim().to_string());
		s.push_str(&style(op_prefix).dim().to_string());
		s.push_str(&style(v.to_string().as_str()).bold().blue().to_string());
		s.push('\n');
	}

	let mut iter = expr.iter().skip(1).peekable();
	while let Some(e) = iter.next() {
		let is_last = iter.peek().is_none();
		print_expr_tree(
			e,
			s,
			&(prev.to_owned() + if top.is_top() || top.is_middle() { "│   " } else { "    " }),
			if is_last { &NodePos::Bottom } else { &NodePos::Middle },
		);
	}
}

fn print_expr_tree(this: &Expr, s: &mut String, prev: &str, pos: &NodePos) {
	match this {
		Expr::Const(v) => {
			print_leaf(s, prev, &pos, v);
		}
		Expr::StringLiteral(v) => {
			print_leaf(s, prev, &pos, v);
		}
		Expr::Id(v) => {
			print_leaf(s, prev, &pos, v);
		}

		Expr::BinOp(BinOp { op, left, right }) => {
			print_binary_node(s, prev, pos, op, left, right);
		}
		Expr::AssignExpr(AssignExpr { left, assign, right }) => {
			print_binary_node(s, prev, pos, assign, left, right);
		}
		Expr::CommaExpr(CommaExpr { expr }) => print_multi_node(s, prev, pos, ",", expr),
		Expr::CondExpr(CondExpr { cond, left, right }) => {
			let prefix_str = if pos.is_top() || prev.is_empty() { "    " } else { "│   " };
			print_expr_tree(&cond, s, &(prev.to_owned() + prefix_str), &NodePos::Top);

			let op_prefix = if prev.is_empty() {
				"────"
			} else if pos.is_top() {
				"┌───"
			} else if pos.is_middle() {
				"├───"
			} else {
				"└───"
			};

			s.push_str(&style(prev).dim().to_string());
			s.push_str(&style(op_prefix).dim().to_string());
			s.push_str(&style("? :").bold().blue().to_string());
			s.push('\n');

			print_expr_tree(
				&left,
				s,
				&(prev.to_owned() + if pos.is_top() || pos.is_middle() { "│   " } else { "    " }),
				&NodePos::Middle,
			);
			print_expr_tree(
				&right,
				s,
				&(prev.to_owned() + if pos.is_top() || pos.is_middle() { "│   " } else { "    " }),
				&NodePos::Bottom,
			);
		}
		Expr::SimplePostfix(PostfixOP { op, expr }) => {
			let op_prefix = if prev.is_empty() {
				"────"
			} else if pos.is_top() {
				"┌───"
			} else if pos.is_middle() {
				"├───"
			} else {
				"└───"
			};

			s.push_str(&style(prev).dim().to_string());
			s.push_str(&style(op_prefix).dim().to_string());
			s.push_str(&style(op.to_string().as_str()).bold().blue().to_string());
			s.push('\n');

			print_expr_tree(expr, s, &(prev.to_owned() + if pos.is_top() { "│   " } else { "    " }), &NodePos::Middle);
		}
		Expr::UnaryOp(UnaryOp { op, expr }) => {
			let op_prefix = if pos.is_init() {
				"────"
			} else if pos.is_top() {
				"┌───"
			} else if pos.is_middle() {
				"├───"
			} else {
				"└───"
			};

			s.push_str(&style(prev).dim().to_string());
			s.push_str(&style(op_prefix).dim().to_string());
			s.push_str(&style(op.to_string().as_str()).bold().blue().to_string());
			s.push('\n');

			print_expr_tree(
				&expr,
				s,
				&(prev.to_owned() + if pos.is_top() { "│   " } else { "    " }),
				&NodePos::Bottom,
			);
		}
	}
}

impl Display for Expr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if f.alternate() {
			f.write_str("expr")
		} else {
			let mut s = String::new();
			print_expr_tree(self, &mut s, "", &NodePos::Init);
			f.write_str(s.as_str())
		}
	}
}
