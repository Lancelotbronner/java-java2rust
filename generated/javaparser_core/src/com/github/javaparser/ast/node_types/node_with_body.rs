use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::stmt::BlockStmt;
use crate::com::github::javaparser::ast::stmt::EmptyStmt;
use crate::com::github::javaparser::ast::stmt::Statement;

pub trait NodeWithBody<N: com::github::javaparser::ast::node::Node>;