use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::nodeTypes::SwitchNode;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::stmt::SwitchEntry;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::SwitchExprMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct SwitchExpr {
	selector: com::github::javaparser::ast::expr::expression::Expression,
	entries: com::github::javaparser::ast::node_list::NodeList,
}

impl SwitchExpr {
	pub fn new() -> com::github::javaparser::ast::expr::switch_expr::SwitchExpr {
		this(null, NameExpr::new(), NodeList<>::new());
	}

	pub fn new(selector: &com::github::javaparser::ast::expr::expression::Expression, entries: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::expr::switch_expr::SwitchExpr {
		this(null, selector, entries);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, selector: &com::github::javaparser::ast::expr::expression::Expression, entries: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::expr::switch_expr::SwitchExpr {
		super(token_range);
		self.set_selector(selector);
		self.set_entries(entries);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_entries(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.entries;
	}

	pub fn get_entry(&self, i: i32) -> com::github::javaparser::ast::stmt::switch_entry::SwitchEntry {
		return self.get_entries().get(i);
	}

	pub fn get_selector(&self) -> com::github::javaparser::ast::expr::expression::Expression {
		return self.selector;
	}

	pub fn set_entries(&mut self, entries: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::switch_expr::SwitchExpr {
		com::github::javaparser::utils::utils::Utils::assert_not_null(entries)?;
		if entries == self.entries {
			return self;
		}
		self.notify_property_change(ObservableProperty::ENTRIES, self.entries, entries);
		if self.entries != null {
			self.entries.set_parent_node(null);
		}
	
		self.entries = entries;
		self.set_as_parent_node_of(entries);
		return self;
	}

	pub fn set_selector(&mut self, selector: &com::github::javaparser::ast::expr::expression::Expression) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::switch_expr::SwitchExpr {
		com::github::javaparser::utils::utils::Utils::assert_not_null(selector)?;
		if selector == self.selector {
			return self;
		}
		self.notify_property_change(ObservableProperty::SELECTOR, self.selector, selector);
		if self.selector != null {
			self.selector.set_parent_node(null);
		}
	
		self.selector = selector;
		self.set_as_parent_node_of(selector);
		return self;
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.entries.size() {
				{
					if self.entries.get(i) == node {
						self.entries.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::switch_expr::SwitchExpr {
		return self.accept(CloneVisitor::new(), null) as SwitchExpr;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.entries.size() {
				{
					if self.entries.get(i) == node {
						self.entries.set(i, replacement_node as SwitchEntry)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		if node == self.selector {
			self.set_selector(replacement_node as Expression)?;
			return true;
		}
		return super.replace(node, replacement_node);
	}

	pub fn is_switch_expr(&self) -> bool {
		return true;
	}

	pub fn as_switch_expr(&self) -> com::github::javaparser::ast::expr::switch_expr::SwitchExpr {
		return self;
	}

	pub fn to_switch_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn if_switch_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::switch_expr_meta_model::SwitchExprMetaModel {
		return JavaParserMetaModel::switchExprMetaModel;
	}
}

impl com::github::javaparser::ast::node_types::switch_node::SwitchNode for SwitchExpr {}

impl /* Java */ java::lang::Cloneable /**/ for SwitchExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for SwitchExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for SwitchExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for SwitchExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for SwitchExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for SwitchExpr {}