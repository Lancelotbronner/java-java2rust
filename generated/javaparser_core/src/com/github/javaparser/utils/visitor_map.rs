use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::Visitable;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use java::util::Collection;
use java::util::HashMap;
use java::util::Map;
use java::util::Set;
use java::util::stream::Collectors;

pub struct VisitorMap<N: com::github::javaparser::ast::node::Node, V> {
	inner_map: /* Java */ java::util::Map /**/ = HashMap<>::new(),
	hashcode_visitor: com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor,
	equals_visitor: com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor,
}

impl<N: com::github::javaparser::ast::node::Node, V> VisitorMap {
	pub fn new(hashcode_visitor: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, equals_visitor: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor) -> com::github::javaparser::utils::visitor_map::VisitorMap {
		self.hashcodeVisitor = hashcode_visitor;
		self.equalsVisitor = equals_visitor;
	}

	pub fn size(&self) -> i32 {
		return self.inner_map.size();
	}

	pub fn is_empty(&self) -> bool {
		return self.inner_map.isEmpty();
	}

	pub fn contains_key(&self, key: &/* Java */ java::lang::Object /**/) -> bool {
		return self.inner_map.containsKey(EqualsHashcodeOverridingFacade::new(key as N));
	}

	pub fn contains_value(&self, value: &/* Java */ java::lang::Object /**/) -> bool {
		return self.inner_map.containsValue(value);
	}

	pub fn get(&self, key: &/* Java */ java::lang::Object /**/) -> V {
		return self.inner_map.get(EqualsHashcodeOverridingFacade::new(key as N));
	}

	pub fn put(&self, key: &N, value: &V) -> V {
		return self.inner_map.put(EqualsHashcodeOverridingFacade::new(key), value);
	}

	pub fn remove(&self, key: &/* Java */ java::lang::Object /**/) -> V {
		return self.inner_map.remove(EqualsHashcodeOverridingFacade::new(key as N));
	}

	pub fn put_all(&self, m: &/* Java */ java::util::Map /**/) {
		m.forEach(self::put);
	}

	pub fn clear(&self) {
		self.inner_map.clear();
	}

	pub fn key_set(&self) -> /* Java */ java::util::Set /**/ {
		return self.inner_map.keySet().stream().map(|k|k.overridden).collect(&Collectors::toSet());
	}

	pub fn values(&self) -> /* Java */ java::util::Collection /**/ {
		return self.inner_map.values();
	}

	pub fn entry_set(&self) -> /* Java */ java::util::Set /**/ {
		return self.inner_map.entrySet().stream().map(|e|HashMap.SimpleEntry<>::new(e.getKey().overridden, &e.getValue())).collect(&Collectors::toSet());
	}
}

impl<N: com::github::javaparser::ast::node::Node, V> /* Java */ java::util::Map /**/ for VisitorMap<N, V> {}

struct EqualsHashcodeOverridingFacade {
	overridden: N,
}

impl EqualsHashcodeOverridingFacade {
	fn new(overridden: &N) -> com::github::javaparser::utils::visitor_map::EqualsHashcodeOverridingFacade {
		self.overridden = overridden;
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) /* thrown(java.lang.AssertionError) */ -> R {
		return Err(AssertionError::new());
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) /* thrown(java.lang.AssertionError) */ {
		return Err(AssertionError::new());
	}

	pub fn hash_code(&self) -> i32 {
		return self.overridden.accept(, null);
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		if obj == null || !(obj instanceof VisitorMap.EqualsHashcodeOverridingFacade) {
			return false;
		}
		return self.overridden.accept(, (obj as EqualsHashcodeOverridingFacade).overridden);
	}
}

impl com::github::javaparser::ast::visitor::visitable::Visitable for EqualsHashcodeOverridingFacade {}