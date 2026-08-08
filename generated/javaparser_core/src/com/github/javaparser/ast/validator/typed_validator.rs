use crate::com::github::javaparser::ParseResult;
use crate::com::github::javaparser::ParserConfiguration;
use crate::com::github::javaparser::Processor;
use crate::com::github::javaparser::ast::Node;
use java::util::function::BiConsumer;

pub trait TypedValidator<N: com::github::javaparser::ast::node::Node>;