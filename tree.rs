struct TreeNode<'a> {
	value: i64,
	parent: Option<&'a TreeNode>,
	child: Vec<Option<&TreeNode>>
}

impl TreeNode {
	fn new(value: i64) -> Node {
		TreeNode {
			value,
			parent: None,
			child: Vec::new()
		}
	}
	fn new_child(value: i64, parent: &TreeNode) {
		parent.push(
			TreeNode {
				value,
				parent: Some(&TreeNode),
				child: Vec::new()
			}
		);
	}
}

fn main() {
	
}
