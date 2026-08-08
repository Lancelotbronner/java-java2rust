use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::body::BodyDeclaration;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithArguments;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithOptionalScope;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithType;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithTypeArguments;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::stmt::ExplicitConstructorInvocationStmt;
use crate::com::github::javaparser::ast::type::ClassOrInterfaceType;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::ObjectCreationExprMetaModel;
use crate::com::github::javaparser::metamodel::OptionalProperty;
use crate::com::github::javaparser::resolution::Resolvable;
use crate::com::github::javaparser::resolution::UnsolvedSymbolException;
use crate::com::github::javaparser::resolution::declarations::ResolvedConstructorDeclaration;
use java::util::Optional;
use java::util::function::Consumer;

pub struct ObjectCreationExpr {
	scope: com::github::javaparser::ast::expr::expression::Expression,
	type: com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType,
	type_arguments: com::github::javaparser::ast::node_list::NodeList,
	arguments: com::github::javaparser::ast::node_list::NodeList,
	anonymous_class_body: com::github::javaparser::ast::node_list::NodeList,
}

impl ObjectCreationExpr {
	pub fn new() -> com::github::javaparser::ast::expr::object_creation_expr::ObjectCreationExpr {
		this(null, null, ClassOrInterfaceType::new(), NodeList<>::new(), NodeList<>::new(), null);
	}

	pub fn new(scope: &com::github::javaparser::ast::expr::expression::Expression, type: &com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType, arguments: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::expr::object_creation_expr::ObjectCreationExpr {
		this(null, scope, type, null, arguments, null);
	}

	pub fn new(scope: &com::github::javaparser::ast::expr::expression::Expression, type: &com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType, type_arguments: &com::github::javaparser::ast::node_list::NodeList, arguments: &com::github::javaparser::ast::node_list::NodeList, anonymous_class_body: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::expr::object_creation_expr::ObjectCreationExpr {
		this(null, scope, type, type_arguments, arguments, anonymous_class_body);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, scope: &com::github::javaparser::ast::expr::expression::Expression, type: &com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType, type_arguments: &com::github::javaparser::ast::node_list::NodeList, arguments: &com::github::javaparser::ast::node_list::NodeList, anonymous_class_body: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::expr::object_creation_expr::ObjectCreationExpr {
		super(token_range);
		self.set_scope(scope);
		self.set_type(type);
		self.set_type_arguments(type_arguments);
		self.set_arguments(arguments);
		self.set_anonymous_class_body(anonymous_class_body);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_anonymous_class_body(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.anonymous_class_body);
	}

	pub fn add_anonymous_class_body(&mut self, body: &com::github::javaparser::ast::body::body_declaration::BodyDeclaration) {
		if self.anonymous_class_body == null {
			self.anonymous_class_body = NodeList<>::new();
		}
	
		self.anonymous_class_body.add(body);
	}

	pub fn get_arguments(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.arguments;
	}

	pub fn get_scope(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.scope);
	}

	pub fn get_type(&self) -> com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType {
		return self.type;
	}

	pub fn set_anonymous_class_body(&mut self, anonymous_class_body: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::expr::object_creation_expr::ObjectCreationExpr {
		if anonymous_class_body == self.anonymousClassBody {
			return self;
		}
		self.notify_property_change(ObservableProperty::ANONYMOUS_CLASS_BODY, self.anonymousClassBody, anonymous_class_body);
		if self.anonymousClassBody != null {
			self.anonymousClassBody.set_parent_node(null);
		}
	
		self.anonymousClassBody = anonymous_class_body;
		self.set_as_parent_node_of(anonymous_class_body);
		return self;
	}

	pub fn set_arguments(&mut self, arguments: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::object_creation_expr::ObjectCreationExpr {
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

	pub fn set_scope(&mut self, scope: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::expr::object_creation_expr::ObjectCreationExpr {
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

	pub fn set_type(&mut self, type: &com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::object_creation_expr::ObjectCreationExpr {
		com::github::javaparser::utils::utils::Utils::assert_not_null(type)?;
		if type == self.type {
			return self;
		}
		self.notify_property_change(ObservableProperty::TYPE, self.type, type);
		if self.type != null {
			self.type.set_parent_node(null);
		}
	
		self.type = type;
		self.set_as_parent_node_of(type);
		return self;
	}

	pub fn get_type_arguments(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.type_arguments);
	}

	pub fn set_type_arguments(&mut self, type_arguments: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::expr::object_creation_expr::ObjectCreationExpr {
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
		if self.anonymous_class_body != null {
			 {
				let i: i32 = 0;
				while i < self.anonymous_class_body.size() {
					{
						if self.anonymous_class_body.get(i) == node {
							self.anonymous_class_body.remove(i);
							return true;
						}
					}
					i += 1;
				 }
			 }
	
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

	pub fn remove_scope(&self) -> com::github::javaparser::ast::expr::object_creation_expr::ObjectCreationExpr {
		return self.set_scope(null as Expression);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::object_creation_expr::ObjectCreationExpr {
		return self.accept(CloneVisitor::new(), null) as ObjectCreationExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::object_creation_expr_meta_model::ObjectCreationExprMetaModel {
		return JavaParserMetaModel::objectCreationExprMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		if self.anonymous_class_body != null {
			 {
				let i: i32 = 0;
				while i < self.anonymous_class_body.size() {
					{
						if self.anonymous_class_body.get(i) == node {
							self.anonymous_class_body.set(i, replacement_node as BodyDeclaration)?;
							return true;
						}
					}
					i += 1;
				 }
			 }
	
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
	
		if self.scope != null {
			if node == self.scope {
				self.set_scope(replacement_node as Expression);
				return true;
			}
		}
		if node == self.type {
			self.set_type(replacement_node as ClassOrInterfaceType)?;
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

	pub fn is_object_creation_expr(&self) -> bool {
		return true;
	}

	pub fn as_object_creation_expr(&self) -> com::github::javaparser::ast::expr::object_creation_expr::ObjectCreationExpr {
		return self;
	}

	pub fn if_object_creation_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::declarations::resolved_constructor_declaration::ResolvedConstructorDeclaration {
		return self.get_symbol_resolver().resolve_declaration(self, ResolvedConstructorDeclaration.class);
	}

	pub fn to_object_creation_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn is_poly_expression(&self) -> bool {
		return self.is_using_diamond_operator() && (self.appears_in_invocation_context() || self.appears_in_assignment_context());
	}
}

impl com::github::javaparser::ast::node_types::node_with_type_arguments::NodeWithTypeArguments for ObjectCreationExpr {}

impl com::github::javaparser::ast::node_types::node_with_type::NodeWithType for ObjectCreationExpr {}

impl com::github::javaparser::ast::node_types::node_with_arguments::NodeWithArguments for ObjectCreationExpr {}

impl com::github::javaparser::ast::node_types::node_with_optional_scope::NodeWithOptionalScope for ObjectCreationExpr {}

impl com::github::javaparser::ast::node_types::node_with_traversable_scope::NodeWithTraversableScope for ObjectCreationExpr {}

impl com::github::javaparser::resolution::resolvable::Resolvable for ObjectCreationExpr {}

impl /* Java */ java::lang::Cloneable /**/ for ObjectCreationExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for ObjectCreationExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for ObjectCreationExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for ObjectCreationExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for ObjectCreationExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for ObjectCreationExpr {}