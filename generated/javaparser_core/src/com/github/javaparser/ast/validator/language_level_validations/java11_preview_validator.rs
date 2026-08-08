pub struct Java11PreviewValidator;

impl Java11PreviewValidator {
	pub fn new() -> com::github::javaparser::ast::validator::language_level_validations::java11_preview_validator::Java11PreviewValidator {
		super();
	// Incubator
	// No incubator language features added within Java 11
	// Preview
	// No preview language features added within Java 11
	// 2nd Preview
	// No 2nd preview language features added within Java 11
	}
}

impl com::github::javaparser::ast::validator::validator::Validator for Java11PreviewValidator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for Java11PreviewValidator {}

impl /* Java */ java::util::function::BiConsumer /**/ for Java11PreviewValidator {}