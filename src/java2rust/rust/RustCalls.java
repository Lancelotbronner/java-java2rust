package java2rust.rust;

import com.github.javaparser.ast.expr.Expression;
import com.github.javaparser.ast.expr.MethodCallExpr;
import com.github.javaparser.resolution.declarations.ResolvedMethodDeclaration;
import com.github.javaparser.resolution.declarations.ResolvedValueDeclaration;
import com.github.javaparser.utils.Pair;
import java2rust.JavaTranspiler;

import java.util.HashSet;
import java.util.Objects;
import java.util.Set;

public class RustCalls {
	public final Set<IRustFunction> callers = new HashSet<>();
	public final Set<IRustFunction> callees = new HashSet<>();
	private final Set<ResolvedMethodDeclaration> unresolvedCalls = new HashSet<>();

	public void analyze(JavaTranspiler transpiler) {
		callees.clear();
		unresolvedCalls
			.stream()
			.map(transpiler::method)
			.filter(Objects::nonNull)
			.forEach(callees::add);
		unresolvedCalls.clear();
		callees.forEach(this::addCaller);
	}

	public void addCaller(IRustFunction caller) {
		callers.add(caller);
	}

	public void addCallee(MethodCallExpr expr) {
		//TODO: propagate mut self from method call to scope (target)
		unresolvedCalls.add(expr.resolve());
	}
}
