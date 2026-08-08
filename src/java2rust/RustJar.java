package java2rust;

import com.github.javaparser.JavaParser;
import com.github.javaparser.StaticJavaParser;
import com.github.javaparser.utils.SourceZip;
import org.apache.commons.io.FilenameUtils;
import org.jspecify.annotations.NonNull;

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
	public final @NonNull RustPackage lib;
	/// The executable module, if applicable.
	public final @NonNull RustPackage main;

	public RustJar(String id, String name, Path path) {
		this.id = id;
		this.name = Java2Rust.camelCaseToSnakeCase(name).replace("-", "_");
		this.path = path;
		this.lib = new RustPackage(this.name, null, RustVisibility.INFERRED, this);
		this.main = new RustPackage(this.name, null, RustVisibility.INFERRED, this);
	}

	/// Parses a source jar, creating units for each source file within.
	public static RustJar sources(String id, String name, SourceZip jar) throws IOException {
		RustJar crate = new RustJar(id, name, jar.getZipPath());
		jar.setParserConfiguration(StaticJavaParser.getParserConfiguration());
		jar.parse((SourceZip.Callback) (relativeZipEntryPath, result) -> {
			RustPackage pkg = crate.lib;
			for (Path trunk : relativeZipEntryPath)
				pkg = pkg.submodule(
					FilenameUtils.removeExtension(trunk.toFile().getName()),
					RustVisibility.PUB);
			RustUnit unit = new RustUnit(
				crate,
				jar.getZipPath().resolve(relativeZipEntryPath),
				pkg,
				result);
			crate.units.add(unit);
		});
		return crate;
	}

	/// Adds a new unit to this jar.
	public void add(Path path, RustPackage pkg) throws IOException {
		units.add(new RustUnit(this, pkg, path));
	}

	public void addSourceCode(Path path, RustPackage pkg, String code) {
		JavaParser parser = new JavaParser(StaticJavaParser.getParserConfiguration());
		units.add(new RustUnit(this, path, pkg, parser.parse(code)));
	}

	public void addScript(Path path, RustPackage pkg, String code) {
		JavaParser parser = new JavaParser(StaticJavaParser.getParserConfiguration());
		units.add(new RustUnit(this, path, pkg, parser.parse(code)));
	}

	//TODO: The transpiler will directly parse source Jars and Java files into CompilationUnit and go through DeclVisitor.
	// Then the analysis can be made on the workspace as a whole.
	// Then the Rust hierarchy will be written to files.

	public void preanalyze(JavaTranspiler transpiler) {
		for (RustUnit unit : units)
			unit.preanalyze(transpiler);
	}

	public void analyze(JavaTranspiler transpiler) {
		lib.analyze(transpiler);
		main.analyze(transpiler);
	}

	public void generate(Path path) throws IOException {
		Path crate = path.resolve(name);
		Path src = crate.resolve("src");
		Files.createDirectories(src);
		if (!lib.isEmpty())
			lib.generate(src);
		if (!main.isEmpty())
			main.generate(src);

		//TODO: version, metadata, dependencies, etc.
		Files.writeString(crate.resolve("Cargo.toml"), cargo());
	}

	public String cargo() {
		return """
			[package]
			name = "%s"
			version = "0.1.0"
			edition = "2024"
			
			[dependencies]
			""".formatted(name);
	}
}
