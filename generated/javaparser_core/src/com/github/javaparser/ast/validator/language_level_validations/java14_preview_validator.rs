pub struct Java14PreviewValidator;

impl Java14PreviewValidator {
	pub fn new() /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::validator::language_level_validations::java14_preview_validator::Java14PreviewValidator {
		super();
		// Incubator
		// No new incubator language features added within Java 14
		// Preview
		// Pattern Matching for instanceof - first preview within Java 14 - https://openjdk.java.net/jeps/305
		self.remove()?;
		{
			// first preview within Java 14 - https://openjdk.java.net/jeps/359
			self.remove()?;
			self.add();
			self.add();
		}
		// 2nd Preview
		// Text Block Literals - 2nd preview within Java 14 - https://openjdk.java.net/jeps/378
		self.remove()?;
	}
}

impl com::github::javaparser::ast::validator::validator::Validator for Java14PreviewValidator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for Java14PreviewValidator {}

impl /* Java */ java::util::function::BiConsumer /**/ for Java14PreviewValidator {}