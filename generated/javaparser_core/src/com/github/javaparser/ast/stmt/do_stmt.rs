use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::expr::BooleanLiteralExpr;
use crate::com::github::javaparser::ast::expr::Expression;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithBody;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithCondition;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::DoStmtMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct DoStmt {
	body: com::github::javaparser::ast::stmt::statement::Statement,
	condition: com::github::javaparser::ast::expr::expression::Expression,
}

impl DoStmt {
	pub fn new() -> com::github::javaparser::ast::stmt::do_stmt::DoStmt {
		this(null, ReturnStmt::new(), BooleanLiteralExpr::new());
	}

	pub fn new(body: &com::github::javaparser::ast::stmt::statement::Statement, condition: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::stmt::do_stmt::DoStmt {
		this(null, body, condition);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, body: &com::github::javaparser::ast::stmt::statement::Statement, condition: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::stmt::do_stmt::DoStmt {
		super(token_range);
		self.set_body(body);
		self.set_condition(condition);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_body(&self) -> com::github::javaparser::ast::stmt::statement::Statement {
		return self.body;
	}

	pub fn get_condition(&self) -> com::github::javaparser::ast::expr::expression::Expression {
		return self.condition;
	}

	pub fn set_body(&mut self, body: &com::github::javaparser::ast::stmt::statement::Statement) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::do_stmt::DoStmt {
		com::github::javaparser::utils::utils::Utils::assert_not_null(body)?;
		if body == self.body {
			return self;
		}
		self.notify_property_change(ObservableProperty::BODY, self.body, body);
		if self.body != null {
			self.body.set_parent_node(null);
		}
	
		self.body = body;
		self.set_as_parent_node_of(body);
		return self;
	}

	pub fn set_condition(&mut self, condition: &com::github::javaparser::ast::expr::expression::Expression) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::do_stmt::DoStmt {
		com::github::javaparser::utils::utils::Utils::assert_not_null(condition)?;
		if condition == self.condition {
			return self;
		}
		self.notify_property_change(ObservableProperty::CONDITION, self.condition, condition);
		if self.condition != null {
			self.condition.set_parent_node(null);
		}
	
		self.condition = condition;
		self.set_as_parent_node_of(condition);
		return self;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::stmt::do_stmt::DoStmt {
		return self.accept(CloneVisitor::new(), null) as DoStmt;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::do_stmt_meta_model::DoStmtMetaModel {
		return JavaParserMetaModel::doStmtMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.body {
			self.set_body(replacement_node as Statement)?;
			return true;
		}
		if node == self.condition {
			self.set_condition(replacement_node as Expression)?;
			return true;
		}
		return super.replace(node, replacement_node)?;
	}

	pub fn is_do_stmt(&self) -> bool {
		return true;
	}

	pub fn as_do_stmt(&self) -> com::github::javaparser::ast::stmt::do_stmt::DoStmt {
		return self;
	}

	pub fn if_do_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_do_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl com::github::javaparser::ast::node_types::node_with_body::NodeWithBody for DoStmt {}

impl com::github::javaparser::ast::node_types::node_with_condition::NodeWithCondition for DoStmt {}

impl /* Java */ java::lang::Cloneable /**/ for DoStmt {}

impl com::github::javaparser::has_parent_node::HasParentNode for DoStmt {}

impl com::github::javaparser::ast::observer::observable::Observable for DoStmt {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for DoStmt {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for DoStmt {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for DoStmt {}