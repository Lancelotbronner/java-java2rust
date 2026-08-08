use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithSimpleName;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::stmt::ExplicitConstructorInvocationStmt;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::NameExprMetaModel;
use crate::com::github::javaparser::resolution::Resolvable;
use crate::com::github::javaparser::resolution::UnsolvedSymbolException;
use crate::com::github::javaparser::resolution::declarations::ResolvedValueDeclaration;
use java::util::Optional;
use java::util::function::Consumer;

pub struct NameExpr {
	name: com::github::javaparser::ast::expr::simple_name::SimpleName,
}

impl NameExpr {
	pub fn new() -> com::github::javaparser::ast::expr::name_expr::NameExpr {
		this(null, SimpleName::new());
	}

	pub fn new(name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::name_expr::NameExpr {
		this(null, SimpleName::new(name));
	}

	pub fn new(name: &com::github::javaparser::ast::expr::simple_name::SimpleName) -> com::github::javaparser::ast::expr::name_expr::NameExpr {
		this(&name.get_token_range().orElse(null), name);
		self.set_range(&name.get_range().orElse(null));
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, name: &com::github::javaparser::ast::expr::simple_name::SimpleName) -> com::github::javaparser::ast::expr::name_expr::NameExpr {
		super(token_range);
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

	pub fn set_name(&mut self, name: &com::github::javaparser::ast::expr::simple_name::SimpleName) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::name_expr::NameExpr {
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

	pub fn clone(&self) -> com::github::javaparser::ast::expr::name_expr::NameExpr {
		return self.accept(CloneVisitor::new(), null) as NameExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::name_expr_meta_model::NameExprMetaModel {
		return JavaParserMetaModel::nameExprMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.name {
			self.set_name(replacement_node as SimpleName)?;
			return true;
		}
		return super.replace(node, replacement_node);
	}

	pub fn is_name_expr(&self) -> bool {
		return true;
	}

	pub fn as_name_expr(&self) -> com::github::javaparser::ast::expr::name_expr::NameExpr {
		return self;
	}

	pub fn if_name_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::declarations::resolved_value_declaration::ResolvedValueDeclaration {
		return self.get_symbol_resolver().resolve_declaration(self, ResolvedValueDeclaration.class);
	}

	pub fn to_name_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl com::github::javaparser::ast::node_types::node_with_simple_name::NodeWithSimpleName for NameExpr {}

impl com::github::javaparser::resolution::resolvable::Resolvable for NameExpr {}

impl /* Java */ java::lang::Cloneable /**/ for NameExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for NameExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for NameExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for NameExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for NameExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for NameExpr {}