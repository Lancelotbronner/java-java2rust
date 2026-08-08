use crate::com::github::javaparser::ast::Modifier::Keyword;
use java::util::Arrays::asList;
use crate::com::github::javaparser::ast::Modifier;
use crate::com::github::javaparser::ast::body;
use crate::com::github::javaparser::ast::expr::LambdaExpr;
use crate::com::github::javaparser::ast::expr::VariableDeclarationExpr;
use crate::com::github::javaparser::ast::modules::ModuleRequiresDirective;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithModifiers;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithTokenRange;
use crate::com::github::javaparser::ast::stmt::CatchClause;
use crate::com::github::javaparser::ast::validator::ProblemReporter;
use crate::com::github::javaparser::ast::validator::VisitorValidator;
use crate::com::github::javaparser::utils::SeparatedItemStringBuilder;
use java::util::ArrayList;
use java::util::List;

pub struct ModifierValidator {
	interface_with_nothing_special: &[com::github::javaparser::ast::modifier::Keyword] = : [Option<Modifier.Keyword>; ] = [None; ],
	interface_with_static_and_default: &[com::github::javaparser::ast::modifier::Keyword] = : [Option<Modifier.Keyword>; ] = [None; ],
	interface_with_static_and_default_and_private: &[com::github::javaparser::ast::modifier::Keyword] = : [Option<Modifier.Keyword>; ] = [None; ],
	has_strictfp: bool,
	has_default_and_static_interface_methods: bool,
	has_private_interface_methods: bool,
}

impl ModifierValidator {
	pub fn new(has_strictfp: bool, has_default_and_static_interface_methods: bool, has_private_interface_methods: bool) -> com::github::javaparser::ast::validator::language_level_validations::chunks::modifier_validator::ModifierValidator {
		self.hasStrictfp = has_strictfp;
		self.hasDefaultAndStaticInterfaceMethods = has_default_and_static_interface_methods;
		self.hasPrivateInterfaceMethods = has_private_interface_methods;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration, reporter: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		if n.is_interface() {
			self.validate_interface_modifiers(n, reporter);
		} else {
			self.validate_class_modifiers(n, reporter);
		}
		super.visit(n, reporter);
	}

	fn validate_class_modifiers(&self, n: &com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration, reporter: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		if n.is_top_level_type() {
			self.validate_modifiers(n, reporter, PUBLIC, ABSTRACT, FINAL, STRICTFP, SEALED, NON_SEALED);
		} else if n.is_nested_type() {
			self.validate_modifiers(n, reporter, PUBLIC, PROTECTED, PRIVATE, ABSTRACT, STATIC, FINAL, STRICTFP, SEALED, NON_SEALED);
		} else if n.is_local_class_declaration() {
			self.validate_modifiers(n, reporter, ABSTRACT, FINAL, STRICTFP, SEALED, NON_SEALED);
		}
	}

	fn validate_interface_modifiers(&self, n: &com::github::javaparser::ast::body::type_declaration::TypeDeclaration, reporter: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		if n.is_top_level_type() {
			self.validate_modifiers(n, reporter, PUBLIC, ABSTRACT, STRICTFP, SEALED, NON_SEALED);
		} else if n.is_nested_type() {
			self.validate_modifiers(n, reporter, PUBLIC, PROTECTED, PRIVATE, ABSTRACT, STATIC, STRICTFP, SEALED, NON_SEALED);
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::enum_declaration::EnumDeclaration, reporter: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		if n.is_top_level_type() {
			self.validate_modifiers(n, reporter, PUBLIC, STRICTFP);
		} else if n.is_nested_type() {
			self.validate_modifiers(n, reporter, PUBLIC, PROTECTED, PRIVATE, STATIC, STRICTFP);
		}
		super.visit(n, reporter);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::annotation_declaration::AnnotationDeclaration, reporter: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		self.validate_interface_modifiers(n, reporter);
		super.visit(n, reporter);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::annotation_member_declaration::AnnotationMemberDeclaration, reporter: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		self.validate_modifiers(n, reporter, PUBLIC, ABSTRACT);
		super.visit(n, reporter);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration, reporter: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		self.validate_modifiers(n, reporter, PUBLIC, PROTECTED, PRIVATE);
		n.get_parameters().forEach(|p|self.validate_modifiers(p, reporter, FINAL));
		super.visit(n, reporter);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::field_declaration::FieldDeclaration, reporter: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		self.validate_modifiers(n, reporter, PUBLIC, PROTECTED, PRIVATE, STATIC, FINAL, TRANSIENT, VOLATILE);
		super.visit(n, reporter);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::method_declaration::MethodDeclaration, reporter: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		if n.is_abstract() {
			/* final */ let builder: SeparatedItemStringBuilder = SeparatedItemStringBuilder::new("Cannot be 'abstract' and also '", "', '", "'.");
			for m in /* Java */ java::util::Arrays /**/::asList(PRIVATE, STATIC, FINAL, NATIVE, STRICTFP, SYNCHRONIZED) {
				if n.has_modifier(m) {
					builder.append(&m.as_string());
				}
			}
			if builder.has_items() {
				reporter.report(n, &builder.to_string());
			}
		}
		if n.get_parent_node().isPresent() {
			if n.get_parent_node().get() instanceof ClassOrInterfaceDeclaration {
				if (n.get_parent_node().get() as ClassOrInterfaceDeclaration).is_interface() {
					if self.has_default_and_static_interface_methods {
						if self.has_private_interface_methods {
							self.validate_modifiers(n, reporter, self.interface_with_static_and_default_and_private);
						} else {
							self.validate_modifiers(n, reporter, self.interface_with_static_and_default);
						}
					} else {
						self.validate_modifiers(n, reporter, self.interface_with_nothing_special);
					}
				} else {
					self.validate_modifiers(n, reporter, PUBLIC, PROTECTED, PRIVATE, ABSTRACT, STATIC, FINAL, SYNCHRONIZED, NATIVE, STRICTFP);
				}
			}
		}
		n.get_parameters().forEach(|p|self.validate_modifiers(p, reporter, FINAL));
		super.visit(n, reporter);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::lambda_expr::LambdaExpr, reporter: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		n.get_parameters().forEach(|p|{
			// Final is not allowed on inferred parameters, but those get caught by the parser.
			self.validate_modifiers(p, reporter, FINAL);
		});
		super.visit(n, reporter);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::catch_clause::CatchClause, reporter: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		self.validate_modifiers(&n.get_parameter(), reporter, FINAL);
		super.visit(n, reporter);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr, reporter: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		self.validate_modifiers(n, reporter, FINAL);
		super.visit(n, reporter);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_requires_directive::ModuleRequiresDirective, reporter: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		self.validate_modifiers(n, reporter, TRANSITIVE, STATIC);
		super.visit(n, reporter);
	}

	fn validate_modifiers<T: com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers+com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange>(&self, n: &T, reporter: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter, mut allowed_modifiers: &com::github::javaparser::ast::modifier::Keyword) {
		self.validate_at_most_one_of(n, reporter, PUBLIC, PROTECTED, PRIVATE);
		self.validate_at_most_one_of(n, reporter, FINAL, ABSTRACT);
		if self.has_strictfp {
			self.validate_at_most_one_of(n, reporter, NATIVE, STRICTFP);
		} else {
			allowed_modifiers = self.remove_modifier_from_array(STRICTFP, allowed_modifiers);
		}
		for m in n.get_modifiers() {
			if !self.array_contains(allowed_modifiers, &m.get_keyword()) {
				reporter.report(n, "'%s' is not allowed here.", &m.get_keyword().as_string());
			}
		}
	}

	fn remove_modifier_from_array(&self, m: &com::github::javaparser::ast::modifier::Keyword, mut allowed_modifiers: &&[com::github::javaparser::ast::modifier::Keyword]) -> &[com::github::javaparser::ast::modifier::Keyword] {
		/* final */ let new_modifiers: List<Modifier.Keyword> = ArrayList<>::new(&/* Java */ java::util::Arrays /**/::asList(allowed_modifiers));
		new_modifiers.remove(m);
		allowed_modifiers = new_modifiers.toArray(: [Option<Modifier.Keyword>; 0] = [None; 0]);
		return allowed_modifiers;
	}

	fn array_contains(&self, items: &&[/* Java */ java::lang::Object /**/], search_item: &/* Java */ java::lang::Object /**/) -> bool {
		for o in items {
			if o == search_item {
				return true;
			}
		}
		return false;
	}

	fn validate_at_most_one_of<T: com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers+com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange>(&self, t: &T, reporter: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter, modifiers: &com::github::javaparser::ast::modifier::Keyword) {
		let found_modifiers: List<Modifier.Keyword> = ArrayList<>::new();
		for m in modifiers {
			if t.has_modifier(m) {
				found_modifiers.add(m);
			}
		}
		if found_modifiers.size() > 1 {
			let builder: SeparatedItemStringBuilder = SeparatedItemStringBuilder::new("Can have only one of '", "', '", "'.");
			for m in found_modifiers {
				builder.append(&m.as_string());
			}
			reporter.report(t, &builder.to_string());
		}
	}
}

impl com::github::javaparser::ast::visitor::void_visitor::VoidVisitor for ModifierValidator {}

impl com::github::javaparser::ast::validator::validator::Validator for ModifierValidator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for ModifierValidator {}

impl /* Java */ java::util::function::BiConsumer /**/ for ModifierValidator {}