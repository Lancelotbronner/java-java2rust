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

pub struct NoCommentHashCodeVisitor;

impl NoCommentHashCodeVisitor {
	static SINGLETON: com::github::javaparser::ast::visitor::no_comment_hash_code_visitor::NoCommentHashCodeVisitor = NoCommentHashCodeVisitor::new();

	pub fn hash_code(&self, node: &com::github::javaparser::ast::node::Node) -> i32 {
		return node.accept(self.SINGLETON, null);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::annotation_declaration::AnnotationDeclaration, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_members().accept(self, arg)) * 31 + (n.get_modifiers().accept(self, arg)) * 31 + (n.get_name().accept(self, arg)) * 31 + (n.get_annotations().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::annotation_member_declaration::AnnotationMemberDeclaration, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return ( if n.get_default_value().isPresent() { n.get_default_value().get().accept(self, arg) } else { 0 }) * 31 + (n.get_modifiers().accept(self, arg)) * 31 + (n.get_name().accept(self, arg)) * 31 + (n.get_type().accept(self, arg)) * 31 + (n.get_annotations().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_access_expr::ArrayAccessExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_index().accept(self, arg)) * 31 + (n.get_name().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_creation_expr::ArrayCreationExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_element_type().accept(self, arg)) * 31 + ( if n.get_initializer().isPresent() { n.get_initializer().get().accept(self, arg) } else { 0 }) * 31 + (n.get_levels().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::array_creation_level::ArrayCreationLevel, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_annotations().accept(self, arg)) * 31 + ( if n.get_dimension().isPresent() { n.get_dimension().get().accept(self, arg) } else { 0 });
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_values().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::array_type::ArrayType, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_component_type().accept(self, arg)) * 31 + (n.get_origin().hashCode()) * 31 + (n.get_annotations().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::assert_stmt::AssertStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_check().accept(self, arg)) * 31 + ( if n.get_message().isPresent() { n.get_message().get().accept(self, arg) } else { 0 });
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::assign_expr::AssignExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_operator().hashCode()) * 31 + (n.get_target().accept(self, arg)) * 31 + (n.get_value().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::binary_expr::BinaryExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_left().accept(self, arg)) * 31 + (n.get_operator().hashCode()) * 31 + (n.get_right().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::block_comment::BlockComment, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return 0;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_statements().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::boolean_literal_expr::BooleanLiteralExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return ( if n.is_value() { 1 } else { 0 });
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::break_stmt::BreakStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return ( if n.get_label().isPresent() { n.get_label().get().accept(self, arg) } else { 0 });
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::cast_expr::CastExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_expression().accept(self, arg)) * 31 + (n.get_type().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::catch_clause::CatchClause, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_body().accept(self, arg)) * 31 + (n.get_parameter().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::char_literal_expr::CharLiteralExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_value().hashCode());
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::class_expr::ClassExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_type().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_extended_types().accept(self, arg)) * 31 + (n.get_implemented_types().accept(self, arg)) * 31 + ( if n.is_compact() { 1 } else { 0 }) * 31 + ( if n.is_interface() { 1 } else { 0 }) * 31 + (n.get_permitted_types().accept(self, arg)) * 31 + (n.get_type_parameters().accept(self, arg)) * 31 + (n.get_members().accept(self, arg)) * 31 + (n.get_modifiers().accept(self, arg)) * 31 + (n.get_name().accept(self, arg)) * 31 + (n.get_annotations().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_name().accept(self, arg)) * 31 + ( if n.get_scope().isPresent() { n.get_scope().get().accept(self, arg) } else { 0 }) * 31 + ( if n.get_type_arguments().isPresent() { n.get_type_arguments().get().accept(self, arg) } else { 0 }) * 31 + (n.get_annotations().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::compilation_unit::CompilationUnit, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_imports().accept(self, arg)) * 31 + ( if n.get_module().isPresent() { n.get_module().get().accept(self, arg) } else { 0 }) * 31 + ( if n.get_package_declaration().isPresent() { n.get_package_declaration().get().accept(self, arg) } else { 0 }) * 31 + (n.get_types().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::conditional_expr::ConditionalExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_condition().accept(self, arg)) * 31 + (n.get_else_expr().accept(self, arg)) * 31 + (n.get_then_expr().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_body().accept(self, arg)) * 31 + (n.get_modifiers().accept(self, arg)) * 31 + (n.get_name().accept(self, arg)) * 31 + (n.get_parameters().accept(self, arg)) * 31 + ( if n.get_receiver_parameter().isPresent() { n.get_receiver_parameter().get().accept(self, arg) } else { 0 }) * 31 + (n.get_thrown_exceptions().accept(self, arg)) * 31 + (n.get_type_parameters().accept(self, arg)) * 31 + (n.get_annotations().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::continue_stmt::ContinueStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return ( if n.get_label().isPresent() { n.get_label().get().accept(self, arg) } else { 0 });
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::do_stmt::DoStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_body().accept(self, arg)) * 31 + (n.get_condition().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::double_literal_expr::DoubleLiteralExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_value().hashCode());
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::empty_stmt::EmptyStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return 0;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::enclosed_expr::EnclosedExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_inner().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::enum_constant_declaration::EnumConstantDeclaration, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_arguments().accept(self, arg)) * 31 + (n.get_class_body().accept(self, arg)) * 31 + (n.get_name().accept(self, arg)) * 31 + (n.get_annotations().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::enum_declaration::EnumDeclaration, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_entries().accept(self, arg)) * 31 + (n.get_implemented_types().accept(self, arg)) * 31 + (n.get_members().accept(self, arg)) * 31 + (n.get_modifiers().accept(self, arg)) * 31 + (n.get_name().accept(self, arg)) * 31 + (n.get_annotations().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::explicit_constructor_invocation_stmt::ExplicitConstructorInvocationStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_arguments().accept(self, arg)) * 31 + ( if n.get_expression().isPresent() { n.get_expression().get().accept(self, arg) } else { 0 }) * 31 + ( if n.is_this() { 1 } else { 0 }) * 31 + ( if n.get_type_arguments().isPresent() { n.get_type_arguments().get().accept(self, arg) } else { 0 });
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::expression_stmt::ExpressionStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_expression().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::field_access_expr::FieldAccessExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_name().accept(self, arg)) * 31 + (n.get_scope().accept(self, arg)) * 31 + ( if n.get_type_arguments().isPresent() { n.get_type_arguments().get().accept(self, arg) } else { 0 });
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::field_declaration::FieldDeclaration, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_modifiers().accept(self, arg)) * 31 + (n.get_variables().accept(self, arg)) * 31 + (n.get_annotations().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::for_stmt::ForStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_body().accept(self, arg)) * 31 + ( if n.get_compare().isPresent() { n.get_compare().get().accept(self, arg) } else { 0 }) * 31 + (n.get_initialization().accept(self, arg)) * 31 + (n.get_update().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::for_each_stmt::ForEachStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_body().accept(self, arg)) * 31 + (n.get_iterable().accept(self, arg)) * 31 + (n.get_variable().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::if_stmt::IfStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_condition().accept(self, arg)) * 31 + ( if n.get_else_stmt().isPresent() { n.get_else_stmt().get().accept(self, arg) } else { 0 }) * 31 + (n.get_then_stmt().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::import_declaration::ImportDeclaration, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return ( if n.is_asterisk() { 1 } else { 0 }) * 31 + ( if n.is_module() { 1 } else { 0 }) * 31 + ( if n.is_static() { 1 } else { 0 }) * 31 + (n.get_name().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::initializer_declaration::InitializerDeclaration, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_body().accept(self, arg)) * 31 + ( if n.is_static() { 1 } else { 0 }) * 31 + (n.get_annotations().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::instance_of_expr::InstanceOfExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_expression().accept(self, arg)) * 31 + ( if n.get_pattern().isPresent() { n.get_pattern().get().accept(self, arg) } else { 0 }) * 31 + (n.get_type().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::integer_literal_expr::IntegerLiteralExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_value().hashCode());
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::intersection_type::IntersectionType, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_elements().accept(self, arg)) * 31 + (n.get_annotations().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::traditional_javadoc_comment::TraditionalJavadocComment, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return 0;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::labeled_stmt::LabeledStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_label().accept(self, arg)) * 31 + (n.get_statement().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::lambda_expr::LambdaExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_body().accept(self, arg)) * 31 + ( if n.is_enclosing_parameters() { 1 } else { 0 }) * 31 + (n.get_parameters().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::line_comment::LineComment, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return 0;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::local_class_declaration_stmt::LocalClassDeclarationStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_class_declaration().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::local_record_declaration_stmt::LocalRecordDeclarationStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_record_declaration().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::long_literal_expr::LongLiteralExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_value().hashCode());
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::marker_annotation_expr::MarkerAnnotationExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_name().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::member_value_pair::MemberValuePair, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_name().accept(self, arg)) * 31 + (n.get_value().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_arguments().accept(self, arg)) * 31 + (n.get_name().accept(self, arg)) * 31 + ( if n.get_scope().isPresent() { n.get_scope().get().accept(self, arg) } else { 0 }) * 31 + ( if n.get_type_arguments().isPresent() { n.get_type_arguments().get().accept(self, arg) } else { 0 });
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::method_declaration::MethodDeclaration, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return ( if n.get_body().isPresent() { n.get_body().get().accept(self, arg) } else { 0 }) * 31 + (n.get_type().accept(self, arg)) * 31 + (n.get_modifiers().accept(self, arg)) * 31 + (n.get_name().accept(self, arg)) * 31 + (n.get_parameters().accept(self, arg)) * 31 + ( if n.get_receiver_parameter().isPresent() { n.get_receiver_parameter().get().accept(self, arg) } else { 0 }) * 31 + (n.get_thrown_exceptions().accept(self, arg)) * 31 + (n.get_type_parameters().accept(self, arg)) * 31 + (n.get_annotations().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::method_reference_expr::MethodReferenceExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_identifier().hashCode()) * 31 + (n.get_scope().accept(self, arg)) * 31 + ( if n.get_type_arguments().isPresent() { n.get_type_arguments().get().accept(self, arg) } else { 0 });
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name_expr::NameExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_name().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name::Name, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_identifier().hashCode()) * 31 + ( if n.get_qualifier().isPresent() { n.get_qualifier().get().accept(self, arg) } else { 0 });
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::node_list::NodeList, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		let result: i32 = 0;
		for node in n {
			result += 31 * (node as Visitable).accept(self, arg);
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::normal_annotation_expr::NormalAnnotationExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_pairs().accept(self, arg)) * 31 + (n.get_name().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::null_literal_expr::NullLiteralExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return 0;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::object_creation_expr::ObjectCreationExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return ( if n.get_anonymous_class_body().isPresent() { n.get_anonymous_class_body().get().accept(self, arg) } else { 0 }) * 31 + (n.get_arguments().accept(self, arg)) * 31 + ( if n.get_scope().isPresent() { n.get_scope().get().accept(self, arg) } else { 0 }) * 31 + (n.get_type().accept(self, arg)) * 31 + ( if n.get_type_arguments().isPresent() { n.get_type_arguments().get().accept(self, arg) } else { 0 });
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::package_declaration::PackageDeclaration, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_annotations().accept(self, arg)) * 31 + (n.get_name().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::parameter::Parameter, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_annotations().accept(self, arg)) * 31 + ( if n.is_var_args() { 1 } else { 0 }) * 31 + (n.get_modifiers().accept(self, arg)) * 31 + (n.get_name().accept(self, arg)) * 31 + (n.get_type().accept(self, arg)) * 31 + (n.get_var_args_annotations().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::primitive_type::PrimitiveType, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_type().hashCode()) * 31 + (n.get_annotations().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::return_stmt::ReturnStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return ( if n.get_expression().isPresent() { n.get_expression().get().accept(self, arg) } else { 0 });
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::simple_name::SimpleName, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_identifier().hashCode());
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::single_member_annotation_expr::SingleMemberAnnotationExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_member_value().accept(self, arg)) * 31 + (n.get_name().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::string_literal_expr::StringLiteralExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_value().hashCode());
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::super_expr::SuperExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return ( if n.get_type_name().isPresent() { n.get_type_name().get().accept(self, arg) } else { 0 });
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::switch_entry::SwitchEntry, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return ( if n.get_guard().isPresent() { n.get_guard().get().accept(self, arg) } else { 0 }) * 31 + ( if n.is_default() { 1 } else { 0 }) * 31 + (n.get_labels().accept(self, arg)) * 31 + (n.get_statements().accept(self, arg)) * 31 + (n.get_type().hashCode());
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::switch_stmt::SwitchStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_entries().accept(self, arg)) * 31 + (n.get_selector().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::synchronized_stmt::SynchronizedStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_body().accept(self, arg)) * 31 + (n.get_expression().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::this_expr::ThisExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return ( if n.get_type_name().isPresent() { n.get_type_name().get().accept(self, arg) } else { 0 });
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::throw_stmt::ThrowStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_expression().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::try_stmt::TryStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_catch_clauses().accept(self, arg)) * 31 + ( if n.get_finally_block().isPresent() { n.get_finally_block().get().accept(self, arg) } else { 0 }) * 31 + (n.get_resources().accept(self, arg)) * 31 + (n.get_try_block().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::type_expr::TypeExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_type().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::type_parameter::TypeParameter, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_name().accept(self, arg)) * 31 + (n.get_type_bound().accept(self, arg)) * 31 + (n.get_annotations().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::unary_expr::UnaryExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_expression().accept(self, arg)) * 31 + (n.get_operator().hashCode());
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::union_type::UnionType, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_elements().accept(self, arg)) * 31 + (n.get_annotations().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::unknown_type::UnknownType, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_annotations().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_annotations().accept(self, arg)) * 31 + (n.get_modifiers().accept(self, arg)) * 31 + (n.get_variables().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::variable_declarator::VariableDeclarator, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return ( if n.get_initializer().isPresent() { n.get_initializer().get().accept(self, arg) } else { 0 }) * 31 + (n.get_name().accept(self, arg)) * 31 + (n.get_type().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::void_type::VoidType, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_annotations().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::while_stmt::WhileStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_body().accept(self, arg)) * 31 + (n.get_condition().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::wildcard_type::WildcardType, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return ( if n.get_extended_type().isPresent() { n.get_extended_type().get().accept(self, arg) } else { 0 }) * 31 + ( if n.get_super_type().isPresent() { n.get_super_type().get().accept(self, arg) } else { 0 }) * 31 + (n.get_annotations().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_annotations().accept(self, arg)) * 31 + (n.get_directives().accept(self, arg)) * 31 + ( if n.is_open() { 1 } else { 0 }) * 31 + (n.get_name().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_requires_directive::ModuleRequiresDirective, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_modifiers().accept(self, arg)) * 31 + (n.get_name().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_exports_directive::ModuleExportsDirective, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_module_names().accept(self, arg)) * 31 + (n.get_name().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_provides_directive::ModuleProvidesDirective, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_name().accept(self, arg)) * 31 + (n.get_with().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_uses_directive::ModuleUsesDirective, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_name().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_opens_directive::ModuleOpensDirective, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_module_names().accept(self, arg)) * 31 + (n.get_name().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::unparsable_stmt::UnparsableStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return 0;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_annotations().accept(self, arg)) * 31 + (n.get_name().accept(self, arg)) * 31 + (n.get_type().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::var_type::VarType, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_annotations().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modifier::Modifier, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_keyword().hashCode());
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::switch_expr::SwitchExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_entries().accept(self, arg)) * 31 + (n.get_selector().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::yield_stmt::YieldStmt, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_expression().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::text_block_literal_expr::TextBlockLiteralExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_value().hashCode());
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::type_pattern_expr::TypePatternExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_modifiers().accept(self, arg)) * 31 + (n.get_name().accept(self, arg)) * 31 + (n.get_type().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::record_declaration::RecordDeclaration, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_implemented_types().accept(self, arg)) * 31 + (n.get_parameters().accept(self, arg)) * 31 + ( if n.get_receiver_parameter().isPresent() { n.get_receiver_parameter().get().accept(self, arg) } else { 0 }) * 31 + (n.get_type_parameters().accept(self, arg)) * 31 + (n.get_members().accept(self, arg)) * 31 + (n.get_modifiers().accept(self, arg)) * 31 + (n.get_name().accept(self, arg)) * 31 + (n.get_annotations().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::compact_constructor_declaration::CompactConstructorDeclaration, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_body().accept(self, arg)) * 31 + (n.get_modifiers().accept(self, arg)) * 31 + (n.get_name().accept(self, arg)) * 31 + (n.get_thrown_exceptions().accept(self, arg)) * 31 + (n.get_type_parameters().accept(self, arg)) * 31 + (n.get_annotations().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::record_pattern_expr::RecordPatternExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.IllegalStateException) */ -> /* Java */ java::lang::Integer /**/ {
		return (n.get_modifiers().accept(self, arg)) * 31 + (n.get_pattern_list().accept(self, arg)) * 31 + (n.get_type()?.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::match_all_pattern_expr::MatchAllPatternExpr, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_modifiers().accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::markdown_comment::MarkdownComment, arg: &/* Java */ java::lang::Void /**/) -> /* Java */ java::lang::Integer /**/ {
		return (n.get_content().hashCode());
	}
}

impl com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor for NoCommentHashCodeVisitor {}