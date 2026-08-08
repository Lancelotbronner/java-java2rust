use crate::com::github::javaparser::ast::validator::ReservedKeywordValidator;
use crate::com::github::javaparser::ast::validator::Validator;
use crate::com::github::javaparser::ast::validator::language_level_validations::chunks::ModifierValidator;

pub struct Java1_2Validator {
	modifiers_without_default_and_static_interface_methods_and_private_interface_methods: com::github::javaparser::ast::validator::validator::Validator = ModifierValidator::new(true, false, false),
	strictfp_not_allowed: com::github::javaparser::ast::validator::validator::Validator = ReservedKeywordValidator::new("strictfp"),
}

impl Java1_2Validator {
	pub fn new() -> com::github::javaparser::ast::validator::language_level_validations::java1_2_validator::Java1_2Validator {
		super();
		self.replace(, self.modifiers_without_default_and_static_interface_methods_and_private_interface_methods);
		self.add(self.strictfp_not_allowed);
	}
}

impl com::github::javaparser::ast::validator::validator::Validator for Java1_2Validator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for Java1_2Validator {}

impl /* Java */ java::util::function::BiConsumer /**/ for Java1_2Validator {}