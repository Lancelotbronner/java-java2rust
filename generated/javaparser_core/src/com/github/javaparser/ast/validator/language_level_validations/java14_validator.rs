use crate::com::github::javaparser::ast::body::RecordDeclaration;
use crate::com::github::javaparser::ast::validator::RecordAsTypeIdentifierNotAllowed;
use crate::com::github::javaparser::ast::validator::SingleNodeTypeValidator;
use crate::com::github::javaparser::ast::validator::Validator;
use crate::com::github::javaparser::ast::validator::language_level_validations::chunks::RecordDeclarationValidator;

pub struct Java14Validator {
	record_as_type_identifier_not_allowed: com::github::javaparser::ast::validator::validator::Validator = RecordAsTypeIdentifierNotAllowed::new(),
	record_declaration_validator: com::github::javaparser::ast::validator::validator::Validator = SingleNodeTypeValidator<>::new(RecordDeclaration.class, RecordDeclarationValidator::new()),
}

impl Java14Validator {
	pub fn new() /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::validator::language_level_validations::java14_validator::Java14Validator {
		super();
		// Released Language Features
		{
			/* 
	             * Switch Expressions (Standard) - released within Java 14 - https://openjdk.java.net/jeps/361
	             * <ul>
	             *     <li>Switch permissions are permitted</li>
	             *     <li>Previous preview allowed only a single label - this permits multiple.</li>
	             *     <li>Yield is now permitted within a switch expression.</li>
	             * </ul>
	             */ 
			self.remove()?;
			self.remove()?;
			self.remove()?;
		}
	}
}

impl com::github::javaparser::ast::validator::validator::Validator for Java14Validator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for Java14Validator {}

impl /* Java */ java::util::function::BiConsumer /**/ for Java14Validator {}