use crate::com::github::javaparser::StaticJavaParser::parseExpression;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::expr::Expression;
use java::util::function::Function;

pub trait NodeWithArguments<N: com::github::javaparser::ast::node::Node>;