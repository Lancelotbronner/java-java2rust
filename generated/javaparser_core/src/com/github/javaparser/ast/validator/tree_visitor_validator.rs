use crate::com::github::javaparser::ast::Node;

pub struct TreeVisitorValidator {
	validator: com::github::javaparser::ast::validator::validator::Validator,
}

impl TreeVisitorValidator {
	pub fn new(validator: &com::github::javaparser::ast::validator::validator::Validator) -> com::github::javaparser::ast::validator::tree_visitor_validator::TreeVisitorValidator {
		self.validator = validator;
	}

	pub fn accept(&self, node: &com::github::javaparser::ast::node::Node, reporter: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		self.validator.accept(node, reporter);
		for child in node.get_child_nodes() {
			self.accept(child, reporter);
		}
	}
}

impl com::github::javaparser::ast::validator::validator::Validator for TreeVisitorValidator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for TreeVisitorValidator {}

impl /* Java */ java::util::function::BiConsumer /**/ for TreeVisitorValidator {}