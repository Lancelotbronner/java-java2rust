use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::expr::AnnotationExpr;
use crate::com::github::javaparser::ast::expr::Expression;
use crate::com::github::javaparser::ast::expr::SimpleName;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithArguments;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithJavadoc;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithSimpleName;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::EnumConstantDeclarationMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::resolution::Resolvable;
use crate::com::github::javaparser::resolution::declarations::ResolvedEnumConstantDeclaration;
use java::util::Optional;
use java::util::function::Consumer;

pub struct EnumConstantDeclaration {
	name: com::github::javaparser::ast::expr::simple_name::SimpleName,
	arguments: com::github::javaparser::ast::node_list::NodeList,
	class_body: com::github::javaparser::ast::node_list::NodeList,
}

impl EnumConstantDeclaration {
	pub fn new() -> com::github::javaparser::ast::body::enum_constant_declaration::EnumConstantDeclaration {
		this(null, NodeList<>::new(), SimpleName::new(), NodeList<>::new(), NodeList<>::new());
	}

	pub fn new(name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::body::enum_constant_declaration::EnumConstantDeclaration {
		this(null, NodeList<>::new(), SimpleName::new(name), NodeList<>::new(), NodeList<>::new());
	}

	pub fn new(annotations: &com::github::javaparser::ast::node_list::NodeList, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, arguments: &com::github::javaparser::ast::node_list::NodeList, class_body: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::body::enum_constant_declaration::EnumConstantDeclaration {
		this(null, annotations, name, arguments, class_body);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, annotations: &com::github::javaparser::ast::node_list::NodeList, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, arguments: &com::github::javaparser::ast::node_list::NodeList, class_body: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::body::enum_constant_declaration::EnumConstantDeclaration {
		super(token_range, annotations);
		self.set_name(name);
		self.set_arguments(arguments);
		self.set_class_body(class_body);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_arguments(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.arguments;
	}

	pub fn get_class_body(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.class_body;
	}

	pub fn get_name(&self) -> com::github::javaparser::ast::expr::simple_name::SimpleName {
		return self.name;
	}

	pub fn set_arguments(&mut self, arguments: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::enum_constant_declaration::EnumConstantDeclaration {
		com::github::javaparser::utils::utils::Utils::assert_not_null(arguments)?;
		if arguments == self.arguments {
			return self;
		}
		self.notify_property_change(ObservableProperty::ARGUMENTS, self.arguments, arguments);
		if self.arguments != null {
			self.arguments.set_parent_node(null);
		}
	
		self.arguments = arguments;
		self.set_as_parent_node_of(arguments);
		return self;
	}

	pub fn set_class_body(&mut self, class_body: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::enum_constant_declaration::EnumConstantDeclaration {
		com::github::javaparser::utils::utils::Utils::assert_not_null(class_body)?;
		if class_body == self.classBody {
			return self;
		}
		self.notify_property_change(ObservableProperty::CLASS_BODY, self.classBody, class_body);
		if self.classBody != null {
			self.classBody.set_parent_node(null);
		}
	
		self.classBody = class_body;
		self.set_as_parent_node_of(class_body);
		return self;
	}

	pub fn set_name(&mut self, name: &com::github::javaparser::ast::expr::simple_name::SimpleName) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::enum_constant_declaration::EnumConstantDeclaration {
		com::github::javaparser::utils::utils::Utils::assert_not_null(name)?;
		if name == self.name {
			return self;
		}
		self.notify_property_change(ObservableProperty::NAME, self.name, name);
		if self.name != null {
			self.name.set_parent_node(null);
		}
	
		self.name = name;
		self.set_as_parent_node_of(name);
		return self;
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.arguments.size() {
				{
					if self.arguments.get(i) == node {
						self.arguments.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		 {
			let i: i32 = 0;
			while i < self.class_body.size() {
				{
					if self.class_body.get(i) == node {
						self.class_body.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::body::enum_constant_declaration::EnumConstantDeclaration {
		return self.accept(CloneVisitor::new(), null) as EnumConstantDeclaration;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::enum_constant_declaration_meta_model::EnumConstantDeclarationMetaModel {
		return JavaParserMetaModel::enumConstantDeclarationMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.arguments.size() {
				{
					if self.arguments.get(i) == node {
						self.arguments.set(i, replacement_node as Expression)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		 {
			let i: i32 = 0;
			while i < self.class_body.size() {
				{
					if self.class_body.get(i) == node {
						self.class_body.set(i, replacement_node as BodyDeclaration)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		if node == self.name {
			self.set_name(replacement_node as SimpleName)?;
			return true;
		}
		return super.replace(node, replacement_node)?;
	}

	pub fn is_enum_constant_declaration(&self) -> bool {
		return true;
	}

	pub fn as_enum_constant_declaration(&self) -> com::github::javaparser::ast::body::enum_constant_declaration::EnumConstantDeclaration {
		return self;
	}

	pub fn if_enum_constant_declaration(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::declarations::resolved_enum_constant_declaration::ResolvedEnumConstantDeclaration {
		return self.get_symbol_resolver().resolve_declaration(self, ResolvedEnumConstantDeclaration.class);
	}

	pub fn to_enum_constant_declaration(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl com::github::javaparser::ast::node_types::node_with_javadoc::NodeWithJavadoc for EnumConstantDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_simple_name::NodeWithSimpleName for EnumConstantDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_arguments::NodeWithArguments for EnumConstantDeclaration {}

impl com::github::javaparser::resolution::resolvable::Resolvable for EnumConstantDeclaration {}

impl /* Java */ java::lang::Cloneable /**/ for EnumConstantDeclaration {}

impl com::github::javaparser::has_parent_node::HasParentNode for EnumConstantDeclaration {}

impl com::github::javaparser::ast::observer::observable::Observable for EnumConstantDeclaration {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for EnumConstantDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for EnumConstantDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for EnumConstantDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_annotations::NodeWithAnnotations for EnumConstantDeclaration {}