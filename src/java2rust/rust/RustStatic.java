package java2rust.rust;

import com.github.javaparser.ast.body.FieldDeclaration;
import com.github.javaparser.ast.body.VariableDeclarator;
import com.github.javaparser.resolution.declarations.ResolvedFieldDeclaration;
import java2rust.JavaTranspiler;

public class RustStatic {
	public final RustItem parent;
	public final RustVisibility visibility;
	/// The Rust name of this static field.
	public final String name;
	public final String id;
	public final VariableDeclarator java;
	public final ResolvedFieldDeclaration resolved;
	public String rustType;
	public String rustInitializer;

	public RustStatic(RustItem parent, FieldDeclaration field, VariableDeclarator declarator) {
		this.parent = parent;
		this.visibility = RustVisibility.pub(field.isPublic());
		name = declarator.getNameAsString();
		//TODO: Pass other modifiers from FieldDeclaration
		java = declarator;
		resolved = declarator.resolve().asField();
		rustType = declarator.getTypeAsString();
		id = resolved.declaringType().getId() + "." + declarator.getNameAsString();
	}

	public void analyze(JavaTranspiler transpiler) {
		this.rustType = transpiler.describe(resolved.getType());
		if (java.getInitializer().isPresent())
			this.rustInitializer = transpiler.describe(java.getInitializer().get());
	}

	@Override
	public String toString() {
		StringBuilder sb = new StringBuilder();
		sb.append(visibility);
		sb.append("static ");
		sb.append(name);
		sb.append(": ");
		sb.append(rustType);
		if (rustInitializer != null) {
			sb.append(" = ");
			sb.append(rustInitializer);
		}
		sb.append(";");
		return sb.toString();
	}
}
