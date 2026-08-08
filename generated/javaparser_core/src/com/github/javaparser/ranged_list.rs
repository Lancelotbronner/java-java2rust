use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;

struct RangedList<T: com::github::javaparser::ast::node::Node> {
	range: com::github::javaparser::token_range::TokenRange = TokenRange::new(JavaToken::com::github::javaparser::java_token::JavaToken::INVALID, JavaToken::com::github::javaparser::java_token::JavaToken::INVALID),
	list: com::github::javaparser::ast::node_list::NodeList,
}

impl<T: com::github::javaparser::ast::node::Node> RangedList {
	fn new(list: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ranged_list::RangedList {
		self.list = list;
	}

	fn begin_at(&mut self, begin: &com::github::javaparser::java_token::JavaToken) /* thrown(java.lang.AssertionError) */ {
		self.range = self.range.with_begin(begin)?;
	}

	fn end_at(&mut self, end: &com::github::javaparser::java_token::JavaToken) /* thrown(java.lang.AssertionError) */ {
		self.range = self.range.with_end(end)?;
	}

	fn add(&mut self, t: &T) {
		if self.list == null {
			self.list = NodeList<>::new();
		}
		self.list.add(t);
	}
}