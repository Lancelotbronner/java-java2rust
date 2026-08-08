use javaparser_core::com::github::javaparser::ast::expr::DoubleLiteralExpr;
use javaparser_core::com::github::javaparser::ast::expr::IntegerLiteralExpr;
use javaparser_core::com::github::javaparser::ast::expr::LongLiteralExpr;
use javaparser_core::com::github::javaparser::ast::visitor::VoidVisitorAdapter;
use commons_lang3::org::apache::commons::lang3::Strings;

pub struct NameVisitor;

impl NameVisitor {
	pub fn visit(&self, n: &com::github::javaparser::ast::expr::double_literal_expr::DoubleLiteralExpr, arg: &java2rust::java_transpiler::JavaTranspiler) /* thrown(java.lang.AssertionError) */ {
		let is_float: bool = Strings::org::apache::commons::lang3::strings::Strings::CI.ends_with(&n.get_value(), "f");
		let value: String = self.remove_plus_and_suffix(&n.get_value(), "d", "D", "f", "F");
		n.set_value(value)?;
		super.visit(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::integer_literal_expr::IntegerLiteralExpr, arg: &java2rust::java_transpiler::JavaTranspiler) /* thrown(java.lang.AssertionError) */ {
		let value: String = self.remove_plus_and_suffix(&n.get_value());
		n.set_value(value)?;
		super.visit(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::long_literal_expr::LongLiteralExpr, arg: &java2rust::java_transpiler::JavaTranspiler) /* thrown(java.lang.AssertionError) */ {
		let value: String = self.remove_plus_and_suffix(&n.get_value(), "l", "L");
		n.set_value(value)?;
		super.visit(n, arg);
	}

	fn remove_plus_and_suffix(&self, mut value: &/* Java */ java::lang::String /**/, search_strings: &/* Java */ java::lang::CharSequence /**/) -> /* Java */ java::lang::String /**/ {
		if value.startsWith("+") {
			value = value.substring(1);
		}
		if Strings::org::apache::commons::lang3::strings::Strings::CS.ends_with_any(value, search_strings) {
			value = value.substring(0, value.length() - 1);
		}
		return value;
	}
}

impl com::github::javaparser::ast::visitor::void_visitor::VoidVisitor for NameVisitor {}