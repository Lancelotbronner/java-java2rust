use crate::com::github::javaparser::utils::CodeGenerationUtils::f;
use crate::com::github::javaparser::ast::expr::Name;
use crate::com::github::javaparser::ast::expr::SimpleName;

pub struct ReservedKeywordValidator {
	keyword: /* Java */ java::lang::String /**/,
	error: /* Java */ java::lang::String /**/,
}

impl ReservedKeywordValidator {
	pub fn new(keyword: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::validator::reserved_keyword_validator::ReservedKeywordValidator {
		self.keyword = keyword;
		self.error = com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("'%s' cannot be used as an identifier as it is a keyword.", keyword);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name::Name, arg: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		if n.get_identifier().equals(self.keyword) {
			arg.report(n, self.error);
		}
		super.visit(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::simple_name::SimpleName, arg: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		if n.get_identifier().equals(self.keyword) {
			arg.report(n, self.error);
		}
		super.visit(n, arg);
	}
}

impl com::github::javaparser::ast::visitor::void_visitor::VoidVisitor for ReservedKeywordValidator {}

impl com::github::javaparser::ast::validator::validator::Validator for ReservedKeywordValidator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for ReservedKeywordValidator {}

impl /* Java */ java::util::function::BiConsumer /**/ for ReservedKeywordValidator {}