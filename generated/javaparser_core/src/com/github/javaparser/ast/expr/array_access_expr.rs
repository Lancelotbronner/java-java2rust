use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::ArrayAccessExprMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct ArrayAccessExpr {
	name: com::github::javaparser::ast::expr::expression::Expression,
	index: com::github::javaparser::ast::expr::expression::Expression,
}

impl ArrayAccessExpr {
	pub fn new() -> com::github::javaparser::ast::expr::array_access_expr::ArrayAccessExpr {
		this(null, NameExpr::new(), IntegerLiteralExpr::new());
	}

	pub fn new(name: &com::github::javaparser::ast::expr::expression::Expression, index: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::expr::array_access_expr::ArrayAccessExpr {
		this(null, name, index);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, name: &com::github::javaparser::ast::expr::expression::Expression, index: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::expr::array_access_expr::ArrayAccessExpr {
		super(token_range);
		self.set_name(name);
		self.set_index(index);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_index(&self) -> com::github::javaparser::ast::expr::expression::Expression {
		return self.index;
	}

	pub fn get_name(&self) -> com::github::javaparser::ast::expr::expression::Expression {
		return self.name;
	}

	pub fn set_index(&mut self, index: &com::github::javaparser::ast::expr::expression::Expression) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::array_access_expr::ArrayAccessExpr {
		com::github::javaparser::utils::utils::Utils::assert_not_null(index)?;
		if index == self.index {
			return self;
		}
		self.notify_property_change(ObservableProperty::INDEX, self.index, index);
		if self.index != null {
			self.index.set_parent_node(null);
		}
	
		self.index = index;
		self.set_as_parent_node_of(index);
		return self;
	}

	pub fn set_name(&mut self, name: &com::github::javaparser::ast::expr::expression::Expression) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::array_access_expr::ArrayAccessExpr {
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

	pub fn clone(&self) -> com::github::javaparser::ast::expr::array_access_expr::ArrayAccessExpr {
		return self.accept(CloneVisitor::new(), null) as ArrayAccessExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::array_access_expr_meta_model::ArrayAccessExprMetaModel {
		return JavaParserMetaModel::arrayAccessExprMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.index {
			self.set_index(replacement_node as Expression)?;
			return true;
		}
		if node == self.name {
			self.set_name(replacement_node as Expression)?;
			return true;
		}
		return super.replace(node, replacement_node);
	}

	pub fn is_array_access_expr(&self) -> bool {
		return true;
	}

	pub fn as_array_access_expr(&self) -> com::github::javaparser::ast::expr::array_access_expr::ArrayAccessExpr {
		return self;
	}

	pub fn if_array_access_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_array_access_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl /* Java */ java::lang::Cloneable /**/ for ArrayAccessExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for ArrayAccessExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for ArrayAccessExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for ArrayAccessExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for ArrayAccessExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for ArrayAccessExpr {}