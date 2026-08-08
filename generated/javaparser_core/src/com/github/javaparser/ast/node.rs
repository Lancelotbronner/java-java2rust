use crate::com::github::javaparser::ast::Node::Parsedness::PARSED;
use crate::com::github::javaparser::ast::Node::TreeTraversal::PREORDER;
use java::util::Collections::emptySet;
use java::util::Collections::unmodifiableList;
use java::util::Spliterator::DISTINCT;
use java::util::Spliterator::NONNULL;
use crate::com::github::javaparser::HasParentNode;
use crate::com::github::javaparser::Position;
use crate::com::github::javaparser::Range;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::comments::BlockComment;
use crate::com::github::javaparser::ast::comments::Comment;
use crate::com::github::javaparser::ast::comments::LineComment;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithOptionalScope;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithRange;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithScope;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithTokenRange;
use crate::com::github::javaparser::ast::observer::AstObserver;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::observer::PropagatingAstObserver;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::EqualsVisitor;
use crate::com::github::javaparser::ast::visitor::HashCodeVisitor;
use crate::com::github::javaparser::ast::visitor::Visitable;
use crate::com::github::javaparser::metamodel::InternalProperty;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::NodeMetaModel;
use crate::com::github::javaparser::metamodel::OptionalProperty;
use crate::com::github::javaparser::metamodel::PropertyMetaModel;
use crate::com::github::javaparser::printer::ConfigurablePrinter;
use crate::com::github::javaparser::printer::DefaultPrettyPrinter;
use crate::com::github::javaparser::printer::Printer;
use crate::com::github::javaparser::printer::configuration::DefaultConfigurationOption;
use crate::com::github::javaparser::printer::configuration::DefaultPrinterConfiguration;
use crate::com::github::javaparser::printer::configuration::DefaultPrinterConfiguration::ConfigOption;
use crate::com::github::javaparser::printer::configuration::PrinterConfiguration;
use crate::com::github::javaparser::resolution::SymbolResolver;
use crate::com::github::javaparser::utils::LineSeparator;
use java::util::ArrayList;
use java::util::Collections;
use java::util::Comparator;
use java::util::IdentityHashMap;
use java::util::Iterator;
use java::util::LinkedList;
use java::util::List;
use java::util::Optional;
use java::util::Queue;
use java::util::Set;
use java::util::Spliterators;
use java::util::Stack;
use java::util::function::Consumer;
use java::util::function::Function;
use java::util::function::Predicate;
use java::util::stream::Stream;
use java::util::stream::StreamSupport;

pub struct Node {
	range: com::github::javaparser::range::Range,
	token_range: com::github::javaparser::token_range::TokenRange,
	parent_node: com::github::javaparser::ast::node::Node,
	child_nodes: /* Java */ java::util::ArrayList /**/ = ArrayList<>::new(0),
	orphan_comments: /* Java */ java::util::ArrayList /**/ = ArrayList<>::new(0),
	data: /* Java */ java::util::IdentityHashMap /**/ = null,
	comment: com::github::javaparser::ast::comments::comment::Comment,
	observers: /* Java */ java::util::ArrayList /**/ = ArrayList<>::new(0),
	parsed: com::github::javaparser::ast::node::Parsedness = PARSED,
}

impl Node {
	pub static NODE_BY_BEGIN_POSITION: /* Java */ java::util::Comparator /**/ = |(a, b)|{
		if a.has_range() && b.has_range() {
			return a.get_range().get().begin.compare_to(b.get_range().get().begin);
		}
		if a.has_range() || b.has_range() {
			if a.has_range() {
				return 1;
			}
			return -1;
		}
		return 0;
	};

	static LEVELS_TO_EXPLORE: i32 = 3;

	static prettyPrinterNoCommentsConfiguration: com::github::javaparser::printer::configuration::printer_configuration::PrinterConfiguration = DefaultPrinterConfiguration::new().remove_option(DefaultConfigurationOption::new(ConfigOption::PRINT_COMMENTS));

	pub static ABSOLUTE_BEGIN_LINE: i32 = Position::ABSOLUTE_BEGIN_LINE;

	pub static ABSOLUTE_END_LINE: i32 = Position::ABSOLUTE_END_LINE;

	pub static SYMBOL_RESOLVER_KEY: com::github::javaparser::ast::data_key::DataKey = DataKey<SymbolResolver>::new() {
	};

	pub static LINE_SEPARATOR_KEY: com::github::javaparser::ast::data_key::DataKey = DataKey<LineSeparator>::new() {
	};

	pub static PRINTER_KEY: com::github::javaparser::ast::data_key::DataKey = DataKey<Printer>::new() {
	};

	static PHANTOM_KEY: com::github::javaparser::ast::data_key::DataKey = DataKey<Boolean>::new() {
	};

	fn new(token_range: &com::github::javaparser::token_range::TokenRange) -> com::github::javaparser::ast::node::Node {
		self.set_token_range(token_range);
	}

	fn custom_initialization(&self) {
	}

	fn get_printer(&self) -> com::github::javaparser::printer::printer::Printer {
		return self.find_compilation_unit().map(|c|c.get_printer()?).orElseGet(|()|self.create_default_printer());
	}

	fn get_printer(&self, configuration: &com::github::javaparser::printer::configuration::printer_configuration::PrinterConfiguration) -> com::github::javaparser::printer::printer::Printer {
		return self.find_compilation_unit().map(|c|c.get_printer(configuration)).orElseGet(|()|self.create_default_printer(configuration));
	}

	fn create_default_printer(&self) -> com::github::javaparser::printer::printer::Printer {
		return self.create_default_printer(&self.get_default_printer_configuration());
	}

	fn create_default_printer(&self, configuration: &com::github::javaparser::printer::configuration::printer_configuration::PrinterConfiguration) -> com::github::javaparser::printer::printer::Printer {
		return DefaultPrettyPrinter::new(configuration);
	}

	fn get_default_printer_configuration(&self) -> com::github::javaparser::printer::configuration::printer_configuration::PrinterConfiguration {
		return DefaultPrinterConfiguration::new();
	}

	pub fn get_comment(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.comment);
	}

	pub fn get_range(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.range);
	}

	pub fn get_token_range(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.token_range);
	}

	pub fn set_token_range(&mut self, token_range: &com::github::javaparser::token_range::TokenRange) -> com::github::javaparser::ast::node::Node {
		self.tokenRange = token_range;
		if token_range == null || !(token_range.get_begin().has_range() && token_range.get_end().has_range()) {
			self.range = null;
		} else {
			self.range = Range::new(token_range.get_begin().get_range().get().begin, token_range.get_end().get_range().get().end);
		}
		return self;
	}

	pub fn set_range(&mut self, range: &com::github::javaparser::range::Range) -> com::github::javaparser::ast::node::Node {
		if self.range == range {
			return self;
		}
		self.notify_property_change(ObservableProperty::RANGE, self.range, range);
		self.range = range;
		return self;
	}

	pub fn set_comment(&mut self, comment: &com::github::javaparser::ast::comments::comment::Comment) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::node::Node {
		if self.comment == comment {
			return self;
		}
		self.notify_property_change(ObservableProperty::COMMENT, self.comment, comment);
		if self.comment != null {
			self.comment.set_commented_node(null)?;
		}
		self.comment = comment;
		if comment != null {
			self.comment.set_commented_node(self)?;
		}
		return self;
	}

	pub fn set_line_comment(&self, comment: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::node::Node {
		return self.set_comment(LineComment::new(comment))?;
	}

	pub fn set_block_comment(&self, comment: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::node::Node {
		return self.set_comment(BlockComment::new(comment))?;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		let printer: Printer = self.get_printer();
		if self.contains_data(self.LINE_SEPARATOR_KEY) {
			let line_separator: LineSeparator = self.get_line_ending_style_or_default(LineSeparator::SYSTEM);
			if printer instanceof ConfigurablePrinter {
				let configurable_printer: ConfigurablePrinter = printer as ConfigurablePrinter;
				let config: PrinterConfiguration = configurable_printer.get_configuration();
				if config != null {
					config.add_option(DefaultConfigurationOption::new(ConfigOption::END_OF_LINE_CHARACTER, &line_separator.as_raw_string()));
					configurable_printer.set_configuration(config);
				}
			}
		}
		return printer.print(self);
	}

	pub fn to_string(&self, configuration: &com::github::javaparser::printer::configuration::printer_configuration::PrinterConfiguration) -> /* Java */ java::lang::String /**/ {
		let printer: Printer = self.get_printer();
		if !(printer instanceof ConfigurablePrinter) {
			return printer.print(self);
		}
		let configurable_printer: ConfigurablePrinter = printer as ConfigurablePrinter;
		// save the current configuration
		let previous_configuration: PrinterConfiguration = configurable_printer.get_configuration();
		// print with the new configuration
		let result: String = self.get_printer(configuration).print(self);
		// restore the previous printer configuration (issue 4163)
		configurable_printer.set_configuration(previous_configuration);
		return result;
	}

	pub fn hash_code(&self) -> i32 {
		return HashCodeVisitor::hash_code(self);
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		if !(obj instanceof Node) {
			return false;
		}
		return EqualsVisitor::equals(self, obj as Node);
	}

	pub fn get_parent_node(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.parent_node);
	}

	pub fn get_child_nodes(&self) -> /* Java */ java::util::List /**/ {
		return /* Java */ java::util::Collections /**/::unmodifiableList(self.child_nodes);
	}

	pub fn add_orphan_comment(&self, comment: &com::github::javaparser::ast::comments::comment::Comment) {
		self.notify_property_change(ObservableProperty::COMMENT, null, comment);
		self.orphan_comments.add(comment);
		comment.set_parent_node(self);
	}

	pub fn remove_orphan_comment(&self, comment: &com::github::javaparser::ast::comments::comment::Comment) -> bool {
		let removed: bool = self.orphan_comments.remove(comment);
		if removed {
			self.notify_property_change(ObservableProperty::COMMENT, comment, null);
			comment.set_parent_node(null);
			self.orphan_comments.trimToSize();
		}
		return removed;
	}

	pub fn get_orphan_comments(&self) -> /* Java */ java::util::List /**/ {
		return /* Java */ java::util::Collections /**/::unmodifiableList(self.orphan_comments);
	}

	pub fn get_all_contained_comments(&self) -> /* Java */ java::util::List /**/ {
		let comments: List<Comment> = LinkedList<>::new(self.orphan_comments);
		for child in self.get_child_nodes() {
			child.get_comment().ifPresent(comments::add);
			comments.addAll(&child.get_all_contained_comments());
		}
		return comments;
	}

	pub fn set_parent_node(&mut self, new_parent_node: &com::github::javaparser::ast::node::Node) -> com::github::javaparser::ast::node::Node {
		if new_parent_node == self.parent_node {
			return self;
		}
		self.observers.forEach(|o|o.parentChange(self, self.parent_node, new_parent_node));
		// remove from old parent, if any
		if self.parent_node != null {
			/* final */ let parent_child_nodes: ArrayList<Node> = self.parent_node.childNodes;
			 {
				let i: i32 = 0;
				while i < parent_child_nodes.size() {
					{
						if parent_child_nodes.get(i) == self {
							parent_child_nodes.remove(i);
						}
					}
					i += 1;
				 }
			 }
	
			parent_child_nodes.trimToSize();
		}
		self.parent_node = new_parent_node;
		// add to new parent, if any
		if self.parent_node != null {
			self.parent_node.childNodes.add(self);
		}
		return self;
	}

	fn set_as_parent_node_of(&self, child_node: &com::github::javaparser::ast::node::Node) {
		if child_node != null {
			child_node.set_parent_node(&self.get_parent_node_for_children());
		}
	}

	pub fn try_add_import_to_parent_compilation_unit(&self, clazz: &/* Java */ java::lang::Class /**/) {
		.findAncestor(CompilationUnit.class).ifPresent(|p|p.addImport(clazz));
	}

	pub fn get_child_nodes_by_type<N: com::github::javaparser::ast::node::Node>(&self, clazz: &/* Java */ java::lang::Class /**/) -> /* Java */ java::util::List /**/ {
		let nodes: List<N> = ArrayList<>::new();
		for child in self.get_child_nodes() {
			if clazz.isInstance(child) {
				nodes.add(&clazz.cast(child));
			}
			nodes.addAll(&child.get_child_nodes_by_type(clazz));
		}
		return nodes;
	}

	pub fn get_nodes_by_type<N: com::github::javaparser::ast::node::Node>(&self, clazz: &/* Java */ java::lang::Class /**/) -> /* Java */ java::util::List /**/ {
		return self.get_child_nodes_by_type(clazz);
	}

	pub fn get_data<M>(&self, key: &com::github::javaparser::ast::data_key::DataKey) /* thrown(java.lang.IllegalStateException) */ -> M {
		if self.data == null {
			return Err(IllegalStateException::new("No data of this type found. Use containsData to check for this first."));
		}
		let value: M = self.data.get(key) as M;
		if value == null {
			return Err(IllegalStateException::new("No data of this type found. Use containsData to check for this first."));
		}
		return value;
	}

	pub fn find_data<M>(&self, key: &com::github::javaparser::ast::data_key::DataKey) /* thrown(java.lang.IllegalStateException) */ -> /* Java */ java::util::Optional /**/ {
		if self.contains_data(key) {
			return Optional::of(&self.get_data(key)?);
		}
		return Optional::empty();
	}

	pub fn get_data_keys(&self) -> /* Java */ java::util::Set /**/ {
		if self.data == null {
			return /* Java */ java::util::Collections /**/::emptySet();
		}
		return self.data.keySet();
	}

	pub fn set_data<M>(&mut self, key: &com::github::javaparser::ast::data_key::DataKey, object: &M) {
		if self.data == null {
			self.data = IdentityHashMap<>::new();
		}
		self.data.put(key, object);
	}

	pub fn contains_data(&self, key: &com::github::javaparser::ast::data_key::DataKey) -> bool {
		if self.data == null {
			return false;
		}
		return self.data.containsKey(key);
	}

	pub fn remove_data(&self, key: &com::github::javaparser::ast::data_key::DataKey) {
		if self.data != null {
			self.data.remove(key);
		}
	}

	pub fn remove(&self) -> bool {
		if self.parent_node == null {
			return false;
		}
		return self.parent_node.remove(self);
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if self.parent_node == null {
			return false;
		}
		return self.parent_node.replace(self, node);
	}

	pub fn remove_forced(&self) {
		if !self.remove() {
			self.get_parent_node().ifPresent(Node::remove);
		}
	}

	pub fn get_parent_node_for_children(&self) -> com::github::javaparser::ast::node::Node {
		return self;
	}

	fn set_as_parent_node_of(&self, list: &com::github::javaparser::ast::node_list::NodeList) {
		if list != null {
			list.set_parent_node(&self.get_parent_node_for_children());
		}
	}

	pub fn notify_property_change<P>(&self, property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, old_value: &P, new_value: &P) {
		self.observers.forEach(|o|o.propertyChange(self, property, old_value, new_value));
	}

	pub fn unregister(&self, observer: &com::github::javaparser::ast::observer::ast_observer::AstObserver) {
		self.observers.remove(observer);
		self.observers.trimToSize();
	}

	pub fn register(&self, observer: &com::github::javaparser::ast::observer::ast_observer::AstObserver) {
		// In this case we use a List instead of Set to save on memory space.
		if !self.observers.contains(observer) {
			self.observers.add(observer);
		}
	}

	pub fn register(&self, observer: &com::github::javaparser::ast::observer::ast_observer::AstObserver, mode: &com::github::javaparser::ast::node::ObserverRegistrationMode) /* thrown(java.lang.UnsupportedOperationException | java.lang.IllegalArgumentException) */ {
		if mode == null {
			return Err(IllegalArgumentException::new("Mode should be not null"));
		}
		match mode {
			JUST_THIS_NODE =>  {
				self.register(observer);
				break;
			}
			THIS_NODE_AND_EXISTING_DESCENDANTS =>  {
				self.register_for_subtree(observer);
				break;
			}
			SELF_PROPAGATING =>  {
				self.register_for_subtree(&PropagatingAstObserver::transform_in_propagating_observer(observer));
				break;
			}
			_ =>  {
				return Err(UnsupportedOperationException::new("This mode is not supported: " + mode));
			}
		}
	}

	pub fn register_for_subtree(&self, observer: &com::github::javaparser::ast::observer::ast_observer::AstObserver) {
		self.register(observer);
		self.get_child_nodes().forEach(|c|c.register_for_subtree(observer));
		for property in self.get_meta_model().get_all_property_meta_models() {
			if property.is_node_list() {
				let node_list: NodeList<?> = property.get_value(self)? as NodeList<?>;
				if node_list != null {
					node_list.register(observer);
				}
	
			}
		}
	}

	pub fn is_registered(&self, observer: &com::github::javaparser::ast::observer::ast_observer::AstObserver) -> bool {
		return self.observers.contains(observer);
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		if self.comment != null {
			if node == self.comment {
				self.remove_comment();
				return true;
			}
		}
		return false;
	}

	pub fn remove_comment(&self) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::node::Node {
		return self.set_comment(null as Comment)?;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::node::Node {
		return self.accept(CloneVisitor::new(), null) as Node;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::node_meta_model::NodeMetaModel {
		return JavaParserMetaModel::nodeMetaModel;
	}

	pub fn get_parsed(&self) -> com::github::javaparser::ast::node::Parsedness {
		return self.parsed;
	}

	pub fn set_parsed(&mut self, parsed: &com::github::javaparser::ast::node::Parsedness) -> com::github::javaparser::ast::node::Node {
		self.parsed = parsed;
		return self;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		if self.comment != null {
			if node == self.comment {
				self.set_comment(replacement_node as Comment)?;
				return true;
			}
		}
		return false;
	}

	pub fn find_root_node(&self) -> com::github::javaparser::ast::node::Node {
		let n: Node = self;
		while n.get_parent_node().isPresent() {
			n = n.get_parent_node().get();
		}
		return n;
	}

	pub fn find_compilation_unit(&self) -> /* Java */ java::util::Optional /**/ {
		let root_node: Node = self.find_root_node();
		if root_node instanceof CompilationUnit {
			return Optional::of(root_node as CompilationUnit);
		}
		return Optional::empty();
	}

	pub fn get_line_ending_style_or_default(&self, default_line_separator: &com::github::javaparser::utils::line_separator::LineSeparator) -> com::github::javaparser::utils::line_separator::LineSeparator {
		if self.get_line_ending_style().is_standard_eol() {
			return self.get_line_ending_style();
		}
		return default_line_separator;
	}

	pub fn get_line_ending_style(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::utils::line_separator::LineSeparator {
		let current: Node = self;
		// First check this node
		if current.contains_data(Node::LINE_SEPARATOR_KEY) {
			let line_separator: LineSeparator = current.get_data(Node::LINE_SEPARATOR_KEY)?;
			return line_separator;
		}
		// Then check parent/ancestor nodes
		while current.get_parent_node().isPresent() {
			current = current.get_parent_node().get();
			if current.contains_data(Node::LINE_SEPARATOR_KEY) {
				return current.get_data(Node::LINE_SEPARATOR_KEY)?;
			}
		}
		// Default to the system line separator if it's not already set within the parsed node/code.
		return LineSeparator::SYSTEM;
	}

	pub fn get_symbol_resolver(&self) -> com::github::javaparser::resolution::symbol_resolver::SymbolResolver {
		return self.find_compilation_unit().map(|cu|{
			if cu.contains_data(self.SYMBOL_RESOLVER_KEY) {
				return cu.get_data(self.SYMBOL_RESOLVER_KEY)?;
			}
			return Err(IllegalStateException::new("Symbol resolution not configured: to configure consider setting a SymbolResolver in the ParserConfiguration"));
		}).orElseThrow(|()|IllegalStateException::new("The node is not inserted in a CompilationUnit"));
	}

	fn tree_iterator(&self, traversal: &com::github::javaparser::ast::node::TreeTraversal) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::util::Iterator /**/ {
		match traversal {
			BREADTHFIRST =>  {
				return BreadthFirstIterator::new(self);
			}
			POSTORDER =>  {
				return PostOrderIterator::new(self);
			}
			PREORDER =>  {
				return PreOrderIterator::new(self);
			}
			DIRECT_CHILDREN =>  {
				return DirectChildrenIterator::new(self);
			}
			PARENTS =>  {
				return ParentsVisitor::new(self);
			}
			_ =>  {
				return Err(IllegalArgumentException::new("Unknown traversal choice."));
			}
		}
	}

	fn tree_iterable(&self, traversal: &com::github::javaparser::ast::node::TreeTraversal) -> /* Java */ java::lang::Iterable /**/ {
		return |()|self.tree_iterator(traversal)?;
	}

	pub fn stream(&self, traversal: &com::github::javaparser::ast::node::TreeTraversal) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::util::stream::Stream /**/ {
		return StreamSupport::stream(&Spliterators::spliteratorUnknownSize(&self.tree_iterator(traversal)?,  | ), false);
	}

	pub fn stream(&self) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::util::stream::Stream /**/ {
		return StreamSupport::stream(&Spliterators::spliteratorUnknownSize(&self.tree_iterator(PREORDER)?,  | ), false);
	}

	pub fn walk(&self, traversal: &com::github::javaparser::ast::node::TreeTraversal, consumer: &/* Java */ java::util::function::Consumer /**/) {
		// Could be implemented as a call to the above walk method, but this is a little more efficient.
		for node in self.tree_iterable(traversal) {
			consumer.accept(node);
		}
	}

	pub fn walk(&self, consumer: &/* Java */ java::util::function::Consumer /**/) {
		self.walk(PREORDER, consumer);
	}

	pub fn walk<T: com::github::javaparser::ast::node::Node>(&self, node_type: &/* Java */ java::lang::Class /**/, consumer: &/* Java */ java::util::function::Consumer /**/) {
		self.walk(TreeTraversal::PREORDER, |node|{
			if node_type.isAssignableFrom(&node.getClass()) {
				consumer.accept(&node_type.cast(node));
			}
		});
	}

	pub fn find_all<T: com::github::javaparser::ast::node::Node>(&self, node_type: &/* Java */ java::lang::Class /**/) -> /* Java */ java::util::List /**/ {
		/* final */ let found: List<T> = ArrayList<>::new();
		self.walk(node_type, found::add);
		return found;
	}

	pub fn find_all<T: com::github::javaparser::ast::node::Node>(&self, node_type: &/* Java */ java::lang::Class /**/, traversal: &com::github::javaparser::ast::node::TreeTraversal) -> /* Java */ java::util::List /**/ {
		/* final */ let found: List<T> = ArrayList<>::new();
		self.walk(traversal, |node|{
			if node_type.isAssignableFrom(&node.getClass()) {
				found.add(&node_type.cast(node));
			}
		});
		return found;
	}

	pub fn find_all<T: com::github::javaparser::ast::node::Node>(&self, node_type: &/* Java */ java::lang::Class /**/, predicate: &/* Java */ java::util::function::Predicate /**/) -> /* Java */ java::util::List /**/ {
		/* final */ let found: List<T> = ArrayList<>::new();
		self.walk(node_type, |n|{
			if predicate.test(n) {
				found.add(n);
			}
	
		});
		return found;
	}

	pub fn find_first<T>(&self, traversal: &com::github::javaparser::ast::node::TreeTraversal, consumer: &/* Java */ java::util::function::Function /**/) -> /* Java */ java::util::Optional /**/ {
		for node in self.tree_iterable(traversal) {
			/* final */ let result: Optional<T> = consumer.apply(node);
			if result.isPresent() {
				return result;
			}
		}
		return Optional::empty();
	}

	pub fn find_first<N: com::github::javaparser::ast::node::Node>(&self, node_type: &/* Java */ java::lang::Class /**/) -> /* Java */ java::util::Optional /**/ {
		return self.find_first(TreeTraversal::PREORDER, |node|{
			if node_type.isAssignableFrom(&node.getClass()) {
				return Optional::of(&node_type.cast(node));
			}
			return Optional::empty();
		});
	}

	pub fn find_first<N: com::github::javaparser::ast::node::Node>(&self, node_type: &/* Java */ java::lang::Class /**/, predicate: &/* Java */ java::util::function::Predicate /**/) -> /* Java */ java::util::Optional /**/ {
		return self.find_first(TreeTraversal::PREORDER, |node|{
			if node_type.isAssignableFrom(&node.getClass()) {
				/* final */ let cast_node: N = node_type.cast(node);
				if predicate.test(cast_node) {
					return Optional::of(cast_node);
				}
			}
			return Optional::empty();
		});
	}

	pub fn find_by_range(&self, range: &com::github::javaparser::range::Range) -> /* Java */ java::util::Optional /**/ {
		if self.is_phantom() {
			return Optional::empty();
		}
		if !self.has_range() {
			return Optional::empty();
		}
		if !self.get_range().get().contains(range) {
			return Optional::empty();
		}
		for child in self.get_child_nodes() {
			let found: Optional<Node> = child.find_by_range(range);
			if found.isPresent() {
				return found;
			}
		}
		return Optional::of(self);
	}

	pub fn is_ancestor_of(&self, descendant: &com::github::javaparser::ast::node::Node) -> bool {
		return self != descendant && self.find_first(Node.class, |n|n == descendant).isPresent();
	}

	pub fn has_scope(&self) -> bool {
		return (NodeWithOptionalScope.class.isAssignableFrom(&self.getClass()) && (self as NodeWithOptionalScope).get_scope().isPresent()) || (NodeWithScope.class.isAssignableFrom(&self.getClass()) && (self as NodeWithScope).get_scope() != null);
	}

	pub fn is_phantom(&self) -> bool {
		return self.is_phantom(self);
	}

	fn is_phantom(&self, node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.IllegalStateException) */ -> bool {
		if !node.contains_data(self.PHANTOM_KEY) {
			let res: bool = (node.get_parent_node().isPresent() && node.get_parent_node().get().has_range() && node.has_range() && !node.get_parent_node().get().get_range().get().contains(&node.get_range().get()) || self.in_phantom_node(node, self.LEVELS_TO_EXPLORE));
			node.set_data(self.PHANTOM_KEY, res);
		}
		return node.get_data(self.PHANTOM_KEY)?;
	}

	fn in_phantom_node(&self, node: &com::github::javaparser::ast::node::Node, levels: i32) /* thrown(java.lang.IllegalStateException) */ -> bool {
		return node.get_parent_node().isPresent() && (self.is_phantom(&node.get_parent_node().get())? || self.in_phantom_node(&node.get_parent_node().get(), levels - 1)?);
	}
}

impl /* Java */ java::lang::Cloneable /**/ for Node {}

impl com::github::javaparser::has_parent_node::HasParentNode for Node {}

impl com::github::javaparser::ast::observer::observable::Observable for Node {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for Node {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for Node {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for Node {}

pub enum ObserverRegistrationMode;

pub enum Parsedness;

pub enum TreeTraversal;

pub struct BreadthFirstIterator {
	queue: /* Java */ java::util::Queue /**/ = LinkedList<>::new(),
}

impl BreadthFirstIterator {
	pub fn new(node: &com::github::javaparser::ast::node::Node) -> com::github::javaparser::ast::node::BreadthFirstIterator {
		self.queue.add(node);
	}

	pub fn has_next(&self) -> bool {
		return !self.queue.isEmpty();
	}

	pub fn next(&self) -> com::github::javaparser::ast::node::Node {
		let next: Node = self.queue.remove();
		self.queue.addAll(&next.get_child_nodes());
		return next;
	}
}

impl /* Java */ java::util::Iterator /**/ for BreadthFirstIterator {}

pub struct DirectChildrenIterator {
	children_iterator: /* Java */ java::util::Iterator /**/,
}

impl DirectChildrenIterator {
	pub fn new(node: &com::github::javaparser::ast::node::Node) -> com::github::javaparser::ast::node::DirectChildrenIterator {
		self.children_iterator = node.get_child_nodes().iterator();
	}

	pub fn has_next(&self) -> bool {
		return self.children_iterator.hasNext();
	}

	pub fn next(&self) -> com::github::javaparser::ast::node::Node {
		return self.children_iterator.next();
	}
}

impl /* Java */ java::util::Iterator /**/ for DirectChildrenIterator {}

pub struct ParentsVisitor {
	node: com::github::javaparser::ast::node::Node,
}

impl ParentsVisitor {
	pub fn new(node: &com::github::javaparser::ast::node::Node) -> com::github::javaparser::ast::node::ParentsVisitor {
		self.node = node;
	}

	pub fn has_next(&self) -> bool {
		return self.node.get_parent_node().isPresent();
	}

	pub fn next(&mut self) -> com::github::javaparser::ast::node::Node {
		self.node = self.node.get_parent_node().orElse(null);
		return self.node;
	}
}

impl /* Java */ java::util::Iterator /**/ for ParentsVisitor {}

pub struct PreOrderIterator {
	stack: /* Java */ java::util::Stack /**/ = Stack<>::new(),
}

impl PreOrderIterator {
	pub fn new(node: &com::github::javaparser::ast::node::Node) -> com::github::javaparser::ast::node::PreOrderIterator {
		self.stack.add(node);
	}

	pub fn has_next(&self) -> bool {
		return !self.stack.isEmpty();
	}

	pub fn next(&self) -> com::github::javaparser::ast::node::Node {
		let next: Node = self.stack.pop();
		let children: List<Node> = next.get_child_nodes();
		 {
			let i: i32 = children.size() - 1;
			while i >= 0 {
				{
					self.stack.add(&children.get(i));
				}
				i -= 1;
			 }
		 }
	
		return next;
	}
}

impl /* Java */ java::util::Iterator /**/ for PreOrderIterator {}

pub struct PostOrderIterator {
	stack: /* Java */ java::util::Stack /**/ = Stack<>::new(),
}

impl PostOrderIterator {
	pub fn new(root: &com::github::javaparser::ast::node::Node) -> com::github::javaparser::ast::node::PostOrderIterator {
		self.stack.push(Level::new(&Collections::singletonList(root)));
	}

	pub fn has_next(&self) -> bool {
		return !self.stack.empty();
	}

	pub fn next(&self) -> com::github::javaparser::ast::node::Node {
		while true {
			let state: Level = self.stack.peek();
			if state.is_current_expanded() {
				return self.get_next_and_cleanup_stack(state);
			}
			self.expand(state);
		}
	}

	fn get_next_and_cleanup_stack(&self, state: &com::github::javaparser::ast::node::Level) -> com::github::javaparser::ast::node::Node {
		let result: Node = state.get_current();
		state.go_to_next();
		self.cleanup_stack(state);
		return result;
	}

	fn cleanup_stack(&self, state: &com::github::javaparser::ast::node::Level) {
		if !state.done() {
			self.stack.pop();
		}
	}

	fn expand(&self, state: &com::github::javaparser::ast::node::Level) {
		let children: List<Node> = state.get_current().get_child_nodes();
		if !children.isEmpty() {
			self.stack.push(Level::new(children));
		}
		state.set_current_expanded();
	}
}

impl /* Java */ java::util::Iterator /**/ for PostOrderIterator {}

struct Level {
	nodes: /* Java */ java::util::List /**/,
	index: i32 = 0,
	expanded: bool = false,
}

impl Level {
	pub fn new(nodes: &/* Java */ java::util::List /**/) -> com::github::javaparser::ast::node::Level {
		self.nodes = nodes;
	}

	pub fn done(&self) -> bool {
		return self.index < self.nodes.size();
	}

	pub fn get_current(&self) -> com::github::javaparser::ast::node::Node {
		return self.nodes.get(self.index);
	}

	pub fn go_to_next(&mut self) {
		self.index += 1;
		self.expanded = false;
	}

	pub fn set_current_expanded(&mut self) {
		self.expanded = true;
	}

	pub fn is_current_expanded(&self) -> bool {
		return self.expanded;
	}
}