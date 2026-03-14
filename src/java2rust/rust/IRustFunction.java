package java2rust.rust;

import com.github.javaparser.resolution.types.ResolvedType;
import java2rust.JavaTranspiler;

import java.util.Set;

public interface IRustFunction {
	RustParams params();
	RustCalls calls();
	Set<ResolvedType> thrown();

	void analyze(JavaTranspiler transpiler);
}
