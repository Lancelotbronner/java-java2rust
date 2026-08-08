use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::utils::Utils;
use java::lang::reflect::InvocationTargetException;
use java::util::Arrays;
use java::util::Collection;
use java::util::Optional;

pub enum ObservableProperty {
	type: com::github::javaparser::ast::observer::observable_property::Type,
	derived: bool,
}

enum Type {
	multiple: bool,
	node: bool,
}