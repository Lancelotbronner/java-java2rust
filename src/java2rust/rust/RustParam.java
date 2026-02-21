package java2rust.rust;

import com.github.javaparser.ast.body.Parameter;
import java2rust.Java2Rust;
import java2rust.JavaTranspiler;

public class RustParam {
	public final RustMethod method;
	public final Parameter java;
	private String name;
	private String type;
	private String cache;

	public RustParam(RustMethod method, Parameter java) {
		this.method = method;
		this.java = java;
	}

	public void analyze(JavaTranspiler transpiler) {
		name = Java2Rust.camelCaseToSnakeCase(java.getNameAsString());
		type = transpiler.describe(java.getType());
		cache = toRust();
	}

	private String toRust() {
		StringBuilder sb = new StringBuilder();
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
