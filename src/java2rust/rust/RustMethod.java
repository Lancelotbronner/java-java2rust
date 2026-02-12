package java2rust.rust;

import com.github.javaparser.ast.body.MethodDeclaration;
import com.github.javaparser.resolution.declarations.ResolvedMethodDeclaration;
import java2rust.Java2Rust;
import java2rust.JavaTranspiler;

public class RustMethod {
	public final RustItem item;
	public final MethodDeclaration java;
	public final ResolvedMethodDeclaration resolved;

	public final String id;
	public final RustVisibility visibility;
	public final String name;
	public final RustTyParams typarams = new RustTyParams();
	public final RustParams params;
	//TODO: track throws
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
		returnType = java.getType()
			.isVoidType() ? "" : " -> %s ".formatted(transpiler.describe(java.getType()));
		if (java.getBody()
			.isPresent())
			body = transpiler.describe(java.getBody()
				.orElse(null));
		else
			body = ";";
	}

	@Override
	public String toString() {
		return visibility + "fn " + name + typarams + params + returnType + body;
	}
}
