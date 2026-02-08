package java2rust;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.stream.IntStream;

import com.github.javaparser.Position;
import com.github.javaparser.ast.Node;
import com.github.javaparser.utils.Pair;

/**
 * Created by aschoerk on 16.05.16.
 */
public class Block {
	public static final long FICTIONAL_LINE_SIZE = 10000000L;
	AtomicInteger blockCount = new AtomicInteger(0);
	Block parentBlock;
	List<Block> children = new ArrayList<>();
	int id;
	Node n;
	HashMap<String, List<Node>> changes = new HashMap<>();
	HashMap<String, Pair<TypeDescription, Node>> declarations = new HashMap<>();
	HashMap<String, List<Node>> usages = new HashMap<>();


	public Block(IdTracker idTracker, Node n) {
		if (!idTracker.currentBlocks.empty()) {
			this.parentBlock = idTracker.currentBlocks.peek();
			this.parentBlock.children.add(this);
		}
		this.n = n;
		this.id = blockCount.incrementAndGet();
	}

	public Block(Block parent, Node n) {
		this(n);
		this.parentBlock = parent;
		parent.children.add(this);
	}

	public Block(Node n) {
		this.n = n;
		this.id = blockCount.incrementAndGet();
	}

	public void addChange(String name, Node node) {
		add(name, node, changes);
	}

	private void add(String name, Node node, HashMap<String, List<Node>> map) {
		List<Node> value = map.get(name);
		if (value == null) {
			map.put(name, new ArrayList<Node>() {{add(node);}});
		} else {
			value.add(node);
		}
	}

	public void addUsage(String name, Node node) {
		add(name, node, usages);
	}

	public void addDeclaration(String name, Pair<TypeDescription, Node> description) {
		if (declarations.get(name) != null) {
			throw new RuntimeException(
				"expected declarations to be added only once: %s at %s, already in %s".formatted(
					description.b,
					description.b.getRange(),
					declarations.get(name).b.getRange()));
		}
		declarations.put(name, description);
	}

	public int getId() {
		return id;
	}

	public long size() {
		Position begin = n
			.getBegin()
			.orElse(Position.HOME);
		Position end = n
			.getEnd()
			.orElse(Position.HOME);
		return (end.line - begin.line) * FICTIONAL_LINE_SIZE + (end.column + FICTIONAL_LINE_SIZE - begin.column);
	}

	public boolean disjunctChildren() {
		return IntStream.range(0, children.size())
			// find blocks whose children overlap.
			.filter(i1 ->
				// following children overlap with this one ??
				IntStream
					.range(i1 + 1, children.size())
					.filter(i2 -> children
						.get(i1)
						.contains(children.get(i2)))
					.findAny()
					.isPresent())
			.findAny()
			.isEmpty();
	}

	boolean contains(Block b) {
		return contains(b.n);
	}

	boolean contains(Node nP) {
		if (n
			.getRange()
			.isEmpty() || nP
			.getRange()
			.isEmpty())
			return false;
		return n
			.getRange()
			.get()
			.contains(nP
				.getRange()
				.get());
	}

	@Override
	public String toString() {
		return "Block[id=%d,%s]".formatted(id, n);
	}

}
