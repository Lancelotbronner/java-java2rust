use crate::com::github::javaparser::ast::Modifier::Keyword::PUBLIC;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithModifiers;

pub trait NodeWithPublicModifier<N: com::github::javaparser::ast::node::Node>;