package java2rust.rust;

import com.github.javaparser.ast.body.MethodDeclaration;
import com.github.javaparser.ast.type.ReferenceType;
import com.github.javaparser.resolution.declarations.ResolvedMethodDeclaration;
import com.github.javaparser.resolution.types.ResolvedType;
import java2rust.AnalyzerVisitor;
import java2rust.Java2Rust;
import java2rust.JavaTranspiler;

import java.util.HashSet;
import java.util.Set;
import java.util.stream.Collectors;

public class RustMethod {
	public final RustItem item;
	public final MethodDeclaration java;
	public final ResolvedMethodDeclaration resolved;

	public final String id;
	public final RustVisibility visibility;
	public final String name;
	public final RustTyParams typarams = new RustTyParams();
	public final RustParams params;
	public final Set<ResolvedType> thrown = new HashSet<>();
	private String returnType;
	private String body;

	RustMethod(RustItem item, MethodDeclaration java) {
		this.item = item;
		this.java = java;
		resolved = java.resolve();
		id = resolved.getQualifiedSignature();
		this.visibility = RustVisibility.pub(java.isPublic());
		this.name = Java2Rust.camelCaseToSnakeCase(java.getNameAsString());
		params = new RustParams(this, java.getParameters());
	}

	public void analyze(JavaTranspiler transpiler) {
		typarams.analyze(resolved, transpiler);
		params.analyze(transpiler);
		String successType = transpiler.describe(java.getType());
		returnType = java.getType().isVoidType() ? " " : " -> %s ".formatted(successType);
		if (java.getBody().isPresent())
			body = transpiler.describe(java.getBody().orElse(null));
		else
			body = ";";
		for (ReferenceType ty : java.getThrownExceptions())
			thrown.add(ty.resolve());
		java.accept(new AnalyzerVisitor(transpiler, this), null);
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
}
