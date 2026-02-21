package java2rust.rust;

import com.github.javaparser.ast.body.MethodDeclaration;
import com.github.javaparser.ast.expr.Expression;
import com.github.javaparser.ast.type.Type;
import java2rust.JavaTranspiler;
import org.jspecify.annotations.Nullable;

import java.util.ArrayList;
import java.util.List;

public abstract class RustItem {
	/// The Rust name of this item.
	public final String name;
	public final RustModule module;
	public final RustVisibility visibility;
	public final List<RustMethod> methods = new ArrayList<>();

	protected RustItem(String name, RustModule module, RustVisibility visibility) {
		this.name = name;
		this.module = module;
		this.visibility = visibility;
	}

	public void analyze(JavaTranspiler transpiler) {
		for (RustMethod method : methods)
			method.analyze(transpiler);
	}

	public abstract String id();

	public abstract String path();

	public String name() {
		return name;
	}

	public RustField field(String name, Type type, @Nullable Expression initializer) {
		return null;
	}

	public RustMethod method(MethodDeclaration java) {
		RustMethod rust = new RustMethod(this, java);
		methods.add(rust);
		return rust;
	}
}
