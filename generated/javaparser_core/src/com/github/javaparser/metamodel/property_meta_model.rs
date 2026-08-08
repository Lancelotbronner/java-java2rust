use crate::com::github::javaparser::utils::CodeGenerationUtils::getterName;
use crate::com::github::javaparser::utils::CodeGenerationUtils::setterName;
use crate::com::github::javaparser::ast::Node;
use java::lang::reflect::Field;
use java::util::Optional;

pub struct PropertyMetaModel {
	containing_node_meta_model: com::github::javaparser::metamodel::base_node_meta_model::BaseNodeMetaModel,
	name: /* Java */ java::lang::String /**/,
	type: /* Java */ java::lang::Class /**/,
	node_reference: /* Java */ java::util::Optional /**/,
	is_optional: bool,
	is_non_empty: bool,
	is_node_list: bool,
	has_wildcard: bool,
}

impl PropertyMetaModel {
	pub fn new(containing_node_meta_model: &com::github::javaparser::metamodel::base_node_meta_model::BaseNodeMetaModel, name: &/* Java */ java::lang::String /**/, type: &/* Java */ java::lang::Class /**/, node_reference: &/* Java */ java::util::Optional /**/, is_optional: bool, is_non_empty: bool, is_node_list: bool, has_wildcard: bool) -> com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel {
		self.containingNodeMetaModel = containing_node_meta_model;
		self.name = name;
		self.type = type;
		self.nodeReference = node_reference;
		self.isOptional = is_optional;
		self.isNonEmpty = is_non_empty;
		self.isNodeList = is_node_list;
		self.hasWildcard = has_wildcard;
	}

	pub fn is(&self, c: &/* Java */ java::lang::Class /**/, field_name: &/* Java */ java::lang::String /**/) -> bool {
		return self.containing_node_meta_model.is(c) && self.name.equals(field_name);
	}

	pub fn is(&self, field_name: &/* Java */ java::lang::String /**/) -> bool {
		return self.name.equals(field_name);
	}

	pub fn get_setter_method_name(&self) -> /* Java */ java::lang::String /**/ {
		return com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::setter_name(self.name);
	}

	pub fn get_getter_method_name(&self) -> /* Java */ java::lang::String /**/ {
		return com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::getter_name(self.type, self.name);
	}

	pub fn get_containing_node_meta_model(&self) -> com::github::javaparser::metamodel::base_node_meta_model::BaseNodeMetaModel {
		return self.containing_node_meta_model;
	}

	pub fn get_name(&self) -> /* Java */ java::lang::String /**/ {
		return self.name;
	}

	pub fn is_non_empty(&self) -> bool {
		return self.is_non_empty;
	}

	pub fn get_type(&self) -> /* Java */ java::lang::Class /**/ {
		return self.type;
	}

	pub fn get_node_reference(&self) -> /* Java */ java::util::Optional /**/ {
		return self.node_reference;
	}

	pub fn is_optional(&self) -> bool {
		return self.is_optional;
	}

	pub fn is_required(&self) -> bool {
		return !self.is_optional;
	}

	pub fn is_node_list(&self) -> bool {
		return self.is_node_list;
	}

	pub fn has_wildcard(&self) -> bool {
		return self.has_wildcard;
	}

	pub fn is_singular(&self) -> bool {
		return !self.is_node_list;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "(" + self.get_type_name() + ")\t" + self.containing_node_meta_model + "#" + self.name;
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if self == o {
			return true;
		}
	
		if o == null || self.getClass() != o.getClass() {
			return false;
		}
	
		let that: PropertyMetaModel = o as PropertyMetaModel;
		if !self.name.equals(that.name) {
			return false;
		}
	
		if !self.type.equals(that.type) {
			return false;
		}
	
		return true;
	}

	pub fn hash_code(&self) -> i32 {
		let result: i32 = self.name.hashCode();
		result = 31 * result + self.type.hashCode();
		return result;
	}

	pub fn get_type_name_generified(&self) -> /* Java */ java::lang::String /**/ {
		if self.has_wildcard {
			return self.get_type_name() + "<?>";
		}
		return self.get_type_name();
	}

	pub fn get_type_name(&self) -> /* Java */ java::lang::String /**/ {
		return self.type.getSimpleName();
	}

	pub fn get_type_name_for_getter(&self) -> /* Java */ java::lang::String /**/ {
		if self.is_optional {
			return "Optional<" + self.get_type_name_for_setter() + ">";
		}
		return self.get_type_name_for_setter();
	}

	pub fn get_type_name_for_setter(&self) -> /* Java */ java::lang::String /**/ {
		if self.is_node_list {
			return "NodeList<" + self.get_type_name_generified() + ">";
		}
		return self.get_type_name_generified();
	}

	pub fn is_node(&self) -> bool {
		return self.get_node_reference().isPresent();
	}

	pub fn get_meta_model_field_name(&self) -> /* Java */ java::lang::String /**/ {
		return self.get_name() + "PropertyMetaModel";
	}

	pub fn is_attribute(&self) -> bool {
		return !self.is_node();
	}

	pub fn get_value(&self, node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.NoSuchFieldError | java.lang.RuntimeException) */ -> /* Java */ java::lang::Object /**/ {
		let r0 = 'try0: {
			 {
				let c: Class<?> = node.getClass();
				while c != null {
					{
						let fields: Vec<Field> = c.getDeclaredFields();
						for class_field in fields {
							if class_field.getName().equals(&self.get_name()) {
								class_field.setAccessible(true);
								return class_field.get(node);
							}
						}
					}
					c = c.getSuperclass();
				 }
			 }
	
			break 'try0 Err(NoSuchFieldError::new(&self.get_name()));
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ IllegalAccessException) => {
				return Err(RuntimeException::new(e));
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}
}