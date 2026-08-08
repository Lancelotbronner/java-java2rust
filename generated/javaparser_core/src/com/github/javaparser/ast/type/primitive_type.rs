use crate::com::github::javaparser::StaticJavaParser::parseClassOrInterfaceType;
use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::expr::AnnotationExpr;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithAnnotations;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::PrimitiveTypeMetaModel;
use crate::com::github::javaparser::printer::Stringable;
use crate::com::github::javaparser::resolution::Context;
use crate::com::github::javaparser::resolution::types::ResolvedPrimitiveType;
use crate::com::github::javaparser::resolution::types::ResolvedType;
use java::util::HashMap;
use java::util::Optional;
use java::util::function::Consumer;

pub struct PrimitiveType {
	type: com::github::javaparser::ast::type::primitive_type::Primitive,
}

impl PrimitiveType {
	static unboxMap: /* Java */ java::util::HashMap /**/ = HashMap<>::new();

	pub fn boolean_type(&self) -> com::github::javaparser::ast::type::primitive_type::PrimitiveType {
		return PrimitiveType::new(Primitive::BOOLEAN);
	}

	pub fn char_type(&self) -> com::github::javaparser::ast::type::primitive_type::PrimitiveType {
		return PrimitiveType::new(Primitive::CHAR);
	}

	pub fn byte_type(&self) -> com::github::javaparser::ast::type::primitive_type::PrimitiveType {
		return PrimitiveType::new(Primitive::BYTE);
	}

	pub fn short_type(&self) -> com::github::javaparser::ast::type::primitive_type::PrimitiveType {
		return PrimitiveType::new(Primitive::SHORT);
	}

	pub fn int_type(&self) -> com::github::javaparser::ast::type::primitive_type::PrimitiveType {
		return PrimitiveType::new(Primitive::INT);
	}

	pub fn long_type(&self) -> com::github::javaparser::ast::type::primitive_type::PrimitiveType {
		return PrimitiveType::new(Primitive::LONG);
	}

	pub fn float_type(&self) -> com::github::javaparser::ast::type::primitive_type::PrimitiveType {
		return PrimitiveType::new(Primitive::FLOAT);
	}

	pub fn double_type(&self) -> com::github::javaparser::ast::type::primitive_type::PrimitiveType {
		return PrimitiveType::new(Primitive::DOUBLE);
	}

	init {
	    for (Primitive unboxedType : Primitive.values()) {
	        unboxMap.put(unboxedType.nameOfBoxedType, unboxedType);
	    }
	}

	pub fn new() -> com::github::javaparser::ast::type::primitive_type::PrimitiveType {
		this(null, Primitive::INT, NodeList<>::new());
	}

	pub fn new(type: &com::github::javaparser::ast::type::primitive_type::Primitive) -> com::github::javaparser::ast::type::primitive_type::PrimitiveType {
		this(null, type, NodeList<>::new());
	}

	pub fn new(type: &com::github::javaparser::ast::type::primitive_type::Primitive, annotations: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::type::primitive_type::PrimitiveType {
		this(null, type, annotations);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, type: &com::github::javaparser::ast::type::primitive_type::Primitive, annotations: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::type::primitive_type::PrimitiveType {
		super(token_range, annotations);
		self.set_type(type);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_type(&self) -> com::github::javaparser::ast::type::primitive_type::Primitive {
		return self.type;
	}

	pub fn to_boxed_type(&self) -> com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType {
		return self.type.to_boxed_type();
	}

	pub fn to_descriptor(&self) -> /* Java */ java::lang::String /**/ {
		return self.type.to_descriptor();
	}

	pub fn set_type(&mut self, type: &com::github::javaparser::ast::type::primitive_type::Primitive) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::type::primitive_type::PrimitiveType {
		com::github::javaparser::utils::utils::Utils::assert_not_null(type)?;
		if type == self.type {
			return self;
		}
		self.notify_property_change(ObservableProperty::TYPE, self.type, type);
		self.type = type;
		return self;
	}

	pub fn as_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.type.as_string();
	}

	pub fn set_annotations(&self, annotations: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::type::primitive_type::PrimitiveType {
		return super.set_annotations(annotations) as PrimitiveType;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::type::primitive_type::PrimitiveType {
		return self.accept(CloneVisitor::new(), null) as PrimitiveType;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::primitive_type_meta_model::PrimitiveTypeMetaModel {
		return JavaParserMetaModel::primitiveTypeMetaModel;
	}

	pub fn is_primitive_type(&self) -> bool {
		return true;
	}

	pub fn as_primitive_type(&self) -> com::github::javaparser::ast::type::primitive_type::PrimitiveType {
		return self;
	}

	pub fn if_primitive_type(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::types::resolved_primitive_type::ResolvedPrimitiveType {
		return self.get_symbol_resolver().to_resolved_type(self, ResolvedPrimitiveType.class);
	}

	pub fn to_primitive_type(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn convert_to_usage(&self, context: &com::github::javaparser::resolution::context::Context) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		return ResolvedPrimitiveType::by_name(&self.get_type().name())?;
	}
}

impl com::github::javaparser::ast::node_types::node_with_annotations::NodeWithAnnotations for PrimitiveType {}

impl /* Java */ java::lang::Cloneable /**/ for PrimitiveType {}

impl com::github::javaparser::has_parent_node::HasParentNode for PrimitiveType {}

impl com::github::javaparser::ast::observer::observable::Observable for PrimitiveType {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for PrimitiveType {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for PrimitiveType {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for PrimitiveType {}

impl com::github::javaparser::resolution::resolvable::Resolvable for PrimitiveType {}

impl com::github::javaparser::ast::type::convertible_to_usage::ConvertibleToUsage for PrimitiveType {}

pub enum Primitive {
	name_of_boxed_type: /* Java */ java::lang::String /**/,
	descriptor: /* Java */ java::lang::String /**/,
	code_representation: /* Java */ java::lang::String /**/,
}