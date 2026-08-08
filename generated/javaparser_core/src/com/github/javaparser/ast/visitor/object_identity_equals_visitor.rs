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

pub struct ObjectIdentityEqualsVisitor;

impl ObjectIdentityEqualsVisitor {
	static SINGLETON: com::github::javaparser::ast::visitor::object_identity_equals_visitor::ObjectIdentityEqualsVisitor = ObjectIdentityEqualsVisitor::new();

	pub fn equals(&self, n: &com::github::javaparser::ast::node::Node, n2: &com::github::javaparser::ast::node::Node) -> bool {
		return n.accept(self.SINGLETON, n2);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::compilation_unit::CompilationUnit, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::package_declaration::PackageDeclaration, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::type_parameter::TypeParameter, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::line_comment::LineComment, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::block_comment::BlockComment, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::enum_declaration::EnumDeclaration, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::enum_constant_declaration::EnumConstantDeclaration, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::annotation_declaration::AnnotationDeclaration, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::annotation_member_declaration::AnnotationMemberDeclaration, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::field_declaration::FieldDeclaration, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::variable_declarator::VariableDeclarator, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::method_declaration::MethodDeclaration, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::parameter::Parameter, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::initializer_declaration::InitializerDeclaration, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::traditional_javadoc_comment::TraditionalJavadocComment, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::primitive_type::PrimitiveType, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::array_type::ArrayType, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::array_creation_level::ArrayCreationLevel, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::intersection_type::IntersectionType, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::union_type::UnionType, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::void_type::VoidType, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::wildcard_type::WildcardType, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::unknown_type::UnknownType, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_access_expr::ArrayAccessExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_creation_expr::ArrayCreationExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::assign_expr::AssignExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::binary_expr::BinaryExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::cast_expr::CastExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::class_expr::ClassExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::conditional_expr::ConditionalExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::enclosed_expr::EnclosedExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::field_access_expr::FieldAccessExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::instance_of_expr::InstanceOfExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::string_literal_expr::StringLiteralExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::integer_literal_expr::IntegerLiteralExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::long_literal_expr::LongLiteralExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::char_literal_expr::CharLiteralExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::double_literal_expr::DoubleLiteralExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::boolean_literal_expr::BooleanLiteralExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::null_literal_expr::NullLiteralExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name_expr::NameExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::object_creation_expr::ObjectCreationExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name::Name, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::simple_name::SimpleName, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::this_expr::ThisExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::super_expr::SuperExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::unary_expr::UnaryExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::marker_annotation_expr::MarkerAnnotationExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::single_member_annotation_expr::SingleMemberAnnotationExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::normal_annotation_expr::NormalAnnotationExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::member_value_pair::MemberValuePair, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::explicit_constructor_invocation_stmt::ExplicitConstructorInvocationStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::local_class_declaration_stmt::LocalClassDeclarationStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::local_record_declaration_stmt::LocalRecordDeclarationStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::assert_stmt::AssertStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::labeled_stmt::LabeledStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::empty_stmt::EmptyStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::expression_stmt::ExpressionStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::switch_stmt::SwitchStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::switch_entry::SwitchEntry, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::break_stmt::BreakStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::return_stmt::ReturnStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::if_stmt::IfStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::while_stmt::WhileStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::continue_stmt::ContinueStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::do_stmt::DoStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::for_each_stmt::ForEachStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::for_stmt::ForStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::throw_stmt::ThrowStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::synchronized_stmt::SynchronizedStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::try_stmt::TryStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::catch_clause::CatchClause, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::lambda_expr::LambdaExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::method_reference_expr::MethodReferenceExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::type_expr::TypeExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::import_declaration::ImportDeclaration, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::node_list::NodeList, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_requires_directive::ModuleRequiresDirective, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_exports_directive::ModuleExportsDirective, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_provides_directive::ModuleProvidesDirective, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_uses_directive::ModuleUsesDirective, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_opens_directive::ModuleOpensDirective, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::unparsable_stmt::UnparsableStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::var_type::VarType, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modifier::Modifier, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::switch_expr::SwitchExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::yield_stmt::YieldStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::text_block_literal_expr::TextBlockLiteralExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::type_pattern_expr::TypePatternExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::record_declaration::RecordDeclaration, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::compact_constructor_declaration::CompactConstructorDeclaration, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::record_pattern_expr::RecordPatternExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::match_all_pattern_expr::MatchAllPatternExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::markdown_comment::MarkdownComment, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return n == arg;
	}
}

impl com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor for ObjectIdentityEqualsVisitor {}