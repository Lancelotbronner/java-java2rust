use crate::com::github::javaparser::ast::Modifier;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::expr::AnnotationExpr;
use crate::com::github::javaparser::utils::Utils::assertNotNull;

struct ModifierHolder {
	modifiers: com::github::javaparser::ast::node_list::NodeList,
	annotations: com::github::javaparser::ast::node_list::NodeList,
	begin: com::github::javaparser::java_token::JavaToken,
}

impl ModifierHolder {
	fn new(begin: &com::github::javaparser::java_token::JavaToken, modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::modifier_holder::ModifierHolder {
		self.begin = begin;
		self.modifiers = com::github::javaparser::utils::utils::Utils::assert_not_null(modifiers)?;
		self.annotations = annotations;
	}
}