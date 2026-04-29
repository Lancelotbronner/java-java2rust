package java2rust;

import com.github.javaparser.ast.ImportDeclaration;
import com.github.javaparser.ast.Node;
import com.github.javaparser.ast.body.*;
import com.github.javaparser.ast.expr.*;
import com.github.javaparser.ast.stmt.ThrowStmt;
import com.github.javaparser.ast.visitor.VoidVisitorAdapter;
import com.github.javaparser.resolution.declarations.ResolvedValueDeclaration;
import com.github.javaparser.resolution.types.ResolvedType;
import java2rust.rust.*;

import java.util.Objects;
import java.util.Optional;
import java.util.Stack;

public class DeclVisitor extends VoidVisitorAdapter<Object> {
	public final JavaTranspiler transpiler;
	private final Stack<RustPackage> modules = new Stack<>();
	private final Stack<RustItem> items = new Stack<>();
	private final Stack<FieldDeclaration> fields = new Stack<>();
	private final Stack<IRustFunction> functions = new Stack<>();
	private boolean isMutableScope;

	public DeclVisitor(JavaTranspiler transpiler, RustPackage module) {
		this.transpiler = transpiler;
		modules.push(module);
	}

	@Override
	public void visit(AssignExpr n, Object arg) {
		isMutableScope = true;
		n.getTarget().accept(this, arg);
		isMutableScope = false;
		n.getValue().accept(this, arg);
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
	public void visit(ConstructorDeclaration n, Object arg) {
		try {
			RustConstructor method = items.peek().constructor(n);
			transpiler.registerName(method.id, method.name);
			functions.push(method);
			super.visit(n, arg);
			functions.pop();
		} catch (Throwable e) {
			System.err.printf("In ConstructorDeclaration: %s\n", e.getLocalizedMessage());
			//TODO: push error method.
		}
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
	public void visit(InitializerDeclaration n, Object arg) {
		try {
			RustInitializer method = items.peek().initializer(n);
			functions.push(method);
			super.visit(n, arg);
			functions.pop();
		} catch (Throwable e) {
			System.err.printf("In InitializerDeclaration: %s\n", e.getLocalizedMessage());
			//TODO: push error method.
		}
	}

	@Override
	public void visit(MethodCallExpr n, Object arg) {
		if (functions.isEmpty()) {
			Optional<Node> n1 = n.getParentNode();
			while (n1.isPresent() && !(n1.get() instanceof BodyDeclaration<?>))
				n1 = n1.get().getParentNode();
			System.err.printf(
				"Unhandled method call context: %s\n",
				n1.map(n2 -> n2.getMetaModel().getTypeName()).orElse("UNKNOWN"));
			return;
		}
		IRustFunction function = functions.peek();
		if (function == null)
			return; //TODO: handle lambdas
		function.calls().addCallee(n);
		super.visit(n, arg);
	}

	@Override
	public void visit(MethodDeclaration n, Object arg) {
		try {
			RustMethod method = items.peek().method(n);
			transpiler.register(method);
			transpiler.registerName(method.id, method.name);
			functions.push(method);
			super.visit(n, arg);
			functions.pop();
		} catch (Throwable e) {
			System.err.printf("In MethodDeclaration: %s\n", e.getLocalizedMessage());
			//TODO: push error method.
		}
	}

	@Override
	public void visit(NameExpr n, Object arg) {
		if (!isMutableScope)
			return;
		//TODO: global name metadata? method signature + name
		ResolvedValueDeclaration resolved = n.resolve();
		if (resolved.isParameter()) {
			RustParam param = functions.peek().params().java(n.getNameAsString());
			if (param != null)
				param.isMutable = true;
		}
		if (resolved.isField())
			if (Objects.equals(items.peek().id(), resolved.asField().declaringType().getId()))
				functions.peek().params().mutateSelf();
		super.visit(n, arg);
	}

	@Override
	public void visit(ThisExpr n, Object arg) {
		if (!isMutableScope)
			return;
		//TODO: handle type name
		functions.peek().params().mutateSelf();
		super.visit(n, arg);
	}

	@Override
	public void visit(ThrowStmt n, Object arg) {
		super.visit(n, arg);
		IRustFunction method = functions.peek();
		try {
			ResolvedType ty = n.getExpression().calculateResolvedType();
			method.thrown().add(ty);
		} catch (Throwable e) {
			System.err.printf("In ThrowStmt: %s\n", e.getLocalizedMessage());
		}
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
			//TODO: add as impl member
			return;
		}
		RustField field = items
			.peek()
			.field(n.getNameAsString(), n.getType(), n.getInitializer().orElse(null));
		String id = "%s.%s".formatted(items.peek().id(), n.getName());
		transpiler.registerName(id, field.name);
	}

	@Override
	public void visit(LambdaExpr n, Object arg) {
		functions.push(null);
		super.visit(n, arg);
		functions.pop();
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
