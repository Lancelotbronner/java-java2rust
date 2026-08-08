use crate::com::github::javaparser::StaticJavaParser::parseClassOrInterfaceType;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::type::ReferenceType;

pub trait NodeWithThrownExceptions<N: com::github::javaparser::ast::node::Node>;