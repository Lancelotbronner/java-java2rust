use crate::com::github::javaparser::StaticJavaParser::parseExpression;
use crate::com::github::javaparser::StaticJavaParser::parseName;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::expr;
use java::lang::annotation::Annotation;
use java::util::Optional;

pub trait NodeWithAnnotations<N: com::github::javaparser::ast::node::Node>;