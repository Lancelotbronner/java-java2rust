use crate::com::github::javaparser::ast::Modifier::Keyword::ABSTRACT;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithModifiers;

pub trait NodeWithAbstractModifier<N: com::github::javaparser::ast::node::Node>;