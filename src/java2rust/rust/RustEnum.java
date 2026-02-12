package java2rust.rust;

import com.github.javaparser.ast.body.EnumDeclaration;
import com.github.javaparser.ast.expr.Expression;
import com.github.javaparser.ast.type.Type;
import com.github.javaparser.resolution.declarations.ResolvedEnumDeclaration;
import java2rust.JavaTranspiler;
import org.jspecify.annotations.Nullable;

import java.util.ArrayList;
import java.util.List;
import java.util.StringJoiner;

public class RustEnum extends RustItem {
	public final EnumDeclaration java;
	public final ResolvedEnumDeclaration resolved;

	public final List<RustField> fields = new ArrayList<>();

	RustEnum(EnumDeclaration java, RustModule module) {
		super(java.getNameAsString(), module, RustVisibility.pub(java.isPublic()));
		this.java = java;
		this.resolved = java.resolve();
	}

	@Override
	public void analyze(JavaTranspiler transpiler) {
		super.analyze(transpiler);
		for (RustField field : fields)
			field.analyze(transpiler);
	}

	@Override
	public String id() {
		return resolved.getId();
	}

	public String path() {
		return "%s::%s".formatted(module.path, name);
	}

	/// Creates and returns a new field.
	public RustField field(String name, Type type, @Nullable Expression initializer) {
		RustField field = new RustField(name, type, initializer);
		fields.add(field);
		return field;
	}

	@Override
	public String toString() {
		StringBuilder sb = new StringBuilder();

		sb.append(visibility);
		sb.append("enum ");
		sb.append(name);

		if (fields.isEmpty()) {
			sb.append(';');
			return sb.toString();
		}

		sb.append(" {\n");
		for (RustField field : fields) {
			sb.append('\t');
			sb.append(field);
			sb.append(",\n");
		}
		sb.append('}');

		return sb.toString();
	}
}

