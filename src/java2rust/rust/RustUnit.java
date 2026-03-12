package java2rust.rust;

import com.github.javaparser.JavaParser;
import com.github.javaparser.ParseResult;
import com.github.javaparser.StaticJavaParser;
import com.github.javaparser.ast.CompilationUnit;

import java.io.IOException;
import java.nio.file.Path;

public final class RustUnit {
	public final RustJar jar;
	public final Path path;
	public final ParseResult<CompilationUnit> java;

	public RustUnit(RustJar jar, Path path, ParseResult<CompilationUnit> java) {
		this.jar = jar;
		this.path = path;
		this.java = java;
	}

	public RustUnit(RustJar jar, Path path) throws IOException {
		JavaParser parser = new JavaParser(StaticJavaParser.getParserConfiguration());
		this(jar, path, parser.parse(path));
	}
}
