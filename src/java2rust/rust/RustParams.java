package java2rust.rust;

import com.github.javaparser.ast.body.Parameter;
import java2rust.JavaTranspiler;

import java.util.List;
import java.util.StringJoiner;

public class RustParams {
	private final List<RustParam> params;
	private String cache;

	public RustParams(RustMethod method, List<Parameter> java) {
		params = java.stream().map(p -> new RustParam(method, p)).toList();
	}

	public void analyze(JavaTranspiler transpiler) {
		for (RustParam param : params)
			param.analyze(transpiler);
		cache = toRust();
	}

	private String toRust() {
		StringJoiner params = new StringJoiner(", ", "(", ")");
		for (RustParam param : this.params)
			params.add(param.toString());
		return params.toString();
	}

	@Override
	public String toString() {
		if (cache == null)
			cache = toRust();
		return cache;
	}
}

