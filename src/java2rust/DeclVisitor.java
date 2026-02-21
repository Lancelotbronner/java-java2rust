package java2rust;

import com.github.javaparser.ast.ImportDeclaration;
import com.github.javaparser.ast.body.*;
import com.github.javaparser.ast.stmt.BlockStmt;
import com.github.javaparser.ast.visitor.VoidVisitorAdapter;
import java2rust.rust.*;

import java.util.Stack;

public class DeclVisitor extends VoidVisitorAdapter<Object> {
	public final JavaTranspiler transpiler;
	public final JavaTranspiler.Task task;
	private final Stack<RustModule> modules = new Stack<>();
	private final Stack<RustItem> items = new Stack<>();
	private final Stack<FieldDeclaration> fields = new Stack<>();
	private final Stack<MethodDeclaration> methods = new Stack<>();

	public DeclVisitor(JavaTranspiler transpiler, JavaTranspiler.Task task) {
		this.transpiler = transpiler;
		this.task = task;
		modules.push(task.module);
	}

	@Override
	public void visit(BlockStmt n, Object arg) {
		// Voluntarely ignore to ensure VariableDeclarator is only the struct fields.
	}

	@Override
	public void visit(ClassOrInterfaceDeclaration n, Object arg) {
		RustItem item;
		if (n.isInterface()) {
			item = modules
				.peek()
				.trait(
					n.getNameAsString(),
					n.resolve().asInterface(),
					RustVisibility.pub(n.isPublic()));
		} else {
			item = modules
				.peek()
				.clazz(
					n.getNameAsString(),
					n.resolve().asClass(),
					RustVisibility.pub(n.isPublic()));
		}
		transpiler.register(item);

		items.push(item);
		super.visit(n, arg);
		items.pop();
	}

	@Override
	public void visit(EnumDeclaration n, Object arg) {
		RustItem item = modules.peek().enumeration(n);
		transpiler.register(item);

		items.push(item);
		super.visit(n, arg);
		items.pop();
	}

	@Override
	public void visit(FieldDeclaration n, Object arg) {
		fields.push(n);
		super.visit(n, arg);
		fields.pop();
	}

	@Override
	public void visit(MethodDeclaration n, Object arg) {
		try {
			RustMethod method = items.peek().method(n);
			transpiler.registerName(method.id, method.name);
		} catch (Throwable e) {
			System.err.println(e);
			//TODO: push error method.
		}

		methods.push(n);
		super.visit(n, arg);
		methods.pop();
	}

	@Override
	public void visit(VariableDeclarator n, Object arg) {
		if (items.isEmpty()) {
			assert false;
			return;
		}
		if (fields.isEmpty()) {
			assert false;
			return;
		}
		if (fields.peek().isStatic()) {
			//TODO: add as member
			return;
		}
		RustField field = items
			.peek()
			.field(n.getNameAsString(), n.getType(), n.getInitializer().orElse(null));
		String id = "%s.%s".formatted(items.peek().id(), n.getName());
		transpiler.registerName(id, field.name);
	}

	@Override
	public void visit(ImportDeclaration n, Object arg) {
		// We'll be using this for name resolution.
		modules.peek().use(n);
	}

	@Override
	public void visit(RecordDeclaration n, Object arg) {
		items.push(modules
			.peek()
			.record(n.getNameAsString(), n.resolve().asRecord(), RustVisibility.pub(n.isPublic())));
		super.visit(n, arg);
		items.pop();
	}
}
