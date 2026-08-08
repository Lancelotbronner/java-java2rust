use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithScope;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithSimpleName;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithTypeArguments;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::stmt::ExplicitConstructorInvocationStmt;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::FieldAccessExprMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::OptionalProperty;
use crate::com::github::javaparser::resolution::Resolvable;
use crate::com::github::javaparser::resolution::UnsolvedSymbolException;
use crate::com::github::javaparser::resolution::declarations::ResolvedValueDeclaration;
use java::util::Optional;
use java::util::function::Consumer;

pub struct FieldAccessExpr {
	scope: com::github::javaparser::ast::expr::expression::Expression,
	type_arguments: com::github::javaparser::ast::node_list::NodeList,
	name: com::github::javaparser::ast::expr::simple_name::SimpleName,
}

impl FieldAccessExpr {
	pub fn new() -> com::github::javaparser::ast::expr::field_access_expr::FieldAccessExpr {
		this(null, ThisExpr::new(), null, SimpleName::new());
	}

	pub fn new(scope: &com::github::javaparser::ast::expr::expression::Expression, name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::field_access_expr::FieldAccessExpr {
		this(null, scope, null, SimpleName::new(name));
	}

	pub fn new(scope: &com::github::javaparser::ast::expr::expression::Expression, type_arguments: &com::github::javaparser::ast::node_list::NodeList, name: &com::github::javaparser::ast::expr::simple_name::SimpleName) -> com::github::javaparser::ast::expr::field_access_expr::FieldAccessExpr {
		this(null, scope, type_arguments, name);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, scope: &com::github::javaparser::ast::expr::expression::Expression, type_arguments: &com::github::javaparser::ast::node_list::NodeList, name: &com::github::javaparser::ast::expr::simple_name::SimpleName) -> com::github::javaparser::ast::expr::field_access_expr::FieldAccessExpr {
		super(token_range);
		self.set_scope(scope);
		self.set_type_arguments(type_arguments);
		self.set_name(name);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_name(&self) -> com::github::javaparser::ast::expr::simple_name::SimpleName {
		return self.name;
	}

	pub fn set_name(&mut self, name: &com::github::javaparser::ast::expr::simple_name::SimpleName) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::field_access_expr::FieldAccessExpr {
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

	pub fn get_scope(&self) -> com::github::javaparser::ast::expr::expression::Expression {
		return self.scope;
	}

	pub fn set_scope(&mut self, scope: &com::github::javaparser::ast::expr::expression::Expression) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::field_access_expr::FieldAccessExpr {
		com::github::javaparser::utils::utils::Utils::assert_not_null(scope)?;
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

	pub fn set_type_arguments(&mut self, type_arguments: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::expr::field_access_expr::FieldAccessExpr {
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

	pub fn clone(&self) -> com::github::javaparser::ast::expr::field_access_expr::FieldAccessExpr {
		return self.accept(CloneVisitor::new(), null) as FieldAccessExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::field_access_expr_meta_model::FieldAccessExprMetaModel {
		return JavaParserMetaModel::fieldAccessExprMetaModel;
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
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

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.name {
			self.set_name(replacement_node as SimpleName)?;
			return true;
		}
		if node == self.scope {
			self.set_scope(replacement_node as Expression)?;
			return true;
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

	pub fn is_field_access_expr(&self) -> bool {
		return true;
	}

	pub fn as_field_access_expr(&self) -> com::github::javaparser::ast::expr::field_access_expr::FieldAccessExpr {
		return self;
	}

	pub fn if_field_access_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::declarations::resolved_value_declaration::ResolvedValueDeclaration {
		return self.get_symbol_resolver().resolve_declaration(self, ResolvedValueDeclaration.class);
	}

	pub fn to_field_access_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn is_internal(&self) -> bool {
		return self.get_parent_node().isPresent() && self.get_parent_node().get() instanceof FieldAccessExpr;
	}

	pub fn is_top_level(&self) -> bool {
		return !self.is_internal();
	}
}

impl com::github::javaparser::ast::node_types::node_with_simple_name::NodeWithSimpleName for FieldAccessExpr {}

impl com::github::javaparser::ast::node_types::node_with_type_arguments::NodeWithTypeArguments for FieldAccessExpr {}

impl com::github::javaparser::ast::node_types::node_with_scope::NodeWithScope for FieldAccessExpr {}

impl com::github::javaparser::ast::node_types::node_with_traversable_scope::NodeWithTraversableScope for FieldAccessExpr {}

impl com::github::javaparser::resolution::resolvable::Resolvable for FieldAccessExpr {}

impl /* Java */ java::lang::Cloneable /**/ for FieldAccessExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for FieldAccessExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for FieldAccessExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for FieldAccessExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for FieldAccessExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for FieldAccessExpr {}