use crate::lex::*;

#[derive(Debug)]

struct SimpleTree<T> {
	root: ExprNode<T>,
}

#[derive(Debug)]
struct ExprNode<T> {
	value: T,
	// parent: Weak<Node<T>>,
	children: Vec<Box<ExprNode<T>>>,
}

impl<T> ExprNode<T>
where
	T: std::fmt::Debug,
{
	fn print_pre_root(&self) {
		self.print_pre_root_(0);
	}

	fn print_all(&self) {
		let depth = 0;
		self.print_all_(depth);
	}

	fn print_all_(&self, depth: usize) {
		println!("{: >2$}{:?}", "", self.value, depth * 2);
		for v in &self.children {
			v.as_ref().print_all_(depth + 1);
		}
	}

	fn print_pre_root_(&self, depth: usize) {
		println!("{: >2$}{:?}", "", self.value, depth * 2);

		if self.children.len() > 0 {
			self.children[0].as_ref().print_pre_root_(depth + 1);
		}
		if self.children.len() > 1 {
			self.children[1].as_ref().print_pre_root_(depth + 1);
		}
	}

	fn print_middle_root(&self) {
		self.print_middle_root_(0);
	}

	fn print_middle_root_(&self, depth: usize) {
		if self.children.len() > 0 {
			self.children[0].as_ref().print_middle_root_(depth + 1);
		}

		println!("{: >2$}{:?}", "", self.value, depth * 2);
		if self.children.len() > 1 {
			self.children[1].as_ref().print_middle_root_(depth + 1);
		}
	}
}

impl<T> SimpleTree<T>
where
	T: std::fmt::Debug,
{
	fn tree(v: T, left: T, right: T) -> Self {
		SimpleTree {
			root: ExprNode {
				value: v,
				children: vec![
					Box::new(ExprNode { value: left, children: vec![] }),
					Box::new(ExprNode { value: right, children: vec![] }),
				],
			},
		}
	}

	fn branch_atom_tree(v: T, left: T, right: Self) -> Self {
		SimpleTree {
			root: ExprNode {
				value: v,
				children: vec![Box::new(ExprNode { value: left, children: vec![] }), Box::new(right.root)],
			},
		}
	}

	fn branch_tree_atom(v: T, left: Self, right: T) -> Self {
		SimpleTree {
			root: ExprNode {
				value: v,
				children: vec![Box::new(left.root), Box::new(ExprNode { value: right, children: vec![] })],
			},
		}
	}
}

#[test]
fn test_tree() {
	let branch = SimpleTree::tree(Token::new_punct(Punct::Add), Token::new_const_i(1), Token::new_const_i(2));
	let tree = SimpleTree::branch_tree_atom(Token::new_punct(Punct::Add), branch, Token::Const(Const::Integer(3)));
	// tree.root.print_middle_root();

	tree.root.print_pre_root();
	println!("------");

	tree.root.print_middle_root();

	// tree.root.print_all();
}
