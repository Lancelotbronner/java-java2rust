use crate::com::github::javaparser::StaticJavaParser::parseType;
use crate::com::github::javaparser::utils::Utils::assertNonEmpty;
use crate::com::github::javaparser::ast::CompilationUnit;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::type::Type;

pub trait NodeWithType<N: com::github::javaparser::ast::node::Node, T: com::github::javaparser::ast::type::type::Type>;