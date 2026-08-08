package java2rust;

import com.github.javaparser.ast.body.BodyDeclaration;
import com.github.javaparser.ast.body.InitializerDeclaration;
import com.github.javaparser.ast.body.TypeDeclaration;
import com.github.javaparser.resolution.types.ResolvedType;
import org.jspecify.annotations.NonNull;

import java.util.HashSet;
import java.util.Set;

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
	public RustItem item() { return item; }

	@Override
	public @NonNull RustParams params() {
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

	public void analyze(JavaTranspiler transpiler, RustItem item) {
		calls.analyze(transpiler);
		// Assign all thrown errors
		for (IRustFunction callee : calls.callees)
			thrown.addAll(callee.thrown());
	}
}

