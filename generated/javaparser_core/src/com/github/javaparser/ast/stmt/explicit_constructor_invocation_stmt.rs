use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::expr;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithArguments;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithTypeArguments;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::ExplicitConstructorInvocationStmtMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::OptionalProperty;
use crate::com::github::javaparser::resolution::Resolvable;
use crate::com::github::javaparser::resolution::UnsolvedSymbolException;
use crate::com::github::javaparser::resolution::declarations::ResolvedConstructorDeclaration;
use java::util::Optional;
use java::util::function::Consumer;

pub struct ExplicitConstructorInvocationStmt {
	type_arguments: com::github::javaparser::ast::node_list::NodeList,
	is_this: bool,
	expression: com::github::javaparser::ast::expr::expression::Expression,
	arguments: com::github::javaparser::ast::node_list::NodeList,
}

impl ExplicitConstructorInvocationStmt {
	pub fn new() -> com::github::javaparser::ast::stmt::explicit_constructor_invocation_stmt::ExplicitConstructorInvocationStmt {
		this(null, null, true, null, NodeList<>::new());
	}

	pub fn new(is_this: bool, expression: &com::github::javaparser::ast::expr::expression::Expression, arguments: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::stmt::explicit_constructor_invocation_stmt::ExplicitConstructorInvocationStmt {
		this(null, null, is_this, expression, arguments);
	}

	pub fn new(type_arguments: &com::github::javaparser::ast::node_list::NodeList, is_this: bool, expression: &com::github::javaparser::ast::expr::expression::Expression, arguments: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::stmt::explicit_constructor_invocation_stmt::ExplicitConstructorInvocationStmt {
		this(null, type_arguments, is_this, expression, arguments);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, type_arguments: &com::github::javaparser::ast::node_list::NodeList, is_this: bool, expression: &com::github::javaparser::ast::expr::expression::Expression, arguments: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::stmt::explicit_constructor_invocation_stmt::ExplicitConstructorInvocationStmt {
		super(token_range);
		self.set_type_arguments(type_arguments);
		self.set_this(is_this);
		self.set_expression(expression);
		self.set_arguments(arguments);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_arguments(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.arguments;
	}

	pub fn get_expression(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.expression);
	}

	pub fn is_this(&self) -> bool {
		return self.is_this;
	}

	pub fn set_arguments(&mut self, arguments: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::explicit_constructor_invocation_stmt::ExplicitConstructorInvocationStmt {
		com::github::javaparser::utils::utils::Utils::assert_not_null(arguments)?;
		if arguments == self.arguments {
			return self;
		}
		self.notify_property_change(ObservableProperty::ARGUMENTS, self.arguments, arguments);
		if self.arguments != null {
			self.arguments.set_parent_node(null);
		}
	
		self.arguments = arguments;
		self.set_as_parent_node_of(arguments);
		return self;
	}

	pub fn set_expression(&mut self, expression: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::stmt::explicit_constructor_invocation_stmt::ExplicitConstructorInvocationStmt {
		if expression == self.expression {
			return self;
		}
		self.notify_property_change(ObservableProperty::EXPRESSION, self.expression, expression);
		if self.expression != null {
			self.expression.set_parent_node(null);
		}
	
		self.expression = expression;
		self.set_as_parent_node_of(expression);
		return self;
	}

	pub fn set_this(&mut self, is_this: bool) -> com::github::javaparser::ast::stmt::explicit_constructor_invocation_stmt::ExplicitConstructorInvocationStmt {
		if is_this == self.isThis {
			return self;
		}
		self.notify_property_change(ObservableProperty::THIS, self.isThis, is_this);
		self.isThis = is_this;
		return self;
	}

	pub fn get_type_arguments(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.type_arguments);
	}

	pub fn set_type_arguments(&mut self, type_arguments: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::stmt::explicit_constructor_invocation_stmt::ExplicitConstructorInvocationStmt {
		if type_arguments == self.typeArguments {
			return self;
		}
		self.notify_property_change(ObservableProperty::TYPE_ARGUMENTS, self.typeArguments, type_arguments);
		if self.typeArguments != null {
			self.typeArguments.set_parent_node(null);
		}
	
		self.typeArguments = type_arguments;
		self.set_as_parent_node_of(type_arguments);
		return self;
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.arguments.size() {
				{
					if self.arguments.get(i) == node {
						self.arguments.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		if self.expression != null {
			if node == self.expression {
				self.remove_expression();
				return true;
			}
		}
		if self.type_arguments != null {
			 {
				let i: i32 = 0;
				while i < self.type_arguments.size() {
					{
						if self.type_arguments.get(i) == node {
							self.type_arguments.remove(i);
							return true;
						}
					}
					i += 1;
				 }
			 }
	
		}
		return super.remove(node);
	}

	pub fn remove_expression(&self) -> com::github::javaparser::ast::stmt::explicit_constructor_invocation_stmt::ExplicitConstructorInvocationStmt {
		return self.set_expression(null as Expression);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::stmt::explicit_constructor_invocation_stmt::ExplicitConstructorInvocationStmt {
		return self.accept(CloneVisitor::new(), null) as ExplicitConstructorInvocationStmt;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::explicit_constructor_invocation_stmt_meta_model::ExplicitConstructorInvocationStmtMetaModel {
		return JavaParserMetaModel::explicitConstructorInvocationStmtMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.arguments.size() {
				{
					if self.arguments.get(i) == node {
						self.arguments.set(i, replacement_node as Expression)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		if self.expression != null {
			if node == self.expression {
				self.set_expression(replacement_node as Expression);
				return true;
			}
		}
		if self.type_arguments != null {
			 {
				let i: i32 = 0;
				while i < self.type_arguments.size() {
					{
						if self.type_arguments.get(i) == node {
							self.type_arguments.set(i, replacement_node as Type)?;
							return true;
						}
					}
					i += 1;
				 }
			 }
	
		}
		return super.replace(node, replacement_node)?;
	}

	pub fn is_explicit_constructor_invocation_stmt(&self) -> bool {
		return true;
	}

	pub fn as_explicit_constructor_invocation_stmt(&self) -> com::github::javaparser::ast::stmt::explicit_constructor_invocation_stmt::ExplicitConstructorInvocationStmt {
		return self;
	}

	pub fn if_explicit_constructor_invocation_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::declarations::resolved_constructor_declaration::ResolvedConstructorDeclaration {
		return self.get_symbol_resolver().resolve_declaration(self, ResolvedConstructorDeclaration.class);
	}

	pub fn to_explicit_constructor_invocation_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl com::github::javaparser::ast::node_types::node_with_type_arguments::NodeWithTypeArguments for ExplicitConstructorInvocationStmt {}

impl com::github::javaparser::ast::node_types::node_with_arguments::NodeWithArguments for ExplicitConstructorInvocationStmt {}

impl com::github::javaparser::resolution::resolvable::Resolvable for ExplicitConstructorInvocationStmt {}

impl /* Java */ java::lang::Cloneable /**/ for ExplicitConstructorInvocationStmt {}

impl com::github::javaparser::has_parent_node::HasParentNode for ExplicitConstructorInvocationStmt {}

impl com::github::javaparser::ast::observer::observable::Observable for ExplicitConstructorInvocationStmt {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for ExplicitConstructorInvocationStmt {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for ExplicitConstructorInvocationStmt {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for ExplicitConstructorInvocationStmt {}