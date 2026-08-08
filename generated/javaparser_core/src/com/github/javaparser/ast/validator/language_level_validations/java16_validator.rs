pub struct Java16Validator;

impl Java16Validator {
	pub fn new() /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::validator::language_level_validations::java16_validator::Java16Validator {
		super();
		// Released Language Features
		// Pattern Matching for instanceof released within Java 16 - https://openjdk.java.net/jeps/305
		self.remove()?;
		{
			// Records released within Java 16 - https://openjdk.java.net/jeps/395
			self.remove()?;
			// local interface released within Java 16 -
			// https://docs.oracle.com/javase/specs/jls/se16/html/jls-14.html#jls-14.3
			self.remove()?;
			self.add();
			self.add();
		}
	}
}

impl com::github::javaparser::ast::validator::validator::Validator for Java16Validator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for Java16Validator {}

impl /* Java */ java::util::function::BiConsumer /**/ for Java16Validator {}