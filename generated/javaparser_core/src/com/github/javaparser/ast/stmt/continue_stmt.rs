use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::expr::SimpleName;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithOptionalLabel;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::ContinueStmtMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::OptionalProperty;
use java::util::Optional;
use java::util::function::Consumer;

pub struct ContinueStmt {
	label: com::github::javaparser::ast::expr::simple_name::SimpleName,
}

impl ContinueStmt {
	pub fn new() -> com::github::javaparser::ast::stmt::continue_stmt::ContinueStmt {
		this(null, null);
	}

	pub fn new(label: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::stmt::continue_stmt::ContinueStmt {
		this(null, SimpleName::new(label));
	}

	pub fn new(label: &com::github::javaparser::ast::expr::simple_name::SimpleName) -> com::github::javaparser::ast::stmt::continue_stmt::ContinueStmt {
		this(null, label);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, label: &com::github::javaparser::ast::expr::simple_name::SimpleName) -> com::github::javaparser::ast::stmt::continue_stmt::ContinueStmt {
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

	pub fn set_label(&mut self, label: &com::github::javaparser::ast::expr::simple_name::SimpleName) -> com::github::javaparser::ast::stmt::continue_stmt::ContinueStmt {
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

	pub fn remove_label(&self) -> com::github::javaparser::ast::stmt::continue_stmt::ContinueStmt {
		return self.set_label(null as SimpleName);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::stmt::continue_stmt::ContinueStmt {
		return self.accept(CloneVisitor::new(), null) as ContinueStmt;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::continue_stmt_meta_model::ContinueStmtMetaModel {
		return JavaParserMetaModel::continueStmtMetaModel;
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

	pub fn is_continue_stmt(&self) -> bool {
		return true;
	}

	pub fn as_continue_stmt(&self) -> com::github::javaparser::ast::stmt::continue_stmt::ContinueStmt {
		return self;
	}

	pub fn if_continue_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_continue_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl com::github::javaparser::ast::node_types::node_with_optional_label::NodeWithOptionalLabel for ContinueStmt {}

impl /* Java */ java::lang::Cloneable /**/ for ContinueStmt {}

impl com::github::javaparser::has_parent_node::HasParentNode for ContinueStmt {}

impl com::github::javaparser::ast::observer::observable::Observable for ContinueStmt {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for ContinueStmt {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for ContinueStmt {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for ContinueStmt {}