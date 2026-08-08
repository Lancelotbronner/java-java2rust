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

pub struct VoidVisitorAdapter<A>;

impl<A> VoidVisitorAdapter {
	pub fn visit(&self, n: &com::github::javaparser::ast::body::annotation_declaration::AnnotationDeclaration, arg: &A) {
		n.get_members().forEach(|p|p.accept(self, arg));
		n.get_modifiers().forEach(|p|p.accept(self, arg));
		n.get_name().accept(self, arg);
		n.get_annotations().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::annotation_member_declaration::AnnotationMemberDeclaration, arg: &A) {
		n.get_default_value().ifPresent(|l|l.accept(self, arg));
		n.get_modifiers().forEach(|p|p.accept(self, arg));
		n.get_name().accept(self, arg);
		n.get_type().accept(self, arg);
		n.get_annotations().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_access_expr::ArrayAccessExpr, arg: &A) {
		n.get_index().accept(self, arg);
		n.get_name().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_creation_expr::ArrayCreationExpr, arg: &A) {
		n.get_element_type().accept(self, arg);
		n.get_initializer().ifPresent(|l|l.accept(self, arg));
		n.get_levels().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr, arg: &A) {
		n.get_values().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::assert_stmt::AssertStmt, arg: &A) {
		n.get_check().accept(self, arg);
		n.get_message().ifPresent(|l|l.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::assign_expr::AssignExpr, arg: &A) {
		n.get_target().accept(self, arg);
		n.get_value().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::binary_expr::BinaryExpr, arg: &A) {
		n.get_left().accept(self, arg);
		n.get_right().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::block_comment::BlockComment, arg: &A) {
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt, arg: &A) {
		n.get_statements().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::boolean_literal_expr::BooleanLiteralExpr, arg: &A) {
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::break_stmt::BreakStmt, arg: &A) {
		n.get_label().ifPresent(|l|l.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::cast_expr::CastExpr, arg: &A) {
		n.get_expression().accept(self, arg);
		n.get_type().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::catch_clause::CatchClause, arg: &A) {
		n.get_body().accept(self, arg);
		n.get_parameter().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::char_literal_expr::CharLiteralExpr, arg: &A) {
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::class_expr::ClassExpr, arg: &A) {
		n.get_type().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration, arg: &A) {
		n.get_extended_types().forEach(|p|p.accept(self, arg));
		n.get_implemented_types().forEach(|p|p.accept(self, arg));
		n.get_permitted_types().forEach(|p|p.accept(self, arg));
		n.get_type_parameters().forEach(|p|p.accept(self, arg));
		n.get_members().forEach(|p|p.accept(self, arg));
		n.get_modifiers().forEach(|p|p.accept(self, arg));
		n.get_name().accept(self, arg);
		n.get_annotations().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType, arg: &A) {
		n.get_name().accept(self, arg);
		n.get_scope().ifPresent(|l|l.accept(self, arg));
		n.get_type_arguments().ifPresent(|l|l.forEach(|v|v.accept(self, arg)));
		n.get_annotations().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::compilation_unit::CompilationUnit, arg: &A) {
		n.get_imports().forEach(|p|p.accept(self, arg));
		n.get_module().ifPresent(|l|l.accept(self, arg));
		n.get_package_declaration().ifPresent(|l|l.accept(self, arg));
		n.get_types().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::conditional_expr::ConditionalExpr, arg: &A) {
		n.get_condition().accept(self, arg);
		n.get_else_expr().accept(self, arg);
		n.get_then_expr().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration, arg: &A) {
		n.get_body().accept(self, arg);
		n.get_modifiers().forEach(|p|p.accept(self, arg));
		n.get_name().accept(self, arg);
		n.get_parameters().forEach(|p|p.accept(self, arg));
		n.get_receiver_parameter().ifPresent(|l|l.accept(self, arg));
		n.get_thrown_exceptions().forEach(|p|p.accept(self, arg));
		n.get_type_parameters().forEach(|p|p.accept(self, arg));
		n.get_annotations().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::continue_stmt::ContinueStmt, arg: &A) {
		n.get_label().ifPresent(|l|l.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::do_stmt::DoStmt, arg: &A) {
		n.get_body().accept(self, arg);
		n.get_condition().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::double_literal_expr::DoubleLiteralExpr, arg: &A) {
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::empty_stmt::EmptyStmt, arg: &A) {
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::enclosed_expr::EnclosedExpr, arg: &A) {
		n.get_inner().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::enum_constant_declaration::EnumConstantDeclaration, arg: &A) {
		n.get_arguments().forEach(|p|p.accept(self, arg));
		n.get_class_body().forEach(|p|p.accept(self, arg));
		n.get_name().accept(self, arg);
		n.get_annotations().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::enum_declaration::EnumDeclaration, arg: &A) {
		n.get_entries().forEach(|p|p.accept(self, arg));
		n.get_implemented_types().forEach(|p|p.accept(self, arg));
		n.get_members().forEach(|p|p.accept(self, arg));
		n.get_modifiers().forEach(|p|p.accept(self, arg));
		n.get_name().accept(self, arg);
		n.get_annotations().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::explicit_constructor_invocation_stmt::ExplicitConstructorInvocationStmt, arg: &A) {
		n.get_arguments().forEach(|p|p.accept(self, arg));
		n.get_expression().ifPresent(|l|l.accept(self, arg));
		n.get_type_arguments().ifPresent(|l|l.forEach(|v|v.accept(self, arg)));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::expression_stmt::ExpressionStmt, arg: &A) {
		n.get_expression().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::field_access_expr::FieldAccessExpr, arg: &A) {
		n.get_name().accept(self, arg);
		n.get_scope().accept(self, arg);
		n.get_type_arguments().ifPresent(|l|l.forEach(|v|v.accept(self, arg)));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::field_declaration::FieldDeclaration, arg: &A) {
		n.get_modifiers().forEach(|p|p.accept(self, arg));
		n.get_variables().forEach(|p|p.accept(self, arg));
		n.get_annotations().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::for_each_stmt::ForEachStmt, arg: &A) {
		n.get_body().accept(self, arg);
		n.get_iterable().accept(self, arg);
		n.get_variable().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::for_stmt::ForStmt, arg: &A) {
		n.get_body().accept(self, arg);
		n.get_compare().ifPresent(|l|l.accept(self, arg));
		n.get_initialization().forEach(|p|p.accept(self, arg));
		n.get_update().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::if_stmt::IfStmt, arg: &A) {
		n.get_condition().accept(self, arg);
		n.get_else_stmt().ifPresent(|l|l.accept(self, arg));
		n.get_then_stmt().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::initializer_declaration::InitializerDeclaration, arg: &A) {
		n.get_body().accept(self, arg);
		n.get_annotations().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::instance_of_expr::InstanceOfExpr, arg: &A) {
		n.get_expression().accept(self, arg);
		n.get_pattern().ifPresent(|l|l.accept(self, arg));
		n.get_type().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::integer_literal_expr::IntegerLiteralExpr, arg: &A) {
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::traditional_javadoc_comment::TraditionalJavadocComment, arg: &A) {
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::labeled_stmt::LabeledStmt, arg: &A) {
		n.get_label().accept(self, arg);
		n.get_statement().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::line_comment::LineComment, arg: &A) {
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::long_literal_expr::LongLiteralExpr, arg: &A) {
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::marker_annotation_expr::MarkerAnnotationExpr, arg: &A) {
		n.get_name().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::member_value_pair::MemberValuePair, arg: &A) {
		n.get_name().accept(self, arg);
		n.get_value().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr, arg: &A) {
		n.get_arguments().forEach(|p|p.accept(self, arg));
		n.get_name().accept(self, arg);
		n.get_scope().ifPresent(|l|l.accept(self, arg));
		n.get_type_arguments().ifPresent(|l|l.forEach(|v|v.accept(self, arg)));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::method_declaration::MethodDeclaration, arg: &A) {
		n.get_body().ifPresent(|l|l.accept(self, arg));
		n.get_type().accept(self, arg);
		n.get_modifiers().forEach(|p|p.accept(self, arg));
		n.get_name().accept(self, arg);
		n.get_parameters().forEach(|p|p.accept(self, arg));
		n.get_receiver_parameter().ifPresent(|l|l.accept(self, arg));
		n.get_thrown_exceptions().forEach(|p|p.accept(self, arg));
		n.get_type_parameters().forEach(|p|p.accept(self, arg));
		n.get_annotations().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name_expr::NameExpr, arg: &A) {
		n.get_name().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::normal_annotation_expr::NormalAnnotationExpr, arg: &A) {
		n.get_pairs().forEach(|p|p.accept(self, arg));
		n.get_name().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::null_literal_expr::NullLiteralExpr, arg: &A) {
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::object_creation_expr::ObjectCreationExpr, arg: &A) {
		n.get_anonymous_class_body().ifPresent(|l|l.forEach(|v|v.accept(self, arg)));
		n.get_arguments().forEach(|p|p.accept(self, arg));
		n.get_scope().ifPresent(|l|l.accept(self, arg));
		n.get_type().accept(self, arg);
		n.get_type_arguments().ifPresent(|l|l.forEach(|v|v.accept(self, arg)));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::package_declaration::PackageDeclaration, arg: &A) {
		n.get_annotations().forEach(|p|p.accept(self, arg));
		n.get_name().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::parameter::Parameter, arg: &A) {
		n.get_annotations().forEach(|p|p.accept(self, arg));
		n.get_modifiers().forEach(|p|p.accept(self, arg));
		n.get_name().accept(self, arg);
		n.get_type().accept(self, arg);
		n.get_var_args_annotations().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::primitive_type::PrimitiveType, arg: &A) {
		n.get_annotations().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name::Name, arg: &A) {
		n.get_qualifier().ifPresent(|l|l.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::simple_name::SimpleName, arg: &A) {
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::array_type::ArrayType, arg: &A) {
		n.get_component_type().accept(self, arg);
		n.get_annotations().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::array_creation_level::ArrayCreationLevel, arg: &A) {
		n.get_annotations().forEach(|p|p.accept(self, arg));
		n.get_dimension().ifPresent(|l|l.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::intersection_type::IntersectionType, arg: &A) {
		n.get_elements().forEach(|p|p.accept(self, arg));
		n.get_annotations().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::union_type::UnionType, arg: &A) {
		n.get_elements().forEach(|p|p.accept(self, arg));
		n.get_annotations().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::return_stmt::ReturnStmt, arg: &A) {
		n.get_expression().ifPresent(|l|l.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::single_member_annotation_expr::SingleMemberAnnotationExpr, arg: &A) {
		n.get_member_value().accept(self, arg);
		n.get_name().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::string_literal_expr::StringLiteralExpr, arg: &A) {
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::super_expr::SuperExpr, arg: &A) {
		n.get_type_name().ifPresent(|l|l.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::switch_entry::SwitchEntry, arg: &A) {
		n.get_guard().ifPresent(|l|l.accept(self, arg));
		n.get_labels().forEach(|p|p.accept(self, arg));
		n.get_statements().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::switch_stmt::SwitchStmt, arg: &A) {
		n.get_entries().forEach(|p|p.accept(self, arg));
		n.get_selector().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::synchronized_stmt::SynchronizedStmt, arg: &A) {
		n.get_body().accept(self, arg);
		n.get_expression().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::this_expr::ThisExpr, arg: &A) {
		n.get_type_name().ifPresent(|l|l.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::throw_stmt::ThrowStmt, arg: &A) {
		n.get_expression().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::try_stmt::TryStmt, arg: &A) {
		n.get_catch_clauses().forEach(|p|p.accept(self, arg));
		n.get_finally_block().ifPresent(|l|l.accept(self, arg));
		n.get_resources().forEach(|p|p.accept(self, arg));
		n.get_try_block().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::local_class_declaration_stmt::LocalClassDeclarationStmt, arg: &A) {
		n.get_class_declaration().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::local_record_declaration_stmt::LocalRecordDeclarationStmt, arg: &A) {
		n.get_record_declaration().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::type_parameter::TypeParameter, arg: &A) {
		n.get_name().accept(self, arg);
		n.get_type_bound().forEach(|p|p.accept(self, arg));
		n.get_annotations().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::unary_expr::UnaryExpr, arg: &A) {
		n.get_expression().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::unknown_type::UnknownType, arg: &A) {
		n.get_annotations().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr, arg: &A) {
		n.get_annotations().forEach(|p|p.accept(self, arg));
		n.get_modifiers().forEach(|p|p.accept(self, arg));
		n.get_variables().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::variable_declarator::VariableDeclarator, arg: &A) {
		n.get_initializer().ifPresent(|l|l.accept(self, arg));
		n.get_name().accept(self, arg);
		n.get_type().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::void_type::VoidType, arg: &A) {
		n.get_annotations().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::while_stmt::WhileStmt, arg: &A) {
		n.get_body().accept(self, arg);
		n.get_condition().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::wildcard_type::WildcardType, arg: &A) {
		n.get_extended_type().ifPresent(|l|l.accept(self, arg));
		n.get_super_type().ifPresent(|l|l.accept(self, arg));
		n.get_annotations().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::lambda_expr::LambdaExpr, arg: &A) {
		n.get_body().accept(self, arg);
		n.get_parameters().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::method_reference_expr::MethodReferenceExpr, arg: &A) {
		n.get_scope().accept(self, arg);
		n.get_type_arguments().ifPresent(|l|l.forEach(|v|v.accept(self, arg)));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::type_expr::TypeExpr, arg: &A) {
		n.get_type().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::node_list::NodeList, arg: &A) {
		for node in n {
			(node as Node).accept(self, arg);
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::import_declaration::ImportDeclaration, arg: &A) {
		n.get_name().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration, arg: &A) {
		n.get_annotations().forEach(|p|p.accept(self, arg));
		n.get_directives().forEach(|p|p.accept(self, arg));
		n.get_name().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_requires_directive::ModuleRequiresDirective, arg: &A) {
		n.get_modifiers().forEach(|p|p.accept(self, arg));
		n.get_name().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_exports_directive::ModuleExportsDirective, arg: &A) {
		n.get_module_names().forEach(|p|p.accept(self, arg));
		n.get_name().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_provides_directive::ModuleProvidesDirective, arg: &A) {
		n.get_name().accept(self, arg);
		n.get_with().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_uses_directive::ModuleUsesDirective, arg: &A) {
		n.get_name().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_opens_directive::ModuleOpensDirective, arg: &A) {
		n.get_module_names().forEach(|p|p.accept(self, arg));
		n.get_name().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::unparsable_stmt::UnparsableStmt, arg: &A) {
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter, arg: &A) {
		n.get_annotations().forEach(|p|p.accept(self, arg));
		n.get_name().accept(self, arg);
		n.get_type().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::var_type::VarType, arg: &A) {
		n.get_annotations().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modifier::Modifier, arg: &A) {
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::switch_expr::SwitchExpr, arg: &A) {
		n.get_entries().forEach(|p|p.accept(self, arg));
		n.get_selector().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::text_block_literal_expr::TextBlockLiteralExpr, arg: &A) {
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::yield_stmt::YieldStmt, arg: &A) {
		n.get_expression().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::type_pattern_expr::TypePatternExpr, arg: &A) {
		n.get_modifiers().forEach(|p|p.accept(self, arg));
		n.get_name().accept(self, arg);
		n.get_type().accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::record_declaration::RecordDeclaration, arg: &A) {
		n.get_implemented_types().forEach(|p|p.accept(self, arg));
		n.get_parameters().forEach(|p|p.accept(self, arg));
		n.get_receiver_parameter().ifPresent(|l|l.accept(self, arg));
		n.get_type_parameters().forEach(|p|p.accept(self, arg));
		n.get_members().forEach(|p|p.accept(self, arg));
		n.get_modifiers().forEach(|p|p.accept(self, arg));
		n.get_name().accept(self, arg);
		n.get_annotations().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::compact_constructor_declaration::CompactConstructorDeclaration, arg: &A) {
		n.get_body().accept(self, arg);
		n.get_modifiers().forEach(|p|p.accept(self, arg));
		n.get_name().accept(self, arg);
		n.get_thrown_exceptions().forEach(|p|p.accept(self, arg));
		n.get_type_parameters().forEach(|p|p.accept(self, arg));
		n.get_annotations().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::record_pattern_expr::RecordPatternExpr, arg: &A) /* thrown(java.lang.IllegalStateException) */ {
		n.get_modifiers().forEach(|p|p.accept(self, arg));
		n.get_pattern_list().forEach(|p|p.accept(self, arg));
		n.get_type()?.accept(self, arg);
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::match_all_pattern_expr::MatchAllPatternExpr, arg: &A) {
		n.get_modifiers().forEach(|p|p.accept(self, arg));
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::markdown_comment::MarkdownComment, arg: &A) {
		n.get_comment().ifPresent(|l|l.accept(self, arg));
	}
}

impl<A> com::github::javaparser::ast::visitor::void_visitor::VoidVisitor for VoidVisitorAdapter<A> {}