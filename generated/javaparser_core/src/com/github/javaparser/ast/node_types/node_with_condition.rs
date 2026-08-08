use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::expr::Expression;

pub trait NodeWithCondition<N: com::github::javaparser::ast::node::Node>;