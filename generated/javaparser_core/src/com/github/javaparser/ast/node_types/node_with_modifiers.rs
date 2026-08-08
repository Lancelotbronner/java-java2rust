use crate::com::github::javaparser::ast::NodeList::toNodeList;
use crate::com::github::javaparser::ast::AccessSpecifier;
use crate::com::github::javaparser::ast::Modifier;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use java::util::Arrays;
use java::util::List;

pub trait NodeWithModifiers<N: com::github::javaparser::ast::node::Node>;