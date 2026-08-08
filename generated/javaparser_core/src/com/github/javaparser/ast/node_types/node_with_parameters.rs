use crate::com::github::javaparser::StaticJavaParser::parseType;
use java::util::stream::Collectors::toList;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::body::Parameter;
use crate::com::github::javaparser::ast::type::Type;
use java::util::Arrays;
use java::util::Optional;
use java::util::stream::Stream;

pub trait NodeWithParameters<N: com::github::javaparser::ast::node::Node>;