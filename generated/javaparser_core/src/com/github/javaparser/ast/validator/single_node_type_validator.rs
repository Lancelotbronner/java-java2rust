use crate::com::github::javaparser::ast::Node;

pub struct SingleNodeTypeValidator<N: com::github::javaparser::ast::node::Node> {
	type: /* Java */ java::lang::Class /**/,
	validator: com::github::javaparser::ast::validator::typed_validator::TypedValidator,
}

impl<N: com::github::javaparser::ast::node::Node> SingleNodeTypeValidator {
	pub fn new(type: &/* Java */ java::lang::Class /**/, validator: &com::github::javaparser::ast::validator::typed_validator::TypedValidator) -> com::github::javaparser::ast::validator::single_node_type_validator::SingleNodeTypeValidator {
		self.type = type;
		self.validator = validator;
	}

	pub fn accept(&self, node: &com::github::javaparser::ast::node::Node, problem_reporter: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		if self.type.isInstance(node) {
			self.validator.accept(&self.type.cast(node), problem_reporter);
		}
		node.find_all(self.type).forEach(|n|self.validator.accept(n, problem_reporter));
	}
}

impl<N: com::github::javaparser::ast::node::Node> com::github::javaparser::ast::validator::validator::Validator for SingleNodeTypeValidator<N> {}

impl<N: com::github::javaparser::ast::node::Node> com::github::javaparser::ast::validator::typed_validator::TypedValidator for SingleNodeTypeValidator<N> {}

impl<N: com::github::javaparser::ast::node::Node> /* Java */ java::util::function::BiConsumer /**/ for SingleNodeTypeValidator<N> {}