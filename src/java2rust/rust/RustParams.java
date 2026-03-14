package java2rust.rust;

import com.github.javaparser.ast.body.Parameter;
import java2rust.JavaTranspiler;
import org.jspecify.annotations.Nullable;

import java.util.List;
import java.util.Objects;
import java.util.StringJoiner;

public class RustParams {
	private final List<RustParam> params;
	public @Nullable RustSelf self;
	private String cache;

	public RustParams(@Nullable RustSelf self, List<Parameter> java) {
		this.self = self;
		params = java.stream().map(RustParam::new).toList();
	}

	public void analyze(JavaTranspiler transpiler) {
		for (RustParam param : params)
			param.analyze(transpiler);
		cache = toRust();
	}

	private String toRust() {
		StringJoiner params = new StringJoiner(", ", "(", ")");
		if (self != null)
			params.add(self.template());
		for (RustParam param : this.params)
			params.add(param.toString());
		return params.toString();
	}

	public void mutateSelf() {
		if (self == null) return;
		self = self.mut().get();
	}

	public RustParam java(String name) {
		return params
			.stream()
			.filter(p -> Objects.equals(p.java.getNameAsString(), name))
			.findFirst()
			.orElse(null);
	}

	@Override
	public String toString() {
		if (cache == null)
			cache = toRust();
		return cache;
	}
}
