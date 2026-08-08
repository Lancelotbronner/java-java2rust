use crate::com::github::javaparser::utils::CodeGenerationUtils::f;
use crate::com::github::javaparser::Problem;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithTokenRange;
use crate::com::github::javaparser::ast::validator::language_level_validations::UpgradeJavaMessage;
use java::util::function::Consumer;

pub struct ProblemReporter {
	problem_consumer: /* Java */ java::util::function::Consumer /**/,
}

impl ProblemReporter {
	pub fn new(problem_consumer: &/* Java */ java::util::function::Consumer /**/) -> com::github::javaparser::ast::validator::problem_reporter::ProblemReporter {
		self.problemConsumer = problem_consumer;
	}

	pub fn report(&self, node: &com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange, message: &com::github::javaparser::ast::validator::language_level_validations::upgrade_java_message::UpgradeJavaMessage, args: &/* Java */ java::lang::Object /**/) {
		self.report(&node.get_token_range().orElse(null), &message.to_string(), args);
	}

	pub fn report(&self, node: &com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange, message: &/* Java */ java::lang::String /**/, args: &/* Java */ java::lang::Object /**/) {
		self.report(&node.get_token_range().orElse(null), message, args);
	}

	pub fn report(&self, range: &com::github::javaparser::token_range::TokenRange, message: &/* Java */ java::lang::String /**/, args: &/* Java */ java::lang::Object /**/) {
		self.problem_consumer.accept(Problem::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f(message, args), range, null));
	}
}