use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast;
use crate::com::github::javaparser::ast::expr::AnnotationExpr;
use crate::com::github::javaparser::ast::expr::SimpleName;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithExtends;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithImplements;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithTypeParameters;
use crate::com::github::javaparser::ast::nodeTypes::modifiers::NodeWithAbstractModifier;
use crate::com::github::javaparser::ast::nodeTypes::modifiers::NodeWithFinalModifier;
use crate::com::github::javaparser::ast::observer::AstObserverAdapter;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::stmt::LocalClassDeclarationStmt;
use crate::com::github::javaparser::ast::type::ClassOrInterfaceType;
use crate::com::github::javaparser::ast::type::TypeParameter;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::ClassOrInterfaceDeclarationMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::resolution::Resolvable;
use crate::com::github::javaparser::resolution::declarations::ResolvedReferenceTypeDeclaration;
use java::util::Optional;
use java::util::function::Consumer;

pub struct ClassOrInterfaceDeclaration {
	is_interface: bool,
	is_compact: bool,
	type_parameters: com::github::javaparser::ast::node_list::NodeList,
	extended_types: com::github::javaparser::ast::node_list::NodeList,
	implemented_types: com::github::javaparser::ast::node_list::NodeList,
	permitted_types: com::github::javaparser::ast::node_list::NodeList,
}

impl ClassOrInterfaceDeclaration {
	pub fn new() -> com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration {
		this(null, NodeList<>::new(), NodeList<>::new(), false, SimpleName::new(), NodeList<>::new(), NodeList<>::new(), NodeList<>::new(), NodeList<>::new(), NodeList<>::new());
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, is_interface: bool, name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration {
		this(null, modifiers, NodeList<>::new(), is_interface, SimpleName::new(name), NodeList<>::new(), NodeList<>::new(), NodeList<>::new(), NodeList<>::new(), NodeList<>::new());
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, is_interface: bool, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, type_parameters: &com::github::javaparser::ast::node_list::NodeList, extended_types: &com::github::javaparser::ast::node_list::NodeList, implemented_types: &com::github::javaparser::ast::node_list::NodeList, permitted_types: &com::github::javaparser::ast::node_list::NodeList, members: &com::github::javaparser::ast::node_list::NodeList, is_compact: bool) -> com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration {
		this(null, modifiers, annotations, is_interface, name, type_parameters, extended_types, implemented_types, permitted_types, members, is_compact);
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, is_interface: bool, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, type_parameters: &com::github::javaparser::ast::node_list::NodeList, extended_types: &com::github::javaparser::ast::node_list::NodeList, implemented_types: &com::github::javaparser::ast::node_list::NodeList, permitted_types: &com::github::javaparser::ast::node_list::NodeList, members: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration {
		this(null, modifiers, annotations, is_interface, name, type_parameters, extended_types, implemented_types, permitted_types, members, false);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, is_interface: bool, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, type_parameters: &com::github::javaparser::ast::node_list::NodeList, extended_types: &com::github::javaparser::ast::node_list::NodeList, implemented_types: &com::github::javaparser::ast::node_list::NodeList, permitted_types: &com::github::javaparser::ast::node_list::NodeList, members: &com::github::javaparser::ast::node_list::NodeList, is_compact: bool) -> com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration {
		super(token_range, modifiers, annotations, name, members);
		self.set_interface(is_interface);
		self.set_type_parameters(type_parameters);
		self.set_extended_types(extended_types);
		self.set_implemented_types(implemented_types);
		self.set_permitted_types(permitted_types);
		self.set_compact(is_compact);
		self.custom_initialization();
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, is_interface: bool, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, type_parameters: &com::github::javaparser::ast::node_list::NodeList, extended_types: &com::github::javaparser::ast::node_list::NodeList, implemented_types: &com::github::javaparser::ast::node_list::NodeList, permitted_types: &com::github::javaparser::ast::node_list::NodeList, members: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration {
		super(token_range, modifiers, annotations, name, members);
		self.set_interface(is_interface);
		self.set_type_parameters(type_parameters);
		self.set_extended_types(extended_types);
		self.set_implemented_types(implemented_types);
		self.set_permitted_types(permitted_types);
		self.custom_initialization();
	}

	fn process_is_compact_change(&self, new_is_compact: bool) {
		let name: SimpleName = self.get_name();
		if name != null {
			self.get_name().set_data(, new_is_compact);
		}
		let modifiers: NodeList<Modifier> = self.get_modifiers();
		if modifiers != null {
			self.get_modifiers().forEach(|modifier|{
				if modifier.get_keyword().equals(Modifier::com::github::javaparser::ast::modifier::Keyword::FINAL) {
					modifier.set_data(, new_is_compact);
				}
			});
		}
	}

	pub fn custom_initialization(&self) /* thrown(java.lang.IllegalArgumentException | java.lang.UnsupportedOperationException) */ {
		// The LPP crashes if the name or modifiers of a class don't have a range, but since the compact class name
		// is synthetic, this will always be the case for the implicit name and final modifier. There is already
		// a mechanism to handle this case in the LPP in the form of the `PHANTOM_KEY` data property. If this is
		// set to true for a given, the LPP does not attempt to find the range for this node.
		// To handle this for classes, an observer is created for all ClassOrInterfaceDeclarations to monitor
		// name/modifier changes along with the isCompact field and to set these as phantom or not when appropriate.
		// Another option would be to override the setName, setCompact etc. methods to include this functionality,
		// but a mechanism to stop the code generators from overwriting these methods would be necessary.
		self.register(AstObserverAdapter::new() {
			pub fn property_change(&self, observed_node: &Node, property: &ObservableProperty, old_value: &Object, new_value: &Object) {
				if !(observed_node instanceof ClassOrInterfaceDeclaration) {
					return Err(IllegalStateException::new("It should not be possible for a compact class observer to be added to anything other than a ClassOrInterfaceDeclaration"));
				}
				if property.equals(ObservableProperty::NAME) {
					// If the name of the class changes, mark it as a phantom node if the class is compact
					let new_name: SimpleName = new_value as SimpleName;
					new_name.set_data(, self.is_compact);
				} else if property.equals(ObservableProperty::MODIFIERS) {
					// If modifiers change, mark them as phantom nodes if the class is compact
					let new_modifiers: NodeList<Modifier> = new_value as NodeList<Modifier>;
					new_modifiers.forEach(|modifier|{
						if modifier.get_keyword().equals(Modifier::com::github::javaparser::ast::modifier::Keyword::FINAL) {
							modifier.set_data(, self.is_compact);
						}
					});
				} else if property.equals(ObservableProperty::COMPACT) {
					// If a compact class is made non-compact or vice versa, handle it properly
					self.process_is_compact_change(new_value as bool);
				}
			}
	
		}, ObserverRegistrationMode::JUST_THIS_NODE)?;
		self.process_is_compact_change(&self.is_compact());
	}

	pub fn property_change(&self, observed_node: &com::github::javaparser::ast::node::Node, property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, old_value: &/* Java */ java::lang::Object /**/, new_value: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalStateException) */ {
		if !(observed_node instanceof ClassOrInterfaceDeclaration) {
			return Err(IllegalStateException::new("It should not be possible for a compact class observer to be added to anything other than a ClassOrInterfaceDeclaration"));
		}
		if property.equals(ObservableProperty::NAME) {
			// If the name of the class changes, mark it as a phantom node if the class is compact
			let new_name: SimpleName = new_value as SimpleName;
			new_name.set_data(, self.is_compact);
		} else if property.equals(ObservableProperty::MODIFIERS) {
			// If modifiers change, mark them as phantom nodes if the class is compact
			let new_modifiers: NodeList<Modifier> = new_value as NodeList<Modifier>;
			new_modifiers.forEach(|modifier|{
				if modifier.get_keyword().equals(Modifier::com::github::javaparser::ast::modifier::Keyword::FINAL) {
					modifier.set_data(, self.is_compact);
				}
			});
		} else if property.equals(ObservableProperty::COMPACT) {
			// If a compact class is made non-compact or vice versa, handle it properly
			self.process_is_compact_change(new_value as bool);
		}
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_extended_types(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.extended_types;
	}

	pub fn get_implemented_types(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.implemented_types;
	}

	pub fn get_permitted_types(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.permitted_types;
	}

	pub fn get_type_parameters(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.type_parameters;
	}

	pub fn is_interface(&self) -> bool {
		return self.is_interface;
	}

	pub fn set_extended_types(&mut self, extended_types: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration {
		com::github::javaparser::utils::utils::Utils::assert_not_null(extended_types)?;
		if extended_types == self.extendedTypes {
			return self;
		}
		self.notify_property_change(ObservableProperty::EXTENDED_TYPES, self.extendedTypes, extended_types);
		if self.extendedTypes != null {
			self.extendedTypes.set_parent_node(null);
		}
	
		self.extendedTypes = extended_types;
		self.set_as_parent_node_of(extended_types);
		return self;
	}

	pub fn set_implemented_types(&mut self, implemented_types: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration {
		com::github::javaparser::utils::utils::Utils::assert_not_null(implemented_types)?;
		if implemented_types == self.implementedTypes {
			return self;
		}
		self.notify_property_change(ObservableProperty::IMPLEMENTED_TYPES, self.implementedTypes, implemented_types);
		if self.implementedTypes != null {
			self.implementedTypes.set_parent_node(null);
		}
	
		self.implementedTypes = implemented_types;
		self.set_as_parent_node_of(implemented_types);
		return self;
	}

	pub fn set_permitted_types(&mut self, permitted_types: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration {
		com::github::javaparser::utils::utils::Utils::assert_not_null(permitted_types)?;
		if permitted_types == self.permittedTypes {
			return self;
		}
		self.notify_property_change(ObservableProperty::PERMITTED_TYPES, self.permittedTypes, permitted_types);
		if self.permittedTypes != null {
			self.permittedTypes.set_parent_node(null);
		}
	
		self.permittedTypes = permitted_types;
		self.set_as_parent_node_of(permitted_types);
		return self;
	}

	pub fn set_interface(&mut self, is_interface: bool) -> com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration {
		if is_interface == self.isInterface {
			return self;
		}
		self.notify_property_change(ObservableProperty::INTERFACE, self.isInterface, is_interface);
		self.isInterface = is_interface;
		return self;
	}

	pub fn set_type_parameters(&mut self, type_parameters: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration {
		com::github::javaparser::utils::utils::Utils::assert_not_null(type_parameters)?;
		if type_parameters == self.typeParameters {
			return self;
		}
		self.notify_property_change(ObservableProperty::TYPE_PARAMETERS, self.typeParameters, type_parameters);
		if self.typeParameters != null {
			self.typeParameters.set_parent_node(null);
		}
	
		self.typeParameters = type_parameters;
		self.set_as_parent_node_of(type_parameters);
		return self;
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.extended_types.size() {
				{
					if self.extended_types.get(i) == node {
						self.extended_types.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		 {
			let i: i32 = 0;
			while i < self.implemented_types.size() {
				{
					if self.implemented_types.get(i) == node {
						self.implemented_types.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		 {
			let i: i32 = 0;
			while i < self.permitted_types.size() {
				{
					if self.permitted_types.get(i) == node {
						self.permitted_types.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		 {
			let i: i32 = 0;
			while i < self.type_parameters.size() {
				{
					if self.type_parameters.get(i) == node {
						self.type_parameters.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn is_local_class_declaration(&self) -> bool {
		return self.get_parent_node().map(|p|p instanceof LocalClassDeclarationStmt).orElse(false);
	}

	pub fn get_fully_qualified_name(&self) -> /* Java */ java::util::Optional /**/ {
		if self.is_local_class_declaration() {
			return Optional::empty();
		}
		return super.get_fully_qualified_name();
	}

	pub fn is_inner_class(&self) -> bool {
		return self.is_nested_type() && !self.is_interface && !self.is_static();
	}

	pub fn clone(&self) -> com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration {
		return self.accept(CloneVisitor::new(), null) as ClassOrInterfaceDeclaration;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::class_or_interface_declaration_meta_model::ClassOrInterfaceDeclarationMetaModel {
		return JavaParserMetaModel::classOrInterfaceDeclarationMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.extended_types.size() {
				{
					if self.extended_types.get(i) == node {
						self.extended_types.set(i, replacement_node as ClassOrInterfaceType)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		 {
			let i: i32 = 0;
			while i < self.implemented_types.size() {
				{
					if self.implemented_types.get(i) == node {
						self.implemented_types.set(i, replacement_node as ClassOrInterfaceType)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		 {
			let i: i32 = 0;
			while i < self.permitted_types.size() {
				{
					if self.permitted_types.get(i) == node {
						self.permitted_types.set(i, replacement_node as ClassOrInterfaceType)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		 {
			let i: i32 = 0;
			while i < self.type_parameters.size() {
				{
					if self.type_parameters.get(i) == node {
						self.type_parameters.set(i, replacement_node as TypeParameter)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.replace(node, replacement_node);
	}

	pub fn is_class_or_interface_declaration(&self) -> bool {
		return true;
	}

	pub fn as_class_or_interface_declaration(&self) -> com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration {
		return self;
	}

	pub fn if_class_or_interface_declaration(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::declarations::resolved_reference_type_declaration::ResolvedReferenceTypeDeclaration {
		return self.get_symbol_resolver().resolve_declaration(self, ResolvedReferenceTypeDeclaration.class);
	}

	pub fn to_class_or_interface_declaration(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn is_compact(&self) -> bool {
		return self.is_compact;
	}

	pub fn set_compact(&mut self, is_compact: bool) -> com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration {
		if is_compact == self.isCompact {
			return self;
		}
		self.notify_property_change(ObservableProperty::COMPACT, self.isCompact, is_compact);
		self.isCompact = is_compact;
		return self;
	}
}

impl com::github::javaparser::ast::node_types::node_with_implements::NodeWithImplements for ClassOrInterfaceDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_extends::NodeWithExtends for ClassOrInterfaceDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_type_parameters::NodeWithTypeParameters for ClassOrInterfaceDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_abstract_modifier::NodeWithAbstractModifier for ClassOrInterfaceDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for ClassOrInterfaceDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_final_modifier::NodeWithFinalModifier for ClassOrInterfaceDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for ClassOrInterfaceDeclaration {}

impl com::github::javaparser::resolution::resolvable::Resolvable for ClassOrInterfaceDeclaration {}

impl /* Java */ java::lang::Cloneable /**/ for ClassOrInterfaceDeclaration {}

impl com::github::javaparser::has_parent_node::HasParentNode for ClassOrInterfaceDeclaration {}

impl com::github::javaparser::ast::observer::observable::Observable for ClassOrInterfaceDeclaration {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for ClassOrInterfaceDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for ClassOrInterfaceDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for ClassOrInterfaceDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_annotations::NodeWithAnnotations for ClassOrInterfaceDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_simple_name::NodeWithSimpleName for ClassOrInterfaceDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_javadoc::NodeWithJavadoc for ClassOrInterfaceDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_members::NodeWithMembers for ClassOrInterfaceDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_simple_name::NodeWithSimpleName for ClassOrInterfaceDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_access_modifiers::NodeWithAccessModifiers for ClassOrInterfaceDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_public_modifier::NodeWithPublicModifier for ClassOrInterfaceDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for ClassOrInterfaceDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_private_modifier::NodeWithPrivateModifier for ClassOrInterfaceDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_protected_modifier::NodeWithProtectedModifier for ClassOrInterfaceDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_static_modifier::NodeWithStaticModifier for ClassOrInterfaceDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for ClassOrInterfaceDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_strictfp_modifier::NodeWithStrictfpModifier for ClassOrInterfaceDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for ClassOrInterfaceDeclaration {}