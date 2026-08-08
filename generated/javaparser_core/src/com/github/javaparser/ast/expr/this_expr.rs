use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::OptionalProperty;
use crate::com::github::javaparser::metamodel::ThisExprMetaModel;
use crate::com::github::javaparser::resolution::Resolvable;
use crate::com::github::javaparser::resolution::declarations::ResolvedTypeDeclaration;
use java::util::Optional;
use java::util::function::Consumer;

pub struct ThisExpr {
	type_name: com::github::javaparser::ast::expr::name::Name,
}

impl ThisExpr {
	pub fn new() -> com::github::javaparser::ast::expr::this_expr::ThisExpr {
		this(null, null);
	}

	pub fn new(type_name: &com::github::javaparser::ast::expr::name::Name) -> com::github::javaparser::ast::expr::this_expr::ThisExpr {
		this(null, type_name);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, type_name: &com::github::javaparser::ast::expr::name::Name) -> com::github::javaparser::ast::expr::this_expr::ThisExpr {
		super(token_range);
		self.set_type_name(type_name);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_type_name(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.type_name);
	}

	pub fn set_type_name(&mut self, type_name: &com::github::javaparser::ast::expr::name::Name) -> com::github::javaparser::ast::expr::this_expr::ThisExpr {
		if type_name == self.typeName {
			return self;
		}
		self.notify_property_change(ObservableProperty::TYPE_NAME, self.typeName, type_name);
		if self.typeName != null {
			self.typeName.set_parent_node(null);
		}
	
		self.typeName = type_name;
		self.set_as_parent_node_of(type_name);
		return self;
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		if self.type_name != null {
			if node == self.type_name {
				self.remove_type_name();
				return true;
			}
		}
		return super.remove(node);
	}

	pub fn remove_class_name(&self) -> com::github::javaparser::ast::expr::this_expr::ThisExpr {
		return self.set_type_name(null as Name);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::this_expr::ThisExpr {
		return self.accept(CloneVisitor::new(), null) as ThisExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::this_expr_meta_model::ThisExprMetaModel {
		return JavaParserMetaModel::thisExprMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		if self.type_name != null {
			if node == self.type_name {
				self.set_type_name(replacement_node as Name);
				return true;
			}
		}
		return super.replace(node, replacement_node);
	}

	pub fn is_this_expr(&self) -> bool {
		return true;
	}

	pub fn as_this_expr(&self) -> com::github::javaparser::ast::expr::this_expr::ThisExpr {
		return self;
	}

	pub fn if_this_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::declarations::resolved_type_declaration::ResolvedTypeDeclaration {
		return self.get_symbol_resolver().resolve_declaration(self, ResolvedTypeDeclaration.class);
	}

	pub fn to_this_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn remove_type_name(&self) -> com::github::javaparser::ast::expr::this_expr::ThisExpr {
		return self.set_type_name(null as Name);
	}
}

impl com::github::javaparser::resolution::resolvable::Resolvable for ThisExpr {}

impl /* Java */ java::lang::Cloneable /**/ for ThisExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for ThisExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for ThisExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for ThisExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for ThisExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for ThisExpr {}