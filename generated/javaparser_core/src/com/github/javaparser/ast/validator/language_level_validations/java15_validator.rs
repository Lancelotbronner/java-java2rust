pub struct Java15Validator;

impl Java15Validator {
	pub fn new() /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::validator::language_level_validations::java15_validator::Java15Validator {
		super();
		// Released Language Features
		// Text Block Literals - released within Java 15 - https://openjdk.java.net/jeps/378
		self.remove()?;
	}
}

impl com::github::javaparser::ast::validator::validator::Validator for Java15Validator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for Java15Validator {}

impl /* Java */ java::util::function::BiConsumer /**/ for Java15Validator {}