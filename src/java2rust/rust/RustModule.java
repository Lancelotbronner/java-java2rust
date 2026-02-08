package java2rust.rust;

import java2rust.Java2Rust;
import org.apache.commons.lang3.StringUtils;
import org.jspecify.annotations.NonNull;
import org.jspecify.annotations.Nullable;

import java.util.ArrayList;
import java.util.List;
import java.util.stream.Stream;

public final class RustModule extends RustItem {
	public final String path;
	private final List<RustModule> submodules = new ArrayList<>();
	private final List<RustItem> items = new ArrayList<>();

	RustModule(String javaName, String rustName, RustModule module, boolean isPublic) {
		super(javaName, rustName, module, isPublic);
		this.path = StringUtils.join(
			hierarchy()
				.map(RustModule::name)
				.toList()
				.reversed(), "::");
	}

	/// Returns a stream that iterates through parent modules starting at the current module.
	public Stream<RustModule> hierarchy() {
		return Stream.iterate(this, m -> m.module != null, m -> m.module);
	}

	public @NonNull String name() {
		return rustName;
	}

	public static RustModule lib(@NonNull String name) {
		return new RustModule(name, Java2Rust.toSnakeCase(name), null, false);
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
	public RustModule submodule(String name, boolean isPublic) {
		RustModule mod = new RustModule(name, Java2Rust.toSnakeCase(name), this, isPublic);
		submodules.add(mod);
		return mod;
	}

	/// Creates and returns a local module.
	public RustModule mod(String name, boolean isPublic) {
		RustModule mod = new RustModule(name, Java2Rust.toSnakeCase(name), this, isPublic);
		items.add(mod);
		return mod;
	}

	/// Declares and returns a new struct.
	public RustStruct struct(String name, boolean isPublic) {
		//TODO: Ensure struct name is PascalCase
		RustStruct item = new RustStruct(name, name, this, isPublic);
		items.add(item);
		return item;
	}

	@Override
	public String toString() {
		StringBuilder sb = new StringBuilder();
		if (!submodules.isEmpty()) {
			for (RustModule submodule : submodules) {
				if (submodule.isPublic)
					sb.append("pub ");
				sb.append("mod ");
				sb.append(submodule.javaName);
				sb.append(";\n");
			}
			sb.append("\n\n");
		}
		for (RustItem item : items) {
			sb.append(item);
			sb.append("\n\n");
		}
		return sb.toString();
	}
}
