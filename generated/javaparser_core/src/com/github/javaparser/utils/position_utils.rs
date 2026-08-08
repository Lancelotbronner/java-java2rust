use java::lang::Integer::signum;
use crate::com::github::javaparser::Position;
use crate::com::github::javaparser::Range;
use crate::com::github::javaparser::ast::Modifier;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::body::ClassOrInterfaceDeclaration;
use crate::com::github::javaparser::ast::body::FieldDeclaration;
use crate::com::github::javaparser::ast::body::MethodDeclaration;
use crate::com::github::javaparser::ast::expr::AnnotationExpr;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithAnnotations;
use java::util::Comparator;
use java::util::List;

pub struct PositionUtils;

impl PositionUtils {
	fn new() -> com::github::javaparser::utils::position_utils::PositionUtils {
	// prevent instantiation
	}

	pub fn sort_by_begin_position<T: com::github::javaparser::ast::node::Node>(&self, nodes: &/* Java */ java::util::List /**/) {
		com::github::javaparser::utils::position_utils::PositionUtils::sort_by_begin_position(nodes, false);
	}

	pub fn sort_by_begin_position<T: com::github::javaparser::ast::node::Node>(&self, nodes: &com::github::javaparser::ast::node_list::NodeList) {
		com::github::javaparser::utils::position_utils::PositionUtils::sort_by_begin_position(nodes, false);
	}

	pub fn sort_by_begin_position<T: com::github::javaparser::ast::node::Node>(&self, nodes: &/* Java */ java::util::List /**/, ignoring_annotations: bool) {
		nodes.sort(|(o1, o2)|PositionUtils::compare(o1, o2, ignoring_annotations));
	}

	pub fn are_in_order(&self, a: &com::github::javaparser::ast::node::Node, b: &com::github::javaparser::ast::node::Node) -> bool {
		return com::github::javaparser::utils::position_utils::PositionUtils::are_in_order(a, b, false);
	}

	pub fn are_in_order(&self, a: &com::github::javaparser::ast::node::Node, b: &com::github::javaparser::ast::node::Node, ignoring_annotations: bool) -> bool {
		return com::github::javaparser::utils::position_utils::PositionUtils::compare(a, b, ignoring_annotations) <= 0;
	}

	fn compare(&self, a: &com::github::javaparser::ast::node::Node, b: &com::github::javaparser::ast::node::Node, ignoring_annotations: bool) -> i32 {
		if a.has_range() && !b.has_range() {
			return -1;
		}
		if !a.has_range() && b.has_range() {
			return 1;
		}
		if !a.has_range() && !b.has_range() {
			return 0;
		}
		if ignoring_annotations {
			let sign_line: i32 = /* Java */ java::lang::Integer /**/::signum(com::github::javaparser::utils::position_utils::PositionUtils::begin_line_without_considering_annotation(a) - com::github::javaparser::utils::position_utils::PositionUtils::begin_line_without_considering_annotation(b));
			if sign_line == 0 {
				return /* Java */ java::lang::Integer /**/::signum(com::github::javaparser::utils::position_utils::PositionUtils::begin_column_without_considering_annotation(a) - com::github::javaparser::utils::position_utils::PositionUtils::begin_column_without_considering_annotation(b));
			}
			return sign_line;
		}
		let a_begin: Position = a.get_begin().get();
		let b_begin: Position = b.get_begin().get();
		let sign_line: i32 = /* Java */ java::lang::Integer /**/::signum(a_begin.line - b_begin.line);
		if sign_line == 0 {
			return /* Java */ java::lang::Integer /**/::signum(a_begin.column - b_begin.column);
		}
		return sign_line;
	}

	pub fn get_last_annotation(&self, node: &com::github::javaparser::ast::node::Node) -> com::github::javaparser::ast::expr::annotation_expr::AnnotationExpr {
		if node instanceof NodeWithAnnotations {
			let annotations: NodeList<AnnotationExpr> = NodeList::node_list(&(node as NodeWithAnnotations<?>).get_annotations());
			if annotations.is_empty() {
				return null;
			}
			com::github::javaparser::utils::position_utils::PositionUtils::sort_by_begin_position(annotations);
			return annotations.get(annotations.size() - 1);
		}
		return null;
	}

	fn begin_line_without_considering_annotation(&self, node: &com::github::javaparser::ast::node::Node) -> i32 {
		return com::github::javaparser::utils::position_utils::PositionUtils::first_non_annotation_node(node).get_range().get().begin.line;
	}

	fn begin_column_without_considering_annotation(&self, node: &com::github::javaparser::ast::node::Node) -> i32 {
		return com::github::javaparser::utils::position_utils::PositionUtils::first_non_annotation_node(node).get_range().get().begin.column;
	}

	fn first_non_annotation_node(&self, node: &com::github::javaparser::ast::node::Node) -> com::github::javaparser::ast::node::Node {
		if node instanceof ClassOrInterfaceDeclaration {
			// Modifiers appear before the class name --
			let casted: ClassOrInterfaceDeclaration = node as ClassOrInterfaceDeclaration;
			let earliest_modifier: Modifier = casted.get_modifiers().stream().filter(|modifier|modifier.has_range()).min(&Comparator::comparing(|o|o.get_range().get().begin)).orElse(null);
			if earliest_modifier == null {
				return casted.get_name();
			}
			return earliest_modifier;
		}
		if node instanceof MethodDeclaration {
			// Modifiers appear before the class name --
			let casted: MethodDeclaration = node as MethodDeclaration;
			let earliest_modifier: Modifier = casted.get_modifiers().stream().filter(|modifier|modifier.has_range()).min(&Comparator::comparing(|o|o.get_range().get().begin)).orElse(null);
			if earliest_modifier == null {
				return casted.get_type();
			}
			return earliest_modifier;
		}
		if node instanceof FieldDeclaration {
			// Modifiers appear before the class name --
			let casted: FieldDeclaration = node as FieldDeclaration;
			let earliest_modifier: Modifier = casted.get_modifiers().stream().filter(|modifier|modifier.has_range()).min(&Comparator::comparing(|o|o.get_range().get().begin)).orElse(null);
			if earliest_modifier == null {
				return casted.get_variable(0).get_type();
			}
			return earliest_modifier;
		}
		return node;
	}

	pub fn node_contains(&self, container: &com::github::javaparser::ast::node::Node, other: &com::github::javaparser::ast::node::Node, ignoring_annotations: bool) /* thrown(java.lang.IllegalArgumentException) */ -> bool {
		if !container.has_range() {
			return Err(IllegalArgumentException::new("Cannot compare the positions of nodes if container node does not have a range."));
		}
		if !other.has_range() {
			return Err(IllegalArgumentException::new("Cannot compare the positions of nodes if contained node does not have a range."));
		}
		// // FIXME: Not all nodes seem to have the compilation unit available?
		// if (!Objects.equals(container.findCompilationUnit(), other.findCompilationUnit())) {
		// // Allow the check to complete if they are both within a known CU (i.e. the CUs are the same),
		// // ... or both not within a CU (i.e. both are Optional.empty())
		// return false;
		// }
		/* final */ let node_can_have_annotations: bool = container instanceof NodeWithAnnotations;
		// final boolean hasAnnotations = PositionUtils.getLastAnnotation(container) != null;
		if !ignoring_annotations || PositionUtils::get_last_annotation(container) == null {
			// No special consideration required - perform simple range check.
			return container.contains_within_range(other);
		}
		if !container.contains_within_range(other) {
			return false;
		}
		if !node_can_have_annotations {
			return true;
		}
		// If the node is contained, but it comes immediately after the annotations,
		// let's not consider it contained (i.e. it must be "strictly contained").
		let node_without_annotations: Node = com::github::javaparser::utils::position_utils::PositionUtils::first_non_annotation_node(container);
		let range_without_annotations: Range = container.get_range().get().with_begin(&node_without_annotations.get_begin().get());
		return // .contains(other.getRange().get());
		range_without_annotations.strictly_contains(&other.get_range().get());
	}
}