use crate::com::github::javaparser::Range;
use crate::com::github::javaparser::ast::ArrayCreationLevel;
use crate::com::github::javaparser::ast::CompilationUnit;
use crate::com::github::javaparser::ast::ImportDeclaration;
use crate::com::github::javaparser::ast::Modifier;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::PackageDeclaration;
use crate::com::github::javaparser::ast::body::AnnotationDeclaration;
use crate::com::github::javaparser::ast::body::AnnotationMemberDeclaration;
use crate::com::github::javaparser::ast::body::ClassOrInterfaceDeclaration;
use crate::com::github::javaparser::ast::body::CompactConstructorDeclaration;
use crate::com::github::javaparser::ast::body::ConstructorDeclaration;
use crate::com::github::javaparser::ast::body::EnumConstantDeclaration;
use crate::com::github::javaparser::ast::body::EnumDeclaration;
use crate::com::github::javaparser::ast::body::FieldDeclaration;
use crate::com::github::javaparser::ast::body::InitializerDeclaration;
use crate::com::github::javaparser::ast::body::MethodDeclaration;
use crate::com::github::javaparser::ast::body::Parameter;
use crate::com::github::javaparser::ast::body::ReceiverParameter;
use crate::com::github::javaparser::ast::body::RecordDeclaration;
use crate::com::github::javaparser::ast::body::VariableDeclarator;
use crate::com::github::javaparser::ast::comments::BlockComment;
use crate::com::github::javaparser::ast::comments::LineComment;
use crate::com::github::javaparser::ast::comments::TraditionalJavadocComment;
use crate::com::github::javaparser::ast::expr::ArrayAccessExpr;
use crate::com::github::javaparser::ast::expr::ArrayCreationExpr;
use crate::com::github::javaparser::ast::expr::ArrayInitializerExpr;
use crate::com::github::javaparser::ast::expr::AssignExpr;
use crate::com::github::javaparser::ast::expr::BinaryExpr;
use crate::com::github::javaparser::ast::expr::BooleanLiteralExpr;
use crate::com::github::javaparser::ast::expr::CastExpr;
use crate::com::github::javaparser::ast::expr::CharLiteralExpr;
use crate::com::github::javaparser::ast::expr::ClassExpr;
use crate::com::github::javaparser::ast::expr::ConditionalExpr;
use crate::com::github::javaparser::ast::expr::DoubleLiteralExpr;
use crate::com::github::javaparser::ast::expr::EnclosedExpr;
use crate::com::github::javaparser::ast::expr::FieldAccessExpr;
use crate::com::github::javaparser::ast::expr::InstanceOfExpr;
use crate::com::github::javaparser::ast::expr::IntegerLiteralExpr;
use crate::com::github::javaparser::ast::expr::LambdaExpr;
use crate::com::github::javaparser::ast::expr::LongLiteralExpr;
use crate::com::github::javaparser::ast::expr::MarkerAnnotationExpr;
use crate::com::github::javaparser::ast::expr::MemberValuePair;
use crate::com::github::javaparser::ast::expr::MethodCallExpr;
use crate::com::github::javaparser::ast::expr::MethodReferenceExpr;
use crate::com::github::javaparser::ast::expr::Name;
use crate::com::github::javaparser::ast::expr::NameExpr;
use crate::com::github::javaparser::ast::expr::NormalAnnotationExpr;
use crate::com::github::javaparser::ast::expr::NullLiteralExpr;
use crate::com::github::javaparser::ast::expr::ObjectCreationExpr;
use crate::com::github::javaparser::ast::expr::SimpleName;
use crate::com::github::javaparser::ast::expr::SingleMemberAnnotationExpr;
use crate::com::github::javaparser::ast::expr::StringLiteralExpr;
use crate::com::github::javaparser::ast::expr::SuperExpr;
use crate::com::github::javaparser::ast::expr::SwitchExpr;
use crate::com::github::javaparser::ast::expr::TextBlockLiteralExpr;
use crate::com::github::javaparser::ast::expr::ThisExpr;
use crate::com::github::javaparser::ast::expr::TypeExpr;
use crate::com::github::javaparser::ast::expr::TypePatternExpr;
use crate::com::github::javaparser::ast::expr::UnaryExpr;
use crate::com::github::javaparser::ast::expr::VariableDeclarationExpr;
use crate::com::github::javaparser::ast::modules::ModuleDeclaration;
use crate::com::github::javaparser::ast::modules::ModuleExportsDirective;
use crate::com::github::javaparser::ast::modules::ModuleOpensDirective;
use crate::com::github::javaparser::ast::modules::ModuleProvidesDirective;
use crate::com::github::javaparser::ast::modules::ModuleRequiresDirective;
use crate::com::github::javaparser::ast::modules::ModuleUsesDirective;
use crate::com::github::javaparser::ast::stmt::AssertStmt;
use crate::com::github::javaparser::ast::stmt::BlockStmt;
use crate::com::github::javaparser::ast::stmt::BreakStmt;
use crate::com::github::javaparser::ast::stmt::CatchClause;
use crate::com::github::javaparser::ast::stmt::ContinueStmt;
use crate::com::github::javaparser::ast::stmt::DoStmt;
use crate::com::github::javaparser::ast::stmt::EmptyStmt;
use crate::com::github::javaparser::ast::stmt::ExplicitConstructorInvocationStmt;
use crate::com::github::javaparser::ast::stmt::ExpressionStmt;
use crate::com::github::javaparser::ast::stmt::ForEachStmt;
use crate::com::github::javaparser::ast::stmt::ForStmt;
use crate::com::github::javaparser::ast::stmt::IfStmt;
use crate::com::github::javaparser::ast::stmt::LabeledStmt;
use crate::com::github::javaparser::ast::stmt::LocalClassDeclarationStmt;
use crate::com::github::javaparser::ast::stmt::LocalRecordDeclarationStmt;
use crate::com::github::javaparser::ast::stmt::ReturnStmt;
use crate::com::github::javaparser::ast::stmt::SwitchEntry;
use crate::com::github::javaparser::ast::stmt::SwitchStmt;
use crate::com::github::javaparser::ast::stmt::SynchronizedStmt;
use crate::com::github::javaparser::ast::stmt::ThrowStmt;
use crate::com::github::javaparser::ast::stmt::TryStmt;
use crate::com::github::javaparser::ast::stmt::UnparsableStmt;
use crate::com::github::javaparser::ast::stmt::WhileStmt;
use crate::com::github::javaparser::ast::stmt::YieldStmt;
use crate::com::github::javaparser::ast::type::ArrayType;
use crate::com::github::javaparser::ast::type::ClassOrInterfaceType;
use crate::com::github::javaparser::ast::type::IntersectionType;
use crate::com::github::javaparser::ast::type::PrimitiveType;
use crate::com::github::javaparser::ast::type::TypeParameter;
use crate::com::github::javaparser::ast::type::UnionType;
use crate::com::github::javaparser::ast::type::UnknownType;
use crate::com::github::javaparser::ast::type::VarType;
use crate::com::github::javaparser::ast::type::VoidType;
use crate::com::github::javaparser::ast::type::WildcardType;
use java::util::function::BiFunction;

pub struct NodeFinderVisitor {
	selected_node: com::github::javaparser::ast::node::Node,
}

impl NodeFinderVisitor {
	pub static fConveringNode: /* Java */ java::util::function::BiFunction /**/ = |(n: &Node, range: &Range)|{
		return n.has_range() && n.get_range().get().contains(range);
	};

	static fn: /* Java */ java::util::function::BiFunction /**/;

	pub fn new(fn: &/* Java */ java::util::function::BiFunction /**/) -> com::github::javaparser::ast::visitor::node_finder_visitor::NodeFinderVisitor {
		self::fn = fn;
	}

	pub fn get_selected_node(&self) -> com::github::javaparser::ast::node::Node {
		return self.selected_node;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::body::annotation_declaration::AnnotationDeclaration, arg: &com::github::javaparser::range::Range) {
		{
			n.get_members().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_modifiers().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_annotations().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::body::annotation_member_declaration::AnnotationMemberDeclaration, arg: &com::github::javaparser::range::Range) {
		if n.get_default_value().isPresent() {
			n.get_default_value().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_modifiers().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_type().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_annotations().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::array_access_expr::ArrayAccessExpr, arg: &com::github::javaparser::range::Range) {
		{
			n.get_index().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::array_creation_expr::ArrayCreationExpr, arg: &com::github::javaparser::range::Range) {
		{
			n.get_element_type().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_initializer().isPresent() {
			n.get_initializer().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_levels().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr, arg: &com::github::javaparser::range::Range) {
		{
			n.get_values().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::stmt::assert_stmt::AssertStmt, arg: &com::github::javaparser::range::Range) {
		{
			n.get_check().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_message().isPresent() {
			n.get_message().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::assign_expr::AssignExpr, arg: &com::github::javaparser::range::Range) {
		{
			n.get_target().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_value().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::binary_expr::BinaryExpr, arg: &com::github::javaparser::range::Range) {
		{
			n.get_left().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_right().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt, arg: &com::github::javaparser::range::Range) {
		{
			n.get_statements().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::boolean_literal_expr::BooleanLiteralExpr, arg: &com::github::javaparser::range::Range) {
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::stmt::break_stmt::BreakStmt, arg: &com::github::javaparser::range::Range) {
		if n.get_label().isPresent() {
			n.get_label().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::cast_expr::CastExpr, arg: &com::github::javaparser::range::Range) {
		{
			n.get_expression().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_type().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::stmt::catch_clause::CatchClause, arg: &com::github::javaparser::range::Range) {
		{
			n.get_body().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_parameter().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::char_literal_expr::CharLiteralExpr, arg: &com::github::javaparser::range::Range) {
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::class_expr::ClassExpr, arg: &com::github::javaparser::range::Range) {
		{
			n.get_type().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration, arg: &com::github::javaparser::range::Range) {
		{
			n.get_extended_types().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_implemented_types().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_permitted_types().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_type_parameters().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_members().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_modifiers().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_annotations().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType, arg: &com::github::javaparser::range::Range) {
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_scope().isPresent() {
			n.get_scope().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_type_arguments().isPresent() {
			n.get_type_arguments().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_annotations().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::compilation_unit::CompilationUnit, arg: &com::github::javaparser::range::Range) {
		{
			n.get_imports().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_module().isPresent() {
			n.get_module().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_package_declaration().isPresent() {
			n.get_package_declaration().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_types().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::conditional_expr::ConditionalExpr, arg: &com::github::javaparser::range::Range) {
		{
			n.get_condition().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_else_expr().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_then_expr().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration, arg: &com::github::javaparser::range::Range) {
		{
			n.get_body().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_modifiers().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_parameters().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_receiver_parameter().isPresent() {
			n.get_receiver_parameter().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_thrown_exceptions().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_type_parameters().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_annotations().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::stmt::continue_stmt::ContinueStmt, arg: &com::github::javaparser::range::Range) {
		if n.get_label().isPresent() {
			n.get_label().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::stmt::do_stmt::DoStmt, arg: &com::github::javaparser::range::Range) {
		{
			n.get_body().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_condition().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::double_literal_expr::DoubleLiteralExpr, arg: &com::github::javaparser::range::Range) {
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::stmt::empty_stmt::EmptyStmt, arg: &com::github::javaparser::range::Range) {
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::enclosed_expr::EnclosedExpr, arg: &com::github::javaparser::range::Range) {
		{
			n.get_inner().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::body::enum_constant_declaration::EnumConstantDeclaration, arg: &com::github::javaparser::range::Range) {
		{
			n.get_arguments().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_class_body().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_annotations().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::body::enum_declaration::EnumDeclaration, arg: &com::github::javaparser::range::Range) {
		{
			n.get_entries().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_implemented_types().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_members().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_modifiers().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_annotations().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::stmt::explicit_constructor_invocation_stmt::ExplicitConstructorInvocationStmt, arg: &com::github::javaparser::range::Range) {
		{
			n.get_arguments().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_expression().isPresent() {
			n.get_expression().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_type_arguments().isPresent() {
			n.get_type_arguments().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::stmt::expression_stmt::ExpressionStmt, arg: &com::github::javaparser::range::Range) {
		{
			n.get_expression().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::field_access_expr::FieldAccessExpr, arg: &com::github::javaparser::range::Range) {
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_scope().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_type_arguments().isPresent() {
			n.get_type_arguments().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::body::field_declaration::FieldDeclaration, arg: &com::github::javaparser::range::Range) {
		{
			n.get_modifiers().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_variables().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_annotations().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::stmt::for_each_stmt::ForEachStmt, arg: &com::github::javaparser::range::Range) {
		{
			n.get_body().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_iterable().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_variable().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::stmt::for_stmt::ForStmt, arg: &com::github::javaparser::range::Range) {
		{
			n.get_body().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_compare().isPresent() {
			n.get_compare().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_initialization().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_update().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::stmt::if_stmt::IfStmt, arg: &com::github::javaparser::range::Range) {
		{
			n.get_condition().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_else_stmt().isPresent() {
			n.get_else_stmt().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_then_stmt().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::body::initializer_declaration::InitializerDeclaration, arg: &com::github::javaparser::range::Range) {
		{
			n.get_body().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_annotations().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::instance_of_expr::InstanceOfExpr, arg: &com::github::javaparser::range::Range) {
		{
			n.get_expression().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_pattern().isPresent() {
			n.get_pattern().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_type().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::integer_literal_expr::IntegerLiteralExpr, arg: &com::github::javaparser::range::Range) {
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::comments::traditional_javadoc_comment::TraditionalJavadocComment, arg: &com::github::javaparser::range::Range) {
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::stmt::labeled_stmt::LabeledStmt, arg: &com::github::javaparser::range::Range) {
		{
			n.get_label().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_statement().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::long_literal_expr::LongLiteralExpr, arg: &com::github::javaparser::range::Range) {
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::marker_annotation_expr::MarkerAnnotationExpr, arg: &com::github::javaparser::range::Range) {
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::member_value_pair::MemberValuePair, arg: &com::github::javaparser::range::Range) {
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_value().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr, arg: &com::github::javaparser::range::Range) {
		{
			n.get_arguments().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_scope().isPresent() {
			n.get_scope().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_type_arguments().isPresent() {
			n.get_type_arguments().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::body::method_declaration::MethodDeclaration, arg: &com::github::javaparser::range::Range) {
		if n.get_body().isPresent() {
			n.get_body().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_type().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_modifiers().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_parameters().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_receiver_parameter().isPresent() {
			n.get_receiver_parameter().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_thrown_exceptions().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_type_parameters().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_annotations().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::name_expr::NameExpr, arg: &com::github::javaparser::range::Range) {
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::normal_annotation_expr::NormalAnnotationExpr, arg: &com::github::javaparser::range::Range) {
		{
			n.get_pairs().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::null_literal_expr::NullLiteralExpr, arg: &com::github::javaparser::range::Range) {
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::object_creation_expr::ObjectCreationExpr, arg: &com::github::javaparser::range::Range) {
		if n.get_anonymous_class_body().isPresent() {
			n.get_anonymous_class_body().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_arguments().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_scope().isPresent() {
			n.get_scope().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_type().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_type_arguments().isPresent() {
			n.get_type_arguments().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::package_declaration::PackageDeclaration, arg: &com::github::javaparser::range::Range) {
		{
			n.get_annotations().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::body::parameter::Parameter, arg: &com::github::javaparser::range::Range) {
		{
			n.get_annotations().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_modifiers().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_type().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_var_args_annotations().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::type::primitive_type::PrimitiveType, arg: &com::github::javaparser::range::Range) {
		{
			n.get_annotations().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::name::Name, arg: &com::github::javaparser::range::Range) {
		if n.get_qualifier().isPresent() {
			n.get_qualifier().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::simple_name::SimpleName, arg: &com::github::javaparser::range::Range) {
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::type::array_type::ArrayType, arg: &com::github::javaparser::range::Range) {
		{
			n.get_component_type().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_annotations().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::array_creation_level::ArrayCreationLevel, arg: &com::github::javaparser::range::Range) {
		{
			n.get_annotations().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_dimension().isPresent() {
			n.get_dimension().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::type::intersection_type::IntersectionType, arg: &com::github::javaparser::range::Range) {
		{
			n.get_elements().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_annotations().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::type::union_type::UnionType, arg: &com::github::javaparser::range::Range) {
		{
			n.get_elements().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_annotations().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::stmt::return_stmt::ReturnStmt, arg: &com::github::javaparser::range::Range) {
		if n.get_expression().isPresent() {
			n.get_expression().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::single_member_annotation_expr::SingleMemberAnnotationExpr, arg: &com::github::javaparser::range::Range) {
		{
			n.get_member_value().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::string_literal_expr::StringLiteralExpr, arg: &com::github::javaparser::range::Range) {
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::super_expr::SuperExpr, arg: &com::github::javaparser::range::Range) {
		if n.get_type_name().isPresent() {
			n.get_type_name().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::stmt::switch_entry::SwitchEntry, arg: &com::github::javaparser::range::Range) {
		{
			n.get_labels().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_statements().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_guard().isPresent() {
			n.get_guard().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::stmt::switch_stmt::SwitchStmt, arg: &com::github::javaparser::range::Range) {
		{
			n.get_entries().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_selector().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::stmt::synchronized_stmt::SynchronizedStmt, arg: &com::github::javaparser::range::Range) {
		{
			n.get_body().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_expression().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::this_expr::ThisExpr, arg: &com::github::javaparser::range::Range) {
		if n.get_type_name().isPresent() {
			n.get_type_name().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::stmt::throw_stmt::ThrowStmt, arg: &com::github::javaparser::range::Range) {
		{
			n.get_expression().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::stmt::try_stmt::TryStmt, arg: &com::github::javaparser::range::Range) {
		{
			n.get_catch_clauses().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_finally_block().isPresent() {
			n.get_finally_block().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_resources().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_try_block().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::stmt::local_class_declaration_stmt::LocalClassDeclarationStmt, arg: &com::github::javaparser::range::Range) {
		{
			n.get_class_declaration().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::stmt::local_record_declaration_stmt::LocalRecordDeclarationStmt, arg: &com::github::javaparser::range::Range) {
		{
			n.get_record_declaration().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::type::type_parameter::TypeParameter, arg: &com::github::javaparser::range::Range) {
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_type_bound().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_annotations().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::unary_expr::UnaryExpr, arg: &com::github::javaparser::range::Range) {
		{
			n.get_expression().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::type::unknown_type::UnknownType, arg: &com::github::javaparser::range::Range) {
		{
			n.get_annotations().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr, arg: &com::github::javaparser::range::Range) {
		{
			n.get_annotations().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_modifiers().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_variables().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::body::variable_declarator::VariableDeclarator, arg: &com::github::javaparser::range::Range) {
		if n.get_initializer().isPresent() {
			n.get_initializer().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_type().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::type::void_type::VoidType, arg: &com::github::javaparser::range::Range) {
		{
			n.get_annotations().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::stmt::while_stmt::WhileStmt, arg: &com::github::javaparser::range::Range) {
		{
			n.get_body().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_condition().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::type::wildcard_type::WildcardType, arg: &com::github::javaparser::range::Range) {
		if n.get_extended_type().isPresent() {
			n.get_extended_type().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_super_type().isPresent() {
			n.get_super_type().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_annotations().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::lambda_expr::LambdaExpr, arg: &com::github::javaparser::range::Range) {
		{
			n.get_body().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_parameters().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::method_reference_expr::MethodReferenceExpr, arg: &com::github::javaparser::range::Range) {
		{
			n.get_scope().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_type_arguments().isPresent() {
			n.get_type_arguments().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::type_expr::TypeExpr, arg: &com::github::javaparser::range::Range) {
		{
			n.get_type().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::import_declaration::ImportDeclaration, arg: &com::github::javaparser::range::Range) {
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::comments::block_comment::BlockComment, arg: &com::github::javaparser::range::Range) {
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::comments::line_comment::LineComment, arg: &com::github::javaparser::range::Range) {
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::node_list::NodeList, arg: &com::github::javaparser::range::Range) {
		for /* final */ v in n {
			(v as Node).accept(self, arg);
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration, arg: &com::github::javaparser::range::Range) {
		{
			n.get_annotations().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_directives().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::modules::module_requires_directive::ModuleRequiresDirective, arg: &com::github::javaparser::range::Range) {
		{
			n.get_modifiers().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::modules::module_exports_directive::ModuleExportsDirective, arg: &com::github::javaparser::range::Range) {
		{
			n.get_module_names().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::modules::module_provides_directive::ModuleProvidesDirective, arg: &com::github::javaparser::range::Range) {
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_with().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::modules::module_uses_directive::ModuleUsesDirective, arg: &com::github::javaparser::range::Range) {
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::modules::module_opens_directive::ModuleOpensDirective, arg: &com::github::javaparser::range::Range) {
		{
			n.get_module_names().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::stmt::unparsable_stmt::UnparsableStmt, arg: &com::github::javaparser::range::Range) {
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter, arg: &com::github::javaparser::range::Range) {
		{
			n.get_annotations().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_type().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::type::var_type::VarType, arg: &com::github::javaparser::range::Range) {
		{
			n.get_annotations().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::modifier::Modifier, arg: &com::github::javaparser::range::Range) {
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::switch_expr::SwitchExpr, arg: &com::github::javaparser::range::Range) {
		{
			n.get_entries().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_selector().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::stmt::yield_stmt::YieldStmt, arg: &com::github::javaparser::range::Range) {
		{
			n.get_expression().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::text_block_literal_expr::TextBlockLiteralExpr, arg: &com::github::javaparser::range::Range) {
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::type_pattern_expr::TypePatternExpr, arg: &com::github::javaparser::range::Range) {
		{
			n.get_modifiers().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_type().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::body::record_declaration::RecordDeclaration, arg: &com::github::javaparser::range::Range) {
		{
			n.get_implemented_types().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_parameters().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_receiver_parameter().isPresent() {
			n.get_receiver_parameter().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_type_parameters().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_members().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_modifiers().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_annotations().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::body::compact_constructor_declaration::CompactConstructorDeclaration, arg: &com::github::javaparser::range::Range) {
		{
			n.get_body().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_modifiers().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_name().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_thrown_exceptions().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_type_parameters().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		{
			n.get_annotations().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if n.get_comment().isPresent() {
			n.get_comment().get().accept(self, arg);
			if self.selected_node != null {
				return;
			}
	
		}
		if self.fn.apply(n, arg) {
			self.selected_node = n;
		}
		return;
	}
}

impl com::github::javaparser::ast::visitor::void_visitor::VoidVisitor for NodeFinderVisitor {}