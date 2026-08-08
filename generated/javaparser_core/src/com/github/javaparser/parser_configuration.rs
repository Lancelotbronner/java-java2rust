use crate::com::github::javaparser::ParserConfiguration::LanguageLevel::POPULAR;
use crate::com::github::javaparser::UnicodeEscapeProcessingProvider::PositionMapping;
use crate::com::github::javaparser::ast::CompilationUnit;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::validator::ProblemReporter;
use crate::com::github::javaparser::ast::validator::Validator;
use crate::com::github::javaparser::ast::validator::language_level_validations;
use crate::com::github::javaparser::ast::validator::postprocessors;
use crate::com::github::javaparser::printer::lexicalpreservation::DefaultLexicalPreservingPrinter;
use crate::com::github::javaparser::printer::lexicalpreservation::LexicalPreservingPrinter;
use crate::com::github::javaparser::resolution::SymbolResolver;
use crate::com::github::javaparser::utils::LineSeparator;
use java::nio::charset::Charset;
use java::util::ArrayList;
use java::util::Arrays;
use java::util::List;
use java::util::Optional;
use java::util::function::Supplier;

pub struct ParserConfiguration {
	detect_original_line_separator: bool = true,
	store_tokens: bool = true,
	attribute_comments: bool = true,
	do_not_assign_comments_preceding_empty_lines: bool = true,
	ignore_annotations_when_attributing_comments: bool = false,
	lexical_preservation_enabled: bool = false,
	preprocess_unicode_escapes: bool = false,
	symbol_resolver: com::github::javaparser::resolution::symbol_resolver::SymbolResolver = null,
	tab_size: i32 = 1,
	language_level: com::github::javaparser::parser_configuration::LanguageLevel = POPULAR,
	character_encoding: /* Java */ java::nio::charset::Charset /**/ = Providers::UTF8,
	processors: /* Java */ java::util::List /**/ = ArrayList<>::new(),
}

impl ParserConfiguration {
	pub fn new() -> com::github::javaparser::parser_configuration::ParserConfiguration {
		self.processors.add(|()|ParserConfiguration.UnicodeEscapeProcessor::new());
		self.processors.add(|()|ParserConfiguration.LineEndingProcessor::new());
		self.processors.add(|()|Processor::new() {
			pub fn post_process(&self, result: &ParseResult<? extends Node>, configuration: &ParserConfiguration) {
				if configuration.is_attribute_comments() {
					result.if_successful(|result_node|result.get_comments_collection().ifPresent(|comments|CommentsInserter::new(configuration).insert_comments(result_node, &comments.copy().get_comments())?));
				}
			}
	
		});
		self.processors.add(|()|Processor::new() {
			pub fn post_process(&self, result: &ParseResult<? extends Node>, configuration: &ParserConfiguration) {
				let language_level: LanguageLevel = self.get_language_level();
				if language_level != null {
					if language_level.postProcessor != null {
						language_level.postProcessor.post_process(result, configuration);
					}
					if language_level.validator != null {
						language_level.validator.accept(&result.get_result().get(), ProblemReporter::new(|new_problem|result.get_problems().add(new_problem)));
					}
				}
			}
	
		});
		self.processors.add(|()|Processor::new() {
			pub fn post_process(&self, result: &ParseResult<? extends Node>, configuration: &ParserConfiguration) {
				configuration.get_symbol_resolver().ifPresent(|symbol_resolver|result.if_successful(|result_node|{
					if result_node instanceof CompilationUnit {
						result_node.set_data(Node::SYMBOL_RESOLVER_KEY, symbol_resolver);
					}
				}));
			}
	
		});
		self.processors.add(|()|Processor::new() {
			pub fn post_process(&self, result: &ParseResult<? extends Node>, configuration: &ParserConfiguration) {
				if configuration.is_lexical_preservation_enabled() {
					result.if_successful(|result_node|{
						LexicalPreservingPrinter::setup(result_node);
						result_node.set_data(Node::PRINTER_KEY, DefaultLexicalPreservingPrinter::new());
					});
				}
			}
	
		});
	}

	pub fn post_process(&self, result: &com::github::javaparser::parse_result::ParseResult, configuration: &com::github::javaparser::parser_configuration::ParserConfiguration) {
		if configuration.is_attribute_comments() {
			result.if_successful(|result_node|result.get_comments_collection().ifPresent(|comments|CommentsInserter::new(configuration).insert_comments(result_node, &comments.copy().get_comments())?));
		}
	}

	pub fn post_process(&self, result: &com::github::javaparser::parse_result::ParseResult, configuration: &com::github::javaparser::parser_configuration::ParserConfiguration) {
		let language_level: LanguageLevel = self.get_language_level();
		if language_level != null {
			if language_level.postProcessor != null {
				language_level.postProcessor.post_process(result, configuration);
			}
			if language_level.validator != null {
				language_level.validator.accept(&result.get_result().get(), ProblemReporter::new(|new_problem|result.get_problems().add(new_problem)));
			}
		}
	}

	pub fn post_process(&self, result: &com::github::javaparser::parse_result::ParseResult, configuration: &com::github::javaparser::parser_configuration::ParserConfiguration) {
		configuration.get_symbol_resolver().ifPresent(|symbol_resolver|result.if_successful(|result_node|{
			if result_node instanceof CompilationUnit {
				result_node.set_data(Node::SYMBOL_RESOLVER_KEY, symbol_resolver);
			}
		}));
	}

	pub fn post_process(&self, result: &com::github::javaparser::parse_result::ParseResult, configuration: &com::github::javaparser::parser_configuration::ParserConfiguration) {
		if configuration.is_lexical_preservation_enabled() {
			result.if_successful(|result_node|{
				LexicalPreservingPrinter::setup(result_node);
				result_node.set_data(Node::PRINTER_KEY, DefaultLexicalPreservingPrinter::new());
			});
		}
	}

	pub fn is_attribute_comments(&self) -> bool {
		return self.attribute_comments;
	}

	pub fn set_attribute_comments(&mut self, attribute_comments: bool) -> com::github::javaparser::parser_configuration::ParserConfiguration {
		self.attributeComments = attribute_comments;
		return self;
	}

	pub fn is_do_not_assign_comments_preceding_empty_lines(&self) -> bool {
		return self.do_not_assign_comments_preceding_empty_lines;
	}

	pub fn set_do_not_assign_comments_preceding_empty_lines(&mut self, do_not_assign_comments_preceding_empty_lines: bool) -> com::github::javaparser::parser_configuration::ParserConfiguration {
		self.doNotAssignCommentsPrecedingEmptyLines = do_not_assign_comments_preceding_empty_lines;
		return self;
	}

	pub fn is_ignore_annotations_when_attributing_comments(&self) -> bool {
		return self.ignore_annotations_when_attributing_comments;
	}

	pub fn set_ignore_annotations_when_attributing_comments(&mut self, ignore_annotations_when_attributing_comments: bool) -> com::github::javaparser::parser_configuration::ParserConfiguration {
		self.ignoreAnnotationsWhenAttributingComments = ignore_annotations_when_attributing_comments;
		return self;
	}

	pub fn set_store_tokens(&mut self, store_tokens: bool) -> com::github::javaparser::parser_configuration::ParserConfiguration {
		self.storeTokens = store_tokens;
		if !store_tokens {
			self.set_attribute_comments(false);
		}
		return self;
	}

	pub fn is_store_tokens(&self) -> bool {
		return self.store_tokens;
	}

	pub fn get_tab_size(&self) -> i32 {
		return self.tab_size;
	}

	pub fn set_tab_size(&mut self, tab_size: i32) -> com::github::javaparser::parser_configuration::ParserConfiguration {
		self.tabSize = tab_size;
		return self;
	}

	pub fn set_lexical_preservation_enabled(&mut self, lexical_preservation_enabled: bool) -> com::github::javaparser::parser_configuration::ParserConfiguration {
		self.lexicalPreservationEnabled = lexical_preservation_enabled;
		return self;
	}

	pub fn is_lexical_preservation_enabled(&self) -> bool {
		return self.lexical_preservation_enabled;
	}

	pub fn get_symbol_resolver(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.symbol_resolver);
	}

	pub fn set_symbol_resolver(&mut self, symbol_resolver: &com::github::javaparser::resolution::symbol_resolver::SymbolResolver) -> com::github::javaparser::parser_configuration::ParserConfiguration {
		self.symbolResolver = symbol_resolver;
		return self;
	}

	pub fn get_processors(&self) -> /* Java */ java::util::List /**/ {
		return self.processors;
	}

	pub fn set_language_level(&mut self, language_level: &com::github::javaparser::parser_configuration::LanguageLevel) -> com::github::javaparser::parser_configuration::ParserConfiguration {
		self.languageLevel = language_level;
		return self;
	}

	pub fn get_language_level(&self) -> com::github::javaparser::parser_configuration::LanguageLevel {
		return self.language_level;
	}

	pub fn set_preprocess_unicode_escapes(&mut self, preprocess_unicode_escapes: bool) -> com::github::javaparser::parser_configuration::ParserConfiguration {
		self.preprocessUnicodeEscapes = preprocess_unicode_escapes;
		return self;
	}

	pub fn is_preprocess_unicode_escapes(&self) -> bool {
		return self.preprocess_unicode_escapes;
	}

	pub fn set_detect_original_line_separator(&mut self, detect_original_line_separator: bool) -> com::github::javaparser::parser_configuration::ParserConfiguration {
		self.detectOriginalLineSeparator = detect_original_line_separator;
		return self;
	}

	pub fn is_detect_original_line_separator(&self) -> bool {
		return self.detect_original_line_separator;
	}

	pub fn get_character_encoding(&self) -> /* Java */ java::nio::charset::Charset /**/ {
		return self.character_encoding;
	}

	pub fn set_character_encoding(&mut self, character_encoding: &/* Java */ java::nio::charset::Charset /**/) -> com::github::javaparser::parser_configuration::ParserConfiguration {
		self.characterEncoding = character_encoding;
		return self;
	}
}

pub enum LanguageLevel {
	validator: com::github::javaparser::ast::validator::validator::Validator,
	post_processor: com::github::javaparser::ast::validator::postprocessors::post_processors::PostProcessors,
}

struct UnicodeEscapeProcessor {
	_unicode_decoder: com::github::javaparser::unicode_escape_processing_provider::UnicodeEscapeProcessingProvider,
}

impl UnicodeEscapeProcessor {
	pub fn pre_process(&mut self, inner_provider: &com::github::javaparser::provider::Provider) -> com::github::javaparser::provider::Provider {
		if self.is_preprocess_unicode_escapes() {
			self._unicodeDecoder = UnicodeEscapeProcessingProvider::new(inner_provider);
			return self._unicodeDecoder;
		}
		return inner_provider;
	}

	pub fn post_process(&self, result: &com::github::javaparser::parse_result::ParseResult, configuration: &com::github::javaparser::parser_configuration::ParserConfiguration) {
		if self.is_preprocess_unicode_escapes() {
			result.get_result().ifPresent(|root|{
				let mapping: PositionMapping = self._unicodeDecoder.get_position_mapping();
				if !mapping.is_empty() {
					root.walk(|node|node.get_range().ifPresent(|range|node.set_range(&mapping.transform(range))));
				}
			});
		}
	}
}

struct LineEndingProcessor {
	_line_ending_processing_provider: com::github::javaparser::line_ending_processing_provider::LineEndingProcessingProvider,
}

impl LineEndingProcessor {
	pub fn pre_process(&mut self, inner_provider: &com::github::javaparser::provider::Provider) -> com::github::javaparser::provider::Provider {
		if self.is_detect_original_line_separator() {
			self._lineEndingProcessingProvider = LineEndingProcessingProvider::new(inner_provider);
			return self._lineEndingProcessingProvider;
		}
		return inner_provider;
	}

	pub fn post_process(&self, result: &com::github::javaparser::parse_result::ParseResult, configuration: &com::github::javaparser::parser_configuration::ParserConfiguration) {
		if self.is_detect_original_line_separator() {
			result.get_result().ifPresent(|root_node|{
				let detected_line_separator: LineSeparator = self._lineEndingProcessingProvider.get_detected_line_ending();
				// Set the line ending on the root node
				root_node.set_data(Node::LINE_SEPARATOR_KEY, detected_line_separator);
			// // Set the line ending on all children of the root node -- FIXME: Should ignore """textblocks"""
			// rootNode.findAll(Node.class)
			// .forEach(node -> node.setData(Node.LINE_SEPARATOR_KEY, detectedLineSeparator));
			});
		}
	}
}