#[derive(Debug)]
struct Tree<T> {
	root: Node<T>,
}

#[derive(Debug)]
struct Node<T> {
	value: T,
	// parent: Weak<Node<T>>,
	children: Vec<Box<Node<T>>>,
}

impl<T> Tree<T>
where
	T: std::fmt::Debug,
{
	fn new(v: T) -> Tree<T> {
		Tree {
			root: Node {
				value: v,
				// parent: Weak::new(),
				children: vec![],
			},
		}
	}

	fn print(&self) {
		println!("root: {:?}", self.root.value);
		if self.root.children.len() > 0 {
			println!("children: ");
		}
		for v in &self.root.children {
			println!("\t {:?}", v.value);
		}
	}

	fn add_child(&mut self, v: T) {
		let child = Box::new(Node { value: v, children: vec![] });
		self.root.children.push(child);
	}
}

#[test]
fn test_tree() {
	let mut tree = Tree::new(1);
	tree.add_child(1);
	tree.add_child(2);
	tree.print();
}
