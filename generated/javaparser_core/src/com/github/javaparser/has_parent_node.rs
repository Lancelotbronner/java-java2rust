use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::observer::Observable;
use java::util::Optional;
use java::util::function::Predicate;

pub trait HasParentNode<T>;