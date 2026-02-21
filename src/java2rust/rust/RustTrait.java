package java2rust.rust;

import com.github.javaparser.resolution.declarations.ResolvedInterfaceDeclaration;
import com.github.javaparser.resolution.declarations.ResolvedTypeParameterDeclaration;
import java2rust.JavaTranspiler;

import java.util.ArrayList;
import java.util.List;
import java.util.StringJoiner;

public class RustTrait extends RustItem {
	public final ResolvedInterfaceDeclaration decl;
	public final List<RustField> fields = new ArrayList<>();
	public final List<String> params = new ArrayList<>();

	RustTrait(
		String name,
		RustModule module,
		ResolvedInterfaceDeclaration decl,
		RustVisibility visibility
	) {
		super(name, module, visibility);
		this.decl = decl;
	}

	//	/// Creates and returns a new field.
	//	public RustField field(String name, Type type, @Nullable Expression initializer) {
	//		RustField field = new RustField(this, name, type, initializer);
	//		fields.add(field);
	//		return field;
	//	}

	@Override
	public void analyze(JavaTranspiler transpiler) {
		super.analyze(transpiler);
		for (RustField field : fields)
			field.analyze(transpiler);

		for (ResolvedTypeParameterDeclaration param : decl.getTypeParameters()) {
			try {
				if (!param.isBounded()) {
					params.add(param.getName());
					continue;
				}
				StringBuilder sb = new StringBuilder();
				sb.append(param.getName());
				sb.append(": ");
				StringJoiner bounds = new StringJoiner("+");
				for (var bound : param.getBounds())
					bounds.add(transpiler.describe(bound.getType()));
				sb.append(bounds);
				params.add(sb.toString());
			} catch (Throwable e) {
				params.add("/* %s */ %s".formatted(e.getMessage(), param.getName()));
			}
		}
	}

	@Override
	public String id() {
		return decl.getId();
	}

	public String path() {
		return "%s::%s".formatted(module.path, name);
	}

	@Override
	public String toString() {
		StringBuilder sb = new StringBuilder();

		sb.append(visibility);
		sb.append("trait ");
		sb.append(name);

		StringJoiner params = new StringJoiner(", ", "<", ">");
		params.setEmptyValue("");
		for (String param : this.params)
			params.add(param);
		sb.append(params);

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
