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

pub struct GenericVisitorAdapter<R, A>;

impl<R, A> GenericVisitorAdapter {
	pub fn visit(&self, n: &com::github::javaparser::ast::body::annotation_declaration::AnnotationDeclaration, arg: &A) -> R {
		let result: R;
		{
			result = n.get_members().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_modifiers().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_annotations().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::annotation_member_declaration::AnnotationMemberDeclaration, arg: &A) -> R {
		let result: R;
		if n.get_default_value().isPresent() {
			result = n.get_default_value().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_modifiers().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_type().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_annotations().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_access_expr::ArrayAccessExpr, arg: &A) -> R {
		let result: R;
		{
			result = n.get_index().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_creation_expr::ArrayCreationExpr, arg: &A) -> R {
		let result: R;
		{
			result = n.get_element_type().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_initializer().isPresent() {
			result = n.get_initializer().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_levels().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr, arg: &A) -> R {
		let result: R;
		{
			result = n.get_values().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::assert_stmt::AssertStmt, arg: &A) -> R {
		let result: R;
		{
			result = n.get_check().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_message().isPresent() {
			result = n.get_message().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::assign_expr::AssignExpr, arg: &A) -> R {
		let result: R;
		{
			result = n.get_target().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_value().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::binary_expr::BinaryExpr, arg: &A) -> R {
		let result: R;
		{
			result = n.get_left().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_right().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt, arg: &A) -> R {
		let result: R;
		{
			result = n.get_statements().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::boolean_literal_expr::BooleanLiteralExpr, arg: &A) -> R {
		let result: R;
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::break_stmt::BreakStmt, arg: &A) -> R {
		let result: R;
		if n.get_label().isPresent() {
			result = n.get_label().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::cast_expr::CastExpr, arg: &A) -> R {
		let result: R;
		{
			result = n.get_expression().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_type().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::catch_clause::CatchClause, arg: &A) -> R {
		let result: R;
		{
			result = n.get_body().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_parameter().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::char_literal_expr::CharLiteralExpr, arg: &A) -> R {
		let result: R;
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::class_expr::ClassExpr, arg: &A) -> R {
		let result: R;
		{
			result = n.get_type().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration, arg: &A) -> R {
		let result: R;
		{
			result = n.get_extended_types().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_implemented_types().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_permitted_types().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_type_parameters().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_members().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_modifiers().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_annotations().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType, arg: &A) -> R {
		let result: R;
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_scope().isPresent() {
			result = n.get_scope().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_type_arguments().isPresent() {
			result = n.get_type_arguments().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_annotations().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::compilation_unit::CompilationUnit, arg: &A) -> R {
		let result: R;
		{
			result = n.get_imports().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_module().isPresent() {
			result = n.get_module().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_package_declaration().isPresent() {
			result = n.get_package_declaration().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_types().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::conditional_expr::ConditionalExpr, arg: &A) -> R {
		let result: R;
		{
			result = n.get_condition().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_else_expr().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_then_expr().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration, arg: &A) -> R {
		let result: R;
		{
			result = n.get_body().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_modifiers().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_parameters().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_receiver_parameter().isPresent() {
			result = n.get_receiver_parameter().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_thrown_exceptions().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_type_parameters().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_annotations().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::continue_stmt::ContinueStmt, arg: &A) -> R {
		let result: R;
		if n.get_label().isPresent() {
			result = n.get_label().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::do_stmt::DoStmt, arg: &A) -> R {
		let result: R;
		{
			result = n.get_body().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_condition().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::double_literal_expr::DoubleLiteralExpr, arg: &A) -> R {
		let result: R;
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::empty_stmt::EmptyStmt, arg: &A) -> R {
		let result: R;
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::enclosed_expr::EnclosedExpr, arg: &A) -> R {
		let result: R;
		{
			result = n.get_inner().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::enum_constant_declaration::EnumConstantDeclaration, arg: &A) -> R {
		let result: R;
		{
			result = n.get_arguments().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_class_body().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_annotations().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::enum_declaration::EnumDeclaration, arg: &A) -> R {
		let result: R;
		{
			result = n.get_entries().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_implemented_types().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_members().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_modifiers().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_annotations().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::explicit_constructor_invocation_stmt::ExplicitConstructorInvocationStmt, arg: &A) -> R {
		let result: R;
		{
			result = n.get_arguments().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_expression().isPresent() {
			result = n.get_expression().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_type_arguments().isPresent() {
			result = n.get_type_arguments().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::expression_stmt::ExpressionStmt, arg: &A) -> R {
		let result: R;
		{
			result = n.get_expression().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::field_access_expr::FieldAccessExpr, arg: &A) -> R {
		let result: R;
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_scope().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_type_arguments().isPresent() {
			result = n.get_type_arguments().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::field_declaration::FieldDeclaration, arg: &A) -> R {
		let result: R;
		{
			result = n.get_modifiers().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_variables().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_annotations().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::for_each_stmt::ForEachStmt, arg: &A) -> R {
		let result: R;
		{
			result = n.get_body().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_iterable().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_variable().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::for_stmt::ForStmt, arg: &A) -> R {
		let result: R;
		{
			result = n.get_body().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_compare().isPresent() {
			result = n.get_compare().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_initialization().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_update().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::if_stmt::IfStmt, arg: &A) -> R {
		let result: R;
		{
			result = n.get_condition().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_else_stmt().isPresent() {
			result = n.get_else_stmt().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_then_stmt().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::initializer_declaration::InitializerDeclaration, arg: &A) -> R {
		let result: R;
		{
			result = n.get_body().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_annotations().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::instance_of_expr::InstanceOfExpr, arg: &A) -> R {
		let result: R;
		{
			result = n.get_expression().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_pattern().isPresent() {
			result = n.get_pattern().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_type().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::integer_literal_expr::IntegerLiteralExpr, arg: &A) -> R {
		let result: R;
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::traditional_javadoc_comment::TraditionalJavadocComment, arg: &A) -> R {
		let result: R;
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::labeled_stmt::LabeledStmt, arg: &A) -> R {
		let result: R;
		{
			result = n.get_label().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_statement().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::long_literal_expr::LongLiteralExpr, arg: &A) -> R {
		let result: R;
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::marker_annotation_expr::MarkerAnnotationExpr, arg: &A) -> R {
		let result: R;
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::member_value_pair::MemberValuePair, arg: &A) -> R {
		let result: R;
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_value().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr, arg: &A) -> R {
		let result: R;
		{
			result = n.get_arguments().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_scope().isPresent() {
			result = n.get_scope().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_type_arguments().isPresent() {
			result = n.get_type_arguments().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::method_declaration::MethodDeclaration, arg: &A) -> R {
		let result: R;
		if n.get_body().isPresent() {
			result = n.get_body().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_type().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_modifiers().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_parameters().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_receiver_parameter().isPresent() {
			result = n.get_receiver_parameter().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_thrown_exceptions().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_type_parameters().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_annotations().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name_expr::NameExpr, arg: &A) -> R {
		let result: R;
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::normal_annotation_expr::NormalAnnotationExpr, arg: &A) -> R {
		let result: R;
		{
			result = n.get_pairs().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::null_literal_expr::NullLiteralExpr, arg: &A) -> R {
		let result: R;
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::object_creation_expr::ObjectCreationExpr, arg: &A) -> R {
		let result: R;
		if n.get_anonymous_class_body().isPresent() {
			result = n.get_anonymous_class_body().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_arguments().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_scope().isPresent() {
			result = n.get_scope().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_type().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_type_arguments().isPresent() {
			result = n.get_type_arguments().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::package_declaration::PackageDeclaration, arg: &A) -> R {
		let result: R;
		{
			result = n.get_annotations().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::parameter::Parameter, arg: &A) -> R {
		let result: R;
		{
			result = n.get_annotations().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_modifiers().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_type().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_var_args_annotations().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::primitive_type::PrimitiveType, arg: &A) -> R {
		let result: R;
		{
			result = n.get_annotations().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name::Name, arg: &A) -> R {
		let result: R;
		if n.get_qualifier().isPresent() {
			result = n.get_qualifier().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::simple_name::SimpleName, arg: &A) -> R {
		let result: R;
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::array_type::ArrayType, arg: &A) -> R {
		let result: R;
		{
			result = n.get_component_type().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_annotations().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::array_creation_level::ArrayCreationLevel, arg: &A) -> R {
		let result: R;
		{
			result = n.get_annotations().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_dimension().isPresent() {
			result = n.get_dimension().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::intersection_type::IntersectionType, arg: &A) -> R {
		let result: R;
		{
			result = n.get_elements().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_annotations().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::union_type::UnionType, arg: &A) -> R {
		let result: R;
		{
			result = n.get_elements().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_annotations().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::return_stmt::ReturnStmt, arg: &A) -> R {
		let result: R;
		if n.get_expression().isPresent() {
			result = n.get_expression().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::single_member_annotation_expr::SingleMemberAnnotationExpr, arg: &A) -> R {
		let result: R;
		{
			result = n.get_member_value().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::string_literal_expr::StringLiteralExpr, arg: &A) -> R {
		let result: R;
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::super_expr::SuperExpr, arg: &A) -> R {
		let result: R;
		if n.get_type_name().isPresent() {
			result = n.get_type_name().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::switch_entry::SwitchEntry, arg: &A) -> R {
		let result: R;
		if n.get_guard().isPresent() {
			result = n.get_guard().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_labels().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_statements().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::switch_stmt::SwitchStmt, arg: &A) -> R {
		let result: R;
		{
			result = n.get_entries().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_selector().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::synchronized_stmt::SynchronizedStmt, arg: &A) -> R {
		let result: R;
		{
			result = n.get_body().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_expression().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::this_expr::ThisExpr, arg: &A) -> R {
		let result: R;
		if n.get_type_name().isPresent() {
			result = n.get_type_name().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::throw_stmt::ThrowStmt, arg: &A) -> R {
		let result: R;
		{
			result = n.get_expression().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::try_stmt::TryStmt, arg: &A) -> R {
		let result: R;
		{
			result = n.get_catch_clauses().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_finally_block().isPresent() {
			result = n.get_finally_block().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_resources().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_try_block().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::local_class_declaration_stmt::LocalClassDeclarationStmt, arg: &A) -> R {
		let result: R;
		{
			result = n.get_class_declaration().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::local_record_declaration_stmt::LocalRecordDeclarationStmt, arg: &A) -> R {
		let result: R;
		{
			result = n.get_record_declaration().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::type_parameter::TypeParameter, arg: &A) -> R {
		let result: R;
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_type_bound().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_annotations().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::unary_expr::UnaryExpr, arg: &A) -> R {
		let result: R;
		{
			result = n.get_expression().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::unknown_type::UnknownType, arg: &A) -> R {
		let result: R;
		{
			result = n.get_annotations().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr, arg: &A) -> R {
		let result: R;
		{
			result = n.get_annotations().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_modifiers().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_variables().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::variable_declarator::VariableDeclarator, arg: &A) -> R {
		let result: R;
		if n.get_initializer().isPresent() {
			result = n.get_initializer().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_type().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::void_type::VoidType, arg: &A) -> R {
		let result: R;
		{
			result = n.get_annotations().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::while_stmt::WhileStmt, arg: &A) -> R {
		let result: R;
		{
			result = n.get_body().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_condition().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::wildcard_type::WildcardType, arg: &A) -> R {
		let result: R;
		if n.get_extended_type().isPresent() {
			result = n.get_extended_type().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_super_type().isPresent() {
			result = n.get_super_type().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_annotations().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::lambda_expr::LambdaExpr, arg: &A) -> R {
		let result: R;
		{
			result = n.get_body().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_parameters().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::method_reference_expr::MethodReferenceExpr, arg: &A) -> R {
		let result: R;
		{
			result = n.get_scope().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_type_arguments().isPresent() {
			result = n.get_type_arguments().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::type_expr::TypeExpr, arg: &A) -> R {
		let result: R;
		{
			result = n.get_type().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::import_declaration::ImportDeclaration, arg: &A) -> R {
		let result: R;
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::block_comment::BlockComment, arg: &A) -> R {
		let result: R;
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::line_comment::LineComment, arg: &A) -> R {
		let result: R;
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::node_list::NodeList, arg: &A) -> R {
		for /* final */ v in n {
			let result: R = (v as Node).accept(self, arg);
			if result != null {
				return result;
			}
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration, arg: &A) -> R {
		let result: R;
		{
			result = n.get_annotations().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_directives().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_requires_directive::ModuleRequiresDirective, arg: &A) -> R {
		let result: R;
		{
			result = n.get_modifiers().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_exports_directive::ModuleExportsDirective, arg: &A) -> R {
		let result: R;
		{
			result = n.get_module_names().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_provides_directive::ModuleProvidesDirective, arg: &A) -> R {
		let result: R;
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_with().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_uses_directive::ModuleUsesDirective, arg: &A) -> R {
		let result: R;
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_opens_directive::ModuleOpensDirective, arg: &A) -> R {
		let result: R;
		{
			result = n.get_module_names().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::unparsable_stmt::UnparsableStmt, arg: &A) -> R {
		let result: R;
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter, arg: &A) -> R {
		let result: R;
		{
			result = n.get_annotations().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_type().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::var_type::VarType, arg: &A) -> R {
		let result: R;
		{
			result = n.get_annotations().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modifier::Modifier, arg: &A) -> R {
		let result: R;
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::switch_expr::SwitchExpr, arg: &A) -> R {
		let result: R;
		{
			result = n.get_entries().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_selector().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::yield_stmt::YieldStmt, arg: &A) -> R {
		let result: R;
		{
			result = n.get_expression().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::text_block_literal_expr::TextBlockLiteralExpr, arg: &A) -> R {
		let result: R;
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::type_pattern_expr::TypePatternExpr, arg: &A) -> R {
		let result: R;
		{
			result = n.get_modifiers().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_type().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::record_declaration::RecordDeclaration, arg: &A) -> R {
		let result: R;
		{
			result = n.get_implemented_types().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_parameters().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_receiver_parameter().isPresent() {
			result = n.get_receiver_parameter().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_type_parameters().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_members().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_modifiers().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_annotations().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::compact_constructor_declaration::CompactConstructorDeclaration, arg: &A) -> R {
		let result: R;
		{
			result = n.get_body().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_modifiers().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_name().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_thrown_exceptions().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_type_parameters().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_annotations().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::record_pattern_expr::RecordPatternExpr, arg: &A) /* thrown(java.lang.IllegalStateException) */ -> R {
		let result: R;
		{
			result = n.get_modifiers().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_pattern_list().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		{
			result = n.get_type()?.accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::match_all_pattern_expr::MatchAllPatternExpr, arg: &A) -> R {
		let result: R;
		{
			result = n.get_modifiers().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::markdown_comment::MarkdownComment, arg: &A) -> R {
		let result: R;
		if n.get_comment().isPresent() {
			result = n.get_comment().get().accept(self, arg);
			if result != null {
				return result;
			}
	
		}
		return null;
	}
}

impl<R, A> com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor for GenericVisitorAdapter<R, A> {}