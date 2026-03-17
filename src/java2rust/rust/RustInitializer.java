package java2rust.rust;

import com.github.javaparser.ast.body.BodyDeclaration;
import com.github.javaparser.ast.body.ConstructorDeclaration;
import com.github.javaparser.ast.body.InitializerDeclaration;
import com.github.javaparser.ast.body.TypeDeclaration;
import com.github.javaparser.ast.type.ReferenceType;
import com.github.javaparser.resolution.declarations.ResolvedConstructorDeclaration;
import com.github.javaparser.resolution.types.ResolvedType;
import java2rust.JavaTranspiler;

import java.util.HashSet;
import java.util.Set;
import java.util.stream.Collectors;

public class RustInitializer implements IRustFunction {
	public final InitializerDeclaration java;
	public final String id;
	private final Set<ResolvedType> thrown = new HashSet<>();
	private final RustCalls calls = new RustCalls();
	public RustItem item;
	private String body;

	public RustInitializer(
		RustItem item,
		InitializerDeclaration java
	) {
		this.item = item;
		this.java = java;
		TypeDeclaration<?> ty = ((BodyDeclaration<?>)(java.getParentNode().get())).asTypeDeclaration();
		id = ty.getFullyQualifiedName().orElse(ty.getNameAsString());
	}

	@Override
	public String toString() {
		return "init %s".formatted(java.getBody());
	}

	@Override
	public RustParams params() {
		return RustParams.EMPTY;
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
		calls.analyze(transpiler);
		// Assign all thrown errors
		for (IRustFunction callee : calls.callees)
			thrown.addAll(callee.thrown());
	}
}

