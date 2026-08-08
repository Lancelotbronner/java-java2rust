use javaparser_core::com::github::javaparser::Position;
use javaparser_core::com::github::javaparser::ast::Node;
use javaparser_core::com::github::javaparser::utils::Pair;
use java::util::ArrayList;
use java::util::HashMap;
use java::util::List;
use java::util::concurrent::atomic::AtomicInteger;
use java::util::stream::IntStream;

pub struct Block {
	block_count: /* Java */ java::util::concurrent::atomic::AtomicInteger /**/ = AtomicInteger::new(0),
	parent_block: java2rust::block::Block,
	children: /* Java */ java::util::List /**/ = ArrayList<>::new(),
	id: i32,
	n: com::github::javaparser::ast::node::Node,
	changes: /* Java */ java::util::HashMap /**/ = HashMap<>::new(),
	declarations: /* Java */ java::util::HashMap /**/ = HashMap<>::new(),
	usages: /* Java */ java::util::HashMap /**/ = HashMap<>::new(),
}

impl Block {
	pub static FICTIONAL_LINE_SIZE: i64 = 10000000;

	pub fn new(parent: &java2rust::block::Block, n: &com::github::javaparser::ast::node::Node) -> java2rust::block::Block {
		this(n);
		self.parent_block = parent;
		parent.children.add(self);
	}

	pub fn new(n: &com::github::javaparser::ast::node::Node) -> java2rust::block::Block {
		self.n = n;
		self.id = self.block_count.incrementAndGet();
	}

	pub fn add_change(&self, name: &/* Java */ java::lang::String /**/, node: &com::github::javaparser::ast::node::Node) {
		self.add(name, node, self.changes);
	}

	fn add(&self, name: &/* Java */ java::lang::String /**/, node: &com::github::javaparser::ast::node::Node, map: &/* Java */ java::util::HashMap /**/) {
		let value: List<Node> = map.get(name);
		if value == null {
			map.put(name, ArrayList<Node>::new() {
				{
					self.add(node);
				}});
		} else {
			value.add(node);
		}
	}

	pub fn add_usage(&self, name: &/* Java */ java::lang::String /**/, node: &com::github::javaparser::ast::node::Node) {
		self.add(name, node, self.usages);
	}

	pub fn add_declaration(&self, name: &/* Java */ java::lang::String /**/, description: &com::github::javaparser::utils::pair::Pair) /* thrown(java.lang.RuntimeException) */ {
		if self.declarations.get(name) != null {
			return Err(RuntimeException::new(&"expected declarations to be added only once: %s at %s, already in %s".formatted(description.b, &description.b.get_range(), &self.declarations.get(name).b.get_range())));
		}
		self.declarations.put(name, description);
	}

	pub fn get_id(&self) -> i32 {
		return self.id;
	}

	pub fn size(&self) -> i64 {
		let begin: Position = self.n.get_begin().orElse(Position::com::github::javaparser::position::Position::HOME);
		let end: Position = self.n.get_end().orElse(Position::com::github::javaparser::position::Position::HOME);
		return (end.line - begin.line) * self.FICTIONAL_LINE_SIZE + (end.column + self.FICTIONAL_LINE_SIZE - begin.column);
	}

	pub fn disjunct_children(&self) -> bool {
		return IntStream::range(0, &self.children.size()).filter(|i1|IntStream::range(i1 + 1, &self.children.size()).filter(|i2|self.children.get(i1).contains(&self.children.get(i2))).findAny().isPresent()).findAny().isEmpty();
	}

	fn contains(&self, b: &java2rust::block::Block) -> bool {
		return self.contains(b.n);
	}

	fn contains(&self, np: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError) */ -> bool {
		if self.n.get_range().isEmpty() || n_p.get_range().isEmpty() {
			return false;
		}
	
		return self.n.get_range().get().contains(&n_p.get_range().get())?;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "Block[id=%d,%s]".formatted(self.id, self.n);
	}
}