package java2rust;

import com.github.javaparser.ast.Node;
import com.github.javaparser.utils.Pair;
import org.jspecify.annotations.Nullable;

import java.util.*;
import java.util.function.Function;

/**
 * Created by aschoerk on 03.05.16.
 */
public class IdTracker {

	private final Set<String> hasThrows = new HashSet<>();
	public IdentityHashMap<Node, Class> types = new IdentityHashMap<>();
	int tryCount;
	String packageName = null;
	String currentMethod = null;
	List<Import> imports = new ArrayList<>();
	List<Block> blocks = new ArrayList<>();
	Stack<Block> currentBlocks = new Stack<>();
	boolean inConstructor = false;

	public boolean hasThrows(String name) {
		return hasThrows.contains(name);
	}

	public boolean hasThrows() {
		return currentMethod != null && hasThrows.contains(currentMethod);
	}

	public void setCurrentMethod(String name) {
		this.currentMethod = name;
	}

	public String getPackageName() {
		return packageName;
	}

	public void setPackageName(final String packageName) {
		this.packageName = packageName;
	}

	public void addImport(Import i) {
		imports.add(i);
	}

	public List<Import> getImports() {
		return imports;
	}

	public void setInConstructor(final boolean inConstructor) {
		this.inConstructor = inConstructor;
	}

	public boolean isOutsideConstructor() {
		return !inConstructor;
	}

	void addChange(String name, Node n) {
		if (!currentBlocks.empty())
			currentBlocks
				.peek()
				.addChange(name, n);
	}

	void addDeclaration(String name, Pair<TypeDescription, Node> description) {
		if (!currentBlocks.empty())
			currentBlocks
				.peek()
				.addDeclaration(name, description);
	}

	void addUsage(String name, Node n) {
		if (!currentBlocks.empty())
			currentBlocks
				.peek()
				.addUsage(name, n);
	}

	void pushBlock(Node n) {
		Block block;
		if (!currentBlocks.isEmpty()) {
			Block parent = currentBlocks.peek();
			if (!parent.contains(n))
				throw new AssertionError();
			block = new Block(parent, n);
		} else {
			block = new Block(n);
		}
		currentBlocks.push(block);
		blocks.add(block);
	}

	void popBlock() {
		currentBlocks.pop();
		if (currentBlocks.isEmpty()) {
			checkBlockStructure();
		}
	}

	String checkBlockStructure() {
		StringBuilder sb = new StringBuilder();
		if (!currentBlocks.isEmpty()) {
			sb.append("Blockstack is not empty\n");
		}
		Optional<Block> root = findRoot();
		if (root.isEmpty()) {
			sb.append("No Blockroot descernable\n");
		} else {
			if (root
				.get()
				.getId() != 1) {
				sb.append("Expected Blockroot to have Id 1\n");
			}
		}
		if (blocks
			.stream()
			.anyMatch(b -> !b.disjunctChildren())) {
			sb.append("Found children which are not disjunct\n");
		}
		return sb.toString();
	}

	Optional<Block> findRoot() {
		return blocks
			.stream()
			.min((block1, block2) -> Long.compare(block2.size(), block1.size()));
	}

	boolean willBeChanged(String name, Node n) {


		throw new RuntimeException("not implemented");
	}

	boolean isLocalTo(String name, Node n) {

		throw new RuntimeException("not implemented");
	}

	Optional<Pair<@Nullable TypeDescription, Node>> findDeclarationNodeFor(String name, Node n) {
		Optional<Block> block = findInnerMostBlock(n);
		do {
			if (block.isPresent()) {
				final Block b = block.get();
				Pair<TypeDescription, Node> descr = b.declarations.get(name);
				if (descr == null) {
					block = b.parentBlock == null ? Optional.empty() : Optional.of(b.parentBlock);
				} else {
					return Optional.of(descr);
				}
			} else {
				return Optional.empty();
			}
		}
		while (true);
	}

	Optional<Block> findInnerMostBlock(Node n) {
		return blocks
			.stream()
			.filter(block -> block.contains(n))
			.min((block1, block2) -> Long.compare(block1.size(), block2.size()));
	}

	public Map<String, List<Node>> getUsages() {
		return getAll(b -> b.usages);
	}

	private HashMap<String, List<Node>> getAll(Function<Block, HashMap<String, List<Node>>> f) {
		final HashMap<String, List<Node>> res = new HashMap<>();
		blocks
			.stream()
			.map(f)
			.forEach(u -> u
				.keySet()
				.forEach(k -> {
					if (res.containsKey(k)) {
						res
							.get(k)
							.addAll(u.get(k));
					} else {
						res.put(k, u.get(k));
					}
				}));
		return res;
	}

	public Map<String, List<Node>> getChanges() {
		return getAll(b -> b.changes);
	}

	private boolean isChangedInSingleBlock(String name, Block b) {
		return b.changes.get(name) != null;
	}

	private boolean isDeclaredInSingleBlock(String name, Block b) {
		return b.declarations.get(name) != null;
	}

	private boolean isChangedInChildrenOfBlock(String name, Block bP) {
		return bP.children
			.stream()
			.anyMatch(child -> !isDeclaredInSingleBlock(
				name,
				child) && (isChangedInSingleBlock(name, child) || isChangedInChildrenOfBlock(
				name,
				child)));
	}

	public boolean isChanged(String name, Node n) {
		Optional<Block> b = findInnerMostBlock(n);
		return b
			.filter(block -> isChangedInSingleBlock(name, block) || isChangedInChildrenOfBlock(
				name,
				block))
			.isPresent();
	}

	public Map<String, List<Node>> getDeclarations() {
		final HashMap<String, List<Node>> res = new HashMap<>();
		blocks
			.stream()
			.map(b -> b.declarations)
			.forEach(u -> u
				.keySet()
				.stream()
				.forEach(k -> {
					if (res.containsKey(k)) {
						res
							.get(k)
							.add(u.get(k).b);
					} else {
						res.put(k, new ArrayList<>(Collections.singletonList(u.get(k).b)));
					}
				}));
		return res;
	}

	public void setHasThrows(String name) {
		this.hasThrows.add(name);
	}

	public void putType(Node n, Class clazz) {
		Class existing = getType(n);
		if (existing == null)
			types.put(n, clazz);
		else {
			if (clazz.isPrimitive()) {
				if (isDiscrete(existing) && isFloat(clazz)) {  // propagate discrete to float
					types.put(n, clazz);
				}
			}
		}
	}

	public Class getType(Node n) {
		return types.get(n);
	}

	public boolean isDiscrete(final Class clazz) {
		if (clazz == null)
			return false;
		return clazz.equals(Integer.TYPE) || clazz.equals(Long.TYPE) || clazz.equals(Byte.TYPE) || clazz.equals(
			Short.TYPE) || clazz.equals(Integer.class) || clazz.equals(Long.class) || clazz.equals(
			Byte.class) || clazz.equals(Short.class);
	}

	public boolean isFloat(final Class clazz) {
		if (clazz == null)
			return false;
		return clazz.equals(Float.TYPE) || clazz.equals(Double.TYPE) || clazz.equals(Float.class) || clazz.equals(
			Double.class) || clazz
			.getTypeName()
			.equals("float") || clazz
			.getTypeName()
			.equals("double");
	}

	public boolean isDiscrete(final Node n) {
		if (n == null)
			return false;
		return isDiscrete(getType(n));
	}

	public boolean isFloat(final Node n) {
		if (n == null)
			return false;
		return isFloat(getType(n));
	}

}
