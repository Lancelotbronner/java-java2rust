pub struct Java25Validator;

impl Java25Validator {
	pub fn new() /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::validator::language_level_validations::java25_validator::Java25Validator {
		super();
		self.remove()?;
		self.remove()?;
	}
}

impl com::github::javaparser::ast::validator::validator::Validator for Java25Validator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for Java25Validator {}

impl /* Java */ java::util::function::BiConsumer /**/ for Java25Validator {}