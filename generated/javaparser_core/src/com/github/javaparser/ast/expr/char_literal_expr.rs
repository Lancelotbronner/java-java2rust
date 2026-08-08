use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::CharLiteralExprMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::utils::StringEscapeUtils;
use crate::com::github::javaparser::utils::Utils;
use java::util::Optional;
use java::util::function::Consumer;

pub struct CharLiteralExpr;

impl CharLiteralExpr {
	pub fn new() -> com::github::javaparser::ast::expr::char_literal_expr::CharLiteralExpr {
		this(null, "?");
	}

	pub fn new(value: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::char_literal_expr::CharLiteralExpr {
		this(null, value);
	}

	pub fn new(value: u16) -> com::github::javaparser::ast::expr::char_literal_expr::CharLiteralExpr {
		this(null, &StringEscapeUtils::escape_java(&String::valueOf(value)));
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, value: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::char_literal_expr::CharLiteralExpr {
		super(token_range, value);
		self.custom_initialization();
	}

	pub fn escape(&self, string: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::char_literal_expr::CharLiteralExpr {
		return CharLiteralExpr::new(&Utils::escape_end_of_lines(string));
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn as_char(&self) -> u16 {
		return StringEscapeUtils::unescape_java().charAt(0);
	}

	pub fn set_char(&mut self, value: u16) -> com::github::javaparser::ast::expr::char_literal_expr::CharLiteralExpr {
		self.value = String::valueOf(value);
		return self;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::char_literal_expr::CharLiteralExpr {
		return self.accept(CloneVisitor::new(), null) as CharLiteralExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::char_literal_expr_meta_model::CharLiteralExprMetaModel {
		return JavaParserMetaModel::charLiteralExprMetaModel;
	}

	pub fn is_char_literal_expr(&self) -> bool {
		return true;
	}

	pub fn as_char_literal_expr(&self) -> com::github::javaparser::ast::expr::char_literal_expr::CharLiteralExpr {
		return self;
	}

	pub fn if_char_literal_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_char_literal_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl /* Java */ java::lang::Cloneable /**/ for CharLiteralExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for CharLiteralExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for CharLiteralExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for CharLiteralExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for CharLiteralExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for CharLiteralExpr {}