use crate::com::github::javaparser::StaticJavaParser::parseTypeParameter;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::type::TypeParameter;

pub trait NodeWithTypeParameters<N: com::github::javaparser::ast::node::Node>;