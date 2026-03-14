package java2rust.rust;

import com.github.javaparser.ast.body.Parameter;
import java2rust.Java2Rust;
import java2rust.JavaTranspiler;

public class RustParam {
	public final Parameter java;
	public boolean isMutable;
	private final String name;
	private String type;
	private String cache;

	public RustParam(Parameter java) {
		this.java = java;
		name = Java2Rust.camelCaseToSnakeCase(java.getNameAsString());
	}

	public void analyze(JavaTranspiler transpiler) {
		type = transpiler.describe(java.getType());
		cache = toRust();
	}

	private String toRust() {
		StringBuilder sb = new StringBuilder();
		if (isMutable)
			sb.append("mut ");
		sb.append(name);
		sb.append(": ");
		if (java.getType().isReferenceType())
			sb.append("&");
		sb.append(type);
		return sb.toString();
	}

	@Override
	public String toString() {
		if (cache == null)
			cache = toRust();
		return cache;
	}
}
