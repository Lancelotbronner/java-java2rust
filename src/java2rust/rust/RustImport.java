package java2rust.rust;

import com.github.javaparser.ast.ImportDeclaration;
import java2rust.JavaTranspiler;

public final class RustImport {
	public final ImportDeclaration java;

	public final RustPackage module;
	private String path;

	public RustImport(ImportDeclaration java, RustPackage mod) {
		this.java = java;
		this.module = mod;
	}

	public void analyze(JavaTranspiler transpiler) {
		// TODO: resolve names, start from root qualifier and resolve using lib
		path = java.getNameAsString().replace(".", "::");
	}

	@Override
	public String toString() {
		return "use %s;".formatted(path);
	}
}
