use crate::com::github::javaparser::TokenTypes::eolTokenKind;
use crate::com::github::javaparser::TokenTypes::spaceTokenKind;
use crate::com::github::javaparser::GeneratedJavaParserConstants;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::printer::SourcePrinter;
use crate::com::github::javaparser::printer::lexicalpreservation::TextElement;
use crate::com::github::javaparser::utils::LineSeparator;
use java::util::Arrays;
use java::util::List;

pub trait CsmElement;