use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::expr::Name;
use crate::com::github::javaparser::ast::expr::SimpleName;
use crate::com::github::javaparser::ast::validator::ProblemReporter;
use crate::com::github::javaparser::ast::validator::VisitorValidator;

pub struct UnderscoreKeywordValidator;

impl UnderscoreKeywordValidator {
	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name::Name, arg: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		com::github::javaparser::ast::validator::language_level_validations::chunks::underscore_keyword_validator::UnderscoreKeywordValidator::validate_identifier(n, &n.get_identifier(), arg);
		super.visit(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::simple_name::SimpleName, arg: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		com::github::javaparser::ast::validator::language_level_validations::chunks::underscore_keyword_validator::UnderscoreKeywordValidator::validate_identifier(n, &n.get_identifier(), arg);
		super.visit(n, arg);
	}

	fn validate_identifier(&self, n: &com::github::javaparser::ast::node::Node, id: &/* Java */ java::lang::String /**/, arg: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		if "_".equals(id) {
			arg.report(n, "'_' is a reserved keyword.");
		}
	}
}

impl com::github::javaparser::ast::visitor::void_visitor::VoidVisitor for UnderscoreKeywordValidator {}

impl com::github::javaparser::ast::validator::validator::Validator for UnderscoreKeywordValidator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for UnderscoreKeywordValidator {}

impl /* Java */ java::util::function::BiConsumer /**/ for UnderscoreKeywordValidator {}