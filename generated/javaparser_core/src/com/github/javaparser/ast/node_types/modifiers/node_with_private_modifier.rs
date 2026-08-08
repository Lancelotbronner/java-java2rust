use crate::com::github::javaparser::ast::Modifier::Keyword::PRIVATE;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithModifiers;

pub trait NodeWithPrivateModifier<N: com::github::javaparser::ast::node::Node>;