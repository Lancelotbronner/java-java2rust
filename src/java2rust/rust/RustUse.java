package java2rust.rust;

import com.github.javaparser.ast.ImportDeclaration;
import java2rust.JavaTranspiler;

public final class RustUse {
	public final ImportDeclaration java;

	public final RustModule module;
	private String path;

	public RustUse(ImportDeclaration java, RustModule mod) {
		this.java = java;
		this.module = mod;
	}

	public void analyze(JavaTranspiler transpiler) {
		// TODO: resolve names, start from root qualifier and resolve using lib
		path = java.getNameAsString();
	}

	@Override
	public String toString() {
		return "use %s;".formatted(path);
	}
}
