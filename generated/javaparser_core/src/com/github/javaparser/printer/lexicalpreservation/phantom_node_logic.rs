use java::util::Collections::synchronizedMap;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::observer::AstObserver;
use crate::com::github::javaparser::ast::observer::AstObserverAdapter;
use crate::com::github::javaparser::ast::type::UnknownType;
use java::util::IdentityHashMap;
use java::util::Map;

pub struct PhantomNodeLogic;

impl PhantomNodeLogic {
	static LEVELS_TO_EXPLORE: i32 = 3;

	static isPhantomNodeCache: /* Java */ java::util::Map /**/ = /* Java */ java::util::Collections /**/::synchronizedMap(IdentityHashMap<>::new());

	static cacheCleaner: com::github::javaparser::ast::observer::ast_observer::AstObserver = AstObserverAdapter::new() {
		pub fn parent_change(&self, observed_node: &Node, previous_parent: &Node, new_parent: &Node) {
			is_phantom_node_cache.remove(observed_node);
		}
	
	};

	fn is_phantom_node(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if self.is_phantom_node_cache.containsKey(node) {
			return self.is_phantom_node_cache.get(node);
		}
		if node instanceof UnknownType {
			return true;
		}
		let res: bool = (node.get_parent_node().isPresent() && node.get_parent_node().get().has_range() && node.has_range() && !node.get_parent_node().get().get_range().get().contains(&node.get_range().get()) || com::github::javaparser::printer::lexicalpreservation::phantom_node_logic::PhantomNodeLogic::in_phantom_node(node, self.LEVELS_TO_EXPLORE));
		self.is_phantom_node_cache.put(node, res);
		node.register(self.cache_cleaner);
		return res;
	}

	fn in_phantom_node(&self, node: &com::github::javaparser::ast::node::Node, levels: i32) -> bool {
		return node.get_parent_node().isPresent() && (com::github::javaparser::printer::lexicalpreservation::phantom_node_logic::PhantomNodeLogic::is_phantom_node(&node.get_parent_node().get()) || com::github::javaparser::printer::lexicalpreservation::phantom_node_logic::PhantomNodeLogic::in_phantom_node(&node.get_parent_node().get(), levels - 1));
	}

	pub fn clean_up_cache(&self) {
		self.is_phantom_node_cache.clear();
	}
}