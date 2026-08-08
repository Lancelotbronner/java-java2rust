use crate::com::github::javaparser::HasParentNode;
use crate::com::github::javaparser::ast::observer::AstObserver;
use crate::com::github::javaparser::ast::observer::Observable;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::Visitable;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::InternalProperty;
use java::util;
use java::util::function::Consumer;
use java::util::function::Predicate;
use java::util::function::UnaryOperator;
use java::util::stream::Collector;
use java::util::stream::Collectors;
use java::util::stream::Stream;

pub struct NodeList<N: com::github::javaparser::ast::node::Node> {
	inner_list: /* Java */ java::util::List /**/ = ArrayList<>::new(0),
	parent_node: com::github::javaparser::ast::node::Node,
	observers: /* Java */ java::util::List /**/ = ArrayList<>::new(),
}

impl<N: com::github::javaparser::ast::node::Node> NodeList {
	pub fn new() -> com::github::javaparser::ast::node_list::NodeList {
		self.parent_node = null;
	}

	pub fn new(n: &/* Java */ java::util::Collection /**/) -> com::github::javaparser::ast::node_list::NodeList {
		self.add_all(n);
	}

	pub fn new(n: &N) -> com::github::javaparser::ast::node_list::NodeList {
		self.add_all(&Arrays::asList(n));
	}

	pub fn add(&self, node: &N) -> bool {
		self.notify_element_added(&self.inner_list.size(), node);
		self.own(node);
		return self.inner_list.add(node);
	}

	fn own(&self, node: &N) {
		if node == null {
			return;
		}
		self.set_as_parent_node_of(node);
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		let index: i32 = self.inner_list.indexOf(node);
		if index != -1 {
			self.notify_element_removed(index, node);
			node.set_parent_node(null);
		}
		return self.inner_list.remove(node);
	}

	pub fn remove_first(&self) -> N {
		return self.remove(0);
	}

	pub fn remove_last(&self) -> N {
		return self.remove(self.inner_list.size() - 1);
	}

	pub fn node_list<X: com::github::javaparser::ast::node::Node>(&self, nodes: &X) -> com::github::javaparser::ast::node_list::NodeList {
		/* final */ let node_list: NodeList<X> = NodeList<>::new();
		Collections::addAll(node_list, nodes);
		return node_list;
	}

	pub fn node_list<X: com::github::javaparser::ast::node::Node>(&self, nodes: &/* Java */ java::util::Collection /**/) -> com::github::javaparser::ast::node_list::NodeList {
		/* final */ let node_list: NodeList<X> = NodeList<>::new();
		node_list.add_all(nodes);
		return node_list;
	}

	pub fn node_list<X: com::github::javaparser::ast::node::Node>(&self, nodes: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::node_list::NodeList {
		/* final */ let node_list: NodeList<X> = NodeList<>::new();
		node_list.add_all(nodes);
		return node_list;
	}

	pub fn contains(&self, node: &N) -> bool {
		return self.inner_list.contains(node);
	}

	pub fn size(&self) -> i32 {
		return self.inner_list.size();
	}

	pub fn get(&self, i: i32) -> N {
		return self.inner_list.get(i);
	}

	pub fn iterator(&self) -> /* Java */ java::util::Iterator /**/ {
		// Custom iterator required, to ensure that the relevant `notifyElement...` methods are called.
		return NodeListIterator::new(self.inner_list);
	}

	pub fn set(&self, index: i32, element: &N) /* thrown(java.lang.IllegalArgumentException) */ -> N {
		if index < 0 || index >= self.inner_list.size() {
			return Err(IllegalArgumentException::new("Illegal index. The index should be between 0 and " + self.inner_list.size() + " excluded. It is instead " + index));
		}
		if element == self.inner_list.get(index) {
			return element;
		}
		self.notify_element_replaced(index, element);
		self.inner_list.get(index).set_parent_node(null);
		self.set_as_parent_node_of(element);
		return self.inner_list.set(index, element);
	}

	pub fn remove(&self, index: i32) -> N {
		self.notify_element_removed(index, &self.inner_list.get(index));
		let remove: N = self.inner_list.remove(index);
		if remove != null {
			remove.set_parent_node(null);
		}
	
		return remove;
	}

	pub fn is_empty(&self) -> bool {
		return self.inner_list.isEmpty();
	}

	pub fn sort(&self, comparator: &/* Java */ java::util::Comparator /**/) {
		self.inner_list.sort(comparator);
	}

	pub fn add_all(&self, other_list: &com::github::javaparser::ast::node_list::NodeList) {
		for node in other_list {
			self.add(node);
		}
	}

	pub fn add(&self, index: i32, node: &N) {
		self.notify_element_added(index, node);
		self.own(node);
		self.inner_list.add(index, node);
	}

	pub fn add_first(&self, node: &N) -> com::github::javaparser::ast::node_list::NodeList {
		self.add(0, node);
		return self;
	}

	pub fn add_last(&self, node: &N) -> com::github::javaparser::ast::node_list::NodeList {
		self.add(node);
		return self;
	}

	pub fn add_after(&self, node: &N, after_this_node: &N) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::node_list::NodeList {
		let i: i32 = self.index_of(after_this_node);
		if i == -1 {
			return Err(IllegalArgumentException::new("Can't find node to insert after."));
		}
		self.add(i + 1, node);
		return self;
	}

	pub fn add_before(&self, node: &N, before_this_node: &N) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::node_list::NodeList {
		let i: i32 = self.index_of(before_this_node);
		if i == -1 {
			return Err(IllegalArgumentException::new("Can't find node to insert before."));
		}
		self.add(i, node);
		return self;
	}

	pub fn get_first(&self) -> /* Java */ java::util::Optional /**/ {
		if self.is_empty() {
			return Optional::empty();
		}
		return Optional::of(&self.get(0));
	}

	pub fn get_last(&self) -> /* Java */ java::util::Optional /**/ {
		if self.is_empty() {
			return Optional::empty();
		}
		return Optional::of(&self.get(self.size() - 1));
	}

	pub fn get_parent_node(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.parent_node);
	}

	pub fn set_parent_node(&mut self, parent_node: &com::github::javaparser::ast::node::Node) -> com::github::javaparser::ast::node_list::NodeList {
		self.parent_node = parent_node;
		self.set_as_parent_node_of(self.inner_list);
		return self;
	}

	pub fn get_parent_node_for_children(&self) -> com::github::javaparser::ast::node::Node {
		return self.parent_node;
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn for_each(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		self.inner_list.forEach(action);
	}

	pub fn contains(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		return self.inner_list.contains(o);
	}

	pub fn to_array(&self) -> &[/* Java */ java::lang::Object /**/] {
		return self.inner_list.toArray();
	}

	pub fn to_array<T>(&self, a: &&[T]) -> &[T] {
		return self.inner_list.toArray(a);
	}

	pub fn remove(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if o instanceof Node {
			return self.remove(o as Node);
		}
		return false;
	}

	pub fn contains_all(&self, c: &/* Java */ java::util::Collection /**/) -> bool {
		return self.inner_list.containsAll(c);
	}

	pub fn add_all(&self, c: &/* Java */ java::util::Collection /**/) -> bool {
		c.forEach(self::add);
		return !c.isEmpty();
	}

	pub fn add_all(&self, index: i32, c: &/* Java */ java::util::Collection /**/) -> bool {
		for e in c {
			self.add(index += 1 !!!check!!! post increment, e);
		}
		return !c.isEmpty();
	}

	pub fn remove_all(&self, c: &/* Java */ java::util::Collection /**/) -> bool {
		let changed: bool = false;
		for e in c {
			changed = self.remove(e) || changed;
		}
		return changed;
	}

	pub fn retain_all(&self, c: &/* Java */ java::util::Collection /**/) -> bool {
		let changed: bool = false;
		for e in self.stream().filter(|it|!c.contains(it)).toArray() {
			if !c.contains(e) {
				changed = self.remove(e) || changed;
			}
		}
		return changed;
	}

	pub fn replace_all(&self, operator: &/* Java */ java::util::function::UnaryOperator /**/) {
		 {
			let i: i32 = 0;
			while i < self.size() {
				{
					.set(i, &operator.apply(&self.get(i)));
				}
				i += 1;
			 }
		 }
	
	}

	pub fn remove_if(&self, filter: &/* Java */ java::util::function::Predicate /**/) -> bool {
		let changed: bool = false;
		for e in self.stream().filter(filter).toArray() {
			changed = self.remove(e) || changed;
		}
		return changed;
	}

	pub fn clear(&self) {
		while !self.is_empty() {
			self.remove(0);
		}
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		return self.inner_list.equals(o);
	}

	pub fn hash_code(&self) -> i32 {
		return self.inner_list.hashCode();
	}

	pub fn index_of(&self, o: &/* Java */ java::lang::Object /**/) -> i32 {
		return self.inner_list.indexOf(o);
	}

	pub fn last_index_of(&self, o: &/* Java */ java::lang::Object /**/) -> i32 {
		return self.inner_list.lastIndexOf(o);
	}

	pub fn list_iterator(&self) -> /* Java */ java::util::ListIterator /**/ {
		// Custom iterator required, to ensure that the relevant `notifyElement...` methods are called.
		return NodeListIterator::new(self.inner_list);
	}

	pub fn list_iterator(&self, index: i32) -> /* Java */ java::util::ListIterator /**/ {
		// Custom iterator required, to ensure that the relevant `notifyElement...` methods are called.
		return NodeListIterator::new(self.inner_list, index);
	}

	pub fn parallel_stream(&self) -> /* Java */ java::util::stream::Stream /**/ {
		return self.inner_list.parallelStream();
	}

	pub fn sub_list(&self, from_index: i32, to_index: i32) -> /* Java */ java::util::List /**/ {
		return self.inner_list.subList(from_index, to_index);
	}

	pub fn spliterator(&self) -> /* Java */ java::util::Spliterator /**/ {
		return self.inner_list.spliterator();
	}

	fn notify_element_added(&self, index: i32, node_added_or_removed: &com::github::javaparser::ast::node::Node) {
		self.observers.forEach(|o|o.list_change(self, AstObserver::com::github::javaparser::ast::observer::ast_observer::ListChangeType::ADDITION, index, node_added_or_removed));
	}

	fn notify_element_removed(&self, index: i32, node_added_or_removed: &com::github::javaparser::ast::node::Node) {
		self.observers.forEach(|o|o.list_change(self, AstObserver::com::github::javaparser::ast::observer::ast_observer::ListChangeType::REMOVAL, index, node_added_or_removed));
	}

	fn notify_element_replaced(&self, index: i32, node_added_or_removed: &com::github::javaparser::ast::node::Node) {
		self.observers.forEach(|o|o.list_replacement(self, index, &self.get(index), node_added_or_removed));
	}

	pub fn unregister(&self, observer: &com::github::javaparser::ast::observer::ast_observer::AstObserver) {
		self.observers.remove(observer);
	}

	pub fn register(&self, observer: &com::github::javaparser::ast::observer::ast_observer::AstObserver) {
		if !self.observers.contains(observer) {
			self.observers.add(observer);
		}
	}

	pub fn is_registered(&self, observer: &com::github::javaparser::ast::observer::ast_observer::AstObserver) -> bool {
		return self.observers.contains(observer);
	}

	pub fn replace(&self, old: &N, replacement: &N) /* thrown(java.lang.IllegalArgumentException) */ -> bool {
		let i: i32 = self.index_of(old);
		if i == -1 {
			return false;
		}
		self.set(i, replacement)?;
		return true;
	}

	pub fn is_non_empty(&self) -> bool {
		return !self.is_empty();
	}

	pub fn if_non_empty(&self, consumer: &/* Java */ java::util::function::Consumer /**/) {
		if self.is_non_empty() {
			consumer.accept(self);
		}
	
	}

	pub fn to_node_list<T: com::github::javaparser::ast::node::Node>(&self) -> /* Java */ java::util::stream::Collector /**/ {
		return Collector::of(NodeList::new, NodeList::add, |(left, right)|{
			left.add_all(right);
			return left;
		});
	}

	fn set_as_parent_node_of(&self, child_nodes: &/* Java */ java::util::List /**/) {
		if child_nodes != null {
			for current in child_nodes {
				current.set_parent_node(&self.get_parent_node_for_children());
			}
		}
	}

	fn set_as_parent_node_of(&self, child_node: &com::github::javaparser::ast::node::Node) {
		if child_node != null {
			child_node.set_parent_node(&self.get_parent_node_for_children());
		}
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.inner_list.stream().map(Node::toString).collect(&Collectors::joining(", ", "[", "]"));
	}
}

impl<N: com::github::javaparser::ast::node::Node> /* Java */ java::util::List /**/ for NodeList<N> {}

impl<N: com::github::javaparser::ast::node::Node> /* Java */ java::util::SequencedCollection /**/ for NodeList<N> {}

impl<N: com::github::javaparser::ast::node::Node> /* Java */ java::util::Collection /**/ for NodeList<N> {}

impl<N: com::github::javaparser::ast::node::Node> /* Java */ java::lang::Iterable /**/ for NodeList<N> {}

impl<N: com::github::javaparser::ast::node::Node> /* Java */ java::lang::Iterable /**/ for NodeList<N> {}

impl<N: com::github::javaparser::ast::node::Node> com::github::javaparser::has_parent_node::HasParentNode for NodeList<N> {}

impl<N: com::github::javaparser::ast::node::Node> com::github::javaparser::ast::observer::observable::Observable for NodeList<N> {}

impl<N: com::github::javaparser::ast::node::Node> com::github::javaparser::ast::visitor::visitable::Visitable for NodeList<N> {}

impl<N: com::github::javaparser::ast::node::Node> com::github::javaparser::ast::observer::observable::Observable for NodeList<N> {}

struct NodeListIterator {
	iterator: /* Java */ java::util::ListIterator /**/,
	current: N = null,
}

impl NodeListIterator {
	pub fn new(list: &/* Java */ java::util::List /**/) -> com::github::javaparser::ast::node_list::NodeListIterator {
		self.iterator = list.listIterator();
	}

	pub fn new(list: &/* Java */ java::util::List /**/, index: i32) -> com::github::javaparser::ast::node_list::NodeListIterator {
		self.iterator = list.listIterator(index);
	}

	pub fn has_next(&self) -> bool {
		return self.iterator.hasNext();
	}

	pub fn next(&mut self) -> N {
		self.current = self.iterator.next();
		return self.current;
	}

	pub fn has_previous(&self) -> bool {
		return self.iterator.hasPrevious();
	}

	pub fn previous(&mut self) -> N {
		self.current = self.iterator.previous();
		return self.current;
	}

	pub fn next_index(&self) -> i32 {
		return self.iterator.nextIndex();
	}

	pub fn previous_index(&self) -> i32 {
		return self.iterator.previousIndex();
	}

	pub fn remove(&self) {
		let index: i32 = .indexOf(self.current);
		if index != -1 {
			self.notify_element_removed(index, self.current);
			self.current.set_parent_node(null);
		}
		self.iterator.remove();
	}

	pub fn set(&self, n: &N) /* thrown(java.lang.IllegalArgumentException) */ {
		let index: i32 = .indexOf(self.current);
		if index < 0 || index >= .size() {
			return Err(IllegalArgumentException::new("Illegal index. The index should be between 0 and " + .size() + " excluded. It is instead " + index));
		}
		if n != .get(index) {
			self.notify_element_replaced(index, n);
			.get(index).set_parent_node(null);
			self.set_as_parent_node_of(n);
			self.iterator.set(n);
		}
	}

	pub fn add(&self, n: &N) {
		self.notify_element_added(&.size(), n);
		self.own(n);
		self.iterator.add(n);
	}

	pub fn for_each_remaining(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		self.iterator.forEachRemaining(action);
	}
}

impl /* Java */ java::util::ListIterator /**/ for NodeListIterator {}

impl /* Java */ java::util::Iterator /**/ for NodeListIterator {}