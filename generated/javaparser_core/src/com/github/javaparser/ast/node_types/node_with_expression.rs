use crate::com::github::javaparser::StaticJavaParser::parseExpression;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::expr::Expression;

pub trait NodeWithExpression<N: com::github::javaparser::ast::node::Node>;