package java2rust.rust;

import com.github.javaparser.JavaParser;
import com.github.javaparser.ParseResult;
import com.github.javaparser.Problem;
import com.github.javaparser.StaticJavaParser;
import com.github.javaparser.ast.CompilationUnit;
import java2rust.DeclVisitor;
import java2rust.JavaTranspiler;

import java.io.IOException;
import java.nio.file.Path;
import java.util.StringJoiner;

public final class RustUnit {
	public final RustJar jar;
	public final Path path;
	public final RustPackage pkg;
	public final ParseResult<CompilationUnit> java;

	public RustUnit(RustJar jar, RustPackage pkg, Path path) throws IOException {
		JavaParser parser = new JavaParser(StaticJavaParser.getParserConfiguration());
		this(jar, path, pkg, parser.parse(path));
	}

	public RustUnit(RustJar jar, Path path, RustPackage pkg, ParseResult<CompilationUnit> java) {
		this.jar = jar;
		this.path = path;
		this.pkg = pkg;
		this.java = java;
	}

	public void preanalyze(JavaTranspiler transpiler) {
		if (java.getResult().isEmpty())
			return;
		java.getResult().get().accept(new DeclVisitor(transpiler, pkg), null);
	}

	@Override
	public String toString() {
		StringJoiner problems = new StringJoiner("\n");
		for (Problem problem : java.getProblems())
			problems.add("// FIXME: %s".formatted(problem.getVerboseMessage()));
		String problemsString = problems.toString();

		StringBuilder sb = new StringBuilder(pkg.toString());
		if (!sb.isEmpty() && !problemsString.isEmpty())
			sb.append("\n\n");
		sb.append(problemsString);

		return sb.toString();
	}
}
