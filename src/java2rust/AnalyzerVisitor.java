package java2rust;

import com.github.javaparser.ast.expr.LambdaExpr;
import com.github.javaparser.ast.expr.MethodCallExpr;
import com.github.javaparser.ast.stmt.ThrowStmt;
import com.github.javaparser.ast.visitor.VoidVisitorAdapter;
import com.github.javaparser.resolution.declarations.ResolvedMethodDeclaration;
import com.github.javaparser.resolution.types.ResolvedType;
import java2rust.rust.RustMethod;

public class AnalyzerVisitor extends VoidVisitorAdapter<Object> {
	public final JavaTranspiler transpiler;
	public final RustMethod method;

	public AnalyzerVisitor(JavaTranspiler transpiler, RustMethod method) {
		this.transpiler = transpiler;
		this.method = method;
	}

	@Override
	public void visit(MethodCallExpr n, Object arg) {
		ResolvedMethodDeclaration resolved = n.resolve();

		// Propagate thrown exceptions
		RustMethod rust = transpiler.method(resolved.getQualifiedSignature());
		if (rust != null)
			method.thrown.addAll(rust.thrown);
		method.thrown.addAll(resolved.getSpecifiedExceptions());
		//TODO: This is running once at Decl, then at Analyze. Is it enough to catch all?

		super.visit(n, arg);
	}

	@Override
	public void visit(ThrowStmt n, Object arg) {
		super.visit(n, arg);
		try {
			ResolvedType ty = n.getExpression().calculateResolvedType();
			method.thrown.add(ty);
		} catch (Throwable e) {
			System.err.println(e);
		}
	}

	@Override
	public void visit(LambdaExpr n, Object arg) {
		// Ignore lambdas for now
	}
}
