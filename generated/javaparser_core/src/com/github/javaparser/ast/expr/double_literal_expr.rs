use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::DoubleLiteralExprMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct DoubleLiteralExpr;

impl DoubleLiteralExpr {
	pub fn new() -> com::github::javaparser::ast::expr::double_literal_expr::DoubleLiteralExpr {
		this(null, "0");
	}

	pub fn new(value: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::double_literal_expr::DoubleLiteralExpr {
		this(null, value);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, value: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::double_literal_expr::DoubleLiteralExpr {
		super(token_range, value);
		self.custom_initialization();
	}

	pub fn new(value: f64) -> com::github::javaparser::ast::expr::double_literal_expr::DoubleLiteralExpr {
		this(null, &String::valueOf(value));
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn as_double(&self) -> f64 {
		// Underscores are allowed in number literals for readability reasons but cause a NumberFormatException if
		// passed along to Double#parseDouble. Hence, we apply a simple filter to remove all underscores.
		// See https://github.com/javaparser/javaparser/issues/1980 for more information.
		let no_underscore_value: String = .replaceAll("_", "");
		return Double::parseDouble(no_underscore_value);
	}

	pub fn set_double(&mut self, value: f64) -> com::github::javaparser::ast::expr::double_literal_expr::DoubleLiteralExpr {
		self.value = String::valueOf(value);
		return self;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::double_literal_expr::DoubleLiteralExpr {
		return self.accept(CloneVisitor::new(), null) as DoubleLiteralExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::double_literal_expr_meta_model::DoubleLiteralExprMetaModel {
		return JavaParserMetaModel::doubleLiteralExprMetaModel;
	}

	pub fn is_double_literal_expr(&self) -> bool {
		return true;
	}

	pub fn as_double_literal_expr(&self) -> com::github::javaparser::ast::expr::double_literal_expr::DoubleLiteralExpr {
		return self;
	}

	pub fn if_double_literal_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_double_literal_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl /* Java */ java::lang::Cloneable /**/ for DoubleLiteralExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for DoubleLiteralExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for DoubleLiteralExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for DoubleLiteralExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for DoubleLiteralExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for DoubleLiteralExpr {}