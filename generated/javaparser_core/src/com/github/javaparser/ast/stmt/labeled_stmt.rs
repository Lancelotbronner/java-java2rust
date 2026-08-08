use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::expr::SimpleName;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::LabeledStmtMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct LabeledStmt {
	label: com::github::javaparser::ast::expr::simple_name::SimpleName,
	statement: com::github::javaparser::ast::stmt::statement::Statement,
}

impl LabeledStmt {
	pub fn new() -> com::github::javaparser::ast::stmt::labeled_stmt::LabeledStmt {
		this(null, SimpleName::new(), ReturnStmt::new());
	}

	pub fn new(label: &/* Java */ java::lang::String /**/, statement: &com::github::javaparser::ast::stmt::statement::Statement) -> com::github::javaparser::ast::stmt::labeled_stmt::LabeledStmt {
		this(null, SimpleName::new(label), statement);
	}

	pub fn new(label: &com::github::javaparser::ast::expr::simple_name::SimpleName, statement: &com::github::javaparser::ast::stmt::statement::Statement) -> com::github::javaparser::ast::stmt::labeled_stmt::LabeledStmt {
		this(null, label, statement);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, label: &com::github::javaparser::ast::expr::simple_name::SimpleName, statement: &com::github::javaparser::ast::stmt::statement::Statement) -> com::github::javaparser::ast::stmt::labeled_stmt::LabeledStmt {
		super(token_range);
		self.set_label(label);
		self.set_statement(statement);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_statement(&self) -> com::github::javaparser::ast::stmt::statement::Statement {
		return self.statement;
	}

	pub fn set_statement(&mut self, statement: &com::github::javaparser::ast::stmt::statement::Statement) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::labeled_stmt::LabeledStmt {
		com::github::javaparser::utils::utils::Utils::assert_not_null(statement)?;
		if statement == self.statement {
			return self;
		}
		self.notify_property_change(ObservableProperty::STATEMENT, self.statement, statement);
		if self.statement != null {
			self.statement.set_parent_node(null);
		}
	
		self.statement = statement;
		self.set_as_parent_node_of(statement);
		return self;
	}

	pub fn get_label(&self) -> com::github::javaparser::ast::expr::simple_name::SimpleName {
		return self.label;
	}

	pub fn set_label(&mut self, label: &com::github::javaparser::ast::expr::simple_name::SimpleName) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::labeled_stmt::LabeledStmt {
		com::github::javaparser::utils::utils::Utils::assert_not_null(label)?;
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

	pub fn clone(&self) -> com::github::javaparser::ast::stmt::labeled_stmt::LabeledStmt {
		return self.accept(CloneVisitor::new(), null) as LabeledStmt;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::labeled_stmt_meta_model::LabeledStmtMetaModel {
		return JavaParserMetaModel::labeledStmtMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.label {
			self.set_label(replacement_node as SimpleName)?;
			return true;
		}
		if node == self.statement {
			self.set_statement(replacement_node as Statement)?;
			return true;
		}
		return super.replace(node, replacement_node)?;
	}

	pub fn is_labeled_stmt(&self) -> bool {
		return true;
	}

	pub fn as_labeled_stmt(&self) -> com::github::javaparser::ast::stmt::labeled_stmt::LabeledStmt {
		return self;
	}

	pub fn if_labeled_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_labeled_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl /* Java */ java::lang::Cloneable /**/ for LabeledStmt {}

impl com::github::javaparser::has_parent_node::HasParentNode for LabeledStmt {}

impl com::github::javaparser::ast::observer::observable::Observable for LabeledStmt {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for LabeledStmt {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for LabeledStmt {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for LabeledStmt {}