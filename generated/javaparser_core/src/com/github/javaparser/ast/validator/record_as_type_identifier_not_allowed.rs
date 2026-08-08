use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::body::TypeDeclaration;
use crate::com::github::javaparser::ast::expr::Name;
use crate::com::github::javaparser::ast::expr::SimpleName;

pub struct RecordAsTypeIdentifierNotAllowed {
	error: /* Java */ java::lang::String /**/,
}

impl RecordAsTypeIdentifierNotAllowed {
	pub fn new() -> com::github::javaparser::ast::validator::record_as_type_identifier_not_allowed::RecordAsTypeIdentifierNotAllowed {
		self.error = "'record' is a restricted identifier and cannot be used for type declarations";
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name::Name, arg: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		if "record".equals(&n.get_identifier()) && !self.valid_usage(n) {
			arg.report(n, self.error);
		}
		super.visit(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::simple_name::SimpleName, arg: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		if "record".equals(&n.get_identifier()) && !self.valid_usage(n) {
			arg.report(n, self.error);
		}
		super.visit(n, arg);
	}

	fn valid_usage(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if !node.get_parent_node().isPresent() {
			return true;
		}
		let parent: Node = node.get_parent_node().get();
		return !(parent instanceof TypeDeclaration);
	}
}

impl com::github::javaparser::ast::visitor::void_visitor::VoidVisitor for RecordAsTypeIdentifierNotAllowed {}

impl com::github::javaparser::ast::validator::validator::Validator for RecordAsTypeIdentifierNotAllowed {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for RecordAsTypeIdentifierNotAllowed {}

impl /* Java */ java::util::function::BiConsumer /**/ for RecordAsTypeIdentifierNotAllowed {}