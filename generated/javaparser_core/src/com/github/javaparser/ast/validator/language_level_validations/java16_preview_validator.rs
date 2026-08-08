pub struct Java16PreviewValidator;

impl Java16PreviewValidator {
	pub fn new() -> com::github::javaparser::ast::validator::language_level_validations::java16_preview_validator::Java16PreviewValidator {
		super();
	// Incubator
	// No new incubator language features added in Java 16
	// Preview
	// No new preview language features added in Java 16
	// 2nd Preview
	// TODO: remove(noSealedClasses); // Sealed Classes - 2nd preview in Java 16 - https://openjdk.java.net/jeps/397
	}
}

impl com::github::javaparser::ast::validator::validator::Validator for Java16PreviewValidator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for Java16PreviewValidator {}

impl /* Java */ java::util::function::BiConsumer /**/ for Java16PreviewValidator {}