use crate::com::github::javaparser::ast::Node;
use java::util::ArrayList;
use java::util::Arrays;
use java::util::List;

pub struct Validators {
	validators: /* Java */ java::util::List /**/ = ArrayList<>::new(),
}

impl Validators {
	pub fn new(validators: &com::github::javaparser::ast::validator::validator::Validator) -> com::github::javaparser::ast::validator::validators::Validators {
		self.validators.addAll(&Arrays::asList(validators));
	}

	pub fn get_validators(&self) -> /* Java */ java::util::List /**/ {
		return self.validators;
	}

	pub fn remove(&self, validator: &com::github::javaparser::ast::validator::validator::Validator) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::validator::validators::Validators {
		if !self.validators.remove(validator) {
			return Err(AssertionError::new("Trying to remove a validator that isn't there."));
		}
		return self;
	}

	pub fn replace(&self, old_validator: &com::github::javaparser::ast::validator::validator::Validator, new_validator: &com::github::javaparser::ast::validator::validator::Validator) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::validator::validators::Validators {
		self.remove(old_validator)?;
		self.add(new_validator);
		return self;
	}

	pub fn add(&self, new_validator: &com::github::javaparser::ast::validator::validator::Validator) -> com::github::javaparser::ast::validator::validators::Validators {
		self.validators.add(new_validator);
		return self;
	}

	pub fn accept(&self, node: &com::github::javaparser::ast::node::Node, problem_reporter: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		self.validators.forEach(|v|v.accept(node, problem_reporter));
	}
}

impl com::github::javaparser::ast::validator::validator::Validator for Validators {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for Validators {}

impl /* Java */ java::util::function::BiConsumer /**/ for Validators {}