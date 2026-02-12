package java2rust.rust;

import com.github.javaparser.resolution.declarations.ResolvedClassDeclaration;
import com.github.javaparser.resolution.declarations.ResolvedEnumDeclaration;
import com.github.javaparser.resolution.declarations.ResolvedRecordDeclaration;
import com.github.javaparser.resolution.types.ResolvedReferenceType;
import java2rust.JavaTranspiler;
import org.jspecify.annotations.NonNull;

import java.util.ArrayList;
import java.util.List;
import java.util.StringJoiner;

public class RustImpls {
	private final RustItem item;
	private final RustTyParams params;
	private final List<String> traits = new ArrayList<>();
	private final List<String> problems = new ArrayList<>();

	public RustImpls(RustItem item, RustTyParams params) {
		this.item = item;
		this.params = params;
	}

	public boolean isEmpty() {
		return traits.isEmpty() && problems.isEmpty();
	}

	public void analyze(ResolvedClassDeclaration decl, JavaTranspiler transpiler) {
		try {
			analyze(decl.getAllInterfaces(), transpiler);
		} catch (Throwable e) {
			problems.add("%s".formatted(e.getMessage()));
		}
	}

	private void analyze(
		@NonNull List<ResolvedReferenceType> implemented,
		JavaTranspiler transpiler
	) {
		for (ResolvedReferenceType i : implemented) {
			try {
				traits.add(transpiler.describe(i));
			} catch (Throwable e) {
				problems.add("%s".formatted(e.getMessage()));
			}
		}
	}

	public void analyze(ResolvedRecordDeclaration decl, JavaTranspiler transpiler) {
		try {
			analyze(decl.getAllInterfaces(), transpiler);
		} catch (Throwable e) {
			problems.add("%s".formatted(e.getMessage()));
		}
	}

	@Override
	public String toString() {
		StringBuilder sb = new StringBuilder();

		StringJoiner traits = new StringJoiner("\n\n");
		for (String trait : this.traits)
			traits.add("impl%s %s for %s%s {}".formatted(
				params.toBounds(),
				trait,
				item.name,
				params.toImpl()));
		sb.append(traits);

		StringJoiner problems = new StringJoiner("\n", "/* ", " */");
		problems.setEmptyValue("");
		for (String problem : this.problems)
			problems.add(problem);
		sb.append(problems);

		return sb.toString();
	}
}
