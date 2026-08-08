use crate::com::github::javaparser::ast::NodeList::nodeList;
use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::expr::AnnotationExpr;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithAnnotations;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::ArrayTypeMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::resolution::Context;
use crate::com::github::javaparser::resolution::types::ResolvedArrayType;
use crate::com::github::javaparser::resolution::types::ResolvedType;
use crate::com::github::javaparser::utils::Pair;
use java::util::ArrayList;
use java::util::List;
use java::util::Optional;
use java::util::function::Consumer;

pub struct ArrayType {
	component_type: com::github::javaparser::ast::type::type::Type,
	origin: com::github::javaparser::ast::type::array_type::Origin,
}

impl ArrayType {
	pub fn resolve(&self) -> com::github::javaparser::resolution::types::resolved_array_type::ResolvedArrayType {
		return self.get_symbol_resolver().to_resolved_type(self, ResolvedArrayType.class);
	}

	pub fn new(component_type: &com::github::javaparser::ast::type::type::Type, origin: &com::github::javaparser::ast::type::array_type::Origin, annotations: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::type::array_type::ArrayType {
		this(null, component_type, origin, annotations);
	}

	pub fn new(type: &com::github::javaparser::ast::type::type::Type, annotations: &com::github::javaparser::ast::expr::annotation_expr::AnnotationExpr) -> com::github::javaparser::ast::type::array_type::ArrayType {
		this(type, Origin::TYPE, &com::github::javaparser::ast::node_list::NodeList::node_list(annotations));
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, component_type: &com::github::javaparser::ast::type::type::Type, origin: &com::github::javaparser::ast::type::array_type::Origin, annotations: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::type::array_type::ArrayType {
		super(token_range, annotations);
		self.set_component_type(component_type);
		self.set_origin(origin);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_component_type(&self) -> com::github::javaparser::ast::type::type::Type {
		return self.component_type;
	}

	pub fn set_component_type(&mut self, component_type: &com::github::javaparser::ast::type::type::Type) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::type::array_type::ArrayType {
		com::github::javaparser::utils::utils::Utils::assert_not_null(component_type)?;
		if component_type == self.componentType {
			return self;
		}
		self.notify_property_change(ObservableProperty::COMPONENT_TYPE, self.componentType, component_type);
		if self.componentType != null {
			self.componentType.set_parent_node(null);
		}
	
		self.componentType = component_type;
		self.set_as_parent_node_of(component_type);
		return self;
	}

	pub fn wrap_in_array_types(&self, mut type: &com::github::javaparser::ast::type::type::Type, array_bracket_pair_lists: &/* Java */ java::util::List /**/) -> com::github::javaparser::ast::type::type::Type {
		let outer_most_token_range: TokenRange = null;
		 {
			let i: i32 = array_bracket_pair_lists.length - 1;
			while i >= 0 {
				{
					/* final */ let array_bracket_pair_list: List<ArrayBracketPair> = array_bracket_pair_lists[i];
					if array_bracket_pair_list != null {
						 {
							let j: i32 = array_bracket_pair_list.size() - 1;
							while j >= 0 {
								{
									let pair: ArrayBracketPair = array_bracket_pair_list.get(j);
									if type.get_token_range().isPresent() && pair.get_token_range().isPresent() {
										let current_token_range: TokenRange = TokenRange::new(&type.get_token_range().get().get_begin(), &pair.get_token_range().get().get_end());
										// The end range must be equals to the last array bracket pair in the list
										// in the example below:
										// Long[][]
										//        ^
										//        |
										// this is the outermost range for the ArrayType
										outer_most_token_range = com::github::javaparser::ast::type::array_type::ArrayType::get_outer_most_token_range(current_token_range, outer_most_token_range);
									}
									type = ArrayType::new(outer_most_token_range, type, &pair.get_origin(), &pair.get_annotations());
								}
								j -= 1;
							 }
						 }
	
					}
				}
				i -= 1;
			 }
		 }
	
		return type;
	}

	fn get_outer_most_token_range(&self, token_range1: &com::github::javaparser::token_range::TokenRange, token_range2: &com::github::javaparser::token_range::TokenRange) -> com::github::javaparser::token_range::TokenRange {
		if token_range2 == null {
			return token_range1;
		}
	
		if token_range1.get_end().get_range().get().is_after(&token_range2.get_end().get_range().get()) {
			return token_range1;
		}
		return TokenRange::new(&token_range1.get_begin(), &token_range2.get_end());
	}

	pub fn unwrap_array_types(&self, mut type: &com::github::javaparser::ast::type::type::Type) -> com::github::javaparser::utils::pair::Pair {
		/* final */ let array_bracket_pairs: List<ArrayBracketPair> = ArrayList<>::new(0);
		while type instanceof ArrayType {
			let array_type: ArrayType = type as ArrayType;
			array_bracket_pairs.add(ArrayBracketPair::new(&type.get_token_range().orElse(null), &array_type.get_origin(), &array_type.get_annotations()));
			type = array_type.get_component_type();
		}
		return Pair<>::new(type, array_bracket_pairs);
	}

	pub fn set_annotations(&self, annotations: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::type::array_type::ArrayType {
		return super.set_annotations(annotations) as ArrayType;
	}

	pub fn get_origin(&self) -> com::github::javaparser::ast::type::array_type::Origin {
		return self.origin;
	}

	pub fn set_origin(&mut self, origin: &com::github::javaparser::ast::type::array_type::Origin) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::type::array_type::ArrayType {
		com::github::javaparser::utils::utils::Utils::assert_not_null(origin)?;
		if origin == self.origin {
			return self;
		}
		self.notify_property_change(ObservableProperty::ORIGIN, self.origin, origin);
		self.origin = origin;
		return self;
	}

	pub fn as_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.component_type.as_string() + "[]";
	}

	pub fn to_descriptor(&self) -> /* Java */ java::lang::String /**/ {
		let sb: StringBuffer = StringBuffer::new();
		sb.append("[");
		sb.append(&self.component_type.to_descriptor());
		return sb.toString();
	}

	pub fn clone(&self) -> com::github::javaparser::ast::type::array_type::ArrayType {
		return self.accept(CloneVisitor::new(), null) as ArrayType;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::array_type_meta_model::ArrayTypeMetaModel {
		return JavaParserMetaModel::arrayTypeMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.component_type {
			self.set_component_type(replacement_node as Type)?;
			return true;
		}
		return super.replace(node, replacement_node);
	}

	pub fn is_array_type(&self) -> bool {
		return true;
	}

	pub fn as_array_type(&self) -> com::github::javaparser::ast::type::array_type::ArrayType {
		return self;
	}

	pub fn if_array_type(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_array_type(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn get_element_type(&self) -> com::github::javaparser::ast::type::type::Type {
		return self.get_component_type().get_element_type();
	}

	pub fn get_array_level(&self) -> i32 {
		return 1 + self.get_component_type().get_array_level();
	}

	pub fn convert_to_usage(&self, context: &com::github::javaparser::resolution::context::Context) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		return ResolvedArrayType::new(&self.get_component_type().convert_to_usage(context));
	}
}

impl com::github::javaparser::ast::node_types::node_with_annotations::NodeWithAnnotations for ArrayType {}

impl /* Java */ java::lang::Cloneable /**/ for ArrayType {}

impl com::github::javaparser::has_parent_node::HasParentNode for ArrayType {}

impl com::github::javaparser::ast::observer::observable::Observable for ArrayType {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for ArrayType {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for ArrayType {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for ArrayType {}

impl com::github::javaparser::resolution::resolvable::Resolvable for ArrayType {}

impl com::github::javaparser::ast::type::convertible_to_usage::ConvertibleToUsage for ArrayType {}

pub enum Origin;

pub struct ArrayBracketPair {
	token_range: com::github::javaparser::token_range::TokenRange,
	annotations: com::github::javaparser::ast::node_list::NodeList = NodeList<>::new(),
	origin: com::github::javaparser::ast::type::array_type::Origin,
}

impl ArrayBracketPair {
	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, origin: &com::github::javaparser::ast::type::array_type::Origin, annotations: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::type::array_type::ArrayBracketPair {
		self.set_token_range(token_range);
		self.set_annotations(annotations);
		self.set_origin(origin);
	}

	pub fn get_annotations(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.annotations;
	}

	pub fn set_annotations(&mut self, annotations: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::type::array_type::ArrayBracketPair {
		self.annotations = com::github::javaparser::utils::utils::Utils::assert_not_null(annotations)?;
		return self;
	}

	pub fn set_token_range(&mut self, range: &com::github::javaparser::token_range::TokenRange) -> com::github::javaparser::ast::type::array_type::ArrayBracketPair {
		self.tokenRange = range;
		return self;
	}

	pub fn get_token_range(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.token_range);
	}

	pub fn get_origin(&self) -> com::github::javaparser::ast::type::array_type::Origin {
		return self.origin;
	}

	pub fn set_origin(&mut self, origin: &com::github::javaparser::ast::type::array_type::Origin) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::type::array_type::ArrayBracketPair {
		self.origin = com::github::javaparser::utils::utils::Utils::assert_not_null(origin)?;
		return self;
	}
}