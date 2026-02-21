package java2rust;

import com.github.javaparser.ast.CompilationUnit;
import com.github.javaparser.ast.ImportDeclaration;
import com.github.javaparser.ast.Node;
import com.github.javaparser.ast.body.*;
import com.github.javaparser.ast.comments.Comment;
import com.github.javaparser.ast.expr.*;
import com.github.javaparser.ast.stmt.*;
import com.github.javaparser.ast.type.ClassOrInterfaceType;
import com.github.javaparser.ast.type.PrimitiveType;
import com.github.javaparser.ast.type.ReferenceType;
import com.github.javaparser.ast.type.Type;
import com.github.javaparser.ast.visitor.VoidVisitorAdapter;
import com.github.javaparser.utils.Pair;

import java.lang.reflect.Method;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

/**
 * Created by aschoerk on 03.05.16.
 */
public class IdTrackerVisitor extends VoidVisitorAdapter<IdTracker> {

	private boolean inAssignTarget = false;

	@Override
	public void visit(AssignExpr n, IdTracker arg) {

		visitComment(n.getComment().orElse(null), arg);
		inAssignTarget = true;
		try {
			n.getTarget().accept(this, arg);
		} finally {
			inAssignTarget = false;
		}
		n.getValue().accept(this, arg);

	}

	@Override
	public void visit(BlockStmt n, IdTracker arg) {
		arg.pushBlock(n);
		super.visit(n, arg);
		arg.popBlock();
	}

	@Override
	public void visit(CatchClause n, IdTracker arg) {
		arg.pushBlock(n);
		try {
			super.visit(n, arg);
		} finally {
			arg.popBlock();
		}
	}

	@Override
	public void visit(ClassOrInterfaceDeclaration n, IdTracker arg) {
		arg.pushBlock(n);
		arg.addDeclaration(n.getName().asString(), new Pair<>(null, n));
		super.visit(n, arg);
		arg.popBlock();
	}

	@Override
	public void visit(ClassOrInterfaceType n, IdTracker arg) {
		super.visit(n, arg);
	}

	@Override
	public void visit(final CompilationUnit n, final IdTracker arg) {
		visitComment(n.getComment().orElse(null), arg);
		if (n.getPackageDeclaration().isPresent()) {
			arg.setPackageName(n.getPackageDeclaration().get().getName().asString());
			n.getPackageDeclaration().get().accept(this, arg);
		}
		if (n.getImports() != null) {
			for (final ImportDeclaration i : n.getImports()) {
				arg.addImport(new Import(i.getName().toString(), i.isStatic(), i.isAsterisk()));
				i.accept(this, arg);
			}
		}
		if (n.getTypes() != null) {
			for (final TypeDeclaration<?> typeDeclaration : n.getTypes()) {
				typeDeclaration.accept(this, arg);
			}
		}
	}

	@Override
	public void visit(final ConstructorDeclaration n, final IdTracker arg) {
		arg.pushBlock(n);
		super.visit(n, arg);
		arg.popBlock();
	}

	@Override
	public void visit(EnumDeclaration n, IdTracker arg) {
		arg.pushBlock(n);
		arg.addDeclaration(n.getName().asString(), new Pair<>(null, n));
		super.visit(n, arg);
		arg.popBlock();
	}

	@Override
	public void visit(final ExpressionStmt n, final IdTracker arg) {
		if (n.getExpression() instanceof VariableDeclarationExpr ve) {
			ve.getCommonType();
		}
		super.visit(n, arg);
	}

	@Override
	public void visit(ForEachStmt n, IdTracker arg) {
		arg.pushBlock(n);
		super.visit(n, arg);
		arg.popBlock();
	}

	@Override
	public void visit(ForStmt n, IdTracker arg) {
		arg.pushBlock(n);
		super.visit(n, arg);
		arg.popBlock();
	}

	@Override
	public void visit(MethodCallExpr n, IdTracker arg) {
		if (n.getScope().isPresent() && n.getScope().get() instanceof NameExpr ne) {
			Class clazz = identifyaClass(arg, ne.getName().asString());
			if (clazz != null) {
				String methodName = n.getName().asString();
				Method[] ms = clazz.getMethods();
				Set<Method> candidates = new HashSet<>();
				for (Method m : ms) {
					if (m.getName().equals(methodName)) {
						candidates.add(m);
					}
				}
				Method resulting = null;
				if (candidates.size() == 1) {
					resulting = candidates.iterator().next();
				} else {
					List<Method> matching = candidates
						.stream()
						.filter(m -> m.getParameterCount() == n
							.getArguments()
							.size() || m.isVarArgs() && m.getParameterCount() <= n
							.getArguments()
							.size())
						.toList();
					if (matching.size() == 1) {
						resulting = matching.getFirst();
					}
				}
				if (resulting != null) {
					for (int i = 0; i < resulting.getParameterCount(); i++) {

						java.lang.reflect.Parameter p = resulting.getParameters()[i];
						if (n.getArguments().size() > i) {
							arg.putType(n.getArguments().get(i), p.getType());
						}
					}
				}
			}

		}
		arg.addUsage(n.getName().asString(), n);
		super.visit(n, arg);
	}

	@Override
	public void visit(MethodDeclaration n, IdTracker arg) {
		try {
			arg.addDeclaration(n.getName().asString(), new Pair<>(null, n));
		} catch (RuntimeException ex) {
			// ignore duplicate Methods with the same name. Let it be declared just once, so that self can be constructed.
		}
		if (n.getThrownExceptions() != null && !n.getThrownExceptions().isEmpty()) {
			arg.setHasThrows(n.getName().asString());
		}
		arg.pushBlock(n);
		super.visit(n, arg);
		arg.popBlock();
	}

	@Override
	public void visit(NameExpr n, IdTracker arg) {
		if (inAssignTarget) {
			arg.addChange(n.getName().asString(), n);
		} else {
			arg.addUsage(n.getName().asString(), n);
		}
		super.visit(n, arg);
	}

	@Override
	public void visit(Name n, IdTracker arg) {
		if (inAssignTarget) {
			arg.addChange(n.getQualifier().toString(), n);
		}
		super.visit(n, arg);
	}

	@Override
	public void visit(UnaryExpr n, IdTracker arg) {
		try {
			switch (n.getOperator()) {
			case POSTFIX_INCREMENT:
			case POSTFIX_DECREMENT:
			case PREFIX_INCREMENT:
			case PREFIX_DECREMENT:
				inAssignTarget = true;
				break;
			default:
				inAssignTarget = false;
			}
			n.getExpression().accept(this, arg);
		} finally {
			inAssignTarget = false;
		}
	}

	@Override
	public void visit(final VariableDeclarationExpr n, final IdTracker arg) {
		TypeDescription typeDescr = getTypeDescription(arg, n.getCommonType());
		String type = getNameOfType(n.getCommonType());

		if (typeDescr != null && arg.isFloat(typeDescr.getClazz())) {
			arg.putType(n, typeDescr.getClazz());
			if (typeDescr.getArrayCount() > 0) {
				try {
					Node initializer = n.getChildNodes().get(1).getChildNodes().get(1);
					if (!(initializer instanceof MethodCallExpr)) {
						List<Node> nodes = initializer.getChildNodes();
						for (Node child : nodes) {
							arg.putType(child, Double.TYPE);
						}
					}
				} catch (RuntimeException _) {

				}
			}
		}
		super.visit(n, arg);
	}

	@Override
	public void visit(VariableDeclarator n, IdTracker arg) {
		boolean isField = n.getParentNode().get().getParentNode().get() instanceof FieldDeclaration;
		TypeDescription clazz = typeOf(n, arg);
		arg.addDeclaration(n.getName().asString(), new Pair<>(clazz, n));
		super.visit(n, arg);
	}

	private Class identifyaClass(final IdTracker arg, final String name) {
		Class clazz = null;
		if (name != null) {
			for (Import i : arg.getImports()) {
				if (!i.isStaticImport()) {
					if (i.isWildcardImport()) {
						clazz = forName(i.getImportString() + "." + name);
					} else {
						if (i.getImportString().endsWith("." + name)) {
							final String importString = i.getImportString();
							clazz = forName(importString);
						}
					}
				}
			}
			if (clazz == null) {
				clazz = forName("java.lang." + name);
			}
			if (clazz == null) {
				clazz = forName(arg.getPackageName() + "." + name);
			}
		}
		return clazz;
	}

	private Class forName(final String importString) {
		Class clazz = null;
		try {
			clazz = Class.forName(importString);
		} catch (ClassNotFoundException | NoClassDefFoundError _) {

		}
		return clazz;
	}

	private void visitComment(final Comment n, final IdTracker arg) {
		if (n != null) {
			n.accept(this, arg);
		}
	}

	String getNameOfType(Type t) {
		if (t instanceof ClassOrInterfaceType) {
			return ((ClassOrInterfaceType) t).getName().asString();
		} else if (t instanceof ReferenceType rtype) {
			return getNameOfType(rtype.getElementType());
		}
		return null;
	}

	TypeDescription typeOf(VariableDeclarator n, IdTracker arg) {
		Type t = null;
		if (n
			.getParentNode()
			.get()
			.getParentNode()
			.get() instanceof FieldDeclaration fieldDeclaration) {
			t = fieldDeclaration.getCommonType();
		} else if (n.getParentNode().get() instanceof Parameter p) {
			t = p.getType();
		} else if (n
			.getParentNode()
			.get()
			.getParentNode()
			.get() instanceof VariableDeclarationExpr variableDeclarationExpr) {
			t = variableDeclarationExpr.getCommonType();
		}
		if (t != null) {
			return getTypeDescription(arg, t);
		}
		return null;
	}

	private TypeDescription getTypeDescription(final IdTracker arg, final Type t) {
		String name = getNameOfType(t);
		Class clazz = identifyaClass(arg, name);
		if (t instanceof ReferenceType rtype) {
			if (clazz == null) {
				clazz = getPotentialPrimitiveType(rtype.getElementType());
			}
			if (clazz != null)
				return new TypeDescription(rtype.getArrayLevel(), clazz);
		}
		if (clazz == null) {
			clazz = getPotentialPrimitiveType(t);
		}
		if (clazz == null)
			return null;
		else {
			return new TypeDescription(0, clazz);
		}
	}

	private Class getPotentialPrimitiveType(final Type t) {
		if (t instanceof PrimitiveType pt) {
			switch (pt.getType().name()) {
			case "Byte":
				return Byte.TYPE;
			case "Short":
				return Short.TYPE;
			case "Int":
				return Integer.TYPE;
			case "Long":
				return Long.TYPE;
			case "Float":
				return Float.TYPE;
			case "Double":
				return Double.TYPE;
			case "Char":
				return Character.TYPE;
			case "Boolean":
				return Boolean.TYPE;
			case "void":
				return Void.TYPE;
			}
		}
		return null;
	}
}
