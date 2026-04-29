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

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.*;
import java.util.stream.Stream;

public final class RustPackage extends RustItem {
	public final String path;
	private final List<RustImport> imports = new ArrayList<>();
	private final List<RustPackage> subpackages = new ArrayList<>();
	private final List<RustItem> items = new ArrayList<>();

	RustPackage(String name, RustPackage module, RustVisibility visibility) {
		super(name, module, visibility);
		path = StringUtils.join(ancestors().map(RustPackage::use).toList().reversed(), "::");
	}

	/// Returns a stream that iterates through parent modules starting at the current module.
	public Stream<RustPackage> ancestors() {
		return Stream.iterate(this, Objects::nonNull, m -> m.module);
	}

	public String use() {
		if (module == null)
			return "crate";
		else
			return name;
	}

	public static RustPackage lib(@NonNull String name) {
		return new RustPackage(Java2Rust.camelCaseToSnakeCase(name), null, RustVisibility.INFERRED);
	}

	public void delete() {
		module.subpackages.remove(this);
	}

	public @Nullable RustPackage parent() {
		return module;
	}

	public @NonNull List<RustPackage> submodules() {
		return subpackages;
	}

	public @NonNull List<RustItem> items() {
		return items;
	}

	/// Creates and returns a directory new submodule.
	public RustPackage submodule(String name, RustVisibility visibility) {
		Optional<RustPackage> existing = subpackages
			.stream()
			.filter(m -> Objects.equals(m.name, name))
			.findFirst();
		if (existing.isPresent())
			return existing.get();
		RustPackage mod = new RustPackage(Java2Rust.camelCaseToSnakeCase(name), this, visibility);
		subpackages.add(mod);
		return mod;
	}

	/// Creates and returns a local module.
	public RustPackage mod(String name, RustVisibility visibility) {
		RustPackage mod = new RustPackage(Java2Rust.camelCaseToSnakeCase(name), this, visibility);
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

//	public <T extends RustItem> item(T item) {
//		items.add(item);
//		return item;
//	}

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
	public RustImport use(
		ImportDeclaration java
	) {
		RustImport item = new RustImport(java, this);
		imports.add(item);
		return item;
	}

	public void analyze(JavaTranspiler transpiler) {
		super.analyze(transpiler);
		for (RustItem item : items)
			item.analyze(transpiler);
		for (RustImport imp : imports)
			imp.analyze(transpiler);
		for (RustPackage mod : subpackages)
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

	public void generate(Path parent) {
		Path dir;
		Path file;

		if (module == null) {
			dir = parent;
			file = parent.resolve("lib.rs");
		} else if (subpackages.isEmpty()) {
			dir = parent;
			file = parent.resolve(name + ".rs");
		} else {
			dir = parent.resolve(name);
			file = dir.resolve("mod.rs");
		}

		try {
			Files.createDirectories(dir);
			Files.writeString(file, toString());
		} catch (IOException e) {
			System.err.println(e.getLocalizedMessage());
		}

		for (RustPackage mod : subpackages)
			mod.generate(dir);
	}

	@Override
	public String toString() {
		StringBuilder sb = new StringBuilder();

		if (!subpackages.isEmpty()) {
			for (RustPackage submodule : subpackages)
				sb.append("%smod %s;\n".formatted(submodule.visibility, submodule.name));
		}

		if (!imports.isEmpty()) {
			for (RustImport imp : imports)
				sb.append("%s\n".formatted(imp.toString()));
			sb.append("\n");
		}

		if (!items.isEmpty()) {
			StringJoiner items = new StringJoiner("\n\n");
			for (RustItem item : this.items)
				items.add(item.toString());
			sb.append(items);
		}

		return sb.toString();
	}
}
