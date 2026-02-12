package java2rust.rust;

import com.github.javaparser.ast.expr.Expression;
import com.github.javaparser.ast.type.Type;
import java2rust.JavaTranspiler;
import org.jspecify.annotations.Nullable;

public class RustField {
	public final String name;
	public final Type javaType;
	public final @Nullable Expression javaInitializer;
	public String rustType;
	public String rustInitializer;

	RustField(String name, Type javaType, @Nullable Expression javaInitializer) {
		this.name = name;
		this.javaType = javaType;
		this.javaInitializer = javaInitializer;
	}

	public void analyze(JavaTranspiler transpiler) {
		rustType = transpiler.describe(javaType);
		if (javaInitializer != null)
			rustInitializer = transpiler.describe(javaInitializer);
	}

	@Override
	public String toString() {
		String type = rustType == null ? javaType.toString() : rustType;
		String initializer = rustInitializer == null ? (javaInitializer == null ? null : javaInitializer.toString()) : rustInitializer;
		if (initializer == null)
			return "%s: %s".formatted(name, type);
		return "%s: %s = %s".formatted(name, type, initializer);
	}
}
