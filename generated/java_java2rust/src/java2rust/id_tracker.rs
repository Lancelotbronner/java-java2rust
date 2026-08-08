use javaparser_core::com::github::javaparser::ast::Node;
use javaparser_core::com::github::javaparser::utils::Pair;
use commons_lang3::org::jspecify::annotations::Nullable;
use java::util;
use java::util::function::Function;

pub struct IdTracker {
	has_throws: /* Java */ java::util::Set /**/ = HashSet<>::new(),
	types: /* Java */ java::util::IdentityHashMap /**/ = IdentityHashMap<>::new(),
	try_count: i32,
	package_name: /* Java */ java::lang::String /**/ = null,
	current_method: /* Java */ java::lang::String /**/ = null,
	imports: /* Java */ java::util::List /**/ = ArrayList<>::new(),
	blocks: /* Java */ java::util::List /**/ = ArrayList<>::new(),
	current_blocks: /* Java */ java::util::Stack /**/ = Stack<>::new(),
	in_constructor: bool = false,
}

impl IdTracker {
	pub fn has_throws(&self, name: &/* Java */ java::lang::String /**/) -> bool {
		return self.has_throws.contains(name);
	}

	pub fn has_throws(&self) -> bool {
		return self.current_method != null && self.has_throws.contains(self.current_method);
	}

	pub fn set_current_method(&mut self, name: &/* Java */ java::lang::String /**/) {
		self.currentMethod = name;
	}

	pub fn get_package_name(&self) -> /* Java */ java::lang::String /**/ {
		return self.package_name;
	}

	pub fn set_package_name(&mut self, package_name: &/* Java */ java::lang::String /**/) {
		self.packageName = package_name;
	}

	pub fn add_import(&self, i: &java2rust::import::Import) {
		self.imports.add(i);
	}

	pub fn get_imports(&self) -> /* Java */ java::util::List /**/ {
		return self.imports;
	}

	pub fn set_in_constructor(&mut self, in_constructor: bool) {
		self.inConstructor = in_constructor;
	}

	pub fn is_outside_constructor(&self) -> bool {
		return !self.in_constructor;
	}

	fn add_change(&self, name: &/* Java */ java::lang::String /**/, n: &com::github::javaparser::ast::node::Node) {
		if !self.current_blocks.empty() {
			self.current_blocks.peek().add_change(name, n);
		}
	
	}

	fn add_declaration(&self, name: &/* Java */ java::lang::String /**/, description: &com::github::javaparser::utils::pair::Pair) /* thrown(java.lang.RuntimeException) */ {
		if !self.current_blocks.empty() {
			self.current_blocks.peek().add_declaration(name, description)?;
		}
	
	}

	fn add_usage(&self, name: &/* Java */ java::lang::String /**/, n: &com::github::javaparser::ast::node::Node) {
		if !self.current_blocks.empty() {
			self.current_blocks.peek().add_usage(name, n);
		}
	
	}

	fn push_block(&self, n: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError) */ {
		let block: Block;
		if !self.current_blocks.isEmpty() {
			let parent: Block = self.current_blocks.peek();
			if !parent.contains(n) {
				return Err(AssertionError::new());
			}
	
			block = Block::new(parent, n);
		} else {
			block = Block::new(n);
		}
		self.current_blocks.push(block);
		self.blocks.add(block);
	}

	fn pop_block(&self) {
		self.current_blocks.pop();
		if self.current_blocks.isEmpty() {
			self.check_block_structure();
		}
	}

	fn check_block_structure(&self) -> /* Java */ java::lang::String /**/ {
		let sb: StringBuilder = StringBuilder::new();
		if !self.current_blocks.isEmpty() {
			sb.append("Blockstack is not empty\n");
		}
		let root: Optional<Block> = self.find_root();
		if root.isEmpty() {
			sb.append("No Blockroot descernable\n");
		} else {
			if root.get().get_id() != 1 {
				sb.append("Expected Blockroot to have Id 1\n");
			}
		}
		if self.blocks.stream().anyMatch(|b|!b.disjunct_children()) {
			sb.append("Found children which are not disjunct\n");
		}
		return sb.toString();
	}

	fn find_root(&self) -> /* Java */ java::util::Optional /**/ {
		return self.blocks.stream().min(|(block1, block2)|Long::compare(&block2.size(), &block1.size()));
	}

	fn will_be_changed(&self, name: &/* Java */ java::lang::String /**/, n: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.RuntimeException) */ -> bool {
		return Err(RuntimeException::new("not implemented"));
	}

	fn is_local_to(&self, name: &/* Java */ java::lang::String /**/, n: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.RuntimeException) */ -> bool {
		return Err(RuntimeException::new("not implemented"));
	}

	fn find_declaration_node_for(&self, name: &/* Java */ java::lang::String /**/, n: &com::github::javaparser::ast::node::Node) -> /* Java */ java::util::Optional /**/ {
		let block: Optional<Block> = self.find_inner_most_block(n);
		loop { {
			if block.isPresent() {
				/* final */ let b: Block = block.get();
				let descr: Pair<TypeDescription, Node> = b.declarations.get(name);
				if descr == null {
					block =  if b.parent_block == null { Optional::empty() } else { Optional::of(b.parent_block) };
				} else {
					return Optional::of(descr);
				}
			} else {
				return Optional::empty();
			}
		}if !(true) break;}
	}

	fn find_inner_most_block(&self, n: &com::github::javaparser::ast::node::Node) -> /* Java */ java::util::Optional /**/ {
		return self.blocks.stream().filter(|block|block.contains(n)).min(|(block1, block2)|Long::compare(&block1.size(), &block2.size()));
	}

	pub fn get_usages(&self) -> /* Java */ java::util::Map /**/ {
		return self.get_all(|b|b.usages);
	}

	fn get_all(&self, f: &/* Java */ java::util::function::Function /**/) -> /* Java */ java::util::HashMap /**/ {
		/* final */ let res: HashMap<String, List<Node>> = HashMap<>::new();
		self.blocks.stream().map(f).forEach(|u|u.keySet().forEach(|k|{
			if res.containsKey(k) {
				res.get(k).addAll(&u.get(k));
			} else {
				res.put(k, &u.get(k));
			}
		}));
		return res;
	}

	pub fn get_changes(&self) -> /* Java */ java::util::Map /**/ {
		return self.get_all(|b|b.changes);
	}

	fn is_changed_in_single_block(&self, name: &/* Java */ java::lang::String /**/, b: &java2rust::block::Block) -> bool {
		return b.changes.get(name) != null;
	}

	fn is_declared_in_single_block(&self, name: &/* Java */ java::lang::String /**/, b: &java2rust::block::Block) -> bool {
		return b.declarations.get(name) != null;
	}

	fn is_changed_in_children_of_block(&self, name: &/* Java */ java::lang::String /**/, bp: &java2rust::block::Block) -> bool {
		return b_p.children.stream().anyMatch(|child|!self.is_declared_in_single_block(name, child) && (self.is_changed_in_single_block(name, child) || self.is_changed_in_children_of_block(name, child)));
	}

	pub fn is_changed(&self, name: &/* Java */ java::lang::String /**/, n: &com::github::javaparser::ast::node::Node) -> bool {
		let b: Optional<Block> = self.find_inner_most_block(n);
		return b.filter(|block|self.is_changed_in_single_block(name, block) || self.is_changed_in_children_of_block(name, block)).isPresent();
	}

	pub fn get_declarations(&self) -> /* Java */ java::util::Map /**/ {
		/* final */ let res: HashMap<String, List<Node>> = HashMap<>::new();
		self.blocks.stream().map(|b|b.declarations).forEach(|u|u.keySet().stream().forEach(|k|{
			if res.containsKey(k) {
				res.get(k).add(u.get(k).b);
			} else {
				res.put(k, ArrayList<>::new(&Collections::singletonList(u.get(k).b)));
			}
		}));
		return res;
	}

	pub fn set_has_throws(&self, name: &/* Java */ java::lang::String /**/) {
		self.hasThrows.add(name);
	}

	pub fn put_type(&self, n: &com::github::javaparser::ast::node::Node, clazz: &/* Java */ java::lang::Class /**/) {
		let existing: Class = self.get_type(n);
		if existing == null {
			self.types.put(n, clazz);
		}
		else {
			if clazz.isPrimitive() {
				if self.is_discrete(existing) && self.is_float(clazz) {
					// propagate discrete to float
					self.types.put(n, clazz);
				}
			}
		}
	}

	pub fn get_type(&self, n: &com::github::javaparser::ast::node::Node) -> /* Java */ java::lang::Class /**/ {
		return self.types.get(n);
	}

	pub fn is_discrete(&self, clazz: &/* Java */ java::lang::Class /**/) -> bool {
		if clazz == null {
			return false;
		}
	
		return clazz.equals(Integer::TYPE) || clazz.equals(Long::TYPE) || clazz.equals(Byte::TYPE) || clazz.equals(Short::TYPE) || clazz.equals(Integer.class) || clazz.equals(Long.class) || clazz.equals(Byte.class) || clazz.equals(Short.class);
	}

	pub fn is_float(&self, clazz: &/* Java */ java::lang::Class /**/) -> bool {
		if clazz == null {
			return false;
		}
	
		return clazz.equals(Float::TYPE) || clazz.equals(Double::TYPE) || clazz.equals(Float.class) || clazz.equals(Double.class) || clazz.getTypeName().equals("float") || clazz.getTypeName().equals("double");
	}

	pub fn is_discrete(&self, n: &com::github::javaparser::ast::node::Node) -> bool {
		if n == null {
			return false;
		}
	
		return self.is_discrete(&self.get_type(n));
	}

	pub fn is_float(&self, n: &com::github::javaparser::ast::node::Node) -> bool {
		if n == null {
			return false;
		}
	
		return self.is_float(&self.get_type(n));
	}
}