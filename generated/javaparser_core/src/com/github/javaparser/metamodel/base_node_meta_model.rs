use crate::com::github::javaparser::utils::Utils::decapitalize;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use java::lang::reflect::Constructor;
use java::lang::reflect::InvocationTargetException;
use java::util::ArrayList;
use java::util::List;
use java::util::Map;
use java::util::Optional;

pub struct BaseNodeMetaModel {
	super_node_meta_model: /* Java */ java::util::Optional /**/,
	declared_property_meta_models: /* Java */ java::util::List /**/ = ArrayList<>::new(),
	derived_property_meta_models: /* Java */ java::util::List /**/ = ArrayList<>::new(),
	constructor_parameters: /* Java */ java::util::List /**/ = ArrayList<>::new(),
	type: /* Java */ java::lang::Class /**/,
	name: /* Java */ java::lang::String /**/,
	package_name: /* Java */ java::lang::String /**/,
	is_abstract: bool,
	has_wildcard: bool,
}

impl BaseNodeMetaModel {
	pub fn new(super_node_meta_model: &/* Java */ java::util::Optional /**/, type: &/* Java */ java::lang::Class /**/, name: &/* Java */ java::lang::String /**/, package_name: &/* Java */ java::lang::String /**/, is_abstract: bool, has_wildcard: bool) -> com::github::javaparser::metamodel::base_node_meta_model::BaseNodeMetaModel {
		self.superNodeMetaModel = super_node_meta_model;
		self.type = type;
		self.name = name;
		self.packageName = package_name;
		self.isAbstract = is_abstract;
		self.hasWildcard = has_wildcard;
	}

	pub fn is(&self, c: &/* Java */ java::lang::Class /**/) -> bool {
		return self.type.equals(c);
	}

	pub fn get_qualified_class_name(&self) -> /* Java */ java::lang::String /**/ {
		return self.package_name + "." + self.name;
	}

	pub fn get_super_node_meta_model(&self) -> /* Java */ java::util::Optional /**/ {
		return self.super_node_meta_model;
	}

	pub fn get_declared_property_meta_models(&self) -> /* Java */ java::util::List /**/ {
		return self.declared_property_meta_models;
	}

	pub fn get_derived_property_meta_models(&self) -> /* Java */ java::util::List /**/ {
		return self.derived_property_meta_models;
	}

	pub fn get_constructor_parameters(&self) -> /* Java */ java::util::List /**/ {
		return self.constructor_parameters;
	}

	pub fn get_all_property_meta_models(&self) -> /* Java */ java::util::List /**/ {
		let all_property_meta_models: List<PropertyMetaModel> = ArrayList<>::new(&self.get_declared_property_meta_models());
		let walk_node: BaseNodeMetaModel = self;
		while walk_node.get_super_node_meta_model().isPresent() {
			walk_node = walk_node.get_super_node_meta_model().get();
			all_property_meta_models.addAll(&walk_node.get_declared_property_meta_models());
		}
		return all_property_meta_models;
	}

	pub fn is_instance_of_meta_model(&self, base_meta_model: &com::github::javaparser::metamodel::base_node_meta_model::BaseNodeMetaModel) -> bool {
		if self == base_meta_model {
			return true;
		}
		if self.is_root_node() {
			return false;
		}
		return self.get_super_node_meta_model().get().is_instance_of_meta_model(base_meta_model);
	}

	pub fn get_type(&self) -> /* Java */ java::lang::Class /**/ {
		return self.type;
	}

	pub fn get_package_name(&self) -> /* Java */ java::lang::String /**/ {
		return self.package_name;
	}

	pub fn is_abstract(&self) -> bool {
		return self.is_abstract;
	}

	pub fn has_wildcard(&self) -> bool {
		return self.has_wildcard;
	}

	pub fn is_root_node(&self) -> bool {
		return !self.super_node_meta_model.isPresent();
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if self == o {
			return true;
		}
	
		if o == null || self.getClass() != o.getClass() {
			return false;
		}
	
		let class_meta_model: BaseNodeMetaModel = o as BaseNodeMetaModel;
		if !self.type.equals(class_meta_model.type) {
			return false;
		}
	
		return true;
	}

	pub fn hash_code(&self) -> i32 {
		return self.type.hashCode();
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.name;
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

	pub fn get_meta_model_field_name(&self) -> /* Java */ java::lang::String /**/ {
		return com::github::javaparser::utils::utils::Utils::decapitalize(&self.getClass().getSimpleName());
	}

	pub fn construct(&self, parameters: &/* Java */ java::util::Map /**/) /* thrown(java.lang.IllegalStateException | java.lang.RuntimeException) */ -> com::github::javaparser::ast::node::Node {
		for constructor in self.get_type().getConstructors() {
			if constructor.getAnnotation(AllFieldsConstructor.class) != null {
				let r0 = 'try0: {
					let param_array: [Option<Object>; constructor.getParameterCount()] = [None; constructor.getParameterCount()];
					let i: i32 = 0;
					for constructor_parameter in self.get_constructor_parameters() {
						param_array[i] = parameters.get(&constructor_parameter.get_name());
						if param_array[i] == null && constructor_parameter.is_required() {
							if constructor_parameter.is_node_list() {
								param_array[i] = NodeList<>::new();
							}
						// We could have more defaults here.
						}
						i += 1;
					}
					return constructor.newInstance(param_array) as Node;
					break 'try0 Ok(());
				};
				match r0 {
					Err(e @ InstantiationExceptionIllegalAccessException | InvocationTargetException | ) => {
						break 'try0 Err(RuntimeException::new(e));
					},
					Err(e) => Err(e)?,
					Ok => (),
				}
			}
		}
		return Err(IllegalStateException::new());
	}
}