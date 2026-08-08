package java2rust;

import com.github.javaparser.resolution.types.ResolvedType;

import java.util.Set;

public interface IRustFunction {
	RustItem item();
	RustParams params();
	RustCalls calls();
	Set<ResolvedType> thrown();

	void analyze(JavaTranspiler transpiler, RustItem item);
}
