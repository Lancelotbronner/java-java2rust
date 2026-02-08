package java2rust;

import com.github.javaparser.ast.body.*;
import com.github.javaparser.ast.stmt.BlockStmt;
import com.github.javaparser.ast.visitor.VoidVisitorAdapter;
import java2rust.rust.RustModule;
import java2rust.rust.RustStruct;

import java.util.Stack;

public class DeclVisitor extends VoidVisitorAdapter<Object> {
	private final Stack<RustModule> modules = new Stack<>();
	private final Stack<RustStruct> structs = new Stack<>();
	private final Stack<FieldDeclaration> fields = new Stack<>();

	public DeclVisitor(RustModule mod) {
		modules.push(mod);
	}

	@Override
	public void visit(BlockStmt n, Object arg) {
		// Voluntarely ignore to ensure VariableDeclarator is only the struct fields.
	}

	@Override
	public void visit(ClassOrInterfaceDeclaration n, Object arg) {
		if (n.isInterface()) {
			//TODO: Declare trait
		} else
			structs.push(modules
				.peek()
				.struct(n.getNameAsString(), n.isPublic()));

		super.visit(n, arg);

		if (n.isInterface()) {

		} else
			structs.pop();
	}

	@Override
	public void visit(FieldDeclaration n, Object arg) {
		fields.push(n);
		super.visit(n, arg);
		fields.pop();
	}

	@Override
	public void visit(MethodDeclaration n, Object arg) {
		super.visit(n, arg);
	}

	@Override
	public void visit(VariableDeclarator n, Object arg) {
		if (structs.isEmpty()) {
			assert false;
			return;
		}
		if (fields.isEmpty()) {
			assert false;
			return;
		}
		if (fields
			.peek()
			.isStatic()) {
			//TODO: add as member
			return;
		}
		structs
			.peek()
			.field(
				n.getNameAsString(),
				n
					.getType(),
				n
					.getInitializer()
					.orElse(null));
	}

	@Override
	public void visit(RecordDeclaration n, Object arg) {
		structs.push(modules
			.peek()
			.struct(n.getNameAsString(), n.isPublic()));
		super.visit(n, arg);
		structs.pop();
	}
}
