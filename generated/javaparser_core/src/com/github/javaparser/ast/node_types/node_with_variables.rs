use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::body::VariableDeclarator;
use crate::com::github::javaparser::ast::type::ArrayType;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::metamodel::DerivedProperty;
use java::util::List;
use java::util::Optional;
use java::util::stream::Collectors;

pub trait NodeWithVariables<N: com::github::javaparser::ast::node::Node>;

struct Helper;

impl Helper {
	fn to_array_level(&self, mut type: &com::github::javaparser::ast::type::type::Type, level: i32) -> /* Java */ java::util::Optional /**/ {
		if level > type.get_array_level() {
			return Optional::empty();
		}
		 {
			let i: i32 = type.get_array_level();
			while i > level {
				{
					if !(type instanceof ArrayType) {
						return Optional::empty();
					}
					type = (type as ArrayType).get_component_type();
				}
				i -= 1;
			 }
		 }
	
		return Optional::of(type);
	}
}