package java2rust;

import com.github.javaparser.resolution.types.ResolvedType;
import org.jspecify.annotations.NonNull;

import java.util.Set;

public interface IRustFunction {
	RustItem item();
	@NonNull RustParams params();
	RustCalls calls();
	Set<ResolvedType> thrown();

	void analyze(JavaTranspiler transpiler, RustItem item);
}
