package java2rust.rust;

import com.github.javaparser.JavaParser;
import com.github.javaparser.ParseResult;
import com.github.javaparser.StaticJavaParser;
import com.github.javaparser.ast.CompilationUnit;
import com.github.javaparser.utils.SourceZip;
import java2rust.DeclVisitor;
import org.apache.commons.io.FilenameUtils;
import org.jspecify.annotations.Nullable;

import java.io.IOException;
import java.nio.file.Files;
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
	public void add(Path path, RustPackage pkg) throws IOException {
		units.add(new RustUnit(this, pkg, path));
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
			RustPackage pkg = lib;
			for (Path trunk : relativeZipEntryPath)
				pkg = pkg.submodule(FilenameUtils.removeExtension(trunk.toFile().getName()), RustVisibility.PUB);
			RustUnit unit = new RustUnit(RustJar.this, jar.getZipPath().resolve(relativeZipEntryPath), pkg, result);
			RustJar.this.units.add(unit);
		});
		//TODO: figure out how to re-assign each unit to its correct RustPackage
	}

	//TODO: The transpiler will directly parse source Jars and Java files into CompilationUnit and go through DeclVisitor.
	// Then the analysis can be made on the workspace as a whole.
	// Then the Rust hierarchy will be written to files.

	public void generate(Path path) throws IOException {
		Path crate = path.resolve(name);
		Path src = crate.resolve("src");
		Files.createDirectories(src);
		if (lib != null)
			lib.generate(src);
		if (main != null)
			main.generate(src);

		String cargo = """
		[package]
		name = "%s"
		version = "0.1.0"
		edition = "2024"
		
		[dependencies]
		""".formatted(name);
		//TODO: version, metadata, dependencies, etc.
		Files.writeString(crate.resolve("Cargo.toml"), cargo);
	}
}
