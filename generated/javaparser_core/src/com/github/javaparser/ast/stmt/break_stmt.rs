use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::expr::SimpleName;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::BreakStmtMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::OptionalProperty;
use java::util::Optional;
use java::util::function::Consumer;

pub struct BreakStmt {
	label: com::github::javaparser::ast::expr::simple_name::SimpleName,
}

impl BreakStmt {
	pub fn new() -> com::github::javaparser::ast::stmt::break_stmt::BreakStmt {
		this(null, null);
	}

	pub fn new(label: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::stmt::break_stmt::BreakStmt {
		this(null, SimpleName::new(label));
	}

	pub fn new(label: &com::github::javaparser::ast::expr::simple_name::SimpleName) -> com::github::javaparser::ast::stmt::break_stmt::BreakStmt {
		this(null, label);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, label: &com::github::javaparser::ast::expr::simple_name::SimpleName) -> com::github::javaparser::ast::stmt::break_stmt::BreakStmt {
		super(token_range);
		self.set_label(label);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_label(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.label);
	}

	pub fn set_label(&mut self, label: &com::github::javaparser::ast::expr::simple_name::SimpleName) -> com::github::javaparser::ast::stmt::break_stmt::BreakStmt {
		if label == self.label {
			return self;
		}
		self.notify_property_change(ObservableProperty::LABEL, self.label, label);
		if self.label != null {
			self.label.set_parent_node(null);
		}
	
		self.label = label;
		self.set_as_parent_node_of(label);
		return self;
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		if self.label != null {
			if node == self.label {
				self.remove_label();
				return true;
			}
		}
		return super.remove(node);
	}

	pub fn remove_label(&self) -> com::github::javaparser::ast::stmt::break_stmt::BreakStmt {
		return self.set_label(null as SimpleName);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::stmt::break_stmt::BreakStmt {
		return self.accept(CloneVisitor::new(), null) as BreakStmt;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::break_stmt_meta_model::BreakStmtMetaModel {
		return JavaParserMetaModel::breakStmtMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		if self.label != null {
			if node == self.label {
				self.set_label(replacement_node as SimpleName);
				return true;
			}
		}
		return super.replace(node, replacement_node)?;
	}

	pub fn is_break_stmt(&self) -> bool {
		return true;
	}

	pub fn as_break_stmt(&self) -> com::github::javaparser::ast::stmt::break_stmt::BreakStmt {
		return self;
	}

	pub fn if_break_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_break_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl /* Java */ java::lang::Cloneable /**/ for BreakStmt {}

impl com::github::javaparser::has_parent_node::HasParentNode for BreakStmt {}

impl com::github::javaparser::ast::observer::observable::Observable for BreakStmt {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for BreakStmt {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for BreakStmt {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for BreakStmt {}