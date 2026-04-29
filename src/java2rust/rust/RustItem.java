package java2rust.rust;

import com.github.javaparser.ast.body.*;
import com.github.javaparser.ast.expr.Expression;
import com.github.javaparser.ast.type.Type;
import java2rust.JavaTranspiler;
import org.jspecify.annotations.Nullable;

import java.util.ArrayList;
import java.util.List;

public abstract class RustItem {
	/// The Rust name of this item.
	public final String name;
	public final RustPackage module;
	public final RustVisibility visibility;
	public final List<IRustFunction> methods = new ArrayList<>();
	public final List<RustStatic> statics = new ArrayList<>();

	protected RustItem(String name, RustPackage module, RustVisibility visibility) {
		this.name = name;
		this.module = module;
		this.visibility = visibility;
	}

	public void analyze(JavaTranspiler transpiler) {
		for (RustStatic field : statics)
			field.analyze(transpiler);
		for (IRustFunction method : methods)
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

	public RustStatic staticField(FieldDeclaration field, VariableDeclarator declarator) {
		RustStatic tmp = new RustStatic(this, field, declarator);
		statics.add(tmp);
		return tmp;
	}

	public RustMethod method(MethodDeclaration java) {
		RustMethod rust = new RustMethod(this, java, java.resolve());
		methods.add(rust);
		return rust;
	}

	public RustConstructor constructor(ConstructorDeclaration java) {
		RustConstructor rust = new RustConstructor(this, java, java.resolve());
		methods.add(rust);
		return rust;
	}

	public RustInitializer initializer(InitializerDeclaration java) {
		RustInitializer rust = new RustInitializer(this, java);
		methods.add(rust);
		return rust;
	}
}
