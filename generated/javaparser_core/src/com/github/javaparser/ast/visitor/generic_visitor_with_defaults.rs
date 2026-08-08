use crate::com::github::javaparser::ast;
use crate::com::github::javaparser::ast::body;
use crate::com::github::javaparser::ast::comments::BlockComment;
use crate::com::github::javaparser::ast::comments::LineComment;
use crate::com::github::javaparser::ast::comments::MarkdownComment;
use crate::com::github::javaparser::ast::comments::TraditionalJavadocComment;
use crate::com::github::javaparser::ast::expr;
use crate::com::github::javaparser::ast::modules;
use crate::com::github::javaparser::ast::stmt;
use crate::com::github::javaparser::ast::type;

pub struct GenericVisitorWithDefaults<R, A>;

impl<R, A> GenericVisitorWithDefaults {
	pub fn default_action(&self, n: &com::github::javaparser::ast::node::Node, arg: &A) -> R {
		return null;
	}

	pub fn default_action(&self, n: &com::github::javaparser::ast::node_list::NodeList, arg: &A) -> R {
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::annotation_declaration::AnnotationDeclaration, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::annotation_member_declaration::AnnotationMemberDeclaration, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_access_expr::ArrayAccessExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_creation_expr::ArrayCreationExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::assert_stmt::AssertStmt, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::assign_expr::AssignExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::binary_expr::BinaryExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::boolean_literal_expr::BooleanLiteralExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::break_stmt::BreakStmt, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::cast_expr::CastExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::catch_clause::CatchClause, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::char_literal_expr::CharLiteralExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::class_expr::ClassExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::compilation_unit::CompilationUnit, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::conditional_expr::ConditionalExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::continue_stmt::ContinueStmt, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::do_stmt::DoStmt, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::double_literal_expr::DoubleLiteralExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::empty_stmt::EmptyStmt, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::enclosed_expr::EnclosedExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::enum_constant_declaration::EnumConstantDeclaration, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::enum_declaration::EnumDeclaration, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::explicit_constructor_invocation_stmt::ExplicitConstructorInvocationStmt, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::expression_stmt::ExpressionStmt, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::field_access_expr::FieldAccessExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::field_declaration::FieldDeclaration, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::for_each_stmt::ForEachStmt, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::for_stmt::ForStmt, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::if_stmt::IfStmt, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::initializer_declaration::InitializerDeclaration, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::instance_of_expr::InstanceOfExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::integer_literal_expr::IntegerLiteralExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::traditional_javadoc_comment::TraditionalJavadocComment, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::labeled_stmt::LabeledStmt, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::long_literal_expr::LongLiteralExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::marker_annotation_expr::MarkerAnnotationExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::member_value_pair::MemberValuePair, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::method_declaration::MethodDeclaration, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name_expr::NameExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::normal_annotation_expr::NormalAnnotationExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::null_literal_expr::NullLiteralExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::object_creation_expr::ObjectCreationExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::package_declaration::PackageDeclaration, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::parameter::Parameter, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::primitive_type::PrimitiveType, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name::Name, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::simple_name::SimpleName, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::array_type::ArrayType, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::array_creation_level::ArrayCreationLevel, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::intersection_type::IntersectionType, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::union_type::UnionType, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::return_stmt::ReturnStmt, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::single_member_annotation_expr::SingleMemberAnnotationExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::string_literal_expr::StringLiteralExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::super_expr::SuperExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::switch_entry::SwitchEntry, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::switch_stmt::SwitchStmt, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::synchronized_stmt::SynchronizedStmt, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::this_expr::ThisExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::throw_stmt::ThrowStmt, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::try_stmt::TryStmt, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::local_class_declaration_stmt::LocalClassDeclarationStmt, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::local_record_declaration_stmt::LocalRecordDeclarationStmt, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::type_parameter::TypeParameter, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::unary_expr::UnaryExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::unknown_type::UnknownType, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::variable_declarator::VariableDeclarator, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::void_type::VoidType, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::while_stmt::WhileStmt, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::wildcard_type::WildcardType, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::lambda_expr::LambdaExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::method_reference_expr::MethodReferenceExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::type_expr::TypeExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::import_declaration::ImportDeclaration, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::block_comment::BlockComment, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::line_comment::LineComment, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::node_list::NodeList, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_requires_directive::ModuleRequiresDirective, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_exports_directive::ModuleExportsDirective, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_provides_directive::ModuleProvidesDirective, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_uses_directive::ModuleUsesDirective, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_opens_directive::ModuleOpensDirective, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::unparsable_stmt::UnparsableStmt, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::var_type::VarType, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modifier::Modifier, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::switch_expr::SwitchExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::yield_stmt::YieldStmt, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::text_block_literal_expr::TextBlockLiteralExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::type_pattern_expr::TypePatternExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::record_declaration::RecordDeclaration, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::compact_constructor_declaration::CompactConstructorDeclaration, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::record_pattern_expr::RecordPatternExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::match_all_pattern_expr::MatchAllPatternExpr, arg: &A) -> R {
		return self.default_action(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::markdown_comment::MarkdownComment, arg: &A) -> R {
		return self.default_action(n, arg);
	}
}

impl<R, A> com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor for GenericVisitorWithDefaults<R, A> {}