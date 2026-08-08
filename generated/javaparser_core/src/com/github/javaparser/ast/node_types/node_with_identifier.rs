use crate::com::github::javaparser::utils::Utils::assertNonEmpty;
use crate::com::github::javaparser::ast::Node;

pub trait NodeWithIdentifier<N: com::github::javaparser::ast::node::Node>;