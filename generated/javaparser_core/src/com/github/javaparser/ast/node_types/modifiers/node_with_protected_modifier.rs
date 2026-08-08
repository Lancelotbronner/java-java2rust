use crate::com::github::javaparser::ast::Modifier::Keyword::PROTECTED;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithModifiers;

pub trait NodeWithProtectedModifier<N: com::github::javaparser::ast::node::Node>;