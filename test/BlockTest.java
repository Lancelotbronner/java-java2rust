package test;

import com.github.javaparser.ast.expr.NameExpr;
import java2rust.Block;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Created by aschoerk on 16.05.16.
 */
public class BlockTest {
	@Test
	public void selfContainsSelf() {
		NameExpr testNode = getNameExprBuilderL1C1L10C120().build();
		Block testBlock = new Block(testNode);
		assertTrue(testBlock.contains(testNode));
		assertTrue(testBlock.contains(getNameExprBuilderL1C1L10C120()
			.bC(testNode.getBeginColumn() + 1)
			.build()));
		assertTrue(!testBlock.contains(getNameExprBuilderL1C1L10C120()
			.bC(testNode.getBeginColumn() - 1)
			.build()));
		assertTrue(!testBlock.contains(getNameExprBuilderL1C1L10C120()
			.bL(testNode.getBeginLine() - 1)
			.build()));
		assertTrue(testBlock.contains(getNameExprBuilderL1C1L10C120()
			.bL(testNode.getBeginLine() + 1)
			.build()));
		assertTrue(testBlock.contains(getNameExprBuilderL1C1L10C120()
			.eC(testNode.getEndColumn() - 1)
			.build()));
		assertTrue(!testBlock.contains(getNameExprBuilderL1C1L10C120()
			.eC(testNode.getEndColumn() + 1)
			.build()));
		assertTrue(!testBlock.contains(getNameExprBuilderL1C1L10C120()
			.eL(testNode.getEndLine() + 1)
			.build()));
		assertTrue(testBlock.contains(getNameExprBuilderL1C1L10C120()
			.eL(testNode.getEndLine() - 1)
			.build()));
	}

	private NameExprBuilder getNameExprBuilderL1C1L10C120() {
		return new NameExprBuilder()
			.bL(1)
			.bC(1)
			.eL(10)
			.eC(120);
	}

	class NodeBuilder<T extends NodeBuilder> {
		protected int beginLine = 0;
		protected int beginColumn = 0;
		protected int endLine = 0;
		protected int endColumn = 0;

		public T bL(int line) {
			beginLine = line;
			return (T) this;
		}

		public T bC(int column) {
			beginColumn = column;
			return (T) this;
		}

		public T eL(int line) {
			endLine = line;
			return (T) this;
		}

		public T eC(int column) {
			endColumn = column;
			return (T) this;
		}

	}

	class NameExprBuilder extends NodeBuilder<NameExprBuilder> {
		private String name = "default";

		NameExprBuilder n(String name) {
			this.name = name;
			return this;
		}

		NameExpr build() {
			return new NameExpr(beginLine, beginColumn, endLine, endColumn, name);
		}
	}
}
