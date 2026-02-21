package java2rust.rust;

import com.github.javaparser.ast.ImportDeclaration;
import com.github.javaparser.ast.body.EnumDeclaration;
import com.github.javaparser.resolution.declarations.ResolvedClassDeclaration;
import com.github.javaparser.resolution.declarations.ResolvedInterfaceDeclaration;
import com.github.javaparser.resolution.declarations.ResolvedRecordDeclaration;
import java2rust.Java2Rust;
import java2rust.JavaTranspiler;
import org.apache.commons.lang3.StringUtils;
import org.jspecify.annotations.NonNull;
import org.jspecify.annotations.Nullable;

import java.util.ArrayList;
import java.util.List;
import java.util.Objects;
import java.util.StringJoiner;
import java.util.stream.Stream;

//TODO: Make RustModule a class, have RustCrate, RustFileModule and RustDirectoryModule, RustMod, etc.
public final class RustModule extends RustItem {
	public final String path;
	private final List<RustUse> uses = new ArrayList<>();
	private final List<RustModule> submodules = new ArrayList<>();
	private final List<RustItem> items = new ArrayList<>();

	RustModule(String name, RustModule module, RustVisibility visibility) {
		super(name, module, visibility);
		path = StringUtils.join(ancestors().map(RustModule::use).toList().reversed(), "::");
	}

	/// Returns a stream that iterates through parent modules starting at the current module.
	public Stream<RustModule> ancestors() {
		return Stream.iterate(this, Objects::nonNull, m -> m.module);
	}

	public String use() {
		if (module == null)
			return "crate";
		else
			return name;
	}

	public static RustModule lib(@NonNull String name) {
		return new RustModule(Java2Rust.camelCaseToSnakeCase(name), null, RustVisibility.PRIVATE);
	}

	public void delete() {
		module.submodules.remove(this);
	}

	public @Nullable RustModule parent() {
		return module;
	}

	public @NonNull List<RustModule> submodules() {
		return submodules;
	}

	public @NonNull List<RustItem> items() {
		return items;
	}

	/// Creates and returns a directory new submodule.
	public RustModule submodule(String name, RustVisibility visibility) {
		RustModule mod = new RustModule(Java2Rust.camelCaseToSnakeCase(name), this, visibility);
		submodules.add(mod);
		return mod;
	}

	/// Creates and returns a local module.
	public RustModule mod(String name, RustVisibility visibility) {
		RustModule mod = new RustModule(Java2Rust.camelCaseToSnakeCase(name), this, visibility);
		items.add(mod);
		return mod;
	}

	/// Declares and returns a new class.
	public RustClass clazz(String name, ResolvedClassDeclaration decl, RustVisibility visibility) {
		//TODO: Ensure struct name is PascalCase
		RustClass item = new RustClass(name, this, decl, visibility);
		items.add(item);
		return item;
	}

	/// Declares and returns a new record.
	public RustRecord record(
		String name,
		ResolvedRecordDeclaration decl,
		RustVisibility visibility
	) {
		//TODO: Ensure struct name is PascalCase
		RustRecord item = new RustRecord(name, this, decl, visibility);
		items.add(item);
		return item;
	}

	/// Declares and returns a new trait.
	public RustInterface trait(
		String name,
		ResolvedInterfaceDeclaration decl,
		RustVisibility visibility
	) {
		//TODO: Ensure trait name is PascalCase
		RustInterface item = new RustInterface(name, this, decl, visibility);
		items.add(item);
		return item;
	}

	/// Declares and returns a new enum.
	public RustEnum enumeration(
		EnumDeclaration java
	) {
		//TODO: Ensure trait name is PascalCase
		RustEnum item = new RustEnum(java, this);
		items.add(item);
		return item;
	}

	/// Declares and returns a new use.
	public RustUse use(
		ImportDeclaration java
	) {
		RustUse item = new RustUse(java, this);
		uses.add(item);
		return item;
	}

	public void analyze(JavaTranspiler transpiler) {
		super.analyze(transpiler);
		for (RustItem item : items)
			item.analyze(transpiler);
		for (RustModule mod : submodules)
			mod.analyze(transpiler);
	}

	@Override
	public String id() {
		return path;
	}

	@Override
	public String path() {
		return path;
	}

	@Override
	public String toString() {
		StringBuilder sb = new StringBuilder();

		if (!submodules.isEmpty()) {
			for (RustModule submodule : submodules)
				sb.append("%smod %s;\n".formatted(submodule.visibility, submodule.name));
			sb.append("\n");
		}

		StringJoiner items = new StringJoiner("\n\n");
		for (RustItem item : this.items)
			items.add(item.toString());
		sb.append(items);

		sb.append("\n");
		return sb.toString();
	}
}
