package java2rust.rust;

import com.github.javaparser.ast.ImportDeclaration;
import com.github.javaparser.ast.expr.Name;
import java2rust.JavaTranspiler;
import org.apache.commons.lang3.StringUtils;
import org.jspecify.annotations.Nullable;

import java.util.Arrays;
import java.util.Optional;
import java.util.stream.Stream;

public final class RustImport {
	public final ImportDeclaration java;

	public final RustPackage module;
	public final String path;
	/// The crate containing the imported items.
	private RustJar crate;

	public RustImport(ImportDeclaration java, RustPackage mod) {
		this.java = java;
		this.module = mod;
		path = java.getNameAsString().replace(".", "::");
	}

	public void analyze(JavaTranspiler transpiler) {
		RustPackage pkg = transpiler.locate(java.getName());
		if (pkg != null)
			crate = pkg.crate;
	}

	public String toString(RustJar crate) {
		String root;
		if (this.crate == null)
			//Note: We're assuming this was obtained via reflection
			return "use %s;".formatted(path);
		if (crate == this.crate)
			root = "crate";
		else
			root = this.crate.name;
		return "use %s::%s;".formatted(root, path);
	}

	@Override
	public String toString() {
		return "use %s;".formatted(path);
	}
}
