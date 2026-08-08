use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::expr::Expression;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithStatements;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::DerivedProperty;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::OptionalProperty;
use crate::com::github::javaparser::metamodel::SwitchEntryMetaModel;
use java::util::Optional;

pub struct SwitchEntry {
	labels: com::github::javaparser::ast::node_list::NodeList,
	statements: com::github::javaparser::ast::node_list::NodeList,
	type: com::github::javaparser::ast::stmt::switch_entry::Type,
	is_default: bool,
	guard: com::github::javaparser::ast::expr::expression::Expression,
}

impl SwitchEntry {
	pub fn new() -> com::github::javaparser::ast::stmt::switch_entry::SwitchEntry {
		this(null, NodeList<Expression>::new(), Type::STATEMENT_GROUP, NodeList<>::new(), false, null);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, labels: &com::github::javaparser::ast::node_list::NodeList, type: &com::github::javaparser::ast::stmt::switch_entry::Type, statements: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::stmt::switch_entry::SwitchEntry {
		this(token_range, labels, type, statements, false, null);
	}

	pub fn new(labels: &com::github::javaparser::ast::node_list::NodeList, type: &com::github::javaparser::ast::stmt::switch_entry::Type, statements: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::stmt::switch_entry::SwitchEntry {
		this(null, labels, type, statements, false, null);
	}

	pub fn new(labels: &com::github::javaparser::ast::node_list::NodeList, type: &com::github::javaparser::ast::stmt::switch_entry::Type, statements: &com::github::javaparser::ast::node_list::NodeList, is_default: bool, guard: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::stmt::switch_entry::SwitchEntry {
		this(null, labels, type, statements, is_default, guard);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, labels: &com::github::javaparser::ast::node_list::NodeList, type: &com::github::javaparser::ast::stmt::switch_entry::Type, statements: &com::github::javaparser::ast::node_list::NodeList, is_default: bool, guard: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::stmt::switch_entry::SwitchEntry {
		super(token_range);
		self.set_labels(labels);
		self.set_type(type);
		self.set_statements(statements);
		self.set_default(is_default);
		self.set_guard(guard);
		self.custom_initialization();
	}

	pub fn is_switch_statement_entry(&self) -> bool {
		return self.type == Type::STATEMENT_GROUP;
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_labels(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.labels;
	}

	pub fn get_statements(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.statements;
	}

	pub fn set_labels(&mut self, labels: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::switch_entry::SwitchEntry {
		com::github::javaparser::utils::utils::Utils::assert_not_null(labels)?;
		if labels == self.labels {
			return self;
		}
		self.notify_property_change(ObservableProperty::LABELS, self.labels, labels);
		if self.labels != null {
			self.labels.set_parent_node(null);
		}
	
		self.labels = labels;
		self.set_as_parent_node_of(labels);
		return self;
	}

	pub fn set_statements(&mut self, statements: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::switch_entry::SwitchEntry {
		com::github::javaparser::utils::utils::Utils::assert_not_null(statements)?;
		if statements == self.statements {
			return self;
		}
		self.notify_property_change(ObservableProperty::STATEMENTS, self.statements, statements);
		if self.statements != null {
			self.statements.set_parent_node(null);
		}
	
		self.statements = statements;
		self.set_as_parent_node_of(statements);
		return self;
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		if self.guard != null {
			if node == self.guard {
				self.remove_guard();
				return true;
			}
		}
		 {
			let i: i32 = 0;
			while i < self.labels.size() {
				{
					if self.labels.get(i) == node {
						self.labels.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		 {
			let i: i32 = 0;
			while i < self.statements.size() {
				{
					if self.statements.get(i) == node {
						self.statements.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::stmt::switch_entry::SwitchEntry {
		return self.accept(CloneVisitor::new(), null) as SwitchEntry;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::switch_entry_meta_model::SwitchEntryMetaModel {
		return JavaParserMetaModel::switchEntryMetaModel;
	}

	pub fn get_type(&self) -> com::github::javaparser::ast::stmt::switch_entry::Type {
		return self.type;
	}

	pub fn set_type(&mut self, type: &com::github::javaparser::ast::stmt::switch_entry::Type) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::switch_entry::SwitchEntry {
		com::github::javaparser::utils::utils::Utils::assert_not_null(type)?;
		if type == self.type {
			return self;
		}
		self.notify_property_change(ObservableProperty::TYPE, self.type, type);
		self.type = type;
		return self;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		if self.guard != null {
			if node == self.guard {
				self.set_guard(replacement_node as Expression);
				return true;
			}
		}
		 {
			let i: i32 = 0;
			while i < self.labels.size() {
				{
					if self.labels.get(i) == node {
						self.labels.set(i, replacement_node as Expression)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		 {
			let i: i32 = 0;
			while i < self.statements.size() {
				{
					if self.statements.get(i) == node {
						self.statements.set(i, replacement_node as Statement)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.replace(node, replacement_node)?;
	}

	pub fn is_default(&self) -> bool {
		return self.is_default;
	}

	pub fn set_default(&mut self, is_default: bool) -> com::github::javaparser::ast::stmt::switch_entry::SwitchEntry {
		if is_default == self.isDefault {
			return self;
		}
		self.notify_property_change(ObservableProperty::DEFAULT, self.isDefault, is_default);
		self.isDefault = is_default;
		return self;
	}

	pub fn get_guard(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.guard);
	}

	pub fn set_guard(&mut self, guard: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::stmt::switch_entry::SwitchEntry {
		if guard == self.guard {
			return self;
		}
		self.notify_property_change(ObservableProperty::GUARD, self.guard, guard);
		if self.guard != null {
			self.guard.set_parent_node(null);
		}
	
		self.guard = guard;
		self.set_as_parent_node_of(guard);
		return self;
	}

	pub fn remove_guard(&self) -> com::github::javaparser::ast::stmt::switch_entry::SwitchEntry {
		return self.set_guard(null as Expression);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, labels: &com::github::javaparser::ast::node_list::NodeList, type: &com::github::javaparser::ast::stmt::switch_entry::Type, statements: &com::github::javaparser::ast::node_list::NodeList, is_default: bool) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::switch_entry::SwitchEntry {
		super(token_range);
		self.set_labels(labels)?;
		self.set_type(type)?;
		self.set_statements(statements)?;
		self.set_default(is_default);
		self.custom_initialization();
	}
}

impl com::github::javaparser::ast::node_types::node_with_statements::NodeWithStatements for SwitchEntry {}

impl /* Java */ java::lang::Cloneable /**/ for SwitchEntry {}

impl com::github::javaparser::has_parent_node::HasParentNode for SwitchEntry {}

impl com::github::javaparser::ast::observer::observable::Observable for SwitchEntry {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for SwitchEntry {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for SwitchEntry {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for SwitchEntry {}

pub enum Type;