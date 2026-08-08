use crate::com::github::javaparser::ParserConfiguration;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::expr::VariableDeclarationExpr;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithTypeArguments;
use crate::com::github::javaparser::ast::stmt::ForEachStmt;
use crate::com::github::javaparser::ast::type::PrimitiveType;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::ast::validator::ReservedKeywordValidator;
use crate::com::github::javaparser::ast::validator::SingleNodeTypeValidator;
use crate::com::github::javaparser::ast::validator::TreeVisitorValidator;
use crate::com::github::javaparser::ast::validator::Validator;
use java::util::Optional;

pub struct Java5Validator {
	generics_without_diamond_operator: com::github::javaparser::ast::validator::validator::Validator = TreeVisitorValidator::new(|(node, reporter)|{
	if node instanceof NodeWithTypeArguments {
		let type_arguments: Optional<NodeList<Type>> = (node as NodeWithTypeArguments<? extends Node>).get_type_arguments();
		if type_arguments.isPresent() && type_arguments.get().is_empty() {
			reporter.report(node, UpgradeJavaMessage::new("The diamond operator is not supported.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_7));
		}
	}
}),
	no_primitive_generic_arguments: com::github::javaparser::ast::validator::validator::Validator = TreeVisitorValidator::new(|(node, reporter)|{
	if node instanceof NodeWithTypeArguments {
		let type_arguments: Optional<NodeList<Type>> = (node as NodeWithTypeArguments<? extends Node>).get_type_arguments();
		type_arguments.ifPresent(|types|types.forEach(|ty|{
			if ty instanceof PrimitiveType {
				reporter.report(node, "Type arguments may not be primitive.");
			}
		}));
	}
}),
	for_each_stmt: com::github::javaparser::ast::validator::validator::Validator = SingleNodeTypeValidator<>::new(ForEachStmt.class, |(node, reporter)|{
	let declaration: VariableDeclarationExpr = node.getVariable();
	// assert that the variable declaration expression has exactly one variable declarator
	if declaration.get_variables().size() != 1 {
		reporter.report(node, "A foreach statement's variable declaration must have exactly one variable " + "declarator. Given: " + declaration.get_variables().size() + ".");
	}
}),
	enum_not_allowed: com::github::javaparser::ast::validator::validator::Validator = ReservedKeywordValidator::new("enum"),
}

impl Java5Validator {
	pub fn new() /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::validator::language_level_validations::java5_validator::Java5Validator {
		super();
		self.replace(, self.generics_without_diamond_operator);
		self.add(self.no_primitive_generic_arguments);
		self.add(self.enum_not_allowed);
		self.add(self.for_each_stmt);
		// TODO validate annotations on classes, fields and methods but nowhere else
		// The following is probably too simple.
		self.remove()?;
		self.remove()?;
		self.remove()?;
		self.remove()?;
		self.remove()?;
	}
}

impl com::github::javaparser::ast::validator::validator::Validator for Java5Validator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for Java5Validator {}

impl /* Java */ java::util::function::BiConsumer /**/ for Java5Validator {}