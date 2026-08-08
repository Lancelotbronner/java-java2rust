use crate::com::github::javaparser::ast::expr::Expression;
use crate::com::github::javaparser::ast::stmt::TryStmt;
use crate::com::github::javaparser::ast::type::UnionType;
use crate::com::github::javaparser::ast::validator::SingleNodeTypeValidator;

pub struct Java7Validator {
	try_with_limited_resources: com::github::javaparser::ast::validator::single_node_type_validator::SingleNodeTypeValidator = SingleNodeTypeValidator<>::new(TryStmt.class, |(n, reporter)|{
	if n.getCatchClauses().isEmpty() && n.getResources().isEmpty() && !n.getFinallyBlock().isPresent() {
		reporter.report(n, "Try has no finally, no catch, and no resources.");
	}
	for resource in n.getResources() {
		if !resource.is_variable_declaration_expr() {
			reporter.report(n, "Try with resources only supports variable declarations.");
		}
	}
}),
	multi_catch: com::github::javaparser::ast::validator::single_node_type_validator::SingleNodeTypeValidator = SingleNodeTypeValidator<>::new(UnionType.class, |(n, reporter)|{
	// Case "0 elements" is caught elsewhere.
	if n.getElements().size() == 1 {
		reporter.report(n, "Union type (multi catch) must have at least two elements.");
	}
}),
}

impl Java7Validator {
	pub fn new() /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::validator::language_level_validations::java7_validator::Java7Validator {
		super();
		self.remove()?;
		self.replace(, self.try_with_limited_resources);
		self.remove()?;
		self.remove()?;
		self.replace(, self.multi_catch);
	}
}

impl com::github::javaparser::ast::validator::validator::Validator for Java7Validator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for Java7Validator {}

impl /* Java */ java::util::function::BiConsumer /**/ for Java7Validator {}