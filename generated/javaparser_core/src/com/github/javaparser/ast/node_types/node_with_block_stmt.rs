use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::stmt::BlockStmt;

pub trait NodeWithBlockStmt<N: com::github::javaparser::ast::node::Node>;