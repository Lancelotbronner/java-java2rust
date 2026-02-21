package java2rust.rust;

import com.github.javaparser.resolution.declarations.ResolvedTypeParameterDeclaration;
import com.github.javaparser.resolution.declarations.ResolvedTypeParametrizable;
import com.github.javaparser.utils.Pair;
import java2rust.JavaTranspiler;

import java.util.ArrayList;
import java.util.List;
import java.util.StringJoiner;

public class RustTyParams {
	private final ArrayList<Pair<String, String>> params = new ArrayList<>();
	private String implCache;
	private String boundsCache;

	public RustTyParams() {}

	public void analyze(ResolvedTypeParametrizable decl, JavaTranspiler transpiler) {
		analyze(decl.getTypeParameters(), transpiler);
	}

	private void analyze(
		List<ResolvedTypeParameterDeclaration> typarams,
		JavaTranspiler transpiler
	) {
		for (ResolvedTypeParameterDeclaration param : typarams) {
			try {
				StringJoiner bounds = new StringJoiner("+");
				for (var bound : param.getBounds())
					bounds.add(transpiler.describe(bound.getType()));
				params.add(new Pair<>(param.getName(), bounds.toString()));
			} catch (Throwable e) {
				params.add(new Pair<>(param.getName(), "/* %s */".formatted(e.getMessage())));
			}
		}
		implCache = toRustImpl();
		boundsCache = toRustBounds();
	}

	/// `struct A<B: C, D>` becomes `<B, D>`
	private String toRustImpl() {
		StringJoiner params = new StringJoiner(", ", "<", ">");
		params.setEmptyValue("");
		for (Pair<String, String> param : this.params) {
			params.add(param.a);
		}
		return params.toString();
	}

	/// `struct A<B: C, D>` becomes `<B: C, D>`
	private String toRustBounds() {
		StringJoiner params = new StringJoiner(", ", "<", ">");
		params.setEmptyValue("");
		for (Pair<String, String> param : this.params) {
			StringBuilder sb = new StringBuilder();
			sb.append(param.a);
			if (param.b.isEmpty())
				continue;
			sb.append(": ");
			sb.append(param.b);
			params.add(sb);
		}
		return params.toString();
	}

	public String toImpl() {
		if (implCache == null)
			implCache = toRustImpl();
		return implCache;
	}

	@Override
	public String toString() {
		return toBounds();
	}

	public String toBounds() {
		if (boundsCache == null)
			boundsCache = toRustBounds();
		return boundsCache;
	}
}
