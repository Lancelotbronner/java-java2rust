package java2rust.rust;

import com.github.javaparser.ast.expr.Expression;
import com.github.javaparser.ast.type.Type;
import org.jspecify.annotations.Nullable;

import java.util.ArrayList;
import java.util.List;

public class RustStruct extends RustItem {
	public final List<RustField> fields = new ArrayList<>();

	RustStruct(String javaName, String rustName, RustModule module, boolean isPublic) {
		super(javaName, rustName, module, isPublic);
	}

	/// Creates and returns a new field.
	public RustField field(String name, Type type, @Nullable Expression initializer) {
		RustField field = new RustField(this, name, type, initializer);
		fields.add(field);
		return field;
	}

	@Override
	public String toString() {
		StringBuilder sb = new StringBuilder();
		if (isPublic)
			sb.append("pub ");
		sb.append("struct ");
		sb.append(javaName);
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