use crate::com::github::javaparser::JavaParser;
use crate::com::github::javaparser::ParseProblemException;
use crate::com::github::javaparser::ParseResult;
use crate::com::github::javaparser::ParserConfiguration;
use crate::com::github::javaparser::ast::CompilationUnit;
use java::io::IOException;
use java::nio::file::FileSystems;
use java::nio::file::Path;
use java::nio::file::PathMatcher;
use java::util::Optional;

pub trait CollectionStrategy;