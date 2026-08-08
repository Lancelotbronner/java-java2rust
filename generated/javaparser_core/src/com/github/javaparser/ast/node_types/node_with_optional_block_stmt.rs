use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::stmt::BlockStmt;
use java::util::Optional;

pub trait NodeWithOptionalBlockStmt<N: com::github::javaparser::ast::node::Node>;