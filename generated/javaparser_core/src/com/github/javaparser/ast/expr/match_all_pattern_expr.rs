use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Modifier;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::nodeTypes::modifiers::NodeWithFinalModifier;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::MatchAllPatternExprMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct MatchAllPatternExpr {
	modifiers: com::github::javaparser::ast::node_list::NodeList,
}

impl MatchAllPatternExpr {
	pub static UNNAMED_PLACEHOLDER: /* Java */ java::lang::String /**/ = "_";

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::expr::match_all_pattern_expr::MatchAllPatternExpr {
		this(null, modifiers);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, modifiers: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::expr::match_all_pattern_expr::MatchAllPatternExpr {
		super(token_range);
		self.set_modifiers(modifiers);
		self.custom_initialization();
	}

	pub fn is_final(&self) -> bool {
		return true;
	}

	pub fn get_modifiers(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.modifiers;
	}

	pub fn set_modifiers(&mut self, modifiers: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::match_all_pattern_expr::MatchAllPatternExpr {
		com::github::javaparser::utils::utils::Utils::assert_not_null(modifiers)?;
		if modifiers == self.modifiers {
			return self;
		}
		self.notify_property_change(ObservableProperty::MODIFIERS, self.modifiers, modifiers);
		if self.modifiers != null {
			self.modifiers.set_parent_node(null);
		}
	
		self.modifiers = modifiers;
		self.set_as_parent_node_of(modifiers);
		return self;
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn is_match_all_pattern_expr(&self) -> bool {
		return true;
	}

	pub fn as_match_all_pattern_expr(&self) -> com::github::javaparser::ast::expr::match_all_pattern_expr::MatchAllPatternExpr {
		return self;
	}

	pub fn to_match_all_pattern_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn if_match_all_pattern_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.modifiers.size() {
				{
					if self.modifiers.get(i) == node {
						self.modifiers.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.modifiers.size() {
				{
					if self.modifiers.get(i) == node {
						self.modifiers.set(i, replacement_node as Modifier)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.replace(node, replacement_node);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::match_all_pattern_expr::MatchAllPatternExpr {
		return self.accept(CloneVisitor::new(), null) as MatchAllPatternExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::match_all_pattern_expr_meta_model::MatchAllPatternExprMetaModel {
		return JavaParserMetaModel::matchAllPatternExprMetaModel;
	}
}

impl com::github::javaparser::ast::node_types::modifiers::node_with_final_modifier::NodeWithFinalModifier for MatchAllPatternExpr {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for MatchAllPatternExpr {}

impl /* Java */ java::lang::Cloneable /**/ for MatchAllPatternExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for MatchAllPatternExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for MatchAllPatternExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for MatchAllPatternExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for MatchAllPatternExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for MatchAllPatternExpr {}