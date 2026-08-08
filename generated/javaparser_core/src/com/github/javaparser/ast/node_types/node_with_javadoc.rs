use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::comments::Comment;
use crate::com::github::javaparser::ast::comments::JavadocComment;
use crate::com::github::javaparser::ast::comments::MarkdownComment;
use crate::com::github::javaparser::ast::comments::TraditionalJavadocComment;
use crate::com::github::javaparser::javadoc::Javadoc;
use java::util::Optional;

pub trait NodeWithJavadoc<N: com::github::javaparser::ast::node::Node>;