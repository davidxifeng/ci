use crate::lex::*;

use super::parse::calc;

#[derive(Debug)]
struct ExprTreeNode<T> {
	value: T,
	// parent: Weak<Node<T>>,
	children: Vec<Box<ExprTreeNode<T>>>,
}

impl<T> ExprTreeNode<T>
where
	T: std::fmt::Debug,
{
	fn new(v: T, left: T, right: T) -> Self {
		ExprTreeNode {
			value: v,
			children: vec![
				Box::new(ExprTreeNode { value: left, children: vec![] }),
				Box::new(ExprTreeNode { value: right, children: vec![] }),
			],
		}
	}

	fn branch_atom_tree(v: T, left: T, right: Self) -> Self {
		ExprTreeNode {
			value: v,
			children: vec![Box::new(ExprTreeNode { value: left, children: vec![] }), Box::new(right)],
		}
	}

	fn branch_tree_atom(v: T, left: Self, right: T) -> Self {
		ExprTreeNode {
			value: v,
			children: vec![Box::new(left), Box::new(ExprTreeNode { value: right, children: vec![] })],
		}
	}

	fn print_pre_order(&self) {
		self.print_pre_order_(0);
	}

	fn print_pre_order_(&self, depth: usize) {
		println!("{: >2$}{:?}", "", self.value, depth * 2);
		for v in &self.children {
			v.as_ref().print_pre_order_(depth + 1);
		}
	}

	fn print_in_order(&self) {
		self.print_in_order_(0);
	}

	fn print_in_order_(&self, depth: usize) {
		if self.children.len() > 0 {
			self.children[0].as_ref().print_in_order_(depth + 1);
		}
		println!("{: >2$}{:?}", "", self.value, depth * 2);
		if self.children.len() > 1 {
			self.children[1].as_ref().print_in_order_(depth + 1);
		}
	}

	fn print_post_order(&self) {
		self.print_post_order_(0);
	}

	fn print_post_order_(&self, depth: usize) {
		for v in &self.children {
			v.as_ref().print_post_order_(depth + 1);
		}
		println!("{: >2$}{:?}", "", self.value, depth * 2);
	}
}

fn eval_tree(this: &ExprTreeNode<Token>) -> i64 {
	match this.value {
		Token::Const(Const::Integer(v)) => v as i64,
		Token::Punct(punct) => {
			let left = &this.children[0];
			let right = &this.children[1];
			calc(&punct, eval_tree(left), eval_tree(right))
		}
		_ => unreachable!(),
	}
}

#[test]
fn test_tree() {
	// 1 + 2 + 3
	let tree = ExprTreeNode::branch_tree_atom(
		Token::new_punct(Punct::Add),
		ExprTreeNode::new(Token::new_punct(Punct::Add), Token::new_const_i(1), Token::new_const_i(2)),
		Token::new_const_i(3),
	);

	tree.print_pre_order();

	println!("------");
	tree.print_in_order();

	println!("------");
	tree.print_post_order();

	println!("\neval tree: {}\n---\n", eval_tree(&tree));

	// 1 + 2 * 3
	let tree = ExprTreeNode::branch_atom_tree(
		Token::new_punct(Punct::Add),
		Token::new_const_i(1),
		ExprTreeNode::new(Token::new_punct(Punct::Mul), Token::new_const_i(2), Token::new_const_i(3)),
	);
	println!("\neval tree: {}\n---\n", eval_tree(&tree));

	// (1 + 2) * 3
	let tree = ExprTreeNode::branch_tree_atom(
		Token::new_punct(Punct::Mul),
		ExprTreeNode::new(Token::new_punct(Punct::Add), Token::new_const_i(1), Token::new_const_i(2)),
		Token::new_const_i(3),
	);
	println!("\neval tree: {}\n---\n", eval_tree(&tree));
}

enum ExprTree {
	Branch(Branch),
	Leaf(i64),
}

struct Branch {
	op: Punct,
	left: Box<ExprTree>,
	right: Box<ExprTree>,
}

impl ExprTree {
	fn tree(op: Punct, left: ExprTree, right: ExprTree) -> Self {
		Self::Branch(Branch { op, left: Box::new(left), right: Box::new(right) })
	}
	fn branch(op: Punct, lhs: i64, rhs: i64) -> Self {
		Self::Branch(Branch { op, left: Box::new(Self::Leaf(lhs)), right: Box::new(Self::Leaf(rhs)) })
	}
	fn leaf(v: i64) -> Self {
		Self::Leaf(v)
	}

	fn do_print(&self, level: usize) {
		match self {
			Self::Leaf(v) => println!("{: >2$}{}", "", v, level * 2),
			Self::Branch(Branch { op, left, right }) => {
				left.do_print(level + 1);
				right.do_print(level + 1);
				println!("{:>2$}{}", "", format!("{}", op), level * 2);
			}
		}
	}
	fn print(&self) {
		self.do_print(0);
	}

	fn eval(&self) -> i64 {
		match self {
			Self::Leaf(v) => *v,
			Self::Branch(Branch { op, left, right }) => calc(&op, left.eval(), right.eval()),
		}
	}
}

#[test]
fn test_expr_tree() {
	let tree = ExprTree::tree(Punct::Add, ExprTree::branch(Punct::Add, 1, 2), ExprTree::leaf(3));
	tree.print();
	println!("eval tree: {}", tree.eval());
}

#[test]
fn test_fmt() {
	let op = Punct::Xor;
	// println!("this is: {:>5}", op, 4 + 1);
	println!("{:>1$}", "^", 2 + 1);
	println!("{:>1$}", op, 2 + 1);
	println!("{:>1$}", format!("{}", op), 2 + 1);
}
