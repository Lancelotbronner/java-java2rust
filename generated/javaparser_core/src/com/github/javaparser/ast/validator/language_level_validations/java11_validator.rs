use crate::com::github::javaparser::ast::type::VarType;
use crate::com::github::javaparser::ast::validator::SingleNodeTypeValidator;
use crate::com::github::javaparser::ast::validator::Validator;
use crate::com::github::javaparser::ast::validator::language_level_validations::chunks::VarValidator;

pub struct Java11Validator {
	var_also_in_lambda_parameters: com::github::javaparser::ast::validator::validator::Validator = SingleNodeTypeValidator<>::new(VarType.class, VarValidator::new(true)),
}

impl Java11Validator {
	pub fn new() -> com::github::javaparser::ast::validator::language_level_validations::java11_validator::Java11Validator {
		super();
		{
			/* 
	             * Java 10 released local variable type inference in for and try-with (JEP286).
	             * Java 11 released local variable type inference for lambda parameters also (JEP323)
	             */ 
			self.replace(, self.var_also_in_lambda_parameters);
		}
	}
}

impl com::github::javaparser::ast::validator::validator::Validator for Java11Validator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for Java11Validator {}

impl /* Java */ java::util::function::BiConsumer /**/ for Java11Validator {}