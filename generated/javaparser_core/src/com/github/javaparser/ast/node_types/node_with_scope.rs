use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::expr::Expression;
use java::util::Optional;

pub trait NodeWithScope<N: com::github::javaparser::ast::node::Node>;