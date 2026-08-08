use crate::com::github::javaparser::ast::Node;
use java::util::ArrayList;
use java::util::LinkedList;
use java::util::Queue;

pub struct TreeVisitor;

impl TreeVisitor {
	pub fn visit_leaves_first(&self, node: &com::github::javaparser::ast::node::Node) {
		for child in node.get_child_nodes() {
			self.visit_leaves_first(child);
		}
		self.process(node);
	}

	pub fn visit_pre_order(&self, node: &com::github::javaparser::ast::node::Node) {
		self.process(node);
		ArrayList<>::new(&node.get_child_nodes()).forEach(self::visitPreOrder);
	}

	pub fn visit_post_order(&self, node: &com::github::javaparser::ast::node::Node) {
		ArrayList<>::new(&node.get_child_nodes()).forEach(self::visitPostOrder);
		self.process(node);
	}

	pub fn visit_breadth_first(&self, node: &com::github::javaparser::ast::node::Node) {
		/* final */ let queue: Queue<Node> = LinkedList<>::new();
		queue.offer(node);
		while queue.size() > 0 {
			/* final */ let head: Node = queue.peek();
			for child in head.get_child_nodes() {
				queue.offer(child);
			}
			self.process(&queue.poll());
		}
	}

	pub fn process(&self, node: &com::github::javaparser::ast::node::Node) ;

	pub fn visit_direct_children(&self, node: &com::github::javaparser::ast::node::Node) {
		ArrayList<>::new(&node.get_child_nodes()).forEach(self::process);
	}
}