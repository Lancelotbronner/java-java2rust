use crate::com::github::javaparser::ast::Node;

pub struct Processor;

impl Processor {
	pub fn post_process(&self, result: &com::github::javaparser::parse_result::ParseResult, configuration: &com::github::javaparser::parser_configuration::ParserConfiguration) {
	}

	pub fn pre_process(&self, inner_provider: &com::github::javaparser::provider::Provider) -> com::github::javaparser::provider::Provider {
		return inner_provider;
	}
}