pub struct Java21Validator;

impl Java21Validator {
	pub fn new() /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::validator::language_level_validations::java21_validator::Java21Validator {
		super();
		self.remove()?;
		self.remove()?;
		self.remove()?;
	}
}

impl com::github::javaparser::ast::validator::validator::Validator for Java21Validator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for Java21Validator {}

impl /* Java */ java::util::function::BiConsumer /**/ for Java21Validator {}