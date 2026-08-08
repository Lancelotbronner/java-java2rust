use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::body::Parameter;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithParameters;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::stmt::BlockStmt;
use crate::com::github::javaparser::ast::stmt::ExpressionStmt;
use crate::com::github::javaparser::ast::stmt::ReturnStmt;
use crate::com::github::javaparser::ast::stmt::Statement;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::DerivedProperty;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::LambdaExprMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct LambdaExpr {
	parameters: com::github::javaparser::ast::node_list::NodeList,
	is_enclosing_parameters: bool,
	body: com::github::javaparser::ast::stmt::statement::Statement,
}

impl LambdaExpr {
	pub fn new() -> com::github::javaparser::ast::expr::lambda_expr::LambdaExpr {
		this(null, NodeList<>::new(), ReturnStmt::new(), false);
	}

	pub fn new(parameter: &com::github::javaparser::ast::body::parameter::Parameter, body: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt) -> com::github::javaparser::ast::expr::lambda_expr::LambdaExpr {
		this(null, NodeList<>::new(parameter), body, false);
	}

	pub fn new(parameters: &com::github::javaparser::ast::node_list::NodeList, body: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt) -> com::github::javaparser::ast::expr::lambda_expr::LambdaExpr {
		this(null, parameters, body, true);
	}

	pub fn new(parameter: &com::github::javaparser::ast::body::parameter::Parameter, body: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::expr::lambda_expr::LambdaExpr {
		this(null, NodeList<>::new(parameter), ExpressionStmt::new(body), false);
	}

	pub fn new(parameters: &com::github::javaparser::ast::node_list::NodeList, body: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::expr::lambda_expr::LambdaExpr {
		this(null, parameters, ExpressionStmt::new(body), true);
	}

	pub fn new(parameters: &com::github::javaparser::ast::node_list::NodeList, body: &com::github::javaparser::ast::stmt::statement::Statement, is_enclosing_parameters: bool) -> com::github::javaparser::ast::expr::lambda_expr::LambdaExpr {
		this(null, parameters, body, is_enclosing_parameters);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, parameters: &com::github::javaparser::ast::node_list::NodeList, body: &com::github::javaparser::ast::stmt::statement::Statement, is_enclosing_parameters: bool) -> com::github::javaparser::ast::expr::lambda_expr::LambdaExpr {
		super(token_range);
		self.set_parameters(parameters);
		self.set_body(body);
		self.set_enclosing_parameters(is_enclosing_parameters);
		self.custom_initialization();
	}

	pub fn get_parameters(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.parameters;
	}

	pub fn set_parameters(&mut self, parameters: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::lambda_expr::LambdaExpr {
		com::github::javaparser::utils::utils::Utils::assert_not_null(parameters)?;
		if parameters == self.parameters {
			return self;
		}
		self.notify_property_change(ObservableProperty::PARAMETERS, self.parameters, parameters);
		if self.parameters != null {
			self.parameters.set_parent_node(null);
		}
	
		self.parameters = parameters;
		self.set_as_parent_node_of(parameters);
		return self;
	}

	pub fn get_body(&self) -> com::github::javaparser::ast::stmt::statement::Statement {
		return self.body;
	}

	pub fn set_body(&mut self, body: &com::github::javaparser::ast::stmt::statement::Statement) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::lambda_expr::LambdaExpr {
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

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn is_enclosing_parameters(&self) -> bool {
		return self.is_enclosing_parameters;
	}

	pub fn set_enclosing_parameters(&mut self, is_enclosing_parameters: bool) -> com::github::javaparser::ast::expr::lambda_expr::LambdaExpr {
		if is_enclosing_parameters == self.isEnclosingParameters {
			return self;
		}
		self.notify_property_change(ObservableProperty::ENCLOSING_PARAMETERS, self.isEnclosingParameters, is_enclosing_parameters);
		self.isEnclosingParameters = is_enclosing_parameters;
		return self;
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.parameters.size() {
				{
					if self.parameters.get(i) == node {
						self.parameters.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn get_expression_body(&self) /* thrown(java.lang.IllegalStateException) */ -> /* Java */ java::util::Optional /**/ {
		if self.body.is_expression_stmt() {
			return Optional::of(&self.body.as_expression_stmt()?.get_expression());
		}
		return Optional::empty();
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::lambda_expr::LambdaExpr {
		return self.accept(CloneVisitor::new(), null) as LambdaExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::lambda_expr_meta_model::LambdaExprMetaModel {
		return JavaParserMetaModel::lambdaExprMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.body {
			self.set_body(replacement_node as Statement)?;
			return true;
		}
		 {
			let i: i32 = 0;
			while i < self.parameters.size() {
				{
					if self.parameters.get(i) == node {
						self.parameters.set(i, replacement_node as Parameter)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.replace(node, replacement_node);
	}

	pub fn is_lambda_expr(&self) -> bool {
		return true;
	}

	pub fn as_lambda_expr(&self) -> com::github::javaparser::ast::expr::lambda_expr::LambdaExpr {
		return self;
	}

	pub fn if_lambda_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_lambda_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn is_poly_expression(&self) -> bool {
		return true;
	}

	pub fn is_explicitly_typed(&self) -> bool {
		return self.get_parameters().stream().allMatch(|p|!(p.get_type().is_unknown_type()));
	}
}

impl com::github::javaparser::ast::node_types::node_with_parameters::NodeWithParameters for LambdaExpr {}

impl /* Java */ java::lang::Cloneable /**/ for LambdaExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for LambdaExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for LambdaExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for LambdaExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for LambdaExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for LambdaExpr {}