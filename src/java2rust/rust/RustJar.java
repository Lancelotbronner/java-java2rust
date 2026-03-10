package java2rust.rust;

import org.jspecify.annotations.Nullable;

public final class RustJar {
	/// The name of the crate.
	public final String name;
	/// The path of the crate on the filesystem.
	public final String path;
	/// The library module, if applicable.
	public final @Nullable RustPackage lib;
	/// The executable module, if applicable.
	public final @Nullable RustPackage main;

	public RustJar(String name, String path, @Nullable RustPackage lib, @Nullable RustPackage main) {
		this.name = name;
		this.path = path;
		this.lib = lib;
		this.main = main;
	}

	//TODO: The transpiler will directly parse source Jars and Java files into CompilationUnit and go through DeclVisitor.
	// Then the analysis can be made on the workspace as a whole.
	// Then the Rust hierarchy will be written to files.
}
