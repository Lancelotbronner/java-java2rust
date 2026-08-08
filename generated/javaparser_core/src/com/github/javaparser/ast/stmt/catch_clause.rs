use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast;
use crate::com::github::javaparser::ast::body::Parameter;
use crate::com::github::javaparser::ast::expr::AnnotationExpr;
use crate::com::github::javaparser::ast::expr::SimpleName;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithBlockStmt;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::type::ClassOrInterfaceType;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::CatchClauseMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;

pub struct CatchClause {
	parameter: com::github::javaparser::ast::body::parameter::Parameter,
	body: com::github::javaparser::ast::stmt::block_stmt::BlockStmt,
}

impl CatchClause {
	pub fn new() -> com::github::javaparser::ast::stmt::catch_clause::CatchClause {
		this(null, Parameter::new(), BlockStmt::new());
	}

	pub fn new(except_modifier: &com::github::javaparser::ast::node_list::NodeList, except_annotations: &com::github::javaparser::ast::node_list::NodeList, except_type: &com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType, except_name: &com::github::javaparser::ast::expr::simple_name::SimpleName, body: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt) -> com::github::javaparser::ast::stmt::catch_clause::CatchClause {
		this(null, Parameter::new(null, except_modifier, except_annotations, except_type, false, NodeList<>::new(), except_name), body);
	}

	pub fn new(parameter: &com::github::javaparser::ast::body::parameter::Parameter, body: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt) -> com::github::javaparser::ast::stmt::catch_clause::CatchClause {
		this(null, parameter, body);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, parameter: &com::github::javaparser::ast::body::parameter::Parameter, body: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt) -> com::github::javaparser::ast::stmt::catch_clause::CatchClause {
		super(token_range);
		self.set_parameter(parameter);
		self.set_body(body);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_parameter(&self) -> com::github::javaparser::ast::body::parameter::Parameter {
		return self.parameter;
	}

	pub fn set_parameter(&mut self, parameter: &com::github::javaparser::ast::body::parameter::Parameter) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::catch_clause::CatchClause {
		com::github::javaparser::utils::utils::Utils::assert_not_null(parameter)?;
		if parameter == self.parameter {
			return self;
		}
		self.notify_property_change(ObservableProperty::PARAMETER, self.parameter, parameter);
		if self.parameter != null {
			self.parameter.set_parent_node(null);
		}
	
		self.parameter = parameter;
		self.set_as_parent_node_of(parameter);
		return self;
	}

	pub fn get_body(&self) -> com::github::javaparser::ast::stmt::block_stmt::BlockStmt {
		return self.body;
	}

	pub fn set_body(&mut self, body: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::catch_clause::CatchClause {
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

	pub fn clone(&self) -> com::github::javaparser::ast::stmt::catch_clause::CatchClause {
		return self.accept(CloneVisitor::new(), null) as CatchClause;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::catch_clause_meta_model::CatchClauseMetaModel {
		return JavaParserMetaModel::catchClauseMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.body {
			self.set_body(replacement_node as BlockStmt)?;
			return true;
		}
		if node == self.parameter {
			self.set_parameter(replacement_node as Parameter)?;
			return true;
		}
		return super.replace(node, replacement_node)?;
	}
}

impl com::github::javaparser::ast::node_types::node_with_block_stmt::NodeWithBlockStmt for CatchClause {}

impl /* Java */ java::lang::Cloneable /**/ for CatchClause {}

impl com::github::javaparser::has_parent_node::HasParentNode for CatchClause {}

impl com::github::javaparser::ast::observer::observable::Observable for CatchClause {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for CatchClause {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for CatchClause {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for CatchClause {}