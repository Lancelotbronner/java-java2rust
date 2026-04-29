package java2rust.rust;

import com.github.javaparser.ast.body.ConstructorDeclaration;
import com.github.javaparser.ast.type.ReferenceType;
import com.github.javaparser.resolution.declarations.ResolvedConstructorDeclaration;
import com.github.javaparser.resolution.types.ResolvedType;
import java2rust.JavaTranspiler;

import java.util.HashSet;
import java.util.Set;
import java.util.stream.Collectors;

public class RustConstructor implements IRustFunction {
	public final ConstructorDeclaration java;
	public final ResolvedConstructorDeclaration resolved;
	public final String id;
	public final RustVisibility visibility;
	public final String name;
	public final RustTyParams typarams = new RustTyParams();
	private final RustParams params;
	private final Set<ResolvedType> thrown = new HashSet<>();
	private final RustCalls calls = new RustCalls();
	public RustItem item;
	private String returnType;
	private String body;

	public RustConstructor(
		RustItem item,
		ConstructorDeclaration java,
		ResolvedConstructorDeclaration resolved
	) {
		this.item = item;
		this.java = java;
		this.resolved = resolved == null ? java.resolve() : resolved;
		id = this.resolved.getQualifiedSignature();
		this.visibility = RustVisibility.pub(java.isPublic());
		this.name = "new";
		params = new RustParams(null, java.getParameters());
	}

	@Override
	public String toString() {
		return visibility + "fn " + name + typarams + params + returnType + body;
	}

	@Override
	public RustItem item() { return item; }

	@Override
	public RustParams params() {
		return params;
	}

	@Override
	public RustCalls calls() {
		return calls;
	}

	@Override
	public Set<ResolvedType> thrown() {
		return thrown;
	}

	public void analyze(JavaTranspiler transpiler) {
		typarams.analyze(resolved, transpiler);
		params.analyze(transpiler);
		calls.analyze(transpiler);
		// Assign all thrown errors
		for (IRustFunction callee : calls.callees)
			thrown.addAll(callee.thrown());
		// Method analysis
		String successType = transpiler.describe(resolved.declaringType());
		returnType = " -> %s ".formatted(successType);
		//TODO: body should start with `let mut self = MyType {}` and end with `self`
		body = transpiler.describe(java.getBody(), this);
		for (ReferenceType ty : java.getThrownExceptions())
			thrown.add(ty.resolve());
		if (!thrown.isEmpty()) {
			String errorType = thrown
				.stream()
				.map(ResolvedType::describe)
				.collect(Collectors.joining(" | "));
			returnType = " /* thrown(%s) */%s".formatted(errorType, returnType);
		}
	}
}
