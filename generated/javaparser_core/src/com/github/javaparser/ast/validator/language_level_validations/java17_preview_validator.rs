pub struct Java17PreviewValidator;

impl Java17PreviewValidator {
	pub fn new() -> com::github::javaparser::ast::validator::language_level_validations::java17_preview_validator::Java17PreviewValidator {
		super();
	// Incubator
	// No new incubator language features added in Java 17
	// Preview
	// No new preview language features added in Java 17
	// 2nd Preview
	// No new 2nd preview language features added in Java 17
	}
}

impl com::github::javaparser::ast::validator::validator::Validator for Java17PreviewValidator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for Java17PreviewValidator {}

impl /* Java */ java::util::function::BiConsumer /**/ for Java17PreviewValidator {}