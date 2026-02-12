package java2rust;

import com.github.javaparser.ast.*;
import com.github.javaparser.ast.body.*;
import com.github.javaparser.ast.comments.*;
import com.github.javaparser.ast.expr.*;
import com.github.javaparser.ast.modules.*;
import com.github.javaparser.ast.stmt.*;
import com.github.javaparser.ast.type.*;
import com.github.javaparser.ast.visitor.VoidVisitorAdapter;
import com.github.javaparser.utils.Pair;
import org.apache.commons.lang3.StringUtils;
import org.apache.commons.lang3.Strings;

import java.util.*;
import java.util.function.Function;
import java.util.stream.Collectors;

import static com.github.javaparser.utils.PositionUtils.sortByBeginPosition;
import static com.github.javaparser.utils.Utils.isNullOrEmpty;
import static java.util.Collections.reverse;

public final class RustVisitor extends VoidVisitorAdapter<Object> {
	public final JavaTranspiler transpiler;
	private final RustPrinter printer = new RustPrinter("\t");

	public RustVisitor(
		JavaTranspiler transpiler
	) {
		this.transpiler = transpiler;
	}

	public String getSource() {
		return printer.getSource();
	}

	private void printJavadoc(final JavadocComment javadoc, final Object arg) {
		if (javadoc != null) {
			javadoc.accept(this, arg);
		}
	}

	String acceptAndCopy(Node n, final Object arg) {
		int mark = printer.push();
		n.accept(this, arg);
		String result = printer.getMark(mark);
		printer.drop();
		return result;
	}

	//	@Override
	//	public void visit(final EmptyTypeDeclaration n, final Object arg) {
	//		printJavaComment(n.getComment(), arg);
	//		printer.print(";");
	//
	//		printOrphanCommentsEnding(n);
	//	}

	@Override
	public void visit(AnnotationDeclaration n, Object arg) {
		printer.startComment();
		printer.print("AnnotationDeclaration");
		printer.println(n.toString());
		printer.endComment();
	}

	@Override
	public void visit(AnnotationMemberDeclaration n, Object arg) {
		printer.startComment();
		printer.print("AnnotationMemberDeclaration");
		printer.println(n.toString());
		printer.endComment();
	}

	@Override
	public void visit(final ArrayAccessExpr n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		n.getName()
			.accept(this, arg);
		printer.print("[");
		n.getIndex()
			.accept(this, arg);
		printer.print("]");
	}

	@Override
	public void visit(final ArrayCreationExpr n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		if (!isNullOrEmpty(n.getLevels())) {
			String type = acceptAndCut(n.getElementType(), arg);
			String typeOrDefaultValue = defaultValue(type);
			if (typeOrDefaultValue.equals("None"))
				type = "Option<" + type + ">";
			List<String> dims = n.getLevels()
				.stream()
				.map(e -> acceptAndCut(e, arg))
				.collect(Collectors.toList());

			printer.print(": ");
			printer.print(getArrayDeclaration(type, dims));

			printer.print(" = ");
			printer.print(getArrayDeclaration(typeOrDefaultValue, dims));

		} else {
			printer.print(" ");
			if (n.getInitializer()
				.isPresent())
				n.getInitializer()
					.get()
					.accept(this, n.getElementType());
		}
	}

	@Override
	public void visit(final ArrayInitializerExpr n, final Object arg) {
		Type t = arg instanceof Type ? (Type) arg : null;
		printJavaComment(
			n.getComment()
				.orElse(null), arg);

		if (!isNullOrEmpty(n.getValues())) {
			if (t != null) {
				List<Integer> dims = getDimensions(n, t);
				StringBuilder sb = new StringBuilder();
				sb.append(acceptAndCut(t, arg));
				reverse(dims);
				for (Integer i : dims) {
					sb.insert(0, "vec![");
					sb.append("; ")
						.append(i)
						.append("]");
				}
				printer.print(": ");
				printer.print(sb.toString());
				printer.print(" = ");
			}
			printer.print("vec![");

			for (Expression val : n.getValues()) {
				val.accept(this, null);
				printer.print(", ");
			}
			printer.println("]");

		}
	}

	List<Integer> getDimensions(ArrayInitializerExpr n, Type t) {
		List<Integer> dimensions = new ArrayList<>();
		Integer actsize = n.getValues()
			.size();
		while (n != null) {
			dimensions.add(actsize);
			actsize = null;
			Expression firstValue = n.getValues()
				.get(0);
			if (firstValue instanceof ArrayInitializerExpr) {
				Integer size = null;
				for (Expression e : n.getValues()) {
					ArrayInitializerExpr ai = (ArrayInitializerExpr) e;
					if (size == null) {
						size = ai.getValues()
							.size();
						n = ai;
					} else {
						if (size < ai.getValues()
							.size()) {
							size = ai.getValues()
								.size();
							n = ai;
						}
					}
				}
				actsize = size;
			} else {
				n = null;
			}
		}
		return dimensions;
	}

	@Override
	public void visit(final AssertStmt n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.print("assert!( ");
		n.getCheck()
			.accept(this, arg);
		if (n.getMessage()
			.isPresent()) {
			printer.print(" : ");
			n.getMessage()
				.get()
				.accept(this, arg);
		}
		printer.print(");");
	}

	@Override
	public void visit(final AssignExpr n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		n.getTarget()
			.accept(this, arg);
		printer.print(" ");
		switch (n.getOperator()) {
		case ASSIGN:
			printer.print("=");
			break;
		case BINARY_AND:
			printer.print("&=");
			break;
		case BINARY_OR:
			printer.print("|=");
			break;
		case XOR:
			printer.print("^=");
			break;
		case PLUS:
			printer.print("+=");
			break;
		case MINUS:
			printer.print("-=");
			break;
		case REMAINDER:
			printer.print("%=");
			break;
		case DIVIDE:
			printer.print("/=");
			break;
		case MULTIPLY:
			printer.print("*=");
			break;
		case LEFT_SHIFT:
			printer.print("<<=");
			break;
		case SIGNED_RIGHT_SHIFT:
			printer.print(">>=");
			break;
		case UNSIGNED_RIGHT_SHIFT:
			printer.print(">>= /* >>>= */");
			break;
		}
		printer.print(" ");
		n.getValue()
			.accept(this, arg);
	}

	@Override
	public void visit(final BinaryExpr n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		n.getLeft()
			.accept(this, arg);
		printer.print(" ");
		switch (n.getOperator()) {
		case OR:
			printer.print("||");
			break;
		case AND:
			printer.print("&&");
			break;
		case BINARY_OR:
			printer.print("|");
			break;
		case BINARY_AND:
			printer.print("&");
			break;
		case XOR:
			printer.print("^");
			break;
		case EQUALS:
			printer.print("==");
			break;
		case NOT_EQUALS:
			printer.print("!=");
			break;
		case LESS:
			printer.print("<");
			break;
		case GREATER:
			printer.print(">");
			break;
		case LESS_EQUALS:
			printer.print("<=");
			break;
		case GREATER_EQUALS:
			printer.print(">=");
			break;
		case LEFT_SHIFT:
			printer.print("<<");
			break;
		case SIGNED_RIGHT_SHIFT:
			printer.print(">>");
			break;
		case UNSIGNED_RIGHT_SHIFT:
			printer.print(">> /* >>> */");
			break;
		case PLUS:
			printer.print("+");
			break;
		case MINUS:
			printer.print("-");
			break;
		case MULTIPLY:
			printer.print("*");
			break;
		case DIVIDE:
			printer.print("/");
			break;
		case REMAINDER:
			printer.print("%");
			break;
		}
		printer.print(" ");
		n.getRight()
			.accept(this, arg);
	}

	@Override
	public void visit(final BlockComment n, final Object arg) {
		printer.comment(n.getContent());
		printer.endComment();
		printer.println();
	}

	@Override
	public void visit(final BlockStmt n, final Object arg) {
		printOrphanCommentsBeforeThisChildNode(n);
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.println("{");
		if (n.getStatements() != null) {
			printer.indent();
			for (final Statement s : n.getStatements()) {
				s.accept(this, arg);
				printer.println();
			}
			printer.unindent();
		}
		printOrphanCommentsEnding(n);
		printer.print("}");
	}

	@Override
	public void visit(final BooleanLiteralExpr n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.print(String.valueOf(n.getValue()));
	}

	@Override
	public void visit(final BreakStmt n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.print("break");
		if (n.getLabel()
			.isPresent()) {
			printer.print(" '");
			printer.print(n.getLabel()
				.get()
				.asString());
		}
		printer.print(";");
	}

	@Override
	public void visit(final CastExpr n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		n.getExpression()
			.accept(this, arg);
		printer.print(" as ");
		n.getType()
			.accept(this, arg);
	}

	@Override
	public void visit(final CatchClause n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.print(" catch (");
		n.getParameter()
			.accept(this, arg);
		printer.print(") ");
		n.getBody()
			.accept(this, arg);

	}

	@Override
	public void visit(final CharLiteralExpr n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.print("'");
		printer.print(n.getValue());
		printer.print("'");
	}

	@Override
	public void visit(final ClassExpr n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		n.getType()
			.accept(this, arg);
		printer.print(".class");
	}

	@Override
	public void visit(final ClassOrInterfaceDeclaration n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);

		final boolean[] staticSearched = { true };
		Function<BodyDeclaration<?>, Boolean> selectFieldDeclarationBooleanFunction = mem -> {
			if (mem instanceof FieldDeclaration fd) {
				return fd.isStatic() == staticSearched[0];
			} else {
				return false;
			}
		};
		if (!isNullOrEmpty(n.getMembers())) {
			printMembers(n.getMembers(), arg, selectFieldDeclarationBooleanFunction);
		}


		if (!isNullOrEmpty(n.getImplementedTypes())) {
			printer.print("#[derive(");
			for (final Iterator<ClassOrInterfaceType> i = n.getImplementedTypes()
				.iterator(); i.hasNext(); ) {
				final ClassOrInterfaceType c = i.next();
				c.accept(this, arg);
				if (i.hasNext()) {
					printer.print(", ");
				}
			}
			printer.println(")]");
		}

		n.getModifiers()
			.accept(this, arg);
		printer.endComment();

		if (n.isInterface()) {
			printer.print("trait ");
		} else {
			printer.print("struct ");
		}

		printer.print(n.getName()
			.asString());

		printTypeParameters(n.getTypeParameters(), arg);

		if (!isNullOrEmpty(n.getExtendedTypes())) {
			if (n.isInterface()) {
				printer.print(" : ");

				boolean first = true;
				for (ClassOrInterfaceType i : n.getExtendedTypes()) {
					if (first)
						first = false;
					else
						printer.print(" + ");
					i.accept(this, arg);
				}
				printer.println(" {");
				printer.indent();
			} else {
				printer.println(" {");
				printer.indent();
				int count = n.getExtendedTypes()
					.size() > 1 ? 0 : -1;
				for (final ClassOrInterfaceType c : n.getExtendedTypes()) {
					printer.print("base" + (count >= 0 ? ++count + "" : "") + ": ");
					c.accept(this, arg);
					printer.println(",");
				}
			}
		} else {
			printer.println(" {");
			printer.indent();
		}


		staticSearched[0] = false;
		if (!isNullOrEmpty(n.getMembers())) {
			printMembers(n.getMembers(), arg, selectFieldDeclarationBooleanFunction);
		}

		printOrphanCommentsEnding(n);

		if (!n.isInterface()) {
			printer.unindent();
			printer.println("}");
			printer.println("");

			printer.print("impl ");
			printer.print(n.getName()
				.asString());

			printer.println(" {");
			printer.indent();
		}
		if (!isNullOrEmpty(n.getMembers())) {
			printMembers(n.getMembers(), arg, mem -> !(mem instanceof FieldDeclaration));
		}
		printer.unindent();
		printer.println("}");

	}

	private void printMembers(
		final List<BodyDeclaration<?>> members,
		final Object arg,
		Function<BodyDeclaration<?>, Boolean> filter
	) {
		for (final BodyDeclaration<?> member : members) {
			if (filter == null || filter.apply(member)) {
				member.accept(this, arg);
			}
		}
	}

	private void printTypeParameters(final List<TypeParameter> args, final Object arg) {
		if (!isNullOrEmpty(args)) {
			printer.print("<");
			for (final Iterator<TypeParameter> i = args.iterator(); i.hasNext(); ) {
				final TypeParameter t = i.next();
				t.accept(this, arg);
				if (i.hasNext()) {
					printer.print(", ");
				}
			}
			printer.print(">");
		}
	}

	@Override
	public void visit(final ClassOrInterfaceType n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);

		if (n.getScope()
			.isPresent()) {
			n.getScope()
				.get()
				.accept(this, arg);
			printer.print(".");
		}
		printer.print(n.getName()
			.asString());

		if (n.isUsingDiamondOperator()) {
			printer.print("<>");
		} else {
			if (n.getTypeArguments()
				.isPresent())
				printTypeArgs(
					n.getTypeArguments()
						.get(), arg);
		}
	}

	private void printTypeArgs(final List<Type> args, final Object arg) {
		if (!isNullOrEmpty(args)) {
			printer.print("<");
			for (final Iterator<Type> i = args.iterator(); i.hasNext(); ) {
				final Type t = i.next();
				t.accept(this, arg);
				if (i.hasNext()) {
					printer.print(", ");
				}
			}
			printer.print(">");
		}
	}

	@Override
	public void visit(final CompilationUnit n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);

		if (n.getPackageDeclaration()
			.isPresent()) {
			n.getPackageDeclaration()
				.get()
				.accept(this, arg);
		}

        /*
        if (!isNullOrEmpty(n.getImports())) {
            for (final ImportDeclaration i : n.getImports()) {
                i.accept(this, arg);
            }
            printer.printLn();
        }
        */

		if (!isNullOrEmpty(n.getTypes())) {
			for (final Iterator<TypeDeclaration<?>> i = n.getTypes()
				.iterator(); i.hasNext(); ) {
				i.next()
					.accept(this, arg);
				printer.println();
				if (i.hasNext()) {
					printer.println();
				}
			}
		}

		printOrphanCommentsEnding(n);
	}

	@Override
	public void visit(final ConditionalExpr n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.print(" if ");
		n.getCondition()
			.accept(this, arg);
		printer.print(" { ");
		n.getThenExpr()
			.accept(this, arg);
		printer.print(" } else { ");
		n.getElseExpr()
			.accept(this, arg);
		printer.print(" }");
	}

	@Override
	public void visit(final ConstructorDeclaration n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		n.getModifiers()
			.accept(this, arg);
		printer.endComment();

		printTypeParameters(n.getTypeParameters(), arg);
		if (!n.getTypeParameters()
			.isEmpty()) {
			printer.print(" ");
		}
		printer.print("fn new");

		printer.print("(");
		if (!n.getParameters()
			.isEmpty()) {
			for (final Iterator<Parameter> i = n.getParameters()
				.iterator(); i.hasNext(); ) {
				final Parameter p = i.next();
				p.accept(this, arg);
				if (i.hasNext()) {
					printer.print(", ");
				}
			}
		}
		printer.print(") -> ");
		printer.print(n.getName()
			.asString());

		if (!isNullOrEmpty(n.getThrownExceptions())) {
			printer.print(" throws ");
			for (final Iterator<ReferenceType> i = n.getThrownExceptions()
				.iterator(); i.hasNext(); ) {
				final ReferenceType referenceType = i.next();
				referenceType.accept(this, arg);
				if (i.hasNext()) {
					printer.print(", ");
				}
			}
		}
		printer.print(" ");
		n.getBody()
			.accept(this, arg);
		printer.println("\n");
	}

	@Override
	public void visit(final ContinueStmt n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.print("continue");
		if (n.getLabel()
			.isPresent()) {
			printer.print(" '");
			printer.print(n.getLabel()
				.get()
				.asString());
		}
		printer.print(";");
	}

	@Override
	public void visit(final DoStmt n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.print("loop { ");
		n.getBody()
			.accept(this, arg);
		printer.print("if !(");
		n.getCondition()
			.accept(this, arg);
		printer.print(") break;");
		printer.print("}");
	}

	@Override
	public void visit(final DoubleLiteralExpr n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		String value = n.getValue();
		if (!StringUtils.containsAny(value, '.', 'e', 'E', 'x', 'X'))
			value = value + ".0";
		printer.print(removePlusAndSuffix(value, "D", "d"));
	}

	private String removePlusAndSuffix(String value, CharSequence... searchStrings) {
		if (value.startsWith("+")) {
			value = value.substring(1);
		}
		if (value.startsWith(".")) {
			value = "0" + value;
		}
		if (Strings.CS.endsWithAny(value, searchStrings)) {
			value = value.substring(0, value.length() - 1);
		}
		if (value.endsWith(".")) {
			value = value + "0";
		}
		value = value.replace("d.", ".");
		return value;
	}

	@Override
	public void visit(final EmptyStmt n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.print(";");
	}

	@Override
	public void visit(final EnclosedExpr n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.print("(");
		if (n.getInner() != null) {
			n.getInner()
				.accept(this, arg);
		}
		printer.print(")");
	}

	@Override
	public void visit(final EnumConstantDeclaration n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.print(n.getName()
			.asString());

		if (n.getArguments() != null) {
			printArguments(n.getArguments(), arg);
		}

		if (!n.getClassBody()
			.isEmpty()) {
			printer.println(" {");
			printer.indent();
			printMembers(n.getClassBody(), arg, null);
			printer.unindent();
			printer.println("}");
		}
	}

	@Override
	public void visit(final EnumDeclaration n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		n.getModifiers()
			.accept(this, arg);
		printer.endComment();

		printer.print("enum ");
		printer.print(n.getName()
			.asString());

		if (!n.getImplementedTypes()
			.isEmpty()) {
			printer.print(" implements ");
			for (final Iterator<ClassOrInterfaceType> i = n.getImplementedTypes()
				.iterator(); i.hasNext(); ) {
				final ClassOrInterfaceType c = i.next();
				c.accept(this, arg);
				if (i.hasNext()) {
					printer.print(", ");
				}
			}
		}

		printer.println(" {");
		printer.indent();
		if (n.getEntries() != null) {
			printer.println();
			for (final Iterator<EnumConstantDeclaration> i = n.getEntries()
				.iterator(); i.hasNext(); ) {
				final EnumConstantDeclaration e = i.next();
				e.accept(this, arg);
				if (i.hasNext()) {
					printer.print(", ");
				}
			}
		}
		if (!n.getMembers()
			.isEmpty()) {
			printer.println(";");
			printMembers(n.getMembers(), arg, null);
		} else {
			if (!n.getEntries()
				.isEmpty()) {
				printer.println();
			}
		}
		printer.unindent();
		printer.print("}");
	}

	@Override
	public void visit(final ExplicitConstructorInvocationStmt n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		if (n.isThis()) {
			if (n.getTypeArguments()
				.isPresent())
				printTypeArgs(
					n.getTypeArguments()
						.get(), arg);
			printer.print("this");
		} else {
			if (n.getExpression()
				.isPresent()) {
				n.getExpression()
					.get()
					.accept(this, arg);
				printer.print(".");
			}
			if (n.getTypeArguments()
				.isPresent())
				printTypeArgs(
					n.getTypeArguments()
						.get(), arg);
			printer.print("super");
		}
		printArguments(n.getArguments(), arg);
		printer.print(";");
	}

	@Override
	public void visit(final ExpressionStmt n, final Object arg) {
		printOrphanCommentsBeforeThisChildNode(n);
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		n.getExpression()
			.accept(this, arg);
		printer.print(";");
	}

	@Override
	public void visit(final FieldAccessExpr n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		int mark = printer.push();
		n.getScope()
			.accept(this, arg);
		String scope = printer.getMark(mark);
		printer.drop();
		int i = StringUtils.lastIndexOfAny(StringUtils.stripEnd(scope, " "), "\n", "\t", " ", ".");
		String accessed = i <= 0 ? scope : scope.substring(i + 1);
		if (Character.isUpperCase(accessed.charAt(0)) && accessed.length() > 1 && Character.isLowerCase(
			accessed.charAt(1))) {
			printer.print("::");
		} else {
			printer.print(".");
		}
		printer.print(replaceLengthAtEnd(n.getName()
			.asString()));
	}

	@Override
	public void visit(final FieldDeclaration n, final Object arg) {
		printOrphanCommentsBeforeThisChildNode(n);

		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		n.getModifiers()
			.accept(this, arg);
		printer.endComment();

		// indent if necessary
		printer.print("");

		for (final Iterator<VariableDeclarator> i = n.getVariables()
			.iterator(); i.hasNext(); ) {
			final VariableDeclarator var = i.next();

			var.accept(this, n.getCommonType());
			if (i.hasNext()) {
				printer.print(", ");
			}
		}

		printer.println(",");
	}

	@Override
	public void visit(final ForEachStmt n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.print("for ");
		n.getVariable()
			.accept(this, arg);
		printer.print(" in ");
		n.getIterable()
			.accept(this, arg);
		printer.print(" ");
		encapsulateIfNotBlock(n.getBody(), arg);
	}

	@Override
	public void visit(final ForStmt n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		if (n.getInitialization() != null && !n.getInitialization()
			.isEmpty()) {
			printer.println(" {");
			printer.indent();
			for (final Expression e : n.getInitialization()) {
				e.accept(this, arg);
				printer.println(";");
			}
		}
		if (n.getCompare()
			.isPresent()) {
			printer.print("while ");
			n.getCompare()
				.get()
				.accept(this, arg);
		} else {
			printer.print("loop ");
		}
		if (n.getUpdate() != null && !n.getUpdate()
			.isEmpty()) {
			printer.println(" {");
			printer.indent();
		}

		encapsulateIfNotBlock(n.getBody(), arg);
		printer.println("");
		if (n.getUpdate() != null && !n.getUpdate()
			.isEmpty()) {
			for (final Expression e : n.getUpdate()) {
				e.accept(this, arg);
				printer.println(";");
			}
			printer.unindent();
			printer.println(" }");
		}
		if (n.getInitialization() != null && !n.getInitialization()
			.isEmpty()) {
			printer.unindent();
			printer.println(" }");
		}
	}

	@Override
	public void visit(final IfStmt n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.print("if ");
		n.getCondition()
			.accept(this, arg);
		final boolean thenBlock = n.getThenStmt() instanceof BlockStmt;
		if (thenBlock) // block statement should start on the same line
			printer.print(" ");
		else {
			printer.println(" {");
			printer.indent();
		}
		n.getThenStmt()
			.accept(this, arg);
		if (!thenBlock) {
			printer.unindent();
			printer.println();
			printer.println("}");
		}
		if (n.getElseStmt()
			.isPresent()) {
			if (thenBlock)
				printer.print(" ");
			final boolean elseIf = n.getElseStmt()
				.get() instanceof IfStmt;
			final boolean elseBlock = n.getElseStmt()
				.get() instanceof BlockStmt;
			if (elseIf || elseBlock) // put chained if and start of block statement on a same level
				printer.print("else ");
			else {
				printer.print("else {");
				printer.indent();
			}
			n.getElseStmt()
				.get()
				.accept(this, arg);
			if (!(elseIf || elseBlock)) {
				printer.unindent();
				printer.println();
				printer.println("}");
			}
		}
	}

	@Override
	public void visit(final InitializerDeclaration n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		if (n.isStatic()) {
			printer.print("static ");
		}
		n.getBody()
			.accept(this, arg);
	}

	@Override
	public void visit(final InstanceOfExpr n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		n.getExpression()
			.accept(this, arg);
		printer.print(" instanceof ");
		n.getType()
			.accept(this, arg);
	}

	@Override
	public void visit(final IntegerLiteralExpr n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		String output = removePlusAndSuffix(n.getValue());
		if (isFloatInHistory(n)) {
			printer.print(output + ".0");

		} else {
			printer.print(output);
		}
	}

	@Override
	public void visit(TraditionalJavadocComment n, Object arg) {
		printer.startComment();
		printer.print("TraditionalJavadocComment");
		printer.println(n.toString());
		printer.endComment();
	}

	@Override
	public void visit(final LabeledStmt n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.print("'");
		printer.print(n.getLabel()
			.asString());
		printer.print(": ");
		n.getStatement()
			.accept(this, arg);
	}

	@Override
	public void visit(final LineComment n, final Object arg) {
		printer.print("//");
		String tmp = n.getContent();
		tmp = tmp.replace('\r', ' ');
		tmp = tmp.replace('\n', ' ');
		printer.println(tmp);
	}

	@Override
	public void visit(final LongLiteralExpr n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.print(removePlusAndSuffix(n.getValue(), "l", "L"));
	}

	@Override
	public void visit(MarkerAnnotationExpr n, Object arg) {
		printer.startComment();
		printer.print("MarkerAnnotationExpr");
		printer.println(n.toString());
		printer.endComment();
	}

	@Override
	public void visit(final MemberValuePair n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.print(n.getName()
			.asString());
		printer.print(" = ");
		n.getValue()
			.accept(this, arg);
	}

	@Override
	public void visit(final MethodCallExpr n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		if (n.getScope()
			.isPresent()) {
			n.getScope()
				.get()
				.accept(this, arg);
			if (Character.isUpperCase(n.getScope()
				.toString()
				.charAt(0)))
				printer.print("::");
			else
				printer.print(".");
		}
		if (n.getTypeArguments()
			.isPresent())
			printTypeArgs(
				n.getTypeArguments()
					.get(), arg);
		if (n.getScope()
			.isEmpty()) {
			/*
			Optional<Pair<TypeDescription, Node>> decl = idTracker.findDeclarationNodeFor(
				n.getName()
					.asString(), n);
			if (decl.isPresent()) {
				Node declNode = decl.get().b;
				if (declNode != null) {
					if (declNode instanceof MethodDeclaration methodDeclaration) {
						if (!methodDeclaration.isStatic())
							printer.print("self.");
						else
							printer.print("::");
					} else {
						printer.print("self.");
					}
				}
			}
			 */
		}
		printer.print(toSnakeIfNecessary(n.getName()
			.asString()));
		printArguments(n.getArguments(), arg);
	}

	@Override
	public void visit(final MethodDeclaration n, final Object arg) {
		try {
			printOrphanCommentsBeforeThisChildNode(n);

			printJavaComment(
				n.getComment()
					.orElse(null), arg);

			for (AnnotationExpr a : n.getAnnotations()) {
				if (a.getName()
					.getIdentifier()
					.equals("Test")) {
					printer.println("#[test]");
				}
			}
			n.getModifiers()
				.accept(this, arg);
			printer.endComment();
			printer.print("fn ");
			if (n.isDefault()) {
				printer.print("default ");
			}
			printTypeParameters(n.getTypeParameters(), arg);
			if (!isNullOrEmpty(n.getTypeParameters())) {
				printer.print(" ");
			}

			int mark = printer.push();
			n.getType()
				.accept(this, arg);
			String typeString = printer.getMark(mark);
			printer.pop();
			printer.print(toSnakeIfNecessary(n.getName()
				.asString()));

			printer.print("(");
			if (!n.isStatic()) {
				printer.print("&self");
				if (!isNullOrEmpty(n.getParameters()))
					printer.print(", ");
			}
			if (!isNullOrEmpty(n.getParameters())) {
				for (final Iterator<Parameter> i = n.getParameters()
					.iterator(); i.hasNext(); ) {
					final Parameter p = i.next();
					p.accept(this, arg);
					if (i.hasNext()) {
						printer.print(", ");
					}
				}
			}
			printer.print(") ");
			if (!typeString.equals("void")) {
				printer.print("-> ");

				//				if (n.getArrayCount() > 0) {
				//					printer.print("/* ");
				//					for (int i = 0; i < n.getArrayCount(); i++) {
				//						printer.print("[]");
				//					}
				//					printer.print(" */");
				//				}

				if (!isNullOrEmpty(n.getThrownExceptions())) {
					replaceThrows(n, arg, typeString);
				} else {
					printer.print(typeString);
				}
				printer.print(" ");
			} else {
				if (!isNullOrEmpty(n.getThrownExceptions())) {
					printer.print(" -> ");
					replaceThrows(n, arg, "Void");
				}
			}
			if (n.getBody()
				.isEmpty()) {
				printer.print(";");
			} else {
				n.getBody()
					.get()
					.accept(this, arg);
			}
		} finally {
		}
		printer.println("\n");
	}

	@Override
	public void visit(final NameExpr n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);

		/*
		Optional<Pair<TypeDescription, Node>> b = idTracker.findDeclarationNodeFor(
			n.getName()
				.asString(), n);

		if (b.isPresent() && (NodeEvaluator.isNonStaticFieldDeclaration(b.get().b) && idTracker.isOutsideConstructor() || NodeEvaluator.isNonStaticMethodDeclaration(
			b.get().b))) {
			printer.print("self.");
		}
		*/
		printer.print(toSnakeIfNecessary(n.getName()
			.asString()));

		printOrphanCommentsEnding(n);
	}

	@Override
	public void visit(NormalAnnotationExpr n, Object arg) {
		printer.startComment();
		printer.print("NormalAnnotationExpr");
		printer.println(n.toString());
		printer.endComment();
	}

	@Override
	public void visit(final NullLiteralExpr n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.print("null");
	}

	@Override
	public void visit(final ObjectCreationExpr n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		if (n.getScope()
			.isPresent()) {
			n.getScope()
				.get()
				.accept(this, arg);
			printer.print(".");
		}


		if (n.getTypeArguments()
			.isPresent()) {
			printTypeArgs(
				n.getTypeArguments()
					.get(), arg);
			if (!isNullOrEmpty(n.getTypeArguments()
				.get())) {
				printer.print(" ");
			}
		}

		n.getType()
			.accept(this, arg);
		printer.print("::new");

		printArguments(n.getArguments(), arg);

		if (n.getAnonymousClassBody()
			.isPresent()) {
			printer.println(" {");
			printer.indent();
			printMembers(
				n.getAnonymousClassBody()
					.get(), arg, null);
			printer.unindent();
			printer.print("}");
		}
	}

	@Override
	public void visit(final PackageDeclaration n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.print("// package ");
		n.getName()
			.accept(this, arg);
		printer.println(";");
		printer.println();

		printOrphanCommentsEnding(n);
	}

	@Override
	public void visit(final Parameter n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		n.getModifiers()
			.accept(this, arg);
		if (n.isVarArgs())
			printer.comment("... ");
		printer.endComment();

		n.getName()
			.accept(this, arg);
		printer.print(": ");
		if (!(n.getType() instanceof PrimitiveType))
			printer.print("&");
		if (n.getType() != null) {
			n.getType()
				.accept(this, arg);
		}
	}

	@Override
	public void visit(final PrimitiveType n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);

		switch (n.getType()) {
		case BOOLEAN:
			printer.print("bool");
			break;
		case BYTE:
			printer.print("i8");
			break;
		case CHAR:
			printer.print("char");
			break;
		case DOUBLE:
			printer.print("f64");
			break;
		case FLOAT:
			printer.print("f32");
			break;
		case INT:
			printer.print("i32");
			break;
		case LONG:
			printer.print("i64");
			break;
		case SHORT:
			printer.print("i16");
			break;
		}
	}

	@Override
	public void visit(Name n, Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		if (n.getQualifier()
			.isPresent()) {
			n.getQualifier()
				.get()
				.accept(this, arg);
			printer.print("::");
		}
		printer.print(n.getIdentifier());
		printOrphanCommentsEnding(n);
	}

	@Override
	public void visit(SimpleName n, Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.print(toSnakeIfNecessary(n.getIdentifier()));
	}

	@Override
	public void visit(ArrayType n, Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);

		for (int i = 0; i < n.getArrayLevel(); i++) {
			printer.print("Vec<");
		}
		n.getElementType()
			.accept(this, arg);
		for (int i = 0; i < n.getArrayLevel(); i++) {
			printer.print(">");
		}
	}

	@Override
	public void visit(ArrayCreationLevel n, Object arg) {
		super.visit(n, arg);
	}

	@Override
	public void visit(final IntersectionType n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		boolean isFirst = true;
		for (ReferenceType element : n.getElements()) {
			element.accept(this, arg);
			if (isFirst) {
				isFirst = false;
			} else {
				printer.print(" & ");
			}
		}
	}

	@Override
	public void visit(final UnionType n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		boolean isFirst = true;
		for (ReferenceType element : n.getElements()) {
			element.accept(this, arg);
			if (isFirst) {
				isFirst = false;
			} else {
				printer.print(" | ");
			}
		}
	}

	@Override
	public void visit(final ReturnStmt n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.print("return");
		if (n.getExpression()
			.isPresent()) {
			printer.print(" ");
//			if (idTracker.hasThrows()) {
//				printer.print("Ok(");
//			}
			n.getExpression()
				.get()
				.accept(this, arg);
//			if (idTracker.hasThrows()) {
//				printer.print(")");
//			}
		}
		printer.print(";");
	}

	@Override
	public void visit(SingleMemberAnnotationExpr n, Object arg) {
		printer.startComment();
		printer.print("SingleMemberAnnotationExpr");
		printer.println(n.toString());
		printer.endComment();
	}

	@Override
	public void visit(final StringLiteralExpr n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.print("\"");
		printer.print(n.getValue());
		printer.print("\"");
	}

	@Override
	public void visit(final SuperExpr n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		if (n.getTypeName()
			.isPresent()) {
			n.getTypeName()
				.get()
				.accept(this, arg);
			printer.print(".");
		}
		printer.print("super");
	}

	@Override
	public void visit(final SwitchEntry n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		if (n.getLabels() != null) {
			printer.print("  ");
			n.getLabels()
				.accept(this, arg);
			printer.print(" => ");
		} else {
			printer.print("_ => ");
		}
		printer.println();
		printer.indent();
		if (n.getStatements() != null) {
			printer.println(" {");
			printer.indent();
			for (final Statement s : n.getStatements()) {
				s.accept(this, arg);
				printer.println();
			}
			printer.unindent();
			printer.println("}");
		}
		printer.unindent();
	}

	@Override
	public void visit(final SwitchStmt n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.print("match ");
		n.getSelector()
			.accept(this, arg);
		printer.println(" {");
		if (n.getEntries() != null) {
			printer.indent();
			for (final SwitchEntry e : n.getEntries()) {
				e.accept(this, arg);
			}
			printer.unindent();
		}
		printer.print("}");

	}

	@Override
	public void visit(final SynchronizedStmt n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.print("synchronized (");
		n.getExpression()
			.accept(this, arg);
		printer.print(") ");
		n.getBody()
			.accept(this, arg);
	}

	@Override
	public void visit(final ThisExpr n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		if (n.getTypeName()
			.isPresent()) {
			n.getTypeName()
				.get()
				.accept(this, arg);
		} else {
//			if (idTracker.isOutsideConstructor())
//				printer.print("self");
//			else {
//				printer.print("let ");
//			}
		}
	}

	@Override
	public void visit(final ThrowStmt n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.print("throw ");
		n.getExpression()
			.accept(this, arg);
		printer.print(";");
	}

	@Override
	public void visit(final TryStmt n, final Object arg) {
//		int tryCount = ++idTracker.tryCount;
		try {
			printJavaComment(
				n.getComment()
					.orElse(null), arg);
//			printer.println("let tryResult" + tryCount + " = 0;");
//			printer.println("'try" + tryCount + ": loop {");
			if (!n.getResources()
				.isEmpty()) {
				printer.print("(");
				Iterator<Expression> resources = n.getResources()
					.iterator();
				boolean first = true;
				while (resources.hasNext()) {
					visit(
						resources.next()
							.asVariableDeclarationExpr(), arg);
					if (resources.hasNext()) {
						printer.print(";");
						printer.println();
						if (first) {
							printer.indent();
						}
					}
					first = false;
				}
				if (n.getResources()
					.size() > 1) {
					printer.unindent();
				}
				printer.print(") ");
			}
			n.getTryBlock()
				.accept(this, arg);
			printer.println();
//			printer.println("break 'try" + tryCount);
			printer.println("}");
			if (n.getCatchClauses() != null) {
//				printer.println("match tryResult" + tryCount + " {");
				printer.indent();
				for (final CatchClause c : n.getCatchClauses()) {
					c.accept(this, arg);
				}
				printer.println("  0 => break");
				printer.unindent();
				printer.println("}");
			}
			if (n.getFinallyBlock()
				.isPresent()) {
				printer.print(" finally ");
				n.getFinallyBlock()
					.get()
					.accept(this, arg);
			}
		} finally {
//			idTracker.tryCount--;
		}
	}

	@Override
	public void visit(LocalClassDeclarationStmt n, Object arg) {
		printer.startComment();
		printer.print("LocalClassDeclarationStmt");
		printer.println(n.toString());
		printer.endComment();
	}

	@Override
	public void visit(LocalRecordDeclarationStmt n, Object arg) {
		printer.startComment();
		printer.print("LocalRecordDeclarationStmt");
		printer.println(n.toString());
		printer.endComment();
	}

	@Override
	public void visit(final TypeParameter n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.print(n.getName()
			.asString());
		if (!isNullOrEmpty(n.getTypeBound())) {
			printer.print(" extends ");
			for (final Iterator<ClassOrInterfaceType> i = n.getTypeBound()
				.iterator(); i.hasNext(); ) {
				final ClassOrInterfaceType c = i.next();
				c.accept(this, arg);
				if (i.hasNext()) {
					printer.print(" & ");
				}
			}
		}
	}

	@Override
	public void visit(UnaryExpr n, Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		String unarySuffix = "";
		switch (n.getOperator()) {
		case PREFIX_INCREMENT:
			unarySuffix = " += 1";
		case POSTFIX_INCREMENT:
			if (unarySuffix.isEmpty())
				unarySuffix = " += 1" + (isEmbeddedInStmt(n) ? " !!!check!!! post increment" : "");
		case PREFIX_DECREMENT:
			if (unarySuffix.isEmpty())
				unarySuffix = " -= 1";
		case POSTFIX_DECREMENT:
			if (unarySuffix.isEmpty())
				unarySuffix = " -= 1" + (isEmbeddedInStmt(n) ? " !!!check!!! post decrement" : "");
		case PLUS:
			n.getExpression()
				.accept(this, arg);
			printer.print(unarySuffix);
			break;
		default:
			orgVisit(n, arg);
		}
	}

	@Override
	public void visit(final UnknownType n, final Object arg) {
		printer.startComment();
		printer.print("UnknownType");
		printer.println(n.toString());
		printer.endComment();
	}

	@Override
	public void visit(final VariableDeclarationExpr n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		n.getModifiers()
			.accept(this, arg);
		printer.endComment();
		printer.print("");

		for (final Iterator<VariableDeclarator> i = n.getVariables()
			.iterator(); i.hasNext(); ) {
			final VariableDeclarator v = i.next();
			v.accept(this, n.getCommonType());
			if (i.hasNext())
				printer.print("; ");
		}
	}

	@Override
	public void visit(final VariableDeclarator n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		String name = acceptAndCut(n.getName(), arg);
		boolean isConstant = false;
		if (Character.isUpperCase(name.charAt(0))) {
			printer.print("const ");
			isConstant = true;
		} else {
			printer.print("let ");
//			if (idTracker.isChanged(name, n)) {
//				printer.print("mut ");
//			}
		}
		printer.print(name);
		boolean isInitializedArray = n.getInitializer()
			.isPresent() && (n.getInitializer()
			.get() instanceof ArrayInitializerExpr || n.getInitializer()
			.get() instanceof ArrayCreationExpr);
		if (arg instanceof Type t && !isInitializedArray) {
			printer.print(": ");
			String tmp = acceptAndCut(t, null);

			if (isConstant && tmp.equals("String")) {
				printer.print("&'static str");
			} else {
				printer.print(tmp);
			}
		}
		if (n.getInitializer()
			.isPresent()) {
			if (!isInitializedArray)
				printer.print(" = ");
			n.getInitializer()
				.get()
				.accept(this, arg);
		}
	}

	@Override
	public void visit(final VoidType n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.print("void");
	}

	@Override
	public void visit(final WhileStmt n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.print("while ");
		n.getCondition()
			.accept(this, arg);
		printer.print(" ");
		n.getBody()
			.accept(this, arg);
	}

	@Override
	public void visit(final WildcardType n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.print("?");
		if (n.getExtendedType()
			.isPresent()) {
			printer.print(" extends ");
			n.getExtendedType()
				.get()
				.accept(this, arg);
		}
		if (n.getSuperType()
			.isPresent()) {
			printer.print(" super ");
			n.getSuperType()
				.get()
				.accept(this, arg);
		}
	}

	@Override
	public void visit(LambdaExpr n, Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);

		List<Parameter> parameters = n.getParameters();
		boolean printPar = false;
		printPar = n.isEnclosingParameters();

		if (printPar) {
			printer.print("(");
		}
		if (parameters != null) {
			for (Iterator<Parameter> i = parameters.iterator(); i.hasNext(); ) {
				Parameter p = i.next();
				p.accept(this, arg);
				if (i.hasNext()) {
					printer.print(", ");
				}
			}
		}
		if (printPar) {
			printer.print(")");
		}

		printer.print(" -> ");
		Statement body = n.getBody();
		if (body instanceof ExpressionStmt) {
			// Print the expression directly
			((ExpressionStmt) body).getExpression()
				.accept(this, arg);
		} else {
			body.accept(this, arg);
		}
	}

	@Override
	public void visit(MethodReferenceExpr n, Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		Expression scope = n.getScope();
		String identifier = n.getIdentifier();
		if (scope != null) {
			n.getScope()
				.accept(this, arg);
		}

		printer.print("::");
		if (n.getTypeArguments()
			//			.getTypeArguments()
			.isPresent()) {
			printer.print("<");
			for (Iterator<Type> i = n.getTypeArguments()
				.get()
				//				.getTypeArguments()
				.iterator(); i.hasNext(); ) {
				Type p = i.next();
				p.accept(this, arg);
				if (i.hasNext()) {
					printer.print(", ");
				}
			}
			printer.print(">");
		}
		if (identifier != null) {
			printer.print(identifier);
		}

	}

	@Override
	public void visit(TypeExpr n, Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		if (n.getType() != null) {
			n.getType()
				.accept(this, arg);
		}
	}

	@Override
	public void visit(final ImportDeclaration n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		printer.print("use ");
		if (n.isStatic()) {
			printer.comment("static");
		}
		n.getName()
			.accept(this, arg);
		if (n.isAsterisk()) {
			printer.print("::*");
		}
		printer.println(";");

		printOrphanCommentsEnding(n);
	}

	@Override
	public void visit(ModuleDeclaration n, Object arg) {
		printer.startComment();
		printer.print("ModuleDeclaration");
		printer.println(n.toString());
		printer.endComment();
	}

	@Override
	public void visit(ModuleRequiresDirective n, Object arg) {
		printer.startComment();
		printer.print("ModuleRequiresDirective");
		printer.println(n.toString());
		printer.endComment();
	}

	@Override
	public void visit(ModuleExportsDirective n, Object arg) {
		printer.startComment();
		printer.print("ModuleExportsDirective");
		printer.println(n.toString());
		printer.endComment();
	}

	@Override
	public void visit(ModuleProvidesDirective n, Object arg) {
		printer.startComment();
		printer.print("ModuleProvidesDirective");
		printer.println(n.toString());
		printer.endComment();
	}

	@Override
	public void visit(ModuleUsesDirective n, Object arg) {
		printer.startComment();
		printer.print("ModuleUsesDirective");
		printer.println(n.toString());
		printer.endComment();
	}

	@Override
	public void visit(ModuleOpensDirective n, Object arg) {
		printer.startComment();
		printer.print("ModuleOpensDirective");
		printer.println(n.toString());
		printer.endComment();
	}

	@Override
	public void visit(UnparsableStmt n, Object arg) {
		printer.startComment();
		printer.print("UnparsableStmt");
		printer.println(n.toString());
		printer.endComment();
	}

	@Override
	public void visit(ReceiverParameter n, Object arg) {
		printer.startComment();
		printer.print("ReceiverParameter");
		printer.println(n.toString());
		printer.endComment();
	}

	@Override
	public void visit(VarType n, Object arg) {
		printer.startComment();
		printer.print("VarType");
		printer.println(n.toString());
		printer.endComment();
	}

	@Override
	public void visit(Modifier n, Object arg) {
		switch (n.getKeyword()) {
		case DEFAULT:
			printer.comment("default ");
			break;
		case PUBLIC:
			printer.endComment();
			printer.print("pub ");
			break;
		case PROTECTED:
			printer.comment("protected ");
			break;
		case PRIVATE:
			// everything is private by default in Rust
			break;
		case ABSTRACT:
			printer.comment("abstract ");
			break;
		case STATIC:
			printer.comment("static ");
			break;
		case FINAL:
			printer.comment("final ");
			break;
		case TRANSIENT:
			printer.comment("transient ");
			break;
		case VOLATILE:
			printer.comment("volatile ");
			break;
		case SYNCHRONIZED:
			printer.comment("synchronized ");
			break;
		case NATIVE:
			printer.comment("native ");
			break;
		case STRICTFP:
			printer.comment("strictfp ");
			break;
		case TRANSITIVE:
			printer.comment("transitive ");
			break;
		case SEALED:
			printer.comment("sealed ");
			break;
		case NON_SEALED:
			printer.comment("nonsealed ");
			break;
		}
	}

	@Override
	public void visit(SwitchExpr n, Object arg) {
		printer.startComment();
		printer.print("SwitchExpr");
		printer.println(n.toString());
		printer.endComment();
	}

	@Override
	public void visit(TextBlockLiteralExpr n, Object arg) {
		printer.startComment();
		printer.print("TextBlockLiteralExpr");
		printer.println(n.toString());
		printer.endComment();
	}

	@Override
	public void visit(YieldStmt n, Object arg) {
		printer.startComment();
		printer.print("YieldStmt");
		printer.println(n.toString());
		printer.endComment();
	}

	@Override
	public void visit(TypePatternExpr n, Object arg) {
		printer.startComment();
		printer.print("TypePatternExpr");
		printer.println(n.toString());
		printer.endComment();
	}

	@Override
	public void visit(RecordDeclaration n, Object arg) {
		printer.startComment();
		printer.print("RecordDeclaration");
		printer.println(n.toString());
		printer.endComment();
	}

	@Override
	public void visit(CompactConstructorDeclaration n, Object arg) {
		printer.startComment();
		printer.print("CompactConstructorDeclaration");
		printer.println(n.toString());
		printer.endComment();
	}

	@Override
	public void visit(RecordPatternExpr n, Object arg) {
		printer.startComment();
		printer.print("RecordPatternExpr");
		printer.println(n.toString());
		printer.endComment();
	}

	@Override
	public void visit(MatchAllPatternExpr n, Object arg) {
		printer.startComment();
		printer.print("MatchAllPatternExpr");
		printer.println(n.toString());
		printer.endComment();
	}

	@Override
	public void visit(final MarkdownComment n, final Object arg) {
		printer.print("/**");
		printer.print(n.getContent());
		printer.println("*/");
	}

	boolean isEmbeddedInStmt(UnaryExpr n) {
		Node parent = n.getParentNode()
			.get();
		return !(parent instanceof ExpressionStmt) && !(parent instanceof ForStmt);
	}

	private void orgVisit(final UnaryExpr n, final Object arg) {
		printJavaComment(
			n.getComment()
				.orElse(null), arg);
		switch (n.getOperator()) {
		case PLUS:
			printer.print("+");
			break;
		case MINUS:
			printer.print("-");
			break;
		case BITWISE_COMPLEMENT:
			printer.print("~");
			break;
		case LOGICAL_COMPLEMENT:
			printer.print("!");
			break;
		case PREFIX_INCREMENT:
			printer.print("++");
			break;
		case PREFIX_DECREMENT:
			printer.print("--");
			break;
		default:
		}

		n.getExpression()
			.accept(this, arg);

		switch (n.getOperator()) {
		case POSTFIX_INCREMENT:
			printer.print("++");
			break;
		case POSTFIX_DECREMENT:
			printer.print("--");
			break;
		default:
		}
	}

	private void replaceThrows(MethodDeclaration n, Object arg, String typeString) {
		printer.startComment();
		printer.comment("throws ");
		for (final Iterator<ReferenceType> i = n.getThrownExceptions()
			.iterator(); i.hasNext(); ) {
			final ReferenceType name = i.next();
			name.accept(this, arg);
			if (i.hasNext()) {
				printer.comment(", ");
			}
		}
		printer.endComment();
		printer.print("Result<");
		printer.print(typeString);
		printer.print(", Rc<Exception>>");
	}

	private String toSnakeIfNecessary(String n) {
		String name = transpiler.nameOf(n);
		if (name != null)
			return name;
		return Java2Rust.identifier(n);
	}

	private void encapsulateIfNotBlock(Statement n, final Object arg) {
		if (n instanceof BlockStmt) {
			n.accept(this, arg);
		} else {
			printer.println(" {");
			printer.indent();
			n.accept(this, arg);
			printer.println("}");
		}
	}

	String replaceLengthAtEnd(String fieldAccess) {
		if (fieldAccess.equals("length"))
			return "len()";
		else
			return fieldAccess;
	}

	private void printArguments(final List<Expression> args, final Object arg) {
		printer.print("(");
		if (!isNullOrEmpty(args)) {
			for (final Iterator<Expression> i = args.iterator(); i.hasNext(); ) {
				final Expression e = i.next();
				if (e instanceof NameExpr ne) {
					/*
					Optional<Pair<TypeDescription, Node>> decl = idTracker.findDeclarationNodeFor(
						ne.getName()
							.asString(), ne);
					if (decl.isPresent() && decl.get().b != null) {
						final TypeDescription left = decl.get().a;
						if (left != null && (!left.clazz.isPrimitive() || left.getArrayCount() > 0)) {
							printer.print("&");
						}
					}
					 */
				} else if (e instanceof MethodCallExpr) {
					printer.print("&");
				}
				e.accept(this, arg);
				if (i.hasNext()) {
					printer.print(", ");
				}
			}
		}
		printer.print(")");
	}

	private void printOrphanCommentsBeforeThisChildNode(final Node node) {
		if (node instanceof Comment)
			return;

		if (node.getParentNode()
			.isEmpty())
			return;
		Node parent = node.getParentNode()
			.get();
		List<Node> everything = new LinkedList<Node>();
		everything.addAll(parent.getChildNodes());
		sortByBeginPosition(everything);
		int positionOfTheChild = -1;
		for (int i = 0; i < everything.size(); i++) {
			if (everything.get(i) == node)
				positionOfTheChild = i;
		}
		if (positionOfTheChild == -1)
			throw new RuntimeException("My index not found!!! " + node);
		int positionOfPreviousChild = -1;
		for (int i = positionOfTheChild - 1; i >= 0 && positionOfPreviousChild == -1; i--) {
			if (!(everything.get(i) instanceof Comment))
				positionOfPreviousChild = i;
		}
		for (int i = positionOfPreviousChild + 1; i < positionOfTheChild; i++) {
			Node nodeToPrint = everything.get(i);
			if (!(nodeToPrint instanceof Comment))
				throw new RuntimeException(
					"Expected comment, instead %s. Position of previous child: %d, position of child %d".formatted(
						nodeToPrint.getClass(),
						positionOfPreviousChild,
						positionOfTheChild));
			nodeToPrint.accept(this, null);
		}
	}

	private void printOrphanCommentsEnding(final Node node) {
		List<Node> everything = new LinkedList<Node>(node.getChildNodes());
		sortByBeginPosition(everything);
		if (everything.isEmpty()) {
			return;
		}

		int commentsAtEnd = 0;
		boolean findingComments = true;
		while (findingComments && commentsAtEnd < everything.size()) {
			Node last = everything.get(everything.size() - 1 - commentsAtEnd);
			findingComments = (last instanceof Comment);
			if (findingComments) {
				commentsAtEnd++;
			}
		}
		for (int i = 0; i < commentsAtEnd; i++) {
			everything.get(everything.size() - commentsAtEnd + i)
				.accept(this, null);
		}
	}

	String acceptAndCut(Node n, final Object arg) {
		int mark = printer.push();
		n.accept(this, arg);
		String result = printer.getMark(mark);
		printer.pop();
		return result;
	}

	String defaultValue(String type) {
		return switch (type) {
			case "f64", "f32" -> "0.0";
			case "u64", "u32", "u16", "u8", "usize", "i64", "i32", "i16", "i8" -> "0";
			case "bool" -> "false";
			default -> "None";
		};
	}

	private String getArrayDeclaration(String typeOrDefaultValue, List<String> dims) {
		StringBuilder sb = new StringBuilder();
		sb.append(typeOrDefaultValue);
		reverse(dims);
		for (String s : dims) {
			sb.insert(0, "[");
			sb.append("; ")
				.append(s)
				.append("]");
		}
		reverse(dims);
		return sb.toString();
	}

	private void printJavaComment(final Comment javacomment, final Object arg) {
		if (javacomment != null) {
			javacomment.accept(this, arg);
		}
	}

	List<Node> genStringExprSequence(BinaryExpr n) {
		List<Node> result = new ArrayList<>();
		if (n.getOperator() == BinaryExpr.Operator.PLUS) {
			genStringPart(n.getLeft(), result);
			genStringPart(n.getRight(), result);
		} else {
			result.add(n);
			return result;
		}


		return result;
	}

	private void genStringPart(Node n, List<Node> result) {
		if (n instanceof BinaryExpr) {
			result.addAll(genStringExprSequence(((BinaryExpr) n)));
		} else {
			result.add(n);
		}
	}

	private void printStringExpression(BinaryExpr n, final Object arg) {
		List<Node> binChain = genStringExprSequence(n);
		printer.print("format!(\"");
		for (Node node : binChain) {
			if (node instanceof StringLiteralExpr) {
				String value = ((StringLiteralExpr) node).getValue();
				printer.print(value);
			} else {
				printer.print("{}");
			}
		}
		printer.print("\"");

		for (Node node : binChain) {
			if (!(node instanceof StringLiteralExpr) && node != n) {
				printer.print(", ");
				node.accept(this, arg);
			}
		}
		printer.print(")");

	}

	boolean isFloatInSiblings(Node n) {
		if (n == null || n.getParentNode()
			.isEmpty())
			return false;
		if (stopHistorySearch(n.getParentNode()
			.get()))
			return false;
		List<Node> siblings = n.getParentNode()
			.get()
			.getChildNodes();
		for (Node sibling : siblings) {
//			if (idTracker.isFloat(sibling)) {
//				return true;
//			}
		}
		return false;
	}

	/*
	@Override
	public void visit(final IntegerLiteralMinValueExpr n, final Object arg) {
		printJavaComment(n.getComment(), arg);
		printer.print(n.getValue());
	}

	@Override
	public void visit(final LongLiteralMinValueExpr n, final Object arg) {
		printJavaComment(n.getComment(), arg);
		printer.print(n.getValue());
	}

	@Override
	public void visit(MultiTypeParameter n, Object arg) {
		printModifiers(n.getModifiers());

		Type type = n.getType();
		if (type != null) {
			type.accept(this, arg);
		}

		printer.print(" ");
		n
			.getId()
			.accept(this, arg);
	}

	@Override
	public void visit(final TypeDeclarationStmt n, final Object arg) {
		printJavaComment(n.getComment(), arg);
		n
			.getTypeDeclaration()
			.accept(this, arg);
	}
	 */

	boolean isFloatInHistory(Node n) {
		if (stopHistorySearch(n))
			return false;
		if (n == null)
			return false;
		if (isFloatInSiblings(n))
			return true;
//		Class clazz = idTracker.getType(n);
//		if (idTracker.isFloat(clazz)) {
//			return true;
//		} else {
			return isFloatInHistory(n.getParentNode()
				.get());
//		}
	}

	/*
	@Override
	public void visit(final EmptyMemberDeclaration n, final Object arg) {
		printJavaComment(n.getComment(), arg);
		printer.print(";");
	}
	 */

	private boolean stopHistorySearch(Node n) {
		return n instanceof VariableDeclarator || n instanceof MethodCallExpr || n instanceof Statement || n instanceof ArrayAccessExpr;
	}
}
