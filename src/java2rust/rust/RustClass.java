package java2rust.rust;

import com.github.javaparser.ast.expr.Expression;
import com.github.javaparser.ast.type.Type;
import com.github.javaparser.resolution.declarations.ResolvedClassDeclaration;
import java2rust.JavaTranspiler;
import org.jspecify.annotations.Nullable;

import java.util.ArrayList;
import java.util.List;
import java.util.StringJoiner;

public class RustClass extends RustItem {
	public final ResolvedClassDeclaration decl;
	public final List<RustField> fields = new ArrayList<>();
	public final RustImpls impls;
	public final RustTyParams typarams = new RustTyParams();

	RustClass(
		String name,
		RustModule module,
		ResolvedClassDeclaration decl,
		RustVisibility visibility
	) {
		super(name, module, visibility);
		this.decl = decl;
		impls = new RustImpls(this, typarams);
	}

	@Override
	public void analyze(JavaTranspiler transpiler) {
		super.analyze(transpiler);
		for (RustField field : fields)
			field.analyze(transpiler);
		typarams.analyze(decl, transpiler);
		impls.analyze(decl, transpiler);
	}

	@Override
	public String id() {
		return decl.getId();
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
		sb.append("struct ");
		sb.append(name);
		sb.append(typarams);

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

		StringJoiner impl = new StringJoiner(
			"\n\n",
			"\n\nimpl%s %s {\n".formatted(typarams, name),
			"\n}");
		impl.setEmptyValue("");
		for (RustMethod method : methods)
			impl.add("\t" + method.toString().replace("\n", "\n\t"));
		sb.append(impl);

		if (!impls.isEmpty()) {
			sb.append("\n\n");
			sb.append(impls);
		}

		return sb.toString();
	}
}
