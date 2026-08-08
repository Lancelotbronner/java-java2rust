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
use java::util::ArrayList;
use java::util::List;
use java::util::Objects;
use java::util::stream::Collectors;

pub struct GenericListVisitorAdapter<R, A>;

impl<R, A> GenericListVisitorAdapter {
	pub fn visit(&self, n: &com::github::javaparser::ast::body::annotation_declaration::AnnotationDeclaration, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_members().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_modifiers().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_annotations().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::annotation_member_declaration::AnnotationMemberDeclaration, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		if n.get_default_value().isPresent() {
			tmp = n.get_default_value().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_modifiers().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_type().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_annotations().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_access_expr::ArrayAccessExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_index().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_creation_expr::ArrayCreationExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_element_type().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_initializer().isPresent() {
			tmp = n.get_initializer().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_levels().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::array_creation_level::ArrayCreationLevel, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_annotations().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_dimension().isPresent() {
			tmp = n.get_dimension().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_values().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::array_type::ArrayType, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_component_type().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_annotations().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::assert_stmt::AssertStmt, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_check().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_message().isPresent() {
			tmp = n.get_message().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::assign_expr::AssignExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_target().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_value().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::binary_expr::BinaryExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_left().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_right().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::block_comment::BlockComment, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_statements().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::boolean_literal_expr::BooleanLiteralExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::break_stmt::BreakStmt, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		if n.get_label().isPresent() {
			tmp = n.get_label().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::cast_expr::CastExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_expression().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_type().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::catch_clause::CatchClause, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_body().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_parameter().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::char_literal_expr::CharLiteralExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::class_expr::ClassExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_type().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_extended_types().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_implemented_types().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_permitted_types().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_type_parameters().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_members().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_modifiers().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_annotations().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_scope().isPresent() {
			tmp = n.get_scope().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_type_arguments().isPresent() {
			tmp = n.get_type_arguments().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_annotations().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::compilation_unit::CompilationUnit, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_imports().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_module().isPresent() {
			tmp = n.get_module().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_package_declaration().isPresent() {
			tmp = n.get_package_declaration().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_types().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::conditional_expr::ConditionalExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_condition().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_else_expr().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_then_expr().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_body().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_modifiers().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_parameters().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_receiver_parameter().isPresent() {
			tmp = n.get_receiver_parameter().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_thrown_exceptions().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_type_parameters().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_annotations().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::continue_stmt::ContinueStmt, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		if n.get_label().isPresent() {
			tmp = n.get_label().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::do_stmt::DoStmt, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_body().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_condition().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::double_literal_expr::DoubleLiteralExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::empty_stmt::EmptyStmt, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::enclosed_expr::EnclosedExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_inner().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::enum_constant_declaration::EnumConstantDeclaration, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_arguments().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_class_body().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_annotations().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::enum_declaration::EnumDeclaration, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_entries().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_implemented_types().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_members().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_modifiers().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_annotations().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::explicit_constructor_invocation_stmt::ExplicitConstructorInvocationStmt, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_arguments().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_expression().isPresent() {
			tmp = n.get_expression().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_type_arguments().isPresent() {
			tmp = n.get_type_arguments().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::expression_stmt::ExpressionStmt, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_expression().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::field_access_expr::FieldAccessExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_scope().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_type_arguments().isPresent() {
			tmp = n.get_type_arguments().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::field_declaration::FieldDeclaration, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_modifiers().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_variables().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_annotations().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::for_stmt::ForStmt, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_body().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_compare().isPresent() {
			tmp = n.get_compare().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_initialization().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_update().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::for_each_stmt::ForEachStmt, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_body().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_iterable().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_variable().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::if_stmt::IfStmt, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_condition().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_else_stmt().isPresent() {
			tmp = n.get_else_stmt().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_then_stmt().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::import_declaration::ImportDeclaration, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::initializer_declaration::InitializerDeclaration, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_body().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_annotations().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::instance_of_expr::InstanceOfExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_expression().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_pattern().isPresent() {
			tmp = n.get_pattern().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_type().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::integer_literal_expr::IntegerLiteralExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::intersection_type::IntersectionType, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_elements().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_annotations().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::traditional_javadoc_comment::TraditionalJavadocComment, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::labeled_stmt::LabeledStmt, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_label().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_statement().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::lambda_expr::LambdaExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_body().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_parameters().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::line_comment::LineComment, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::local_class_declaration_stmt::LocalClassDeclarationStmt, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_class_declaration().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::local_record_declaration_stmt::LocalRecordDeclarationStmt, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_record_declaration().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::long_literal_expr::LongLiteralExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::marker_annotation_expr::MarkerAnnotationExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::member_value_pair::MemberValuePair, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_value().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_arguments().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_scope().isPresent() {
			tmp = n.get_scope().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_type_arguments().isPresent() {
			tmp = n.get_type_arguments().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::method_declaration::MethodDeclaration, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		if n.get_body().isPresent() {
			tmp = n.get_body().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_type().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_modifiers().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_parameters().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_receiver_parameter().isPresent() {
			tmp = n.get_receiver_parameter().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_thrown_exceptions().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_type_parameters().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_annotations().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::method_reference_expr::MethodReferenceExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_scope().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_type_arguments().isPresent() {
			tmp = n.get_type_arguments().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name_expr::NameExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name::Name, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		if n.get_qualifier().isPresent() {
			tmp = n.get_qualifier().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::normal_annotation_expr::NormalAnnotationExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_pairs().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::null_literal_expr::NullLiteralExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::object_creation_expr::ObjectCreationExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		if n.get_anonymous_class_body().isPresent() {
			tmp = n.get_anonymous_class_body().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_arguments().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_scope().isPresent() {
			tmp = n.get_scope().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_type().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_type_arguments().isPresent() {
			tmp = n.get_type_arguments().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::package_declaration::PackageDeclaration, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_annotations().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::parameter::Parameter, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_annotations().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_modifiers().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_type().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_var_args_annotations().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::primitive_type::PrimitiveType, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_annotations().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::return_stmt::ReturnStmt, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		if n.get_expression().isPresent() {
			tmp = n.get_expression().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::simple_name::SimpleName, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::single_member_annotation_expr::SingleMemberAnnotationExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_member_value().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::string_literal_expr::StringLiteralExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::super_expr::SuperExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		if n.get_type_name().isPresent() {
			tmp = n.get_type_name().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::switch_entry::SwitchEntry, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		if n.get_guard().isPresent() {
			tmp = n.get_guard().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_labels().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_statements().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::switch_stmt::SwitchStmt, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_entries().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_selector().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::synchronized_stmt::SynchronizedStmt, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_body().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_expression().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::this_expr::ThisExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		if n.get_type_name().isPresent() {
			tmp = n.get_type_name().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::throw_stmt::ThrowStmt, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_expression().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::try_stmt::TryStmt, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_catch_clauses().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_finally_block().isPresent() {
			tmp = n.get_finally_block().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_resources().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_try_block().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::type_expr::TypeExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_type().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::type_parameter::TypeParameter, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_type_bound().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_annotations().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::unary_expr::UnaryExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_expression().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::union_type::UnionType, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_elements().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_annotations().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::unknown_type::UnknownType, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_annotations().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_annotations().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_modifiers().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_variables().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::variable_declarator::VariableDeclarator, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		if n.get_initializer().isPresent() {
			tmp = n.get_initializer().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_type().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::void_type::VoidType, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_annotations().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::while_stmt::WhileStmt, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_body().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_condition().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::wildcard_type::WildcardType, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		if n.get_extended_type().isPresent() {
			tmp = n.get_extended_type().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_super_type().isPresent() {
			tmp = n.get_super_type().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_annotations().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::node_list::NodeList, arg: &A) -> /* Java */ java::util::List /**/ {
		return (n as NodeList<? extends Node>).stream().filter(Objects::nonNull).flatMap(|v|v.accept(self, arg).stream()).collect(&Collectors::toList());
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_annotations().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_directives().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_exports_directive::ModuleExportsDirective, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_module_names().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_opens_directive::ModuleOpensDirective, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_module_names().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_provides_directive::ModuleProvidesDirective, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_with().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_requires_directive::ModuleRequiresDirective, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_modifiers().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_uses_directive::ModuleUsesDirective, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::unparsable_stmt::UnparsableStmt, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_annotations().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_type().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::var_type::VarType, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_annotations().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modifier::Modifier, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::switch_expr::SwitchExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_entries().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_selector().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::yield_stmt::YieldStmt, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_expression().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::text_block_literal_expr::TextBlockLiteralExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::type_pattern_expr::TypePatternExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_modifiers().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_type().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::record_declaration::RecordDeclaration, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_implemented_types().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_parameters().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_receiver_parameter().isPresent() {
			tmp = n.get_receiver_parameter().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_type_parameters().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_members().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_modifiers().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_annotations().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::compact_constructor_declaration::CompactConstructorDeclaration, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_body().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_modifiers().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_name().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_thrown_exceptions().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_type_parameters().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_annotations().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::record_pattern_expr::RecordPatternExpr, arg: &A) /* thrown(java.lang.IllegalStateException) */ -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_modifiers().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_pattern_list().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		{
			tmp = n.get_type()?.accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::match_all_pattern_expr::MatchAllPatternExpr, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		{
			tmp = n.get_modifiers().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::markdown_comment::MarkdownComment, arg: &A) -> /* Java */ java::util::List /**/ {
		let result: List<R> = ArrayList<>::new();
		let tmp: List<R>;
		if n.get_comment().isPresent() {
			tmp = n.get_comment().get().accept(self, arg);
			if tmp != null {
				result.addAll(tmp);
			}
	
		}
		return result;
	}
}

impl<R, A> com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor for GenericListVisitorAdapter<R, A> {}