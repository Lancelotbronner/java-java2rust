use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::ArrayInitializerExprMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct ArrayInitializerExpr {
	values: com::github::javaparser::ast::node_list::NodeList,
}

impl ArrayInitializerExpr {
	pub fn new() -> com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr {
		this(null, NodeList<>::new());
	}

	pub fn new(values: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr {
		this(null, values);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, values: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr {
		super(token_range);
		self.set_values(values);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_values(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.values;
	}

	pub fn set_values(&mut self, values: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr {
		com::github::javaparser::utils::utils::Utils::assert_not_null(values)?;
		if values == self.values {
			return self;
		}
		self.notify_property_change(ObservableProperty::VALUES, self.values, values);
		if self.values != null {
			self.values.set_parent_node(null);
		}
	
		self.values = values;
		self.set_as_parent_node_of(values);
		return self;
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.values.size() {
				{
					if self.values.get(i) == node {
						self.values.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr {
		return self.accept(CloneVisitor::new(), null) as ArrayInitializerExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::array_initializer_expr_meta_model::ArrayInitializerExprMetaModel {
		return JavaParserMetaModel::arrayInitializerExprMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.values.size() {
				{
					if self.values.get(i) == node {
						self.values.set(i, replacement_node as Expression)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.replace(node, replacement_node);
	}

	pub fn is_array_initializer_expr(&self) -> bool {
		return true;
	}

	pub fn as_array_initializer_expr(&self) -> com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr {
		return self;
	}

	pub fn if_array_initializer_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_array_initializer_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl /* Java */ java::lang::Cloneable /**/ for ArrayInitializerExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for ArrayInitializerExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for ArrayInitializerExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for ArrayInitializerExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for ArrayInitializerExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for ArrayInitializerExpr {}