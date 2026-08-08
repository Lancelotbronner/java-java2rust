use crate::com::github::javaparser::ParseResult;
use crate::com::github::javaparser::ParserConfiguration;
use crate::com::github::javaparser::Processor;
use crate::com::github::javaparser::ast::Node;
use java::util::ArrayList;
use java::util::Arrays;
use java::util::List;

pub struct PostProcessors {
	post_processors: /* Java */ java::util::List /**/ = ArrayList<>::new(),
}

impl PostProcessors {
	pub fn new(post_processors: &com::github::javaparser::processor::Processor) -> com::github::javaparser::ast::validator::postprocessors::post_processors::PostProcessors {
		self.postProcessors.addAll(&Arrays::asList(post_processors));
	}

	pub fn get_post_processors(&self) -> /* Java */ java::util::List /**/ {
		return self.post_processors;
	}

	pub fn remove(&self, post_processor: &com::github::javaparser::processor::Processor) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::validator::postprocessors::post_processors::PostProcessors {
		if !self.post_processors.remove(post_processor) {
			return Err(AssertionError::new("Trying to remove a post processor that isn't there."));
		}
		return self;
	}

	pub fn replace(&self, old_processor: &com::github::javaparser::processor::Processor, new_processor: &com::github::javaparser::processor::Processor) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::validator::postprocessors::post_processors::PostProcessors {
		self.remove(old_processor)?;
		self.add(new_processor);
		return self;
	}

	pub fn add(&self, new_processor: &com::github::javaparser::processor::Processor) -> com::github::javaparser::ast::validator::postprocessors::post_processors::PostProcessors {
		self.post_processors.add(new_processor);
		return self;
	}

	pub fn post_process(&self, result: &com::github::javaparser::parse_result::ParseResult, configuration: &com::github::javaparser::parser_configuration::ParserConfiguration) {
		self.post_processors.forEach(|pp|pp.post_process(result, configuration));
	}
}