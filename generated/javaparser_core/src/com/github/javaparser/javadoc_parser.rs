use crate::com::github::javaparser::utils::Utils;
use crate::com::github::javaparser::ast::comments::JavadocComment;
use crate::com::github::javaparser::javadoc::Javadoc;
use crate::com::github::javaparser::javadoc::JavadocBlockTag;
use crate::com::github::javaparser::javadoc::description::JavadocDescription;
use crate::com::github::javaparser::utils::LineSeparator;
use java::util::Arrays;
use java::util::Collections;
use java::util::List;
use java::util::regex::Pattern;
use java::util::stream::Collectors;

struct JavadocParser;

impl JavadocParser {
	static BLOCK_TAG_PREFIX: /* Java */ java::lang::String /**/ = "@";

	static BLOCK_PATTERN: /* Java */ java::util::regex::Pattern /**/ = Pattern::compile("^\\s*" + BLOCK_TAG_PREFIX, Pattern::MULTILINE);

	pub fn parse(&self, comment: &com::github::javaparser::ast::comments::javadoc_comment::JavadocComment) -> com::github::javaparser::javadoc::javadoc::Javadoc {
		return com::github::javaparser::javadoc_parser::JavadocParser::parse(&comment.get_content(), &comment.is_markdown_comment());
	}

	pub fn parse(&self, comment_content: &/* Java */ java::lang::String /**/) -> com::github::javaparser::javadoc::javadoc::Javadoc {
		return com::github::javaparser::javadoc_parser::JavadocParser::parse(comment_content, false);
	}

	pub fn parse(&self, comment_content: &/* Java */ java::lang::String /**/, is_markdown_comment: bool) -> com::github::javaparser::javadoc::javadoc::Javadoc {
		let clean_lines: List<String> = com::github::javaparser::javadoc_parser::JavadocParser::clean_lines(&com::github::javaparser::utils::utils::Utils::normalize_eol_in_text_block(comment_content, LineSeparator::SYSTEM), is_markdown_comment);
		let index_of_first_block_tag: i32 = clean_lines.stream().filter(JavadocParser::isABlockLine).map(cleanLines::indexOf).findFirst().orElse(-1);
		let block_lines: List<String>;
		let description_text: String;
		if index_of_first_block_tag == -1 {
			description_text = com::github::javaparser::javadoc_parser::JavadocParser::trim_right(&String::join(&LineSeparator::SYSTEM.as_raw_string(), clean_lines));
			block_lines = Collections::emptyList();
		} else {
			description_text = com::github::javaparser::javadoc_parser::JavadocParser::trim_right(&String::join(&LineSeparator::SYSTEM.as_raw_string(), &clean_lines.subList(0, index_of_first_block_tag)));
			// Combine cleaned lines, but only starting with the first block tag till the end
			// In this combined string it is easier to handle multiple lines which actually belong together
			let tag_block: String = clean_lines.subList(index_of_first_block_tag, &clean_lines.size()).stream().collect(&Collectors::joining(&LineSeparator::SYSTEM.as_raw_string()));
			// Split up the entire tag back again, considering now that some lines belong to the same block tag.
			// The pattern splits the block at each new line starting with the '@' symbol, thus the symbol
			// then needs to be added again so that the block parsers handles everything correctly.
			block_lines = self.BLOCK_PATTERN.splitAsStream(tag_block).filter(|s1|!s1.isEmpty()).map(|s|self.BLOCK_TAG_PREFIX + s).collect(&Collectors::toList());
		}
		let document: Javadoc = Javadoc::new(&JavadocDescription::parse_text(description_text)?, is_markdown_comment);
		block_lines.forEach(|l|document.add_block_tag(&com::github::javaparser::javadoc_parser::JavadocParser::parse_block_tag(l)));
		return document;
	}

	fn parse_block_tag(&self, mut line: &/* Java */ java::lang::String /**/) -> com::github::javaparser::javadoc::javadoc_block_tag::JavadocBlockTag {
		line = line.trim().substring(1);
		let tag_name: String = com::github::javaparser::utils::utils::Utils::next_word(line);
		let rest: String = line.substring(&tag_name.length()).trim();
		return JavadocBlockTag::new(tag_name, rest);
	}

	fn isa_block_line(&self, line: &/* Java */ java::lang::String /**/) -> bool {
		return line.trim().startsWith(self.BLOCK_TAG_PREFIX);
	}

	fn trim_right(&self, mut string: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		while !string.isEmpty() && Character::isWhitespace(&string.charAt(string.length() - 1)) {
			string = string.substring(0, string.length() - 1);
		}
		return string;
	}

	fn clean_lines(&self, content: &/* Java */ java::lang::String /**/, is_markdown_comment: bool) -> /* Java */ java::util::List /**/ {
		let lines: Vec<String> = content.split(&LineSeparator::SYSTEM.as_raw_string());
		if lines.length == 0 {
			return Collections::emptyList();
		}
		let cleaned_lines: List<String> = Arrays::stream(lines).map(|l|{
			let asterisk_or_last_md_slash_index: i32 = com::github::javaparser::javadoc_parser::JavadocParser::starts_with_asterisk_or_md_slash(l);
			if asterisk_or_last_md_slash_index == -1 {
				return l;
			}
			if l.length() > (asterisk_or_last_md_slash_index + 1) {
				let c: char = l.charAt(asterisk_or_last_md_slash_index + 1);
				if c == ' ' || c == '\t' {
					return l.substring(asterisk_or_last_md_slash_index + 2);
				}
			}
			return l.substring(asterisk_or_last_md_slash_index + 1);
		}).collect(&Collectors::toList());
		// lines containing only whitespace are normalized to empty lines
		cleaned_lines = cleaned_lines.stream().map(|l| if l.trim().isEmpty() { "" } else { l }).collect(&Collectors::toList());
		// if the first starts with a space, remove it
		if !cleaned_lines.get(0).isEmpty() && (cleaned_lines.get(0).charAt(0) == ' ' || cleaned_lines.get(0).charAt(0) == '\t') {
			cleaned_lines.set(0, &cleaned_lines.get(0).substring(1));
		}
		// drop empty lines at the beginning and at the end
		while cleaned_lines.size() > 0 && cleaned_lines.get(0).trim().isEmpty() {
			cleaned_lines = cleaned_lines.subList(1, &cleaned_lines.size());
		}
		while cleaned_lines.size() > 0 && cleaned_lines.get(cleaned_lines.size() - 1).trim().isEmpty() {
			cleaned_lines = cleaned_lines.subList(0, cleaned_lines.size() - 1);
		}
		return cleaned_lines;
	}

	fn starts_with_asterisk_or_md_slash(&self, line: &/* Java */ java::lang::String /**/) -> i32 {
		 {
			let i: i32 = 0; let md_slash_count: i32 = 0;
			while i < line.length() {
				{
					let current_char: char = line.charAt(i);
					if current_char == '/' {
						if md_slash_count == 2 {
							return i;
						} else {
							md_slash_count += 1;
						}
					} else if current_char == '*' && md_slash_count == 0 {
						return i;
					} else if current_char != ' ' && current_char != '\t' {
						return -1;
					}
				}
				i += 1;
			 }
		 }
	
		return -1;
	}
}