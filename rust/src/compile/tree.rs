use std::collections::VecDeque;

use crate::lex::*;

use super::parse::calc;

enum ExprTree {
	Branch(Branch),
	Leaf(i64),
}

struct Branch {
	op: Punct,
	left: Box<ExprTree>,
	right: Box<ExprTree>,
}

enum VisitOrder {
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
		let mut op_s = vec![];
		let mut v_s = vec![];
		self.visit(
			&VisitOrder::Post,
			&mut |p, _| {
				op_s.push(*p);
			},
			&mut |p, _| {
				v_s.push(*p);
			},
		);
		for op in op_s.iter().rev() {
			let lhs = v_s.pop().unwrap();
			let rhs = v_s.pop().unwrap();
			v_s.push(calc(op, lhs, rhs));
		}
		v_s.pop().unwrap()
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
	}
}

#[test]
fn test_expr_tree() {
	let tree = ExprTree::tree(Punct::Add, ExprTree::branch(Punct::Mul, 1, 2), ExprTree::leaf(3));
	tree.print(&VisitOrder::Pre);
	println!("---");
	tree.print(&VisitOrder::In);
	println!("---");
	tree.print(&VisitOrder::Post);
	println!("---");
	println!("eval tree: {}", tree.eval());
	println!("eval tree with stack: {}", tree.eval_stack());
	println!("tree is\n{}", tree);
}

#[test]
#[ignore]
fn test_fmt() {
	assert_eq!(format!("{:>1$}", "^", 2), " ^");
	assert_eq!(format!("{:>1$}", Punct::Xor, 2), " ^");
	assert_eq!(format!("{}", Punct::Xor), "^");
}
