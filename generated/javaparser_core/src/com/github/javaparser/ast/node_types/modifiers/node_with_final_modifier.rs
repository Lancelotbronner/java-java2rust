use crate::com::github::javaparser::ast::Modifier::Keyword::FINAL;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithModifiers;

pub trait NodeWithFinalModifier<N: com::github::javaparser::ast::node::Node>;