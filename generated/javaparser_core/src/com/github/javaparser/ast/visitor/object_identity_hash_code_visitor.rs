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

pub struct ObjectIdentityHashCodeVisitor;

impl ObjectIdentityHashCodeVisitor {
	static SINGLETON: com::github::javaparser::ast::visitor::object_identity_hash_code_visitor::ObjectIdentityHashCodeVisitor = ObjectIdentityHashCodeVisitor::new();

	pub fn hash_code(&self, node: &com::github::javaparser::ast::node::Node) -> i32 {
		return node.accept(self.SINGLETON, null);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::annotation_declaration::AnnotationDeclaration, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::annotation_member_declaration::AnnotationMemberDeclaration, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_access_expr::ArrayAccessExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_creation_expr::ArrayCreationExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::array_creation_level::ArrayCreationLevel, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::array_type::ArrayType, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::assert_stmt::AssertStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::assign_expr::AssignExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::binary_expr::BinaryExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::block_comment::BlockComment, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::boolean_literal_expr::BooleanLiteralExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::break_stmt::BreakStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::cast_expr::CastExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::catch_clause::CatchClause, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::char_literal_expr::CharLiteralExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::class_expr::ClassExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::compilation_unit::CompilationUnit, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::conditional_expr::ConditionalExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::continue_stmt::ContinueStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::do_stmt::DoStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::double_literal_expr::DoubleLiteralExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::empty_stmt::EmptyStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::enclosed_expr::EnclosedExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::enum_constant_declaration::EnumConstantDeclaration, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::enum_declaration::EnumDeclaration, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::explicit_constructor_invocation_stmt::ExplicitConstructorInvocationStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::expression_stmt::ExpressionStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::field_access_expr::FieldAccessExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::field_declaration::FieldDeclaration, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::for_stmt::ForStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::for_each_stmt::ForEachStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::if_stmt::IfStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::import_declaration::ImportDeclaration, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::initializer_declaration::InitializerDeclaration, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::instance_of_expr::InstanceOfExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::integer_literal_expr::IntegerLiteralExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::intersection_type::IntersectionType, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::traditional_javadoc_comment::TraditionalJavadocComment, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::labeled_stmt::LabeledStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::lambda_expr::LambdaExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::line_comment::LineComment, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::local_class_declaration_stmt::LocalClassDeclarationStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::local_record_declaration_stmt::LocalRecordDeclarationStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::long_literal_expr::LongLiteralExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::marker_annotation_expr::MarkerAnnotationExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::member_value_pair::MemberValuePair, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::method_declaration::MethodDeclaration, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::method_reference_expr::MethodReferenceExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name_expr::NameExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name::Name, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::node_list::NodeList, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::normal_annotation_expr::NormalAnnotationExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::null_literal_expr::NullLiteralExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::object_creation_expr::ObjectCreationExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::package_declaration::PackageDeclaration, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::parameter::Parameter, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::primitive_type::PrimitiveType, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::return_stmt::ReturnStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::simple_name::SimpleName, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::single_member_annotation_expr::SingleMemberAnnotationExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::string_literal_expr::StringLiteralExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::super_expr::SuperExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::switch_entry::SwitchEntry, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::switch_stmt::SwitchStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::synchronized_stmt::SynchronizedStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::this_expr::ThisExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::throw_stmt::ThrowStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::try_stmt::TryStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::type_expr::TypeExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::type_parameter::TypeParameter, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::unary_expr::UnaryExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::union_type::UnionType, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::unknown_type::UnknownType, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::variable_declarator::VariableDeclarator, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::void_type::VoidType, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::while_stmt::WhileStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::wildcard_type::WildcardType, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_requires_directive::ModuleRequiresDirective, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_exports_directive::ModuleExportsDirective, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_provides_directive::ModuleProvidesDirective, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_uses_directive::ModuleUsesDirective, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_opens_directive::ModuleOpensDirective, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::unparsable_stmt::UnparsableStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::var_type::VarType, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modifier::Modifier, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::switch_expr::SwitchExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::yield_stmt::YieldStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::text_block_literal_expr::TextBlockLiteralExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::type_pattern_expr::TypePatternExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::record_declaration::RecordDeclaration, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::compact_constructor_declaration::CompactConstructorDeclaration, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::record_pattern_expr::RecordPatternExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::match_all_pattern_expr::MatchAllPatternExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::markdown_comment::MarkdownComment, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return n.hash_code();
	}
}

impl com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor for ObjectIdentityHashCodeVisitor {}