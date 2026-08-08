pub struct Java10PreviewValidator;

impl Java10PreviewValidator {
	pub fn new() -> com::github::javaparser::ast::validator::language_level_validations::java10_preview_validator::Java10PreviewValidator {
		super();
	// Incubator
	// No incubator language features added within Java 10
	// Preview
	// No preview language features added within Java 10
	// 2nd Preview
	// No 2nd preview language features added within Java 10
	}
}

impl com::github::javaparser::ast::validator::validator::Validator for Java10PreviewValidator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for Java10PreviewValidator {}

impl /* Java */ java::util::function::BiConsumer /**/ for Java10PreviewValidator {}