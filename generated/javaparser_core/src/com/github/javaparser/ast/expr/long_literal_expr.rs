use crate::com::github::javaparser::utils::Utils::hasUnaryMinusAsParent;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::LongLiteralExprMetaModel;
use java::math::BigInteger;
use java::util::Objects;
use java::util::Optional;
use java::util::function::Consumer;

pub struct LongLiteralExpr;

impl LongLiteralExpr {
	pub static MAX_63_BIT_UNSIGNED_VALUE_AS_STRING: /* Java */ java::lang::String /**/ = "9223372036854775808L";

	pub static MAX_63_BIT_UNSIGNED_VALUE_AS_BIG_INTEGER: /* Java */ java::math::BigInteger /**/ = BigInteger::new("9223372036854775808");

	pub fn new() -> com::github::javaparser::ast::expr::long_literal_expr::LongLiteralExpr {
		this(null, "0");
	}

	pub fn new(value: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::long_literal_expr::LongLiteralExpr {
		this(null, value);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, value: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::long_literal_expr::LongLiteralExpr {
		super(token_range, value);
		self.custom_initialization();
	}

	pub fn new(value: i64) -> com::github::javaparser::ast::expr::long_literal_expr::LongLiteralExpr {
		this(null, &String::valueOf(value));
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn as_long(&self) -> i64 {
		let result: String = .replaceAll("_", "");
		let last_char: char = result.charAt(result.length() - 1);
		if last_char == 'l' || last_char == 'L' {
			result = result.substring(0, result.length() - 1);
		}
		if result.startsWith("0x") || result.startsWith("0X") {
			return Long::parseUnsignedLong(&result.substring(2), 16);
		}
		if result.startsWith("0b") || result.startsWith("0B") {
			return Long::parseUnsignedLong(&result.substring(2), 2);
		}
		if result.length() > 1 && result.startsWith("0") {
			return Long::parseUnsignedLong(&result.substring(1), 8);
		}
		return Long::parseLong(result);
	}

	pub fn as_number(&self) -> /* Java */ java::lang::Number /**/ {
		if Objects::equals(, self.MAX_63_BIT_UNSIGNED_VALUE_AS_STRING) && com::github::javaparser::utils::utils::Utils::has_unary_minus_as_parent(self) {
			return self.MAX_63_BIT_UNSIGNED_VALUE_AS_BIG_INTEGER;
		}
		return self.as_long();
	}

	pub fn set_long(&mut self, value: i64) -> com::github::javaparser::ast::expr::long_literal_expr::LongLiteralExpr {
		self.value = String::valueOf(value);
		return self;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::long_literal_expr::LongLiteralExpr {
		return self.accept(CloneVisitor::new(), null) as LongLiteralExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::long_literal_expr_meta_model::LongLiteralExprMetaModel {
		return JavaParserMetaModel::longLiteralExprMetaModel;
	}

	pub fn is_long_literal_expr(&self) -> bool {
		return true;
	}

	pub fn as_long_literal_expr(&self) -> com::github::javaparser::ast::expr::long_literal_expr::LongLiteralExpr {
		return self;
	}

	pub fn if_long_literal_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_long_literal_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl /* Java */ java::lang::Cloneable /**/ for LongLiteralExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for LongLiteralExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for LongLiteralExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for LongLiteralExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for LongLiteralExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for LongLiteralExpr {}