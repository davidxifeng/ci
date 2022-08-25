use std::collections::VecDeque;

use console::style;

use crate::compile::token::{Token, Const};

use super::{parse::calc, token::Punct};

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
				ExprTree::Leaf(v) => fe(&v, &d),
				ExprTree::Branch(Branch { op, left, right }) => {
					fb(op, &d);
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
				ExprTree::Leaf(v) => fe(&v, &d),
				ExprTree::Branch(Branch { op, left, right }) => {
					i(left, fb, fe, &(d + 1));
					fb(op, &d);
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
				ExprTree::Leaf(v) => fe(&v, &d),
				ExprTree::Branch(Branch { op, left, right }) => {
					po(left, fb, fe, &(d + 1));
					po(right, fb, fe, &(d + 1));
					fb(op, &d);
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
				ExprTree::Leaf(v) => stack.push(Token::Const(Const::Integer(*v as i128))),
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
				Token::Const(Const::Integer(v)) => stack.push_back(*v as i64),
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
			Self::Branch(Branch { op, left, right }) => calc(&op, left.eval(), right.eval()),
		}
	}
}

impl std::fmt::Display for ExprTree {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if f.alternate() {
			let mut s = String::new();
			let mut queue = VecDeque::from([self]);
			while !queue.is_empty() {
				let mut lc = queue.len();
				while lc > 0 {
					match queue.pop_front().unwrap() {
						ExprTree::Leaf(v) => {
							s.push_str(format!(" {}", v).as_str());
						}
						ExprTree::Branch(Branch { op, left, right }) => {
							s.push_str(format!("{}", op).as_str());
							queue.push_back(left);
							queue.push_back(right);
						}
					}
					lc -= 1;
				}
				s.push('\n');
			}
			f.write_str(s.as_str())
		} else {
			fn pr(this: &ExprTree, s: &mut String, p: &str, cp: &str) {
				match this {
					ExprTree::Leaf(v) => {
						s.push_str(format!("{}", style(p).dim()).as_str());
						s.push_str(format!("{}\n", style(v.to_string().as_str()).green()).as_str());
					}
					ExprTree::Branch(Branch { op, left, right }) => {
						s.push_str(format!("{}", style(p).dim()).as_str());
						s.push_str(format!("{}\n", style(op.to_string().as_str()).bold().blue()).as_str());

						pr(left, s, (cp.to_owned() + "├───").as_str(), &(cp.to_owned() + "│   "));
						pr(right, s, (cp.to_owned() + "└───").as_str(), &(cp.to_owned() + "    "));
					}
				}
			}
			let mut s = String::new();
			pr(self, &mut s, "", "");
			f.write_str(s.as_str())
		}
	}
}

#[test]
fn test_expr_tree() {
	let tree = ExprTree::tree(Punct::Add, ExprTree::branch(Punct::Mul, 1, 2), ExprTree::leaf(3));
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
