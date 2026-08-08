use crate::com::github::javaparser::utils::Utils::assertNonEmpty;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::expr::NameExpr;
use crate::com::github::javaparser::ast::expr::SimpleName;

pub trait NodeWithSimpleName<N: com::github::javaparser::ast::node::Node>;