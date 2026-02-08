package java2rust;

import com.github.javaparser.ast.Node;
import com.github.javaparser.ast.comments.Comment;
import com.github.javaparser.ast.expr.*;
import com.github.javaparser.ast.type.Type;
import com.github.javaparser.ast.visitor.VoidVisitorAdapter;
import com.github.javaparser.utils.Pair;

import java.util.Optional;

/**
 * @author aschoerk
 */
public class TypeTrackerVisitor extends VoidVisitorAdapter<Object> {

	IdTracker idTracker;


	public TypeTrackerVisitor(final IdTracker idTracker) {
		this.idTracker = idTracker;
	}

	/*
	@Override
	public void visit(final IntegerLiteralMinValueExpr n, final Object arg) {
		idTracker.putType(n, Integer.TYPE);
		super.visit(n, arg);
	}

	@Override
	public void visit(final LongLiteralMinValueExpr n, final Object arg) {
		idTracker.putType(n, Integer.TYPE);
		super.visit(n, arg);
	}
	 */

	@Override
	public void visit(final ArrayAccessExpr n, final Object arg) {
		if (n.getName() instanceof NameExpr ne) {
			Optional<Pair<TypeDescription, Node>> b = idTracker.findDeclarationNodeFor(
				ne
					.getName()
					.asString(), ne);
			if (b.isPresent()) {
				if (b.get().a != null)
					idTracker.putType(n, b.get().a.getClazz());
			}
		}
		idTracker.putType(n.getIndex(), Integer.TYPE);
		if (n.getIndex() instanceof NameExpr ne) {
			Optional<Pair<TypeDescription, Node>> b = idTracker.findDeclarationNodeFor(
				ne
					.getName()
					.asString(), ne);
			if (b.isPresent()) {
				if (b.get().a != null)
					b.get().a.clazz = Integer.TYPE;
			}
		}
		super.visit(n, arg);
	}

	@Override
	public void visit(final ArrayInitializerExpr n, final Object arg) {
		super.visit(n, arg);
	}

	@Override
	public void visit(final AssignExpr n, final Object arg) {
		int i = 5;
		super.visit(n, arg);
	}

	@Override
	public void visit(final BinaryExpr n, final Object arg) {
		visitComment(
			n
				.getComment()
				.orElse(null), arg);
		n
			.getLeft()
			.accept(this, arg);
		n
			.getRight()
			.accept(this, arg);
		switch (n.getOperator()) {
		case EQUALS:
		case NOT_EQUALS:
		case AND:
		case OR:
		case LESS:
		case GREATER:
		case LESS_EQUALS:
		case GREATER_EQUALS:
			idTracker.putType(n, Boolean.TYPE);
			break;
		case BINARY_OR:
		case BINARY_AND:
		case XOR:
			propagateIntBool(n, n.getLeft(), n.getRight());
			break;
		case LEFT_SHIFT:
		case SIGNED_RIGHT_SHIFT:
		case UNSIGNED_RIGHT_SHIFT:
			idTracker.putType(n, Integer.TYPE);
			break;
		case PLUS:
		case MINUS:
		case MULTIPLY:
		case DIVIDE:
		case REMAINDER:
			propagateTypes(n, n.getLeft(), n.getRight());
			break;
		}
	}

	private void visitComment(final Comment n, final Object arg) {
		if (n != null) {
			n.accept(this, arg);
		}
	}

	private void propagateIntBool(
		final BinaryExpr destination,
		final Expression left,
		final Expression right
	) {
		Class leftClazz = idTracker.getType(left);
		Class rightClazz = idTracker.getType(right);
		if (Boolean.TYPE.equals(leftClazz) || Boolean.TYPE.equals(rightClazz))
			idTracker.putType(destination, Boolean.TYPE);
		else
			idTracker.putType(destination, Integer.TYPE);
	}

	public void propagateTypes(Node destination, Node left, Node right) {
		Class leftClazz = idTracker.getType(left);
		Class rightClazz = idTracker.getType(right);
		if (String.class.equals(leftClazz) || String.class.equals(rightClazz))
			idTracker.putType(destination, String.class);
		else if (Double.TYPE.equals(leftClazz) || Double.TYPE.equals(rightClazz) || Double.class.equals(
			leftClazz) || Double.class.equals(rightClazz) || Float.TYPE.equals(leftClazz) || Float.TYPE.equals(
			rightClazz) || Float.class.equals(leftClazz) || Float.class.equals(rightClazz))
			idTracker.putType(destination, Double.TYPE);
		else if (Boolean.TYPE.equals(leftClazz) || Boolean.TYPE.equals(rightClazz))
			idTracker.putType(destination, Boolean.TYPE);
		else if (Integer.TYPE.equals(leftClazz) || Integer.TYPE.equals(rightClazz) || Short.TYPE.equals(
			leftClazz) || Short.TYPE.equals(rightClazz) || Long.TYPE.equals(leftClazz) || Long.TYPE.equals(
			rightClazz) || Byte.TYPE.equals(leftClazz) || Byte.TYPE.equals(rightClazz) || Integer.class.equals(
			leftClazz) || Integer.class.equals(rightClazz) || Short.class.equals(leftClazz) || Short.class.equals(
			rightClazz) || Long.class.equals(leftClazz) || Long.class.equals(rightClazz) || Byte.class.equals(
			leftClazz) || Byte.class.equals(rightClazz))
			idTracker.putType(destination, Integer.TYPE);
	}

	@Override
	public void visit(final BooleanLiteralExpr n, final Object arg) {
		idTracker.putType(n, Boolean.TYPE);
		super.visit(n, arg);
	}

	@Override
	public void visit(final CharLiteralExpr n, final Object arg) {
		idTracker.putType(n, Character.TYPE);
		super.visit(n, arg);
	}

	@Override
	public void visit(final ConditionalExpr n, final Object arg) {
		super.visit(n, arg);
	}

	@Override
	public void visit(final DoubleLiteralExpr n, final Object arg) {
		idTracker.putType(n, Double.TYPE);
		super.visit(n, arg);
	}

	@Override
	public void visit(final IntegerLiteralExpr n, final Object arg) {
		idTracker.putType(n, Integer.TYPE);
		super.visit(n, arg);
	}

	@Override
	public void visit(final LongLiteralExpr n, final Object arg) {
		idTracker.putType(n, Integer.TYPE);
		super.visit(n, arg);
	}

	@Override
	public void visit(final MethodCallExpr n, final Object arg) {
		visitComment(
			n
				.getComment()
				.orElse(null), arg);
		if (n
			.getScope()
			.isPresent()) {
			n
				.getScope()
				.get()
				.accept(this, arg);
		}
		if (n
			.getTypeArguments()
			.isPresent()) {
			for (final Type t : n
				.getTypeArguments()
				.get()) {
				t.accept(this, arg);
			}
		}
		if (n.getArguments() != null) {
			for (final Expression e : n.getArguments()) {
				e.accept(this, arg);
			}
		}
	}

	@Override
	public void visit(final NameExpr n, final Object arg) {
		Optional<Pair<TypeDescription, Node>> b = idTracker.findDeclarationNodeFor(
			n
				.getName()
				.asString(), n);
		if (b.isPresent()) {
			if (b.get().a != null)
				idTracker.putType(n, b.get().a.getClazz());
		}
		super.visit(n, arg);
	}

	@Override
	public void visit(final StringLiteralExpr n, final Object arg) {
		idTracker.putType(n, String.class);
		super.visit(n, arg);
	}

	@Override
	public void visit(final UnaryExpr n, final Object arg) {
		switch (n.getOperator()) {
		case PLUS:
		case MINUS:
			propagateTypes(n, n.getExpression(), n.getExpression());
			break;
		case LOGICAL_COMPLEMENT:
			idTracker.putType(n, Boolean.TYPE);
			break;
		case BITWISE_COMPLEMENT:
		case POSTFIX_INCREMENT:
		case POSTFIX_DECREMENT:
		case PREFIX_INCREMENT:
		case PREFIX_DECREMENT:
			idTracker.putType(n, Integer.TYPE);
			break;
		default:
		}
		super.visit(n, arg);
	}
}
