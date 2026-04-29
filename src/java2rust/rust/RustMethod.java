package java2rust.rust;

import com.github.javaparser.ast.body.MethodDeclaration;
import com.github.javaparser.ast.type.ReferenceType;
import com.github.javaparser.resolution.declarations.ResolvedMethodDeclaration;
import com.github.javaparser.resolution.types.ResolvedType;
import java2rust.Java2Rust;
import java2rust.JavaTranspiler;

import java.util.HashSet;
import java.util.Set;
import java.util.stream.Collectors;

public class RustMethod implements IRustFunction {
	public final MethodDeclaration java;
	public final ResolvedMethodDeclaration resolved;
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

	public RustMethod(RustItem item, MethodDeclaration java, ResolvedMethodDeclaration resolved) {
		this.item = item;
		this.java = java;
		this.resolved = resolved == null ? java.resolve() : resolved;
		id = this.resolved.getQualifiedSignature();
		this.visibility = RustVisibility.pub(java.isPublic());
		this.name = Java2Rust.camelCaseToSnakeCase(java.getNameAsString());
		params = new RustParams(RustSelf.REF, java.getParameters());
	}

	public void analyze(JavaTranspiler transpiler) {
		typarams.analyze(resolved, transpiler);
		params.analyze(transpiler);
		calls.analyze(transpiler);
		// Assign all thrown errors
		for (IRustFunction callee : calls.callees)
			thrown.addAll(callee.thrown());
		// Method analysis
		String successType = transpiler.describe(java.getType());
		returnType = java.getType().isVoidType() ? " " : " -> %s ".formatted(successType);
		if (java.getBody().isPresent())
			body = transpiler.describe(java.getBody().orElse(null), this);
		else
			body = ";";
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
}
