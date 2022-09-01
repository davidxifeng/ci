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
	fn node_prelude(&self) -> &'static str {
		match self {
			NodePos::Init => "────",
			NodePos::Top => "┌───",
			NodePos::Middle => "├───",
			NodePos::Bottom => "└───",
		}
	}

	fn is_init(&self) -> bool {
		*self == NodePos::Init
	}
	fn is_top(&self) -> bool {
		*self == NodePos::Top
	}
	fn is_middle(&self) -> bool {
		*self == NodePos::Middle
	}
}

fn print_op(s: &mut String, prev: &str, pos: &NodePos, v: impl Display) {
	s.push_str(&style(prev).dim().to_string());
	s.push_str(&style(pos.node_prelude()).dim().to_string());
	s.push_str(&style(v.to_string().as_str()).blue().to_string());
	s.push('\n');
}

fn print_leaf(s: &mut String, prev: &str, pos: &NodePos, v: impl Display) {
	// if prev.is_empty() { s.push_str(&style(v.to_string().as_str()).green().to_string()); } else {
	s.push_str(&style(prev).dim().to_string());
	s.push_str(&style(pos.node_prelude()).dim().to_string());
	s.push_str(&style(v.to_string().as_str()).green().to_string());
	s.push('\n');
	// }
}

fn print_binary_node(s: &mut String, prev: &str, pos: &NodePos, v: impl Display, left: &Expr, right: &Expr) {
	let next_prefix_top = if pos.is_init() || pos.is_top() { "    " } else { "│   " };
	print_expr_tree(left, s, &(prev.to_owned() + next_prefix_top), &NodePos::Top);

	print_op(s, prev, pos, v);

	let next_prefix = if pos.is_top() || pos.is_middle() { "│   " } else { "    " };
	print_expr_tree(right, s, &(prev.to_owned() + next_prefix), &NodePos::Bottom);
}

fn print_multi_node(s: &mut String, prev: &str, pos: &NodePos, v: impl Display, expr: &Vec<Expr>) {
	if let Some((first, elems)) = expr.split_first() {
		let prefix_str = if pos.is_top() || pos.is_init() { "    " } else { "│   " };
		print_expr_tree(first, s, &(prev.to_owned() + prefix_str), &NodePos::Top);
		print_op(s, prev, pos, v);

		if let Some((last, elems)) = elems.split_last() {
			let next_prefix = prev.to_owned() + if pos.is_top() || pos.is_middle() { "│   " } else { "    " };
			for e in elems {
				print_expr_tree(e, s, &next_prefix, &NodePos::Middle);
			}
			print_expr_tree(last, s, &next_prefix, &NodePos::Bottom);
		}
	}
}

fn print_expr_tree(this: &Expr, s: &mut String, prev: &str, pos: &NodePos) {
	match this {
		Expr::Const(v) => print_leaf(s, prev, pos, v),

		Expr::StringLiteral(v) => print_leaf(s, prev, pos, v),

		Expr::Id(v) => print_leaf(s, prev, pos, v),

		Expr::BinOp(BinOp { op, left, right }) => print_binary_node(s, prev, pos, op, left, right),

		Expr::AssignExpr(AssignExpr { left, assign, right }) => print_binary_node(s, prev, pos, assign, left, right),

		Expr::CommaExpr(CommaExpr { expr }) => print_multi_node(s, prev, pos, ",", expr),

		Expr::CondExpr(CondExpr { cond, left, right }) => {
			let prefix_str = if pos.is_top() || prev.is_empty() { "    " } else { "│   " };
			print_expr_tree(&cond, s, &(prev.to_owned() + prefix_str), &NodePos::Top);

			print_op(s, prev, pos, "? :");

			let next_prefix = prev.to_owned() + if pos.is_top() { "│   " } else { "    " };
			print_expr_tree(&left, s, &next_prefix, &NodePos::Middle);
			print_expr_tree(&right, s, &next_prefix, &NodePos::Bottom);
		}
		Expr::SimplePostfix(PostfixOP { op, expr }) => {
			print_op(s, prev, pos, op);

			let next_prefix = prev.to_owned() + if pos.is_top() { "│   " } else { "    " };
			print_expr_tree(expr, s, &next_prefix, &NodePos::Middle);
		}
		Expr::UnaryOp(UnaryOp { op, expr }) => {
			print_op(s, prev, pos, op);

			let next_prefix = prev.to_owned() + if pos.is_top() { "│   " } else { "    " };
			print_expr_tree(&expr, s, &next_prefix, &NodePos::Bottom);
		}
	}
}

impl Display for Expr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut s = String::new();
		print_expr_tree(self, &mut s, "", &NodePos::Init);
		f.write_str(s.as_str())
	}
}
