package java2rust.rust;

import com.github.javaparser.ast.expr.Expression;
import com.github.javaparser.ast.type.Type;
import org.jspecify.annotations.Nullable;

public class RustField {
	public final RustStruct struct;
	public final String name;
	public final Type type;
	public final @Nullable Expression initializer;

	RustField(RustStruct struct, String name, Type type, @Nullable Expression initializer) {
		this.struct = struct;
		this.name = name;
		this.type = type;
		this.initializer = initializer;
	}

	@Override
	public String toString() {
		if (initializer == null)
			return "%s: %s".formatted(name, type);
		return "%s: %s = %s".formatted(name, type, initializer.toString());
	}
}
