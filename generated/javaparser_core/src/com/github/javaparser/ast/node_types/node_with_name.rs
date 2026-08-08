use crate::com::github::javaparser::StaticJavaParser::parseName;
use crate::com::github::javaparser::utils::Utils::assertNonEmpty;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::expr::Name;

pub trait NodeWithName<N: com::github::javaparser::ast::node::Node>;