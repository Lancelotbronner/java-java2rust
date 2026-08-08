use crate::com::github::javaparser::ast::stmt::TryStmt;
use crate::com::github::javaparser::ast::validator::SingleNodeTypeValidator;
use crate::com::github::javaparser::ast::validator::Validator;
use crate::com::github::javaparser::ast::validator::language_level_validations::chunks::ModifierValidator;
use crate::com::github::javaparser::ast::validator::language_level_validations::chunks::UnderscoreKeywordValidator;

pub struct Java9Validator {
	underscore_keyword_validator: com::github::javaparser::ast::validator::validator::Validator = UnderscoreKeywordValidator::new(),
	modifiers: com::github::javaparser::ast::validator::validator::Validator = ModifierValidator::new(true, true, true),
	try_with_resources: com::github::javaparser::ast::validator::single_node_type_validator::SingleNodeTypeValidator = SingleNodeTypeValidator<>::new(TryStmt.class, |(n, reporter)|{
	if n.getCatchClauses().isEmpty() && n.getResources().isEmpty() && !n.getFinallyBlock().isPresent() {
		reporter.report(n, "Try has no finally, no catch, and no resources.");
	}
}),
}

impl Java9Validator {
	pub fn new() /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::validator::language_level_validations::java9_validator::Java9Validator {
		super();
		// Released Language Features
		/* 
	         * Note there is no validator that validates that "var" is not used in Java 9 and lower, since
	         * the parser will never create a VarType node (that is done by the Java 10 post-processor).
	         * You can add the node by hand, but that is obscure enough to ignore.
	         */ 
		self.add(self.underscore_keyword_validator);
		self.remove()?;
		self.replace(, self.modifiers);
		self.replace(, self.try_with_resources);
	}
}

impl com::github::javaparser::ast::validator::validator::Validator for Java9Validator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for Java9Validator {}

impl /* Java */ java::util::function::BiConsumer /**/ for Java9Validator {}