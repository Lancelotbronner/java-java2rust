use crate::com::github::javaparser::utils::CodeGenerationUtils::f;
use crate::com::github::javaparser::GeneratedJavaParserConstants;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::expr::IntegerLiteralExpr;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::printer::SourcePrinter;

pub struct CsmAttribute {
	property: com::github::javaparser::ast::observer::observable_property::ObservableProperty,
}

impl CsmAttribute {
	pub fn get_property(&self) -> com::github::javaparser::ast::observer::observable_property::ObservableProperty {
		return self.property;
	}

	pub fn new(property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty) -> com::github::javaparser::printer::concretesyntaxmodel::csm_attribute::CsmAttribute {
		self.property = property;
	}

	pub fn pretty_print(&self, node: &com::github::javaparser::ast::node::Node, printer: &com::github::javaparser::printer::source_printer::SourcePrinter) {
		let value: Object = self.property.get_raw_value(node)?;
		printer.print(&PrintingHelper::print_to_string(value));
	}

	pub fn get_token_type(&self, node: &com::github::javaparser::ast::node::Node, text: &/* Java */ java::lang::String /**/, token_text: &/* Java */ java::lang::String /**/) /* thrown(java.lang.UnsupportedOperationException | java.lang.RuntimeException) */ -> i32 {
		match self.property {
			IDENTIFIER =>  {
				return GeneratedJavaParserConstants.IDENTIFIER;
			}
			TYPE =>  {
				{
					let expected_image: String = "\"" + text.toLowerCase() + "\"";
					 {
						let i: i32 = 0;
						while i < GeneratedJavaParserConstants.tokenImage.length {
							{
								if GeneratedJavaParserConstants.tokenImage[i].equals(expected_image) {
									return i;
								}
							}
							i += 1;
						 }
					 }
	
					return Err(RuntimeException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("Attribute '%s' does not corresponding to any expected value. Text: %s", &self.property.camel_case_name(), text)));
				}
			}
			KEYWORD =>  {
			}
			OPERATOR =>  {
				{
					let expected_image: String = "\"" + token_text.toLowerCase() + "\"";
					 {
						let i: i32 = 0;
						while i < GeneratedJavaParserConstants.tokenImage.length {
							{
								if GeneratedJavaParserConstants.tokenImage[i].equals(expected_image) {
									return i;
								}
							}
							i += 1;
						 }
					 }
	
					return Err(RuntimeException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("Attribute '%s' does not corresponding to any expected value. Text: %s", &self.property.camel_case_name(), token_text)));
				}
			}
			VALUE =>  {
				if node instanceof IntegerLiteralExpr {
					return GeneratedJavaParserConstants.INTEGER_LITERAL;
				}
			}
			NAME =>  {
				return GeneratedJavaParserConstants.IDENTIFIER;
			}
		}
		return Err(UnsupportedOperationException::new("getTokenType does not know how to handle property " + self.property + " with text: " + text));
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return String::format("%s(property:%s)", &self.getClass().getSimpleName(), &self.get_property());
	}
}

impl com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement for CsmAttribute {}