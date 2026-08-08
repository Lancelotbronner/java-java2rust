use crate::com::github::javaparser::StaticJavaParser::parseStatement;
use crate::com::github::javaparser::JavaParser;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::expr::Expression;
use crate::com::github::javaparser::ast::expr::NameExpr;
use crate::com::github::javaparser::ast::stmt::ExpressionStmt;
use crate::com::github::javaparser::ast::stmt::Statement;

pub trait NodeWithStatements<N: com::github::javaparser::ast::node::Node>;