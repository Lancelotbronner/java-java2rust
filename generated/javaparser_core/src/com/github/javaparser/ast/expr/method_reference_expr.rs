use crate::com::github::javaparser::utils::Utils::assertNonEmpty;
use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithIdentifier;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithTypeArguments;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::MethodReferenceExprMetaModel;
use crate::com::github::javaparser::metamodel::NonEmptyProperty;
use crate::com::github::javaparser::metamodel::OptionalProperty;
use crate::com::github::javaparser::resolution::Resolvable;
use crate::com::github::javaparser::resolution::declarations::ResolvedMethodDeclaration;
use java::util::Optional;
use java::util::function::Consumer;

pub struct MethodReferenceExpr {
	scope: com::github::javaparser::ast::expr::expression::Expression,
	type_arguments: com::github::javaparser::ast::node_list::NodeList,
	identifier: /* Java */ java::lang::String /**/,
}

impl MethodReferenceExpr {
	pub fn new() -> com::github::javaparser::ast::expr::method_reference_expr::MethodReferenceExpr {
		this(null, ClassExpr::new(), null, "empty");
	}

	pub fn new(scope: &com::github::javaparser::ast::expr::expression::Expression, type_arguments: &com::github::javaparser::ast::node_list::NodeList, identifier: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::method_reference_expr::MethodReferenceExpr {
		this(null, scope, type_arguments, identifier);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, scope: &com::github::javaparser::ast::expr::expression::Expression, type_arguments: &com::github::javaparser::ast::node_list::NodeList, identifier: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::method_reference_expr::MethodReferenceExpr {
		super(token_range);
		self.set_scope(scope);
		self.set_type_arguments(type_arguments);
		self.set_identifier(identifier);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_scope(&self) -> com::github::javaparser::ast::expr::expression::Expression {
		return self.scope;
	}

	pub fn set_scope(&mut self, scope: &com::github::javaparser::ast::expr::expression::Expression) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::method_reference_expr::MethodReferenceExpr {
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

	pub fn set_type_arguments(&mut self, type_arguments: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::expr::method_reference_expr::MethodReferenceExpr {
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

	pub fn get_identifier(&self) -> /* Java */ java::lang::String /**/ {
		return self.identifier;
	}

	pub fn set_identifier(&mut self, identifier: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::method_reference_expr::MethodReferenceExpr {
		com::github::javaparser::utils::utils::Utils::assert_non_empty(identifier)?;
		if identifier.equals(self.identifier) {
			return self;
		}
		self.notify_property_change(ObservableProperty::IDENTIFIER, self.identifier, identifier);
		self.identifier = identifier;
		return self;
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

	pub fn clone(&self) -> com::github::javaparser::ast::expr::method_reference_expr::MethodReferenceExpr {
		return self.accept(CloneVisitor::new(), null) as MethodReferenceExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::method_reference_expr_meta_model::MethodReferenceExprMetaModel {
		return JavaParserMetaModel::methodReferenceExprMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
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

	pub fn is_method_reference_expr(&self) -> bool {
		return true;
	}

	pub fn as_method_reference_expr(&self) -> com::github::javaparser::ast::expr::method_reference_expr::MethodReferenceExpr {
		return self;
	}

	pub fn if_method_reference_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_method_reference_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::declarations::resolved_method_declaration::ResolvedMethodDeclaration {
		return self.get_symbol_resolver().resolve_declaration(self, ResolvedMethodDeclaration.class);
	}

	pub fn is_poly_expression(&self) -> bool {
		return true;
	}

	pub fn is_scope_primary_expr(&self) -> bool {
		return !self.get_scope().calculate_resolved_type().erasure().describe().endsWith(&self.get_scope().to_string());
	}
}

impl com::github::javaparser::ast::node_types::node_with_type_arguments::NodeWithTypeArguments for MethodReferenceExpr {}

impl com::github::javaparser::ast::node_types::node_with_identifier::NodeWithIdentifier for MethodReferenceExpr {}

impl com::github::javaparser::resolution::resolvable::Resolvable for MethodReferenceExpr {}

impl /* Java */ java::lang::Cloneable /**/ for MethodReferenceExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for MethodReferenceExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for MethodReferenceExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for MethodReferenceExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for MethodReferenceExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for MethodReferenceExpr {}