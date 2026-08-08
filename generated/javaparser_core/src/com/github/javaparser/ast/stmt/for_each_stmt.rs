use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Modifier;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::body::VariableDeclarator;
use crate::com::github::javaparser::ast::expr::Expression;
use crate::com::github::javaparser::ast::expr::NameExpr;
use crate::com::github::javaparser::ast::expr::VariableDeclarationExpr;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithBody;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::ForEachStmtMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct ForEachStmt {
	variable: com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr,
	iterable: com::github::javaparser::ast::expr::expression::Expression,
	body: com::github::javaparser::ast::stmt::statement::Statement,
}

impl ForEachStmt {
	pub fn new() -> com::github::javaparser::ast::stmt::for_each_stmt::ForEachStmt {
		this(null, VariableDeclarationExpr::new(), NameExpr::new(), ReturnStmt::new());
	}

	pub fn new(variable: &com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr, iterable: &com::github::javaparser::ast::expr::expression::Expression, body: &com::github::javaparser::ast::stmt::statement::Statement) -> com::github::javaparser::ast::stmt::for_each_stmt::ForEachStmt {
		this(null, variable, iterable, body);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, variable: &com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr, iterable: &com::github::javaparser::ast::expr::expression::Expression, body: &com::github::javaparser::ast::stmt::statement::Statement) -> com::github::javaparser::ast::stmt::for_each_stmt::ForEachStmt {
		super(token_range);
		self.set_variable(variable);
		self.set_iterable(iterable);
		self.set_body(body);
		self.custom_initialization();
	}

	pub fn new(variable: &com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr, iterable: &/* Java */ java::lang::String /**/, body: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt) -> com::github::javaparser::ast::stmt::for_each_stmt::ForEachStmt {
		this(null, variable, NameExpr::new(iterable), body);
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

	pub fn get_iterable(&self) -> com::github::javaparser::ast::expr::expression::Expression {
		return self.iterable;
	}

	pub fn get_variable(&self) -> com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr {
		return self.variable;
	}

	pub fn set_body(&mut self, body: &com::github::javaparser::ast::stmt::statement::Statement) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::for_each_stmt::ForEachStmt {
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

	pub fn set_iterable(&mut self, iterable: &com::github::javaparser::ast::expr::expression::Expression) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::for_each_stmt::ForEachStmt {
		com::github::javaparser::utils::utils::Utils::assert_not_null(iterable)?;
		if iterable == self.iterable {
			return self;
		}
		self.notify_property_change(ObservableProperty::ITERABLE, self.iterable, iterable);
		if self.iterable != null {
			self.iterable.set_parent_node(null);
		}
	
		self.iterable = iterable;
		self.set_as_parent_node_of(iterable);
		return self;
	}

	pub fn set_variable(&mut self, variable: &com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::for_each_stmt::ForEachStmt {
		com::github::javaparser::utils::utils::Utils::assert_not_null(variable)?;
		if variable == self.variable {
			return self;
		}
		self.notify_property_change(ObservableProperty::VARIABLE, self.variable, variable);
		if self.variable != null {
			self.variable.set_parent_node(null);
		}
	
		self.variable = variable;
		self.set_as_parent_node_of(variable);
		return self;
	}

	pub fn get_variable_declarator(&self) -> com::github::javaparser::ast::body::variable_declarator::VariableDeclarator {
		return self.get_variable().get_variable(0);
	}

	pub fn has_final_variable(&self) -> bool {
		return self.get_variable().get_modifiers().is_non_empty() && self.get_variable().get_modifiers().get(0).get_keyword() == Modifier::com::github::javaparser::ast::modifier::Keyword::FINAL;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::stmt::for_each_stmt::ForEachStmt {
		return self.accept(CloneVisitor::new(), null) as ForEachStmt;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.body {
			self.set_body(replacement_node as Statement)?;
			return true;
		}
		if node == self.iterable {
			self.set_iterable(replacement_node as Expression)?;
			return true;
		}
		if node == self.variable {
			self.set_variable(replacement_node as VariableDeclarationExpr)?;
			return true;
		}
		return super.replace(node, replacement_node)?;
	}

	pub fn is_for_each_stmt(&self) -> bool {
		return true;
	}

	pub fn as_for_each_stmt(&self) -> com::github::javaparser::ast::stmt::for_each_stmt::ForEachStmt {
		return self;
	}

	pub fn to_for_each_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn if_for_each_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::for_each_stmt_meta_model::ForEachStmtMetaModel {
		return JavaParserMetaModel::forEachStmtMetaModel;
	}
}

impl com::github::javaparser::ast::node_types::node_with_body::NodeWithBody for ForEachStmt {}

impl /* Java */ java::lang::Cloneable /**/ for ForEachStmt {}

impl com::github::javaparser::has_parent_node::HasParentNode for ForEachStmt {}

impl com::github::javaparser::ast::observer::observable::Observable for ForEachStmt {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for ForEachStmt {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for ForEachStmt {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for ForEachStmt {}