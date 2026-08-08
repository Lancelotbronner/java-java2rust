use crate::com::github::javaparser::ParserConfiguration;
use crate::com::github::javaparser::ast::body::ClassOrInterfaceDeclaration;
use crate::com::github::javaparser::ast::validator::SimpleValidator;
use crate::com::github::javaparser::ast::validator::Validator;

pub struct Java17Validator {
	sealed_not_allowed_as_identifier: com::github::javaparser::ast::validator::validator::Validator = SimpleValidator<>::new(ClassOrInterfaceDeclaration.class, |n|n.getName().getIdentifier().equals("sealed"), |(n, reporter)|reporter.report(n, UpgradeJavaMessage::new("'sealed' identifier is not authorised in this context.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_17))),
	permits_not_allowed_as_identifier: com::github::javaparser::ast::validator::validator::Validator = SimpleValidator<>::new(ClassOrInterfaceDeclaration.class, |n|n.getName().getIdentifier().equals("permits"), |(n, reporter)|reporter.report(n, UpgradeJavaMessage::new("'permits' identifier is not authorised in this context.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_17))),
}

impl Java17Validator {
	pub fn new() /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::validator::language_level_validations::java17_validator::Java17Validator {
		super();
		// Released Language Features
		// Sealed Classes - https://openjdk.java.net/jeps/409
		self.add(self.sealed_not_allowed_as_identifier);
		self.add(self.permits_not_allowed_as_identifier);
		self.remove()?;
		self.remove()?;
	}
}

impl com::github::javaparser::ast::validator::validator::Validator for Java17Validator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for Java17Validator {}

impl /* Java */ java::util::function::BiConsumer /**/ for Java17Validator {}