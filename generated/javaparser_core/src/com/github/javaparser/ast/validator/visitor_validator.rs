use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::visitor::VoidVisitorAdapter;

pub struct VisitorValidator;

impl VisitorValidator {
	pub fn accept(&self, node: &com::github::javaparser::ast::node::Node, problem_reporter: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		node.accept(self, problem_reporter);
	}
}

impl com::github::javaparser::ast::validator::validator::Validator for VisitorValidator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for VisitorValidator {}

impl /* Java */ java::util::function::BiConsumer /**/ for VisitorValidator {}

impl com::github::javaparser::ast::visitor::void_visitor::VoidVisitor for VisitorValidator {}