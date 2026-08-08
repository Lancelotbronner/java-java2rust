use crate::com::github::javaparser::StaticJavaParser::parseType;
use crate::com::github::javaparser::ast::Modifier::Keyword;
use crate::com::github::javaparser::ast::Modifier::Keyword;
use crate::com::github::javaparser::ast::Modifier::createModifierList;
use java::util::Collections::unmodifiableList;
use java::util::stream::Collectors::toList;
use crate::com::github::javaparser::ast::Modifier;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::body;
use crate::com::github::javaparser::ast::expr::Expression;
use crate::com::github::javaparser::ast::stmt::BlockStmt;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::ast::type::VoidType;
use java::util::List;
use java::util::Optional;

pub trait NodeWithMembers<N: com::github::javaparser::ast::node::Node>;