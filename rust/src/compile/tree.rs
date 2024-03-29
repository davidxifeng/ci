use std::collections::VecDeque;

use console::style;

use crate::compile::token::{Const, Token};

use super::{
	errors::ParseError,
	token::{Punct, TokenList},
};

fn calc(op: &Punct, a: i64, b: i64) -> i64 {
	match op {
		Punct::Add => a + b,
		Punct::Sub => a - b,
		Punct::Mul => a * b,
		Punct::Div => a / b,
		Punct::Xor => pow(a, b),
		_ => 0,
	}
}

fn pow(a: i64, b: i64) -> i64 {
	let mut r = 1;
	for _ in 0..b {
		r *= a
	}
	r
}

fn op_info(op: &Punct) -> i8 {
	match op {
		Punct::Add => 1,
		Punct::Sub => 1,
		Punct::Mul => 2,
		Punct::Div => 2,
		Punct::Xor => 3,
		_ => panic!("bad input"),
	}
}

pub enum ExprTree {
	Branch(Branch),
	Leaf(i64),
}

pub struct Branch {
	pub op: Punct,
	pub left: Box<ExprTree>,
	pub right: Box<ExprTree>,
}

pub enum VisitOrder {
	Pre,
	In,
	Post,
}

impl ExprTree {
	pub fn tree(op: Punct, left: ExprTree, right: ExprTree) -> Self {
		Self::Branch(Branch { op, left: Box::new(left), right: Box::new(right) })
	}
	pub fn branch(op: Punct, lhs: i64, rhs: i64) -> Self {
		Self::Branch(Branch { op, left: Box::new(Self::Leaf(lhs)), right: Box::new(Self::Leaf(rhs)) })
	}
	pub fn leaf(v: i64) -> Self {
		Self::Leaf(v)
	}

	#[cfg(test)]
	pub fn print_by_level(&self) {
		// 广度优先遍历 + 按层次分组.
		let mut s = String::new();
		let mut queue = VecDeque::from([self]);
		while !queue.is_empty() {
			let mut lc = queue.len();
			while lc > 0 {
				match queue.pop_front().unwrap() {
					ExprTree::Leaf(v) => {
						s.push_str(format!("{} ", v).as_str());
					}
					ExprTree::Branch(Branch { op, left, right }) => {
						s.push_str(format!("{} ", op).as_str());
						queue.push_back(left);
						queue.push_back(right);
					}
				}
				lc -= 1;
			}
			s.pop();
			s.push('\n');
		}
		println!("{}", s);
	}

	pub fn print(&self, order: &VisitOrder) {
		self.visit(
			order,
			&mut |v, depth| {
				println!("{: >2$}{}", "", v, depth * 2);
			},
			&mut |op, depth| {
				println!("{: >2$}{}", "", op, depth * 2);
			},
		)
	}

	pub fn visit<F1, F2>(&self, order: &VisitOrder, fb: &mut F1, fe: &mut F2)
	where
		F1: FnMut(&Punct, &usize),
		F2: FnMut(&i64, &usize),
	{
		fn pr<F1, F2>(this: &ExprTree, fb: &mut F1, fe: &mut F2, d: &usize)
		where
			F1: FnMut(&Punct, &usize),
			F2: FnMut(&i64, &usize),
		{
			match this {
				ExprTree::Leaf(v) => fe(v, d),
				ExprTree::Branch(Branch { op, left, right }) => {
					fb(op, d);
					pr(left, fb, fe, &(d + 1));
					pr(right, fb, fe, &(d + 1));
				}
			}
		}
		fn i<F1, F2>(this: &ExprTree, fb: &mut F1, fe: &mut F2, d: &usize)
		where
			F1: FnMut(&Punct, &usize),
			F2: FnMut(&i64, &usize),
		{
			match this {
				ExprTree::Leaf(v) => fe(v, d),
				ExprTree::Branch(Branch { op, left, right }) => {
					i(left, fb, fe, &(d + 1));
					fb(op, d);
					i(right, fb, fe, &(d + 1));
				}
			}
		}
		fn po<F1, F2>(this: &ExprTree, fb: &mut F1, fe: &mut F2, d: &usize)
		where
			F1: FnMut(&Punct, &usize),
			F2: FnMut(&i64, &usize),
		{
			match this {
				ExprTree::Leaf(v) => fe(v, d),
				ExprTree::Branch(Branch { op, left, right }) => {
					po(left, fb, fe, &(d + 1));
					po(right, fb, fe, &(d + 1));
					fb(op, d);
				}
			}
		}
		match order {
			VisitOrder::Pre => pr(self, fb, fe, &0),
			VisitOrder::In => i(self, fb, fe, &0),
			VisitOrder::Post => po(self, fb, fe, &0),
		}
	}

	pub fn eval_stack(&self) -> i64 {
		fn po(this: &ExprTree, stack: &mut Vec<Token>) {
			match this {
				ExprTree::Leaf(v) => stack.push(Token::Const(Const::Integer(v.to_string()))),
				ExprTree::Branch(Branch { op, left, right }) => {
					po(left, stack);
					po(right, stack);
					stack.push(Token::Punct(*op));
				}
			}
		}
		let mut list = vec![];
		po(self, &mut list);

		let mut stack = VecDeque::<i64>::new();
		for token in list.iter() {
			match token {
				Token::Const(Const::Integer(v)) => stack.push_back(v.parse::<i64>().unwrap()),
				Token::Punct(p) => {
					let lhs = stack.pop_back().unwrap();
					let rhs = stack.pop_back().unwrap();
					println!("calc: {} {} {}", p, lhs, rhs);
					stack.push_back(calc(p, lhs, rhs));
				}
				_ => unreachable!(),
			}
		}
		stack.pop_back().unwrap()
	}

	pub fn eval(&self) -> i64 {
		match self {
			Self::Leaf(v) => *v,
			Self::Branch(Branch { op, left, right }) => calc(op, left.eval(), right.eval()),
		}
	}
}

impl std::fmt::Display for ExprTree {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if f.alternate() {
			fn pr(this: &ExprTree, s: &mut String, p: &str, cp: &str) {
				match this {
					ExprTree::Leaf(v) => {
						s.push_str(&style(p).dim().to_string());
						s.push_str(format!("{}\n", style(v.to_string().as_str()).green()).as_str());
					}
					ExprTree::Branch(Branch { op, left, right }) => {
						s.push_str(format!("{}", style(p).dim()).as_str());
						s.push_str(format!("{}\n", style(op.to_string().as_str()).bold().blue()).as_str());

						pr(right, s, (cp.to_owned() + "├───").as_str(), &(cp.to_owned() + "│   "));
						pr(left, s, (cp.to_owned() + "└───").as_str(), &(cp.to_owned() + "    "));
					}
				}
			}

			let mut s = String::new();
			pr(self, &mut s, "", "");
			f.write_str(s.as_str())
		} else {
			// ref: https://www.techiedelight.com/c-program-print-binary-tree/
			fn pr2(this: &ExprTree, s: &mut String, prev: &str, is_right: bool) {
				match this {
					ExprTree::Leaf(v) => {
						let prefix = if is_right { "┌───" } else { "└───" };
						s.push_str(&style(prev).dim().to_string());
						s.push_str(&style(prefix).dim().to_string());
						s.push_str(&style(v.to_string().as_str()).green().to_string());
						s.push('\n');
					}
					ExprTree::Branch(Branch { op, left, right }) => {
						let prefix_str = if is_right || prev.is_empty() { "    " } else { "│   " };
						pr2(right, s, &(prev.to_owned() + prefix_str), true);

						let op_prefix = if prev.is_empty() {
							"────"
						} else if is_right {
							"┌───"
						} else {
							"└───"
						};

						s.push_str(&style(prev).dim().to_string());
						s.push_str(&style(op_prefix).dim().to_string());
						s.push_str(&style(op.to_string().as_str()).bold().blue().to_string());
						s.push('\n');

						pr2(left, s, &(prev.to_owned() + if is_right { "│   " } else { "    " }), false);
					}
				}
			}
			let mut s = String::new();
			pr2(self, &mut s, "", false);
			f.write_str(s.as_str())
		}
	}
}

#[test]
fn test_expr_tree() {
	let tree = ExprTree::tree(Punct::Add, ExprTree::branch(Punct::Mul, 1, 2), ExprTree::leaf(3));
	tree.print_by_level();
	tree.print(&VisitOrder::Pre);
	println!("─────");
	tree.print(&VisitOrder::In);
	println!("─────");
	tree.print(&VisitOrder::Post);
	println!("─────");
	println!("eval tree: {}", tree.eval());
	println!("eval tree with stack: {}", tree.eval_stack());
	println!("tree is\n{:#}", tree);
	println!("tree is\n{}", tree);
	let tree = ExprTree::tree(
		Punct::Add,
		ExprTree::branch(Punct::Mul, 1, 2),
		ExprTree::tree(Punct::Mul, ExprTree::Leaf(3), ExprTree::branch(Punct::Xor, 4, 5)),
	);
	println!("tree:\n{}eval to {}", tree, tree.eval());
}

#[test]
#[ignore]
fn test_fmt() {
	assert_eq!(format!("{:>1$}", "^", 2), " ^");
	assert_eq!(format!("{:>1$}", Punct::Xor, 2), " ^");
	assert_eq!(format!("{}", Punct::Xor), "^");
}

type EvalResultTree = Result<ExprTree, ParseError>;

fn parse_leaf<'a>(iter: &mut impl Iterator<Item = &'a Token>, cop: &mut Option<Punct>) -> EvalResultTree {
	if let Some(tk) = iter.next() {
		match tk {
			Token::Const(Const::Integer(lhs)) => Ok(ExprTree::leaf(lhs.parse::<i64>().unwrap())),
			Token::Punct(Punct::ParentheseL) => Ok(parse_expr_tree(iter, 1, cop)?),
			_ => Err(ParseError::Unexpected("".into())),
		}
	} else {
		Err(ParseError::EndOfToken)
	}
}

fn parse_expr_tree<'a>(
	iter: &mut impl Iterator<Item = &'a Token>,
	mp: i8,
	maybe_current_op: &mut Option<Punct>,
) -> EvalResultTree {
	let mut lhs = parse_leaf(iter, maybe_current_op)?;

	*maybe_current_op = match iter.next() {
		Some(Token::Punct(mop)) => Some(*mop),
		None => None,
		_ => unreachable!(),
	};

	while let Some(current_op) = *maybe_current_op {
		if Punct::ParentheseR != current_op && op_info(&current_op) >= mp {
			let next_mp = op_info(&current_op) + if current_op == Punct::Xor { 0 } else { 1 };
			lhs = ExprTree::tree(current_op, lhs, parse_expr_tree(iter, next_mp, maybe_current_op)?);
		} else {
			break;
		}
	}

	Ok(lhs)
}

pub fn build_tree(input: &str) -> EvalResultTree {
	match input.parse::<TokenList>() {
		Ok(r) => {
			let mut iter = r.data.iter();
			let mut cop = None;
			parse_expr_tree(&mut iter, 1, &mut cop)
		}
		Err(err) => Err(ParseError::LexError(err)),
	}
}

#[test]
#[ignore]
fn test_tree() {
	fn tp(i: &str) {
		match build_tree(i) {
			Ok(tree) => {
				println!("{} = {}\n{}------\n", i, tree.eval(), tree)
			}
			Err(err) => println!("err: {}", err),
		}
	}

	tp("(1 + 2) * ((3 - 5) * 2) ^ 2 + 2 * 6");
	tp("2 ^ 3 ^ 2");
	tp("2 * 2 ^ 3 ^ 2");
	tp("2 * 2 ^ 3 ^ 2 * 2 / 2 + 1 * 2 ^ 2 * 20");
	tp("1 + 2 + 3 + 4 + 5");
	tp("1 + 2 + 3");
	tp("1 ^ 2 ^ 3");
	tp("1 + 2 + 3 * 4 + 5");
	tp("1 * 2 * 3 + 4 * 5 * 6");
}
