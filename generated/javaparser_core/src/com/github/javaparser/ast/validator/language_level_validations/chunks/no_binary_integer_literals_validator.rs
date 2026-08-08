use crate::com::github::javaparser::ast::expr::IntegerLiteralExpr;
use crate::com::github::javaparser::ast::expr::LiteralStringValueExpr;
use crate::com::github::javaparser::ast::expr::LongLiteralExpr;
use crate::com::github::javaparser::ast::validator::ProblemReporter;
use crate::com::github::javaparser::ast::validator::VisitorValidator;

pub struct NoBinaryIntegerLiteralsValidator;

impl NoBinaryIntegerLiteralsValidator {
	pub fn visit(&self, n: &com::github::javaparser::ast::expr::integer_literal_expr::IntegerLiteralExpr, arg: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		com::github::javaparser::ast::validator::language_level_validations::chunks::no_binary_integer_literals_validator::NoBinaryIntegerLiteralsValidator::validate(n, arg);
		super.visit(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::long_literal_expr::LongLiteralExpr, arg: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		com::github::javaparser::ast::validator::language_level_validations::chunks::no_binary_integer_literals_validator::NoBinaryIntegerLiteralsValidator::validate(n, arg);
		super.visit(n, arg);
	}

	fn validate(&self, n: &com::github::javaparser::ast::expr::literal_string_value_expr::LiteralStringValueExpr, arg: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		if n.get_value().toUpperCase().startsWith("0B") {
			arg.report(n, "Binary literal values are not supported.");
		}
	}
}

impl com::github::javaparser::ast::visitor::void_visitor::VoidVisitor for NoBinaryIntegerLiteralsValidator {}

impl com::github::javaparser::ast::validator::validator::Validator for NoBinaryIntegerLiteralsValidator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for NoBinaryIntegerLiteralsValidator {}

impl /* Java */ java::util::function::BiConsumer /**/ for NoBinaryIntegerLiteralsValidator {}