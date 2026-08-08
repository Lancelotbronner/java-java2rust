package java2rust;

import com.github.javaparser.ast.CompilationUnit;
import com.github.javaparser.ast.Node;
import com.github.javaparser.ast.body.TypeDeclaration;
import com.github.javaparser.resolution.TypeSolver;
import com.github.javaparser.resolution.declarations.ResolvedReferenceTypeDeclaration;
import com.github.javaparser.resolution.model.SymbolReference;

import java.util.HashMap;

public final class TranspilerTypeSolver implements TypeSolver {
	private static final SymbolReference<ResolvedReferenceTypeDeclaration> UNSOLVED = SymbolReference.unsolved();
	public final JavaTranspiler transpiler;
	private final HashMap<String, SymbolReference<ResolvedReferenceTypeDeclaration>> types = new HashMap<>();
	private TypeSolver parent;

	public TranspilerTypeSolver(JavaTranspiler transpiler) {
		this.transpiler = transpiler;
	}

	@Override
	public TypeSolver getParent() {
		return parent;
	}

	@Override
	public void setParent(TypeSolver parent) {
		this.parent = parent;
	}

	@Override
	public SymbolReference<ResolvedReferenceTypeDeclaration> tryToSolveType(String name) {
		SymbolReference<ResolvedReferenceTypeDeclaration> result = types.get(name);
		if (result == null)
			return UNSOLVED;
		return result;
	}

	@Override
	public SymbolReference<ResolvedReferenceTypeDeclaration> tryToSolveTypeInModule(
		String qualifiedModuleName,
		String simpleTypeName
	) {
		String id = "%s.%s".formatted(qualifiedModuleName, simpleTypeName);
		return tryToSolveType(id);
	}

	public void reload() {
		types.clear();
		for (RustJar crate : transpiler.crates) {
			for (RustUnit unit : crate.units) {
				if (unit.java.getResult().isEmpty())
					continue;
				CompilationUnit compiled = unit.java.getResult().get();
				visit(compiled);
			}
		}
	}

	private void visit(Node node) {
		if (node instanceof TypeDeclaration<?> td) {
			ResolvedReferenceTypeDeclaration resolved = td.resolve();
			types.put(resolved.getId(), SymbolReference.solved(resolved));
		}
		node.getChildNodes().forEach(this::visit);
	}
}
