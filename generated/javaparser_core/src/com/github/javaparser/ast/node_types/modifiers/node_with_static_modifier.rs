use crate::com::github::javaparser::ast::Modifier::Keyword::STATIC;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithModifiers;

pub trait NodeWithStaticModifier<N: com::github::javaparser::ast::node::Node>;