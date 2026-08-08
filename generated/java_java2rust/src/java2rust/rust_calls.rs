use javaparser_core::com::github::javaparser::ast::expr::MethodCallExpr;
use javaparser_core::com::github::javaparser::resolution::declarations::ResolvedMethodDeclaration;
use java::util::HashSet;
use java::util::Objects;
use java::util::Set;

pub struct RustCalls {
	callers: /* Java */ java::util::Set /**/ = HashSet<>::new(),
	callees: /* Java */ java::util::Set /**/ = HashSet<>::new(),
	unresolved_calls: /* Java */ java::util::Set /**/ = HashSet<>::new(),
}

impl RustCalls {
	pub fn analyze(&self, transpiler: &java2rust::java_transpiler::JavaTranspiler) {
		self.callees.clear();
		self.unresolved_calls.stream().map(transpiler::method).filter(Objects::nonNull).forEach(callees::add);
		self.unresolved_calls.clear();
		self.callees.forEach(self::addCaller);
	}

	pub fn add_caller(&self, caller: &java2rust::i_rust_function::IRustFunction) {
		self.callers.add(caller);
	}

	pub fn add_callee(&self, expr: &com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr) {
		//TODO: propagate mut self from method call to scope (target)
		self.unresolved_calls.add(&expr.resolve());
	}
}