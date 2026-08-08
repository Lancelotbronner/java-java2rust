use crate::com::github::javaparser::ast::body::ClassOrInterfaceDeclaration;
use crate::com::github::javaparser::ast::validator::SingleNodeTypeValidator;
use crate::com::github::javaparser::ast::validator::Validator;
use crate::com::github::javaparser::ast::validator::language_level_validations::chunks::ModifierValidator;

pub struct Java8Validator {
	modifiers_without_private_interface_methods: com::github::javaparser::ast::validator::validator::Validator = ModifierValidator::new(true, true, false),
	default_methods_in_interface: com::github::javaparser::ast::validator::validator::Validator = SingleNodeTypeValidator<>::new(ClassOrInterfaceDeclaration.class, |(n, reporter)|{
	if n.isInterface() {
		n.getMethods().forEach(|m|{
			if m.isDefault() && !m.getBody().isPresent() {
				reporter.report(m, "'default' methods must have a body.");
			}
		});
	}
}),
}

impl Java8Validator {
	pub fn new() /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::validator::language_level_validations::java8_validator::Java8Validator {
		super();
		self.replace(, self.modifiers_without_private_interface_methods);
		self.add(self.default_methods_in_interface);
		self.remove()?;
	// TODO validate more annotation locations http://openjdk.java.net/jeps/104
	// TODO validate repeating annotations http://openjdk.java.net/jeps/120
	}
}

impl com::github::javaparser::ast::validator::validator::Validator for Java8Validator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for Java8Validator {}

impl /* Java */ java::util::function::BiConsumer /**/ for Java8Validator {}