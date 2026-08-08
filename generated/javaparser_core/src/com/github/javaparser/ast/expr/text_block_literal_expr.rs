use crate::com::github::javaparser::utils::StringEscapeUtils::unescapeJavaTextBlock;
use java::util::stream::Collectors::joining;
use java::util::stream::IntStream::range;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::TextBlockLiteralExprMetaModel;
use crate::com::github::javaparser::utils::Pair;
use java::util::Arrays;
use java::util::Optional;
use java::util::function::Consumer;
use java::util::stream::Stream;

pub struct TextBlockLiteralExpr;

impl TextBlockLiteralExpr {
	pub fn new() -> com::github::javaparser::ast::expr::text_block_literal_expr::TextBlockLiteralExpr {
		this(null, "empty");
	}

	pub fn new(value: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::text_block_literal_expr::TextBlockLiteralExpr {
		this(null, value);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, value: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::text_block_literal_expr::TextBlockLiteralExpr {
		super(token_range, value);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn is_text_block_literal_expr(&self) -> bool {
		return true;
	}

	pub fn as_text_block_literal_expr(&self) -> com::github::javaparser::ast::expr::text_block_literal_expr::TextBlockLiteralExpr {
		return self;
	}

	pub fn to_text_block_literal_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn if_text_block_literal_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::text_block_literal_expr::TextBlockLiteralExpr {
		return self.accept(CloneVisitor::new(), null) as TextBlockLiteralExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::text_block_literal_expr_meta_model::TextBlockLiteralExprMetaModel {
		return JavaParserMetaModel::textBlockLiteralExprMetaModel;
	}

	pub fn strip_indent_of_lines(&self) -> /* Java */ java::util::stream::Stream /**/ {
		/*  Split the content of the text block at every LF, producing a list of individual lines.
	        Note that any line in the content which was just an LF will become an empty line in the list of individual lines. */ 
		let raw_lines: Vec<String> = self.get_value().split("\\R", -1);
		/*  Add all non-blank lines from the list of individual lines into a set of determining lines.
	        (Blank lines -- lines that are empty or are composed wholly of white space -- have no visible influence on the indentation.
	        Excluding blank lines from the set of determining lines avoids throwing off step 4 of the algorithm.) */ 
		/*  If the last line in the list of individual lines (i.e., the line with the closing delimiter) is blank, then add it to the set of determining lines.
	        (The indentation of the closing delimiter should influence the indentation of the content as a whole -- a "significant trailing line" policy.) */ 
		/*  Compute the common white space prefix of the set of determining lines, by counting the number of leading white space characters on each line and taking the minimum count. */ 
		let common_white_space_prefix_size: i32 = /* Java */ java::util::stream::IntStream /**/::range(0, raw_lines.length).mapToObj(|nr|Pair<>::new(nr, raw_lines[nr])).filter(|l|!.emptyOrWhitespace(l.b) || .isLastLine(raw_lines, l.a)).map(|l|.indentSize(l.b)).min(Integer::compare).orElse(0);
		/*  Remove all trailing white space from all lines in the modified list of individual lines from step 5.
	        This step collapses wholly-whitespace lines in the modified list so that they are empty, but does not discard them. */ 
		return Arrays::stream(raw_lines).map(|l| if l.length() < common_white_space_prefix_size { l } else { l.substring(common_white_space_prefix_size) }).map(self::trimTrailing);
	}

	pub fn strip_indent(&self) -> /* Java */ java::lang::String /**/ {
		/*  Construct the result string by joining all the lines in the modified list of individual lines from step 6, using LF as the separator between lines.
	        If the final line in the list from step 6 is empty, then the joining LF from the previous line will be the last character in the result string. */ 
		return self.strip_indent_of_lines().collect(&/* Java */ java::util::stream::Collectors /**/::joining("\n"));
	}

	pub fn translate_escapes(&self) -> /* Java */ java::lang::String /**/ {
		return com::github::javaparser::utils::string_escape_utils::StringEscapeUtils::unescape_java_text_block(&self.strip_indent());
	}

	pub fn as_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.translate_escapes();
	}

	fn is_last_line(&self, raw_lines: &&[/* Java */ java::lang::String /**/], line_nr: &/* Java */ java::lang::Integer /**/) -> bool {
		return line_nr == raw_lines.length - 1;
	}

	fn empty_or_whitespace(&self, raw_line: &/* Java */ java::lang::String /**/) -> bool {
		return raw_line.trim().isEmpty();
	}

	fn indent_size(&self, s: &/* Java */ java::lang::String /**/) -> i32 {
		let content: String = s.trim();
		if content.isEmpty() {
			return s.length();
		}
		return s.indexOf(content);
	}

	fn trim_trailing(&self, source: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		let pos: i32 = source.length() - 1;
		while (pos >= 0) && Character::isWhitespace(&source.charAt(pos)) {
			pos -= 1;
		}
		pos += 1;
		return  if (pos < source.length()) { source.substring(0, pos) } else { source };
	}
}

impl /* Java */ java::lang::Cloneable /**/ for TextBlockLiteralExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for TextBlockLiteralExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for TextBlockLiteralExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for TextBlockLiteralExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for TextBlockLiteralExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for TextBlockLiteralExpr {}