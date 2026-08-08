use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::expr::Expression;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::resolution::declarations::ResolvedReferenceTypeDeclaration;
use crate::com::github::javaparser::resolution::types::ResolvedType;

pub trait SymbolResolver;