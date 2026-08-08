use crate::com::github::javaparser::ast::NodeList::nodeList;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::metamodel::DerivedProperty;
use java::util::Optional;

pub trait NodeWithTypeArguments<N: com::github::javaparser::ast::node::Node>;