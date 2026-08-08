use crate::com::github::javaparser::ast::Modifier::Keyword::STRICTFP;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithModifiers;

pub trait NodeWithStrictfpModifier<N: com::github::javaparser::ast::node::Node>;