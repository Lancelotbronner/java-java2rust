use crate::com::github::javaparser::ParserConfiguration;
use crate::com::github::javaparser::ast::ImportDeclaration;
use crate::com::github::javaparser::ast::Modifier;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::body;
use crate::com::github::javaparser::ast::expr;
use crate::com::github::javaparser::ast::modules::ModuleDeclaration;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithIdentifier;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithTypeArguments;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithTypeParameters;
use crate::com::github::javaparser::ast::stmt;
use crate::com::github::javaparser::ast::type::UnionType;
use crate::com::github::javaparser::ast::validator;
use crate::com::github::javaparser::ast::validator::language_level_validations::chunks::CommonValidators;
use crate::com::github::javaparser::ast::validator::language_level_validations::chunks::ModifierValidator;
use crate::com::github::javaparser::ast::validator::language_level_validations::chunks::NoBinaryIntegerLiteralsValidator;
use crate::com::github::javaparser::ast::validator::language_level_validations::chunks::NoUnderscoresInIntegerLiteralsValidator;

pub struct Java1_0Validator {
	modifiers_without_strictfp_and_default_and_static_interface_methods_and_private_interface_methods: com::github::javaparser::ast::validator::validator::Validator = ModifierValidator::new(false, false, false),
	no_assert_keyword: com::github::javaparser::ast::validator::validator::Validator = SimpleValidator<>::new(AssertStmt.class, |n|true, |(n, reporter)|reporter.report(n, UpgradeJavaMessage::new("'assert' keyword is not supported.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_1_4))),
	no_assert_identifer: com::github::javaparser::ast::validator::validator::Validator = TreeVisitorValidator::new(|(node, reporter)|{
	if node instanceof NodeWithIdentifier && (node as NodeWithIdentifier).get_identifier().equals("assert") {
		reporter.report(node, UpgradeJavaMessage::new("'assert' identifier is not supported.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_1_4, false));
	}
}),
	no_inner_classes: com::github::javaparser::ast::validator::validator::Validator = SimpleValidator<>::new(ClassOrInterfaceDeclaration.class, |n|!n.isTopLevelType(), |(n, reporter)|reporter.report(n, UpgradeJavaMessage::new("inner classes or interfaces are not supported.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_1_1))),
	no_reflection: com::github::javaparser::ast::validator::validator::Validator = SimpleValidator<>::new(ClassExpr.class, |n|true, |(n, reporter)|reporter.report(n, UpgradeJavaMessage::new("Reflection is not supported.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_1_1))),
	no_generics: com::github::javaparser::ast::validator::validator::Validator = TreeVisitorValidator::new(|(node, reporter)|{
	if node instanceof NodeWithTypeArguments {
		if (node as NodeWithTypeArguments<? extends Node>).get_type_arguments().isPresent() {
			reporter.report(node, UpgradeJavaMessage::new("Generics are not supported.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_5));
		}
	}
	if node instanceof NodeWithTypeParameters {
		if (node as NodeWithTypeParameters<? extends Node>).get_type_parameters().is_non_empty() {
			reporter.report(node, UpgradeJavaMessage::new("Generics are not supported.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_5));
		}
	}
}),
	try_without_resources: com::github::javaparser::ast::validator::single_node_type_validator::SingleNodeTypeValidator = SingleNodeTypeValidator<>::new(TryStmt.class, |(n, reporter)|{
	if n.getCatchClauses().isEmpty() && !n.getFinallyBlock().isPresent() {
		reporter.report(n, UpgradeJavaMessage::new("Try has no finally and no catch.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_7));
	}
	if n.getResources().isNonEmpty() {
		reporter.report(n, UpgradeJavaMessage::new("Catch with resource is not supported.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_7));
	}
}),
	no_annotations: com::github::javaparser::ast::validator::validator::Validator = TreeVisitorValidator::new(|(node, reporter)|{
	if node instanceof AnnotationExpr || node instanceof AnnotationDeclaration {
		reporter.report(node, UpgradeJavaMessage::new("Annotations are not supported.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_5));
	}
}),
	no_enums: com::github::javaparser::ast::validator::validator::Validator = SimpleValidator<>::new(EnumDeclaration.class, |n|true, |(n, reporter)|reporter.report(n, UpgradeJavaMessage::new("Enumerations are not supported.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_5))),
	no_varargs: com::github::javaparser::ast::validator::validator::Validator = SimpleValidator<>::new(Parameter.class, Parameter::isVarArgs, |(n, reporter)|reporter.report(n, UpgradeJavaMessage::new("Varargs are not supported.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_5))),
	no_for_each: com::github::javaparser::ast::validator::validator::Validator = SimpleValidator<>::new(ForEachStmt.class, |n|true, |(n, reporter)|reporter.report(n, UpgradeJavaMessage::new("For-each loops are not supported.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_5))),
	no_static_imports: com::github::javaparser::ast::validator::validator::Validator = SimpleValidator<>::new(ImportDeclaration.class, ImportDeclaration::isStatic, |(n, reporter)|reporter.report(n, UpgradeJavaMessage::new("Static imports are not supported.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_5))),
	only_one_label_in_switch_case: com::github::javaparser::ast::validator::validator::Validator = SimpleValidator<>::new(SwitchEntry.class, |n|n.getLabels().size() > 1, |(n, reporter)|reporter.report(&n.getLabels().getParentNode().get(), UpgradeJavaMessage::new("Only one label allowed in a switch-case.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_7))),
	no_yield: com::github::javaparser::ast::validator::validator::Validator = SimpleValidator<>::new(YieldStmt.class, |n|true, |(n, reporter)|reporter.report(n, UpgradeJavaMessage::new("Only labels allowed in break statements.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_13))),
	no_binary_integer_literals: com::github::javaparser::ast::validator::validator::Validator = NoBinaryIntegerLiteralsValidator::new(),
	no_underscores_in_integer_literals: com::github::javaparser::ast::validator::validator::Validator = NoUnderscoresInIntegerLiteralsValidator::new(),
	no_multi_catch: com::github::javaparser::ast::validator::validator::Validator = SimpleValidator<>::new(UnionType.class, |n|true, |(n, reporter)|reporter.report(n, UpgradeJavaMessage::new("Multi-catch is not supported.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_7))),
	no_lambdas: com::github::javaparser::ast::validator::validator::Validator = SimpleValidator<>::new(LambdaExpr.class, |n|true, |(n, reporter)|reporter.report(n, UpgradeJavaMessage::new("Lambdas are not supported.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_8))),
	no_modules: com::github::javaparser::ast::validator::validator::Validator = SimpleValidator<>::new(ModuleDeclaration.class, |n|true, |(n, reporter)|reporter.report(n, UpgradeJavaMessage::new("Modules are not supported.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_9))),
	no_switch_expressions: com::github::javaparser::ast::validator::validator::Validator = SimpleValidator<>::new(SwitchExpr.class, |n|true, |(n, reporter)|reporter.report(n, UpgradeJavaMessage::new("Switch expressions are not supported.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_12))),
	no_pattern_matching_instance_of: com::github::javaparser::ast::validator::validator::Validator = SimpleValidator<>::new(InstanceOfExpr.class, |n|n.getPattern().isPresent(), |(n, reporter)|reporter.report(n, UpgradeJavaMessage::new("Use of patterns with instanceof is not supported.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_14))),
	no_text_block_literal: com::github::javaparser::ast::validator::validator::Validator = SimpleValidator<>::new(TextBlockLiteralExpr.class, |n|true, |(n, reporter)|reporter.report(n, UpgradeJavaMessage::new("Text Block Literals are not supported.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_15))),
	no_record_declaration: com::github::javaparser::ast::validator::validator::Validator = SimpleValidator<>::new(RecordDeclaration.class, |n|true, |(n, reporter)|reporter.report(n, UpgradeJavaMessage::new("Record Declarations are not supported.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_14))),
	no_sealed_classes: com::github::javaparser::ast::validator::validator::Validator = SimpleValidator<>::new(ClassOrInterfaceDeclaration.class, |n|n.hasModifier(Modifier::com::github::javaparser::ast::modifier::Keyword::SEALED) || n.hasModifier(Modifier::com::github::javaparser::ast::modifier::Keyword::NON_SEALED), |(n, reporter)|reporter.report(n, UpgradeJavaMessage::new("Sealed classes are not supported.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_15))),
	no_permits_list_in_classes: com::github::javaparser::ast::validator::validator::Validator = SimpleValidator<>::new(ClassOrInterfaceDeclaration.class, |n|n.getPermittedTypes().isNonEmpty(), |(n, reporter)|reporter.report(n, UpgradeJavaMessage::new("Permitted sub-classes are not supported.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_17))),
	no_switch_null_default: com::github::javaparser::ast::validator::validator::Validator = SingleNodeTypeValidator<>::new(SwitchEntry.class, |(n, reporter)|{
	if n.getLabels().isNonEmpty() && n.isDefault() {
		reporter.report(n, UpgradeJavaMessage::new("Switch case null, default not supported.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_21));
	}
}),
	no_switch_patterns: com::github::javaparser::ast::validator::validator::Validator = SingleNodeTypeValidator<>::new(SwitchEntry.class, |(n, reporter)|{
	if n.getGuard().isPresent() || n.getLabels().stream().anyMatch(|expr|expr.isComponentPatternExpr()) {
		reporter.report(n, UpgradeJavaMessage::new("Switch patterns not supported.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_21));
	}
}),
	no_record_patterns: com::github::javaparser::ast::validator::validator::Validator = TreeVisitorValidator::new(|(node, reporter)|{
	if node instanceof RecordPatternExpr {
		reporter.report(node, UpgradeJavaMessage::new("Record patterns are not supported.", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_21));
	}
}),
	no_module_imports: com::github::javaparser::ast::validator::validator::Validator = TreeVisitorValidator::new(|(node, reporter)|{
	if node instanceof ImportDeclaration && (node as ImportDeclaration).is_module() {
		reporter.report(node, UpgradeJavaMessage::new("Module imports are not supported", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_25));
	}
}),
	explicit_constructor_invocation_must_be_first_statement: com::github::javaparser::ast::validator::validator::Validator = TreeVisitorValidator::new(|(node: &Node, reporter: &ProblemReporter)|{
	// be the case for code snippets being parsed.
	if node instanceof ExplicitConstructorInvocationStmt && node.get_parent_node().isPresent() {
		let parent: Node = node.get_parent_node().get();
		if parent instanceof BlockStmt && (parent as BlockStmt).get_statements().index_of(node) > 0 {
			reporter.report(node, UpgradeJavaMessage::new("Flexible constructor bodies are not supported", ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_25));
		}
	}
}),
}

impl Java1_0Validator {
	pub fn new() -> com::github::javaparser::ast::validator::language_level_validations::java1_0_validator::Java1_0Validator {
		super(CommonValidators::new());
		self.add(self.modifiers_without_strictfp_and_default_and_static_interface_methods_and_private_interface_methods);
		self.add(self.no_assert_keyword);
		self.add(self.no_inner_classes);
		self.add(self.no_reflection);
		self.add(self.no_generics);
		self.add(self.try_without_resources);
		self.add(self.no_annotations);
		self.add(self.no_enums);
		self.add(self.no_varargs);
		self.add(self.no_for_each);
		self.add(self.no_static_imports);
		self.add(self.no_yield);
		self.add(self.only_one_label_in_switch_case);
		self.add(self.no_binary_integer_literals);
		self.add(self.no_underscores_in_integer_literals);
		self.add(self.no_multi_catch);
		self.add(self.no_lambdas);
		self.add(self.no_modules);
		self.add(self.no_switch_expressions);
		self.add(self.no_pattern_matching_instance_of);
		self.add(self.no_text_block_literal);
		self.add(self.no_record_declaration);
		self.add(self.no_sealed_classes);
		self.add(self.no_permits_list_in_classes);
		self.add(self.no_switch_null_default);
		self.add(self.no_switch_patterns);
		self.add(self.no_record_patterns);
		self.add(self.no_module_imports);
		self.add(self.explicit_constructor_invocation_must_be_first_statement);
	}
}

impl com::github::javaparser::ast::validator::validator::Validator for Java1_0Validator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for Java1_0Validator {}

impl /* Java */ java::util::function::BiConsumer /**/ for Java1_0Validator {}