pub struct Java15PreviewValidator;

impl Java15PreviewValidator {
	pub fn new() /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::validator::language_level_validations::java15_preview_validator::Java15PreviewValidator {
		super();
		// Incubator
		// No new incubator language features added within Java 15
		// Preview
		// remove(noSealedClasses); // Sealed Classes - first preview within Java 15 - https://openjdk.java.net/jeps/360
		// 2nd Preview
		// Pattern Matching for instanceof - 2nd preview in Java 15 - https://openjdk.java.net/jeps/305
		self.remove()?;
		{
			// Records - 2nd preview within Java 15 - https://openjdk.java.net/jeps/384
			self.remove()?;
			self.add();
			self.add();
		}
	}
}

impl com::github::javaparser::ast::validator::validator::Validator for Java15PreviewValidator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for Java15PreviewValidator {}

impl /* Java */ java::util::function::BiConsumer /**/ for Java15PreviewValidator {}