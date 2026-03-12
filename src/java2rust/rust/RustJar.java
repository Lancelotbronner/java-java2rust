package java2rust.rust;

import com.github.javaparser.JavaParser;
import com.github.javaparser.ParseResult;
import com.github.javaparser.StaticJavaParser;
import com.github.javaparser.ast.CompilationUnit;
import com.github.javaparser.utils.SourceZip;
import java2rust.DeclVisitor;
import org.jspecify.annotations.Nullable;

import java.io.IOException;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;

public final class RustJar {
	/// A unique identifier for this jar, either a path or registry id.
	public final String id;
	/// The name of the crate.
	public final String name;
	/// The path of the crate on the filesystem.
	public final Path path;
	/// The units in this jar.
	public final List<RustUnit> units = new ArrayList<>();
	/// The library module, if applicable.
	public final @Nullable RustPackage lib;
	/// The executable module, if applicable.
	public final @Nullable RustPackage main;

	public RustJar(String id, String name, Path path, @Nullable RustPackage lib, @Nullable RustPackage main) {
		this.id = id;
		this.name = name;
		this.path = path;
		this.lib = lib;
		this.main = main;
	}

	/// Adds a new unit to this jar.
	public void add(Path path) throws IOException {
		units.add(new RustUnit(this, path));
	}

	/// Parses a source jar, creating units for each source file within.
	public RustJar(String id, String name, SourceZip jar) throws IOException {
		this.id = id;
		this.name = name;
		this.path = jar.getZipPath();
		lib = new RustPackage(name, null, RustVisibility.PUB);
		main = null;
		jar.setParserConfiguration(StaticJavaParser.getParserConfiguration());
		jar.parse((SourceZip.Callback) (relativeZipEntryPath, result) -> {
			RustUnit pkg = new RustUnit(RustJar.this, jar.getZipPath().resolve(relativeZipEntryPath), result);
			RustJar.this.units.add(pkg);
		});
		//TODO: figure out how to re-assign each unit to its correct RustPackage
	}

	//TODO: The transpiler will directly parse source Jars and Java files into CompilationUnit and go through DeclVisitor.
	// Then the analysis can be made on the workspace as a whole.
	// Then the Rust hierarchy will be written to files.
}
