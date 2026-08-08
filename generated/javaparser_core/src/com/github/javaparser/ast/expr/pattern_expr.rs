use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithType;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::PatternExprMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct PatternExpr {
	type: com::github::javaparser::ast::type::type::Type,
}

impl PatternExpr {
	pub fn new(type: &com::github::javaparser::ast::type::type::Type) -> com::github::javaparser::ast::expr::pattern_expr::PatternExpr {
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, type: &com::github::javaparser::ast::type::type::Type) -> com::github::javaparser::ast::expr::pattern_expr::PatternExpr {
		super(token_range);
		self.set_type(type);
		self.custom_initialization();
	}

	pub fn get_type(&self) -> com::github::javaparser::ast::type::type::Type {
		return self.type;
	}

	pub fn set_type(&mut self, type: &com::github::javaparser::ast::type::type::Type) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::pattern_expr::PatternExpr {
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

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return null;
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
	}

	pub fn is_pattern_expr(&self) -> bool {
		return true;
	}

	pub fn as_pattern_expr(&self) -> com::github::javaparser::ast::expr::pattern_expr::PatternExpr {
		return self;
	}

	pub fn to_pattern_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn if_pattern_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.type {
			self.set_type(replacement_node as Type)?;
			return true;
		}
		return super.replace(node, replacement_node);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::pattern_expr::PatternExpr {
		return self.accept(CloneVisitor::new(), null) as PatternExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::pattern_expr_meta_model::PatternExprMetaModel {
		return JavaParserMetaModel::patternExprMetaModel;
	}
}

impl com::github::javaparser::ast::node_types::node_with_type::NodeWithType for PatternExpr {}

impl /* Java */ java::lang::Cloneable /**/ for PatternExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for PatternExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for PatternExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for PatternExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for PatternExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for PatternExpr {}