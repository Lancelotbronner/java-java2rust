use crate::com::github::javaparser::ast::type::VarType;
use crate::com::github::javaparser::ast::validator::SingleNodeTypeValidator;
use crate::com::github::javaparser::ast::validator::Validator;
use crate::com::github::javaparser::ast::validator::language_level_validations::chunks::VarValidator;

pub struct Java10Validator {
	var_only_on_local_variable_definition_and_for_and_try: com::github::javaparser::ast::validator::validator::Validator = SingleNodeTypeValidator<>::new(VarType.class, VarValidator::new(false)),
}

impl Java10Validator {
	pub fn new() -> com::github::javaparser::ast::validator::language_level_validations::java10_validator::Java10Validator {
		super();
		// Released Language Features
		{
			/* 
	             * Java 10 released local variable type inference in for and try-with (JEP286).
	             * Java 11 released local variable type inference for lambda parameters also (JEP323)
	             */ 
			self.add(self.var_only_on_local_variable_definition_and_for_and_try);
		}
	}
}

impl com::github::javaparser::ast::validator::validator::Validator for Java10Validator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for Java10Validator {}

impl /* Java */ java::util::function::BiConsumer /**/ for Java10Validator {}