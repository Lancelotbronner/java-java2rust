use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithArguments;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithOptionalScope;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithSimpleName;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithTypeArguments;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::stmt::ExplicitConstructorInvocationStmt;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::MethodCallExprMetaModel;
use crate::com::github::javaparser::metamodel::OptionalProperty;
use crate::com::github::javaparser::resolution::Resolvable;
use crate::com::github::javaparser::resolution::UnsolvedSymbolException;
use crate::com::github::javaparser::resolution::declarations::ResolvedMethodDeclaration;
use crate::com::github::javaparser::resolution::types::ResolvedType;
use java::util::Optional;
use java::util::function::Consumer;

pub struct MethodCallExpr {
	scope: com::github::javaparser::ast::expr::expression::Expression,
	type_arguments: com::github::javaparser::ast::node_list::NodeList,
	name: com::github::javaparser::ast::expr::simple_name::SimpleName,
	arguments: com::github::javaparser::ast::node_list::NodeList,
}

impl MethodCallExpr {
	pub fn new() -> com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr {
		this(null, null, null, SimpleName::new(), NodeList<>::new());
	}

	pub fn new(name: &/* Java */ java::lang::String /**/, arguments: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr {
		this(null, null, null, SimpleName::new(name), NodeList<>::new(arguments));
	}

	pub fn new(scope: &com::github::javaparser::ast::expr::expression::Expression, name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr {
		this(null, scope, null, SimpleName::new(name), NodeList<>::new());
	}

	pub fn new(scope: &com::github::javaparser::ast::expr::expression::Expression, name: &com::github::javaparser::ast::expr::simple_name::SimpleName) -> com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr {
		this(null, scope, null, name, NodeList<>::new());
	}

	pub fn new(scope: &com::github::javaparser::ast::expr::expression::Expression, name: &/* Java */ java::lang::String /**/, arguments: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr {
		this(null, scope, null, SimpleName::new(name), arguments);
	}

	pub fn new(scope: &com::github::javaparser::ast::expr::expression::Expression, type_arguments: &com::github::javaparser::ast::node_list::NodeList, name: &/* Java */ java::lang::String /**/, arguments: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr {
		this(null, scope, type_arguments, SimpleName::new(name), arguments);
	}

	pub fn new(scope: &com::github::javaparser::ast::expr::expression::Expression, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, arguments: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr {
		this(null, scope, null, name, arguments);
	}

	pub fn new(scope: &com::github::javaparser::ast::expr::expression::Expression, type_arguments: &com::github::javaparser::ast::node_list::NodeList, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, arguments: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr {
		this(null, scope, type_arguments, name, arguments);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, scope: &com::github::javaparser::ast::expr::expression::Expression, type_arguments: &com::github::javaparser::ast::node_list::NodeList, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, arguments: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr {
		super(token_range);
		self.set_scope(scope);
		self.set_type_arguments(type_arguments);
		self.set_name(name);
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

	pub fn get_name(&self) -> com::github::javaparser::ast::expr::simple_name::SimpleName {
		return self.name;
	}

	pub fn get_scope(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.scope);
	}

	pub fn set_arguments(&mut self, arguments: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr {
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

	pub fn set_name(&mut self, name: &com::github::javaparser::ast::expr::simple_name::SimpleName) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr {
		com::github::javaparser::utils::utils::Utils::assert_not_null(name)?;
		if name == self.name {
			return self;
		}
		self.notify_property_change(ObservableProperty::NAME, self.name, name);
		if self.name != null {
			self.name.set_parent_node(null);
		}
	
		self.name = name;
		self.set_as_parent_node_of(name);
		return self;
	}

	pub fn set_scope(&mut self, scope: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr {
		if scope == self.scope {
			return self;
		}
		self.notify_property_change(ObservableProperty::SCOPE, self.scope, scope);
		if self.scope != null {
			self.scope.set_parent_node(null);
		}
	
		self.scope = scope;
		self.set_as_parent_node_of(scope);
		return self;
	}

	pub fn get_type_arguments(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.type_arguments);
	}

	pub fn set_type_arguments(&mut self, type_arguments: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr {
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
	
		if self.scope != null {
			if node == self.scope {
				self.remove_scope();
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

	pub fn remove_scope(&self) -> com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr {
		return self.set_scope(null as Expression);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr {
		return self.accept(CloneVisitor::new(), null) as MethodCallExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::method_call_expr_meta_model::MethodCallExprMetaModel {
		return JavaParserMetaModel::methodCallExprMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
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
	
		if node == self.name {
			self.set_name(replacement_node as SimpleName)?;
			return true;
		}
		if self.scope != null {
			if node == self.scope {
				self.set_scope(replacement_node as Expression);
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
		return super.replace(node, replacement_node);
	}

	pub fn is_method_call_expr(&self) -> bool {
		return true;
	}

	pub fn as_method_call_expr(&self) -> com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr {
		return self;
	}

	pub fn if_method_call_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::declarations::resolved_method_declaration::ResolvedMethodDeclaration {
		return self.get_symbol_resolver().resolve_declaration(self, ResolvedMethodDeclaration.class);
	}

	pub fn to_method_call_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn is_poly_expression(&self) -> bool {
		// 1. The invocation appears in an assignment context or an invocation context (§5.2, §5.3).
		if !(self.appears_in_assignment_context() || self.appears_in_invocation_context()) {
			return false;
		}
		// [ArgumentList] )]), then the invocation elides TypeArguments to the left of the Identifier.
		if self.is_qualified() && !self.elides_type_arguments() {
			return false;
		}
		// A method is generic if it declares one or more type variables (§4.4).
		if self.is_generic_method() && self.has_parameterwith_same_type_than_result_type(&self.resolve().get_return_type()) {
			// it's a poly expression
			return true;
		}
		// Otherwise, the method invocation expression is a standalone expression.
		return false;
	}

	fn is_generic_method(&self) -> bool {
		return self.get_type_arguments().isPresent() && !self.get_type_arguments().get().is_empty();
	}

	fn has_parameterwith_same_type_than_result_type(&self, resolved_return_type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) -> bool {
		return self.get_type_arguments().isPresent() && self.get_type_arguments().get().stream().anyMatch(|arg_type|arg_type.resolve().is_assignable_by(resolved_return_type));
	}

	fn is_invocation_context(&self) -> bool {
		return true;
	}
}

impl com::github::javaparser::ast::node_types::node_with_type_arguments::NodeWithTypeArguments for MethodCallExpr {}

impl com::github::javaparser::ast::node_types::node_with_arguments::NodeWithArguments for MethodCallExpr {}

impl com::github::javaparser::ast::node_types::node_with_simple_name::NodeWithSimpleName for MethodCallExpr {}

impl com::github::javaparser::ast::node_types::node_with_optional_scope::NodeWithOptionalScope for MethodCallExpr {}

impl com::github::javaparser::ast::node_types::node_with_traversable_scope::NodeWithTraversableScope for MethodCallExpr {}

impl com::github::javaparser::resolution::resolvable::Resolvable for MethodCallExpr {}

impl /* Java */ java::lang::Cloneable /**/ for MethodCallExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for MethodCallExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for MethodCallExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for MethodCallExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for MethodCallExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for MethodCallExpr {}