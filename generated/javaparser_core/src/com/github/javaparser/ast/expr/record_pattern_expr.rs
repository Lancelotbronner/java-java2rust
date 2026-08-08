use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Modifier;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::nodeTypes::modifiers::NodeWithFinalModifier;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::type::ClassOrInterfaceType;
use crate::com::github::javaparser::ast::type::ReferenceType;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::RecordPatternExprMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct RecordPatternExpr {
	modifiers: com::github::javaparser::ast::node_list::NodeList,
	pattern_list: com::github::javaparser::ast::node_list::NodeList,
}

impl RecordPatternExpr {
	pub fn new() -> com::github::javaparser::ast::expr::record_pattern_expr::RecordPatternExpr {
		this(NodeList<>::new(), ClassOrInterfaceType::new(), NodeList<>::new());
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, type: &com::github::javaparser::ast::type::type::Type, pattern_list: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::expr::record_pattern_expr::RecordPatternExpr {
		this(null, modifiers, type, pattern_list);
	}

	pub fn get_type(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::type::reference_type::ReferenceType {
		return super.get_type().as_reference_type()?;
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_modifiers(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.modifiers;
	}

	pub fn set_modifiers(&mut self, modifiers: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::record_pattern_expr::RecordPatternExpr {
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

	pub fn is_record_pattern_expr(&self) -> bool {
		return true;
	}

	pub fn as_record_pattern_expr(&self) -> com::github::javaparser::ast::expr::record_pattern_expr::RecordPatternExpr {
		return self;
	}

	pub fn to_record_pattern_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn if_record_pattern_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn get_pattern_list(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.pattern_list;
	}

	pub fn set_pattern_list(&mut self, pattern_list: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::record_pattern_expr::RecordPatternExpr {
		com::github::javaparser::utils::utils::Utils::assert_not_null(pattern_list)?;
		if pattern_list == self.patternList {
			return self;
		}
		self.notify_property_change(ObservableProperty::PATTERN_LIST, self.patternList, pattern_list);
		if self.patternList != null {
			self.patternList.set_parent_node(null);
		}
	
		self.patternList = pattern_list;
		self.set_as_parent_node_of(pattern_list);
		return self;
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
	
		 {
			let i: i32 = 0;
			while i < self.pattern_list.size() {
				{
					if self.pattern_list.get(i) == node {
						self.pattern_list.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
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
	
		 {
			let i: i32 = 0;
			while i < self.pattern_list.size() {
				{
					if self.pattern_list.get(i) == node {
						self.pattern_list.set(i, replacement_node as ComponentPatternExpr)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.replace(node, replacement_node)?;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::record_pattern_expr::RecordPatternExpr {
		return self.accept(CloneVisitor::new(), null) as RecordPatternExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::record_pattern_expr_meta_model::RecordPatternExprMetaModel {
		return JavaParserMetaModel::recordPatternExprMetaModel;
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, modifiers: &com::github::javaparser::ast::node_list::NodeList, type: &com::github::javaparser::ast::type::type::Type, pattern_list: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::record_pattern_expr::RecordPatternExpr {
		super(token_range, type);
		self.set_modifiers(modifiers)?;
		self.set_pattern_list(pattern_list)?;
		self.custom_initialization();
	}
}

impl com::github::javaparser::ast::node_types::modifiers::node_with_final_modifier::NodeWithFinalModifier for RecordPatternExpr {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for RecordPatternExpr {}

impl /* Java */ java::lang::Cloneable /**/ for RecordPatternExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for RecordPatternExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for RecordPatternExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for RecordPatternExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for RecordPatternExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for RecordPatternExpr {}

impl com::github::javaparser::ast::node_types::node_with_type::NodeWithType for RecordPatternExpr {}