use crate::com::github::javaparser::utils::StringEscapeUtils::escapeJava;
use crate::com::github::javaparser::utils::StringEscapeUtils::unescapeJava;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::StringLiteralExprMetaModel;
use crate::com::github::javaparser::utils::Utils;
use java::util::Optional;
use java::util::function::Consumer;

pub struct StringLiteralExpr;

impl StringLiteralExpr {
	pub fn new() -> com::github::javaparser::ast::expr::string_literal_expr::StringLiteralExpr {
		this(null, "empty");
	}

	pub fn new(value: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::string_literal_expr::StringLiteralExpr {
		this(null, &Utils::escape_end_of_lines(value));
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, value: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::string_literal_expr::StringLiteralExpr {
		super(token_range, value);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn set_escaped_value(&mut self, value: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::string_literal_expr::StringLiteralExpr {
		self.value = Utils::escape_end_of_lines(value);
		return self;
	}

	pub fn as_string(&self) -> /* Java */ java::lang::String /**/ {
		return com::github::javaparser::utils::string_escape_utils::StringEscapeUtils::unescape_java();
	}

	pub fn set_string(&mut self, value: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::string_literal_expr::StringLiteralExpr {
		self.value = com::github::javaparser::utils::string_escape_utils::StringEscapeUtils::escape_java(value);
		return self;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::string_literal_expr::StringLiteralExpr {
		return self.accept(CloneVisitor::new(), null) as StringLiteralExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::string_literal_expr_meta_model::StringLiteralExprMetaModel {
		return JavaParserMetaModel::stringLiteralExprMetaModel;
	}

	pub fn is_string_literal_expr(&self) -> bool {
		return true;
	}

	pub fn as_string_literal_expr(&self) -> com::github::javaparser::ast::expr::string_literal_expr::StringLiteralExpr {
		return self;
	}

	pub fn if_string_literal_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_string_literal_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl /* Java */ java::lang::Cloneable /**/ for StringLiteralExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for StringLiteralExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for StringLiteralExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for StringLiteralExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for StringLiteralExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for StringLiteralExpr {}