use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::metamodel::ComponentPatternExprMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct ComponentPatternExpr;

impl ComponentPatternExpr {
	pub fn new() -> com::github::javaparser::ast::expr::component_pattern_expr::ComponentPatternExpr {
	}

	pub fn is_component_pattern_expr(&self) -> bool {
		return true;
	}

	pub fn as_component_pattern_expr(&self) -> com::github::javaparser::ast::expr::component_pattern_expr::ComponentPatternExpr {
		return self;
	}

	pub fn to_component_pattern_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn if_component_pattern_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::component_pattern_expr::ComponentPatternExpr {
		return self.accept(CloneVisitor::new(), null) as ComponentPatternExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::component_pattern_expr_meta_model::ComponentPatternExprMetaModel {
		return JavaParserMetaModel::componentPatternExprMetaModel;
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange) -> com::github::javaparser::ast::expr::component_pattern_expr::ComponentPatternExpr {
		super(token_range);
		self.custom_initialization();
	}
}

impl /* Java */ java::lang::Cloneable /**/ for ComponentPatternExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for ComponentPatternExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for ComponentPatternExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for ComponentPatternExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for ComponentPatternExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for ComponentPatternExpr {}