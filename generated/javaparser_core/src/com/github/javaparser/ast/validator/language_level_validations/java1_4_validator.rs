pub struct Java1_4Validator;

impl Java1_4Validator {
	pub fn new() /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::validator::language_level_validations::java1_4_validator::Java1_4Validator {
		super();
		self.remove()?;
		self.add();
	}
}

impl com::github::javaparser::ast::validator::validator::Validator for Java1_4Validator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for Java1_4Validator {}

impl /* Java */ java::util::function::BiConsumer /**/ for Java1_4Validator {}