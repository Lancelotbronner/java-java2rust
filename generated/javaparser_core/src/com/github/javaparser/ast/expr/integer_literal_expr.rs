use crate::com::github::javaparser::utils::Utils::hasUnaryMinusAsParent;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::IntegerLiteralExprMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use java::util::Objects;
use java::util::Optional;
use java::util::function::Consumer;

pub struct IntegerLiteralExpr;

impl IntegerLiteralExpr {
	pub static MAX_31_BIT_UNSIGNED_VALUE_AS_STRING: /* Java */ java::lang::String /**/ = "2147483648";

	pub static MAX_31_BIT_UNSIGNED_VALUE_AS_LONG: i64 = 2147483648;

	pub fn new() -> com::github::javaparser::ast::expr::integer_literal_expr::IntegerLiteralExpr {
		this(null, "0");
	}

	pub fn new(value: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::integer_literal_expr::IntegerLiteralExpr {
		this(null, value);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, value: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::integer_literal_expr::IntegerLiteralExpr {
		super(token_range, value);
		self.custom_initialization();
	}

	pub fn new(value: i32) -> com::github::javaparser::ast::expr::integer_literal_expr::IntegerLiteralExpr {
		this(null, &String::valueOf(value));
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn as_int(&self) -> i32 {
		let result: String = .replaceAll("_", "");
		if result.startsWith("0x") || result.startsWith("0X") {
			return Integer::parseUnsignedInt(&result.substring(2), 16);
		}
		if result.startsWith("0b") || result.startsWith("0B") {
			return Integer::parseUnsignedInt(&result.substring(2), 2);
		}
		if result.length() > 1 && result.startsWith("0") {
			return Integer::parseUnsignedInt(&result.substring(1), 8);
		}
		return Integer::parseInt(result);
	}

	pub fn as_number(&self) -> /* Java */ java::lang::Number /**/ {
		if Objects::equals(, self.MAX_31_BIT_UNSIGNED_VALUE_AS_STRING) && com::github::javaparser::utils::utils::Utils::has_unary_minus_as_parent(self) {
			return self.MAX_31_BIT_UNSIGNED_VALUE_AS_LONG;
		}
		return self.as_int();
	}

	pub fn set_int(&mut self, value: i32) -> com::github::javaparser::ast::expr::integer_literal_expr::IntegerLiteralExpr {
		self.value = String::valueOf(value);
		return self;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::integer_literal_expr::IntegerLiteralExpr {
		return self.accept(CloneVisitor::new(), null) as IntegerLiteralExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::integer_literal_expr_meta_model::IntegerLiteralExprMetaModel {
		return JavaParserMetaModel::integerLiteralExprMetaModel;
	}

	pub fn is_integer_literal_expr(&self) -> bool {
		return true;
	}

	pub fn as_integer_literal_expr(&self) -> com::github::javaparser::ast::expr::integer_literal_expr::IntegerLiteralExpr {
		return self;
	}

	pub fn if_integer_literal_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_integer_literal_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl /* Java */ java::lang::Cloneable /**/ for IntegerLiteralExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for IntegerLiteralExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for IntegerLiteralExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for IntegerLiteralExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for IntegerLiteralExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for IntegerLiteralExpr {}