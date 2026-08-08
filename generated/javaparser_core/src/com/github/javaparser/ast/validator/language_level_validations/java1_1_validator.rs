use crate::com::github::javaparser::ParserConfiguration;
use crate::com::github::javaparser::ast::body::ClassOrInterfaceDeclaration;
use crate::com::github::javaparser::ast::stmt::LocalClassDeclarationStmt;
use crate::com::github::javaparser::ast::validator::SingleNodeTypeValidator;
use crate::com::github::javaparser::ast::validator::Validator;

pub struct Java1_1Validator {
	inner_classes: com::github::javaparser::ast::validator::validator::Validator = SingleNodeTypeValidator<>::new(ClassOrInterfaceDeclaration.class, |(n, reporter)|n.getParentNode().ifPresent(|p|{
	if p instanceof LocalClassDeclarationStmt && n.isInterface() {
		reporter.report(n, UpgradeJavaMessage::new("There is no such thing as a local interface.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_16));
	}

})),
}

impl Java1_1Validator {
	pub fn new() /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::validator::language_level_validations::java1_1_validator::Java1_1Validator {
		super();
		self.replace(, self.inner_classes);
		self.remove()?;
	}
}

impl com::github::javaparser::ast::validator::validator::Validator for Java1_1Validator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for Java1_1Validator {}

impl /* Java */ java::util::function::BiConsumer /**/ for Java1_1Validator {}