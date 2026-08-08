use crate::com::github::javaparser::GeneratedJavaParserConstants;
use crate::com::github::javaparser::TokenTypes::eolTokenKind;
use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::utils::Utils::decapitalize;
use java::util::Comparator::comparing;
use java::util::stream::Collectors::toList;
use crate::com::github::javaparser::JavaToken;
use crate::com::github::javaparser::Range;
use crate::com::github::javaparser::ast::DataKey;
use crate::com::github::javaparser::ast::Modifier;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::body::VariableDeclarator;
use crate::com::github::javaparser::ast::comments;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithVariables;
use crate::com::github::javaparser::ast::observer::AstObserver;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::observer::PropagatingAstObserver;
use crate::com::github::javaparser::ast::type::PrimitiveType;
use crate::com::github::javaparser::ast::visitor::TreeVisitor;
use crate::com::github::javaparser::printer::ConcreteSyntaxModel;
use crate::com::github::javaparser::printer::Printer;
use crate::com::github::javaparser::printer::concretesyntaxmodel;
use crate::com::github::javaparser::printer::lexicalpreservation::LexicalDifferenceCalculator::CsmChild;
use crate::com::github::javaparser::utils::LineSeparator;
use crate::com::github::javaparser::utils::Pair;
use java::lang::reflect::InvocationTargetException;
use java::lang::reflect::Method;
use java::lang::reflect::ParameterizedType;
use java::util;

pub struct LexicalPreservingPrinter;

impl LexicalPreservingPrinter {
	static JAVA_UTIL_OPTIONAL: /* Java */ java::lang::String /**/ = Optional.class.getCanonicalName();

	static JAVAPARSER_AST_NODELIST: /* Java */ java::lang::String /**/ = NodeList.class.getCanonicalName();

	static observer: com::github::javaparser::ast::observer::ast_observer::AstObserver;

	pub static NODE_TEXT_DATA: com::github::javaparser::ast::data_key::DataKey = DataKey<NodeText>::new() {
	};

	static LEXICAL_DIFFERENCE_CALCULATOR: com::github::javaparser::printer::lexicalpreservation::lexical_difference_calculator::LexicalDifferenceCalculator = LexicalDifferenceCalculator::new();

	pub fn setup<N: com::github::javaparser::ast::node::Node>(&mut self, node: &N) /* thrown(java.lang.AssertionError) */ -> N {
		com::github::javaparser::utils::utils::Utils::assert_not_null(node)?;
		if self.observer == null {
			self.observer = com::github::javaparser::printer::lexicalpreservation::lexical_preserving_printer::LexicalPreservingPrinter::create_observer();
		}
		node.get_token_range().ifPresent(|r|{
			com::github::javaparser::printer::lexicalpreservation::lexical_preserving_printer::LexicalPreservingPrinter::store_initial_text(node);
			// Setup observer
			if !node.is_registered(self.observer) {
				node.register_for_subtree(self.observer);
			}
		});
		return node;
	}

	pub fn is_available_on(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		return node.contains_data(self.NODE_TEXT_DATA);
	}

	fn create_observer(&self) -> com::github::javaparser::ast::observer::ast_observer::AstObserver {
		return LexicalPreservingPrinter.Observer::new();
	}

	fn store_initial_text(&self, root: &com::github::javaparser::ast::node::Node) {
		let tokens_by_node: Map<Node, List<JavaToken>> = IdentityHashMap<>::new();
		// We go over tokens and find to which nodes they belong. Note that we do not traverse the tokens as they were
		// on a list but as they were organized in a tree. At each time we select only the branch corresponding to the
		// range of interest and ignore all other branches
		root.get_token_range().ifPresent(|root_token_range|{
			for token in root_token_range {
				let token_range: Range = token.get_range().orElseThrow(|()|RuntimeException::new("Token without range: " + token));
				let owner: Node = root.find_by_range(token_range).orElseThrow(|()|RuntimeException::new("Token without node owning it: " + token));
				if !tokens_by_node.containsKey(owner) {
					tokens_by_node.put(owner, LinkedList<>::new());
				}
				tokens_by_node.get(owner).add(token);
			}
			// Now that we know the tokens we use them to create the initial NodeText for each node
			TreeVisitor::new() {
				pub fn process(&self, node: &Node) {
					if !node.is_phantom() {
						com::github::javaparser::printer::lexicalpreservation::lexical_preserving_printer::LexicalPreservingPrinter::store_initial_text_for_one_node(node, &tokens_by_node.get(node))?;
					}
				}
	
			}.visit_breadth_first(root);
		});
	}

	fn store_initial_text_for_one_node(&self, node: &com::github::javaparser::ast::node::Node, mut node_tokens: &/* Java */ java::util::List /**/) /* thrown(java.lang.RuntimeException) */ {
		if node_tokens == null {
			node_tokens = Collections::emptyList();
		}
		let elements: List<Pair<Range, TextElement>> = LinkedList<>::new();
		for child in node.get_child_nodes() {
			if !child.is_phantom() {
				if !child.has_range() {
					return Err(RuntimeException::new("Range not present on node " + child));
				}
				elements.add(Pair<>::new(&child.get_range().get(), ChildTextElement::new(child)));
			}
		}
		for token in node_tokens {
			elements.add(Pair<>::new(&token.get_range().get(), TokenTextElement::new(token)));
		}
		elements.sort(&/* Java */ java::util::Comparator /**/::comparing(|e|e.a.begin));
		node.set_data(self.NODE_TEXT_DATA, NodeText::new(&elements.stream().map(|p|p.b).collect(&/* Java */ java::util::stream::Collectors /**/::toList())));
	}

	fn tokens_preceeding(&self, node: &com::github::javaparser::ast::node::Node) -> /* Java */ java::util::Iterator /**/ {
		if !node.get_parent_node().isPresent() {
			return TextElementIteratorsFactory.EmptyIterator<>::new();
		}
		// There is the awfully painful case of the fake types involved in variable declarators and
		// fields or variable declaration that are, of course, an exception...
		let parent_node_text: NodeText = com::github::javaparser::printer::lexicalpreservation::lexical_preserving_printer::LexicalPreservingPrinter::get_or_create_node_text(&node.get_parent_node().get());
		let index: i32 = parent_node_text.try_to_find_child(node);
		if index == NodeText::NOT_FOUND {
			if node.get_parent_node().get() instanceof VariableDeclarator {
				return com::github::javaparser::printer::lexicalpreservation::lexical_preserving_printer::LexicalPreservingPrinter::tokens_preceeding(&node.get_parent_node().get());
			}
			return TextElementIteratorsFactory.EmptyIterator<TokenTextElement>::new();
		}
		return TextElementIteratorsFactory.CascadingIterator<>::new(&TextElementIteratorsFactory::partial_reverse_iterator(parent_node_text, index - 1), |()|com::github::javaparser::printer::lexicalpreservation::lexical_preserving_printer::LexicalPreservingPrinter::tokens_preceeding(&node.get_parent_node().get()));
	}

	pub fn print(&self, node: &com::github::javaparser::ast::node::Node) -> /* Java */ java::lang::String /**/ {
		let printer: Printer = DefaultLexicalPreservingPrinter::new();
		return printer.print(node);
	}

	fn pretty_printing_text_node(&self, node: &com::github::javaparser::ast::node::Node, node_text: &com::github::javaparser::printer::lexicalpreservation::node_text::NodeText) /* thrown(java.lang.UnsupportedOperationException) */ {
		if node instanceof PrimitiveType {
			com::github::javaparser::printer::lexicalpreservation::lexical_preserving_printer::LexicalPreservingPrinter::interpret(node, &ConcreteSyntaxModel::for_class(&node.getClass())?, node_text)?;
			return;
		}
		if node instanceof TraditionalJavadocComment {
			let comment: Comment = node as TraditionalJavadocComment;
			node_text.add_token(, comment.get_header() + (node as TraditionalJavadocComment).get_content() + comment.get_footer());
			return;
		}
		if node instanceof BlockComment {
			let comment: Comment = node as BlockComment;
			node_text.add_token(, comment.get_header() + (node as BlockComment).get_content() + comment.get_footer());
			return;
		}
		if node instanceof LineComment {
			let comment: Comment = node as LineComment;
			node_text.add_token(, comment.get_header() + comment.get_content());
			return;
		}
		if node instanceof Modifier {
			let modifier: Modifier = node as Modifier;
			node_text.add_token(&LexicalDifferenceCalculator::to_token(modifier)?, &modifier.get_keyword().as_string());
			return;
		}
		com::github::javaparser::printer::lexicalpreservation::lexical_preserving_printer::LexicalPreservingPrinter::interpret(node, &ConcreteSyntaxModel::for_class(&node.getClass())?, node_text)?;
	}

	fn interpret(&self, node: &com::github::javaparser::ast::node::Node, csm: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement, node_text: &com::github::javaparser::printer::lexicalpreservation::node_text::NodeText) /* thrown(java.lang.UnsupportedOperationException) */ -> com::github::javaparser::printer::lexicalpreservation::node_text::NodeText {
		let calculated_syntax_model: LexicalDifferenceCalculator.CalculatedSyntaxModel = LexicalDifferenceCalculator::new().calculated_syntax_model_for_node(csm, node)?;
		let indentation: List<TextElement> = com::github::javaparser::printer::lexicalpreservation::lexical_preserving_printer::LexicalPreservingPrinter::find_indentation(node);
		let pending_indentation: bool = false;
		// Add a comment and line separator if necessary
		node.get_comment().ifPresent(|comment|{
			// new comment has no range so in this case we want to force the comment before the node
			if !comment.has_range() {
				let line_separator: LineSeparator = node.get_line_ending_style_or_default(LineSeparator::SYSTEM);
				calculated_syntax_model.elements.add(0, CsmToken::new(&com::github::javaparser::token_types::TokenTypes::eol_token_kind(line_separator)?, &line_separator.as_raw_string()));
				calculated_syntax_model.elements.add(0, CsmChild::new(comment));
			}
		});
		for element in calculated_syntax_model.elements {
			if element instanceof CsmIndent {
				let index_current_element: i32 = calculated_syntax_model.elements.indexOf(element);
				if calculated_syntax_model.elements.size() > index_current_element && !(calculated_syntax_model.elements.get(index_current_element + 1) instanceof CsmUnindent) {
					 {
						let i: i32 = 0;
						while i < Difference::STANDARD_INDENTATION_SIZE {
							{
								indentation.add(TokenTextElement::new(, " "));
							}
							i += 1;
						 }
					 }
	
				}
			} else if element instanceof CsmUnindent {
				 {
					let i: i32 = 0;
					while i < Difference::STANDARD_INDENTATION_SIZE && indentation.size() > 0 {
						{
							indentation.remove(indentation.size() - 1);
						}
						i += 1;
					 }
				 }
	
			}
			if pending_indentation && !(element instanceof CsmToken && (element as CsmToken).is_new_line()) {
				indentation.forEach(nodeText::addElement);
			}
			pending_indentation = false;
			if element instanceof LexicalDifferenceCalculator.CsmChild {
				node_text.add_child(&(element as LexicalDifferenceCalculator.CsmChild).get_child());
			} else if element instanceof CsmToken {
				let csm_token: CsmToken = element as CsmToken;
				node_text.add_token(&csm_token.get_token_type(), &csm_token.get_content());
				if csm_token.is_new_line() {
					pending_indentation = true;
				}
			} else if element instanceof CsmMix {
				let csm_mix: CsmMix = element as CsmMix;
				csm_mix.get_elements().forEach(|e|com::github::javaparser::printer::lexicalpreservation::lexical_preserving_printer::LexicalPreservingPrinter::interpret(node, e, node_text)?);
			} else {
				// following lines
				if !(element instanceof CsmIndent) && !(element instanceof CsmUnindent) {
					return Err(UnsupportedOperationException::new("Unknown element type: " + element.getClass().getSimpleName()));
				}
			}
		}
		// so they have to be handled in a special way
		if node instanceof VariableDeclarator {
			let variable_declarator: VariableDeclarator = node as VariableDeclarator;
			variable_declarator.get_parent_node().ifPresent(|parent|(parent as NodeWithVariables<?>).get_maximum_common_type().ifPresent(|mct|{
				let extra_array_levels: i32 = variable_declarator.get_type().get_array_level() - mct.get_array_level();
				 {
					let i: i32 = 0;
					while i < extra_array_levels {
						{
							node_text.add_element(TokenTextElement::new());
							node_text.add_element(TokenTextElement::new());
						}
						i += 1;
					 }
				 }
	
			}));
		}
		return node_text;
	}

	fn get_or_create_node_text(&self, node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.UnsupportedOperationException | java.lang.IllegalStateException) */ -> com::github::javaparser::printer::lexicalpreservation::node_text::NodeText {
		if !node.contains_data(self.NODE_TEXT_DATA) {
			let node_text: NodeText = NodeText::new();
			node.set_data(self.NODE_TEXT_DATA, node_text);
			com::github::javaparser::printer::lexicalpreservation::lexical_preserving_printer::LexicalPreservingPrinter::pretty_printing_text_node(node, node_text)?;
		}
		return node.get_data(self.NODE_TEXT_DATA)?;
	}

	fn find_indentation(&self, node: &com::github::javaparser::ast::node::Node) -> /* Java */ java::util::List /**/ {
		let following_newlines: List<TextElement> = LinkedList<>::new();
		let it: Iterator<TokenTextElement> = com::github::javaparser::printer::lexicalpreservation::lexical_preserving_printer::LexicalPreservingPrinter::tokens_preceeding(node);
		while it.hasNext() {
			let tte: TokenTextElement = it.next();
			if tte.get_token_kind() ==  || tte.is_newline() {
				break;
			}
			following_newlines.add(tte);
		}
		Collections::reverse(following_newlines);
		 {
			let i: i32 = 0;
			while i < following_newlines.size() {
				{
					if !following_newlines.get(i).is_space_or_tab() {
						return following_newlines.subList(0, i);
					}
				}
				i += 1;
			 }
		 }
	
		return following_newlines;
	}

	fn is_returning_optional_node_list(&self, m: &/* Java */ java::lang::reflect::Method /**/) -> bool {
		if !m.getReturnType().getCanonicalName().equals(self.JAVA_UTIL_OPTIONAL) {
			return false;
		}
		if !(m.getGenericReturnType() instanceof ParameterizedType) {
			return false;
		}
		let parameterized_type: ParameterizedType = m.getGenericReturnType() as ParameterizedType;
		let optional_argument: java.lang.reflect.Type = parameterized_type.getActualTypeArguments()[0];
		return (optional_argument.getTypeName().startsWith(self.JAVAPARSER_AST_NODELIST));
	}

	fn find_node_list_name(&self, node_list: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.IllegalStateException | java.lang.RuntimeException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::observer::observable_property::ObservableProperty {
		let parent: Node = node_list.get_parent_node_for_children();
		for m in parent.getClass().getMethods() {
			if m.getParameterCount() == 0 && m.getReturnType().getCanonicalName().equals(self.JAVAPARSER_AST_NODELIST) {
				let r0 = 'try0: {
					let raw: Object = m.invoke(parent);
					if !(raw instanceof NodeList) {
						break 'try0 Err(IllegalStateException::new("Expected NodeList, found " + raw.getClass().getCanonicalName()));
					}
					let result: NodeList<?> = raw as NodeList<?>;
					if result == node_list {
						let name: String = m.getName();
						if name.startsWith("get") {
							name = name.substring(&"get".length());
						}
						return ObservableProperty::from_camel_case_name(&com::github::javaparser::utils::utils::Utils::decapitalize(name))?;
					}
					break 'try0 Ok(());
				};
				match r0 {
					Err(e @ IllegalAccessExceptionInvocationTargetException | ) => {
						return Err(RuntimeException::new(e));
					},
					Err(e) => Err(e)?,
					Ok => (),
				}
			} else if m.getParameterCount() == 0 && com::github::javaparser::printer::lexicalpreservation::lexical_preserving_printer::LexicalPreservingPrinter::is_returning_optional_node_list(m) {
				let r0 = 'try0: {
					let raw: Optional<NodeList<?>> = m.invoke(parent) as Optional<NodeList<?>>;
					if raw.isPresent() && raw.get() == node_list {
						let name: String = m.getName();
						if name.startsWith("get") {
							name = name.substring(&"get".length());
						}
						return match ObservableProperty::from_camel_case_name(&com::github::javaparser::utils::utils::Utils::decapitalize(name)) {
							Err(e) => break 'try0 Err(e),
							Ok(s) => s,
						};
					}
					break 'try0 Ok(());
				};
				match r0 {
					Err(e @ IllegalAccessExceptionInvocationTargetException | ) => {
						break 'try0 Err(RuntimeException::new(e));
					},
					Err(e) => Err(e)?,
					Ok => (),
				}
			}
		}
		return Err(IllegalArgumentException::new("Cannot find list name of NodeList of size " + node_list.size()));
	}
}

struct Observer;

impl Observer {
	pub fn concrete_property_change(&self, observed_node: &com::github::javaparser::ast::node::Node, property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, old_value: &/* Java */ java::lang::Object /**/, new_value: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.NullPointerException | java.lang.UnsupportedOperationException | java.lang.AssertionError | java.lang.IllegalStateException) */ {
		if old_value == new_value {
			// Not really a change, ignore
			return;
		}
		if property == ObservableProperty::RANGE || property == ObservableProperty::COMMENTED_NODE {
			return;
		}
		if property == ObservableProperty::COMMENT {
			let parent_node: Optional<Node> = observed_node.get_parent_node();
			let node_text: NodeText = parent_node.map(|parent|com::github::javaparser::printer::lexicalpreservation::lexical_preserving_printer::LexicalPreservingPrinter::get_or_create_node_text(&parent_node.get())?).orElseGet(|()|com::github::javaparser::printer::lexicalpreservation::lexical_preserving_printer::LexicalPreservingPrinter::get_or_create_node_text(observed_node)?);
			if old_value == null {
				// this case corresponds to the addition of a comment
				// Find the position of the comment node and put in front of it the [...]
				let index: i32 =  if parent_node.isPresent() { node_text.find_child(observed_node) } else { 0 };
				/*  Add the same indentation to the comment as the previous node
	                     * for example if we want to add a comment on the body of the method declaration :
	                     * Actual code
	                     * {@code
	                     * public class Foo {
	                     *   void visit(final UnknownType n, final Void arg)
	                     *   {
	                     *   }
	                     * }
	                     * }
	                     * Expected result
	                     * {@code
	                     * public class Foo {
	                     *   void visit(final UnknownType n, final Void arg)
	                     *   //added comment <-- we should insert indentation before the comment
	                     *   {
	                     *   }
	                     * }
	                     * }
	                     */ 
				self.fix_indent_of_added_node(node_text, index - 1);
				let line_separator: LineSeparator = observed_node.get_line_ending_style_or_default(LineSeparator::SYSTEM);
				for element in self.make_comment_tokens(new_value as Comment)? {
					node_text.add_element(index += 1 !!!check!!! post increment, element);
				}
				node_text.add_token(index, &com::github::javaparser::token_types::TokenTypes::eol_token_kind(line_separator)?, &line_separator.as_raw_string());
			// code indentation after inserting an eol token may be wrong
			} else if new_value == null {
				// this case corresponds to a deletion of a comment
				if old_value instanceof Comment {
					if (old_value as Comment).is_orphan() {
						node_text = com::github::javaparser::printer::lexicalpreservation::lexical_preserving_printer::LexicalPreservingPrinter::get_or_create_node_text(observed_node)?;
					}
					let index_and_count: Pair<Integer, Integer> = self.get_index_and_count_of_comment_tokens(old_value as Comment, node_text);
					let index: i32 = index_and_count.a;
					 {
						let i: i32 = 0;
						while i < index_and_count.b {
							{
								node_text.remove_element(index);
							}
							i += 1;
						 }
					 }
	
					if self.is_complete_line(&node_text.get_elements(), index) {
						self.remove_all_extra_characters(&node_text.get_elements(), index);
					} else {
						self.remove_all_extra_characters_starting_from(&node_text.get_elements().listIterator(index));
					}
				} else {
					return Err(UnsupportedOperationException::new("Trying to remove something that is not a comment!"));
				}
			} else {
				// this is a replacement of a comment
				let matching_tokens: List<TokenTextElement> = self.find_token_text_element_for_comment(old_value as Comment, node_text);
				if (old_value instanceof MarkdownComment && matching_tokens.isEmpty()) || (!(old_value instanceof MarkdownComment) && matching_tokens.size() != 1) {
					return Err(IllegalStateException::new("The matching comment to be replaced could not be found"));
				}
				let new_comment: Comment = new_value as Comment;
				let first_matching_element: TokenTextElement = matching_tokens.get(0);
				let index: i32 = node_text.find_element(&first_matching_element.and(&first_matching_element.match_by_range()));
				// When replacing a MarkdownComment, all matching tokens must be removed before adding new ones
				 {
					let i: i32 = 0;
					while i < matching_tokens.size() {
						{
							node_text.remove_element(index);
						}
						i += 1;
					 }
				 }
	
				for new_element in self.make_comment_tokens(new_comment)? {
					node_text.add_element(index += 1 !!!check!!! post increment, new_element);
				}
			}
		}
		let node_text: NodeText = com::github::javaparser::printer::lexicalpreservation::lexical_preserving_printer::LexicalPreservingPrinter::get_or_create_node_text(observed_node)?;
		if node_text == null {
			return Err(NullPointerException::new(&observed_node.getClass().getSimpleName()));
		}
		.calculate_property_change(node_text, observed_node, property, old_value, new_value)?;
	}

	fn is_complete_line(&self, elements: &/* Java */ java::util::List /**/, index: i32) -> bool {
		if index <= 0 || index >= elements.size() {
			return false;
		}
	
		let is_complete_line: bool = true;
		let iterator: ListIterator<TextElement> = elements.listIterator(index);
		// verify if elements after the index are only spaces or tabs
		while iterator.hasNext() {
			let text_element: TextElement = iterator.next();
			if text_element.is_newline() {
				break;
			}
	
			if text_element.is_space_or_tab() {
				continue;
			}
	
			is_complete_line = false;
			break;
		}
		// verify if elements before the index are only spaces or tabs
		iterator = elements.listIterator(index);
		while iterator.hasPrevious() && is_complete_line {
			let text_element: TextElement = iterator.previous();
			if text_element.is_newline() {
				break;
			}
	
			if text_element.is_space_or_tab() {
				continue;
			}
	
			is_complete_line = false;
		}
		return is_complete_line;
	}

	fn remove_all_extra_characters(&self, elements: &/* Java */ java::util::List /**/, index: i32) {
		if index < 0 || index >= elements.size() {
			return;
		}
	
		self.remove_all_extra_characters_starting_from(&elements.listIterator(index));
		self.remove_all_extra_characters_before_position(&elements.listIterator(index));
	}

	fn remove_all_extra_characters_before_position(&self, iterator: &/* Java */ java::util::ListIterator /**/) {
		while iterator.hasPrevious() {
			let text_element: TextElement = iterator.previous();
			if text_element.is_space_or_tab() {
				iterator.remove();
				continue;
			}
			break;
		}
	}

	fn remove_all_extra_characters_starting_from(&self, iterator: &/* Java */ java::util::ListIterator /**/) {
		while iterator.hasNext() {
			let text_element: TextElement = iterator.next();
			if text_element.is_space_or_tab() {
				iterator.remove();
				continue;
			}
			if text_element.is_newline() {
				iterator.remove();
			}
			break;
		}
	}

	fn convert_markdown_comment_content_to_tokens(&self, comment: &com::github::javaparser::ast::comments::markdown_comment::MarkdownComment) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::util::List /**/ {
		let tokens: ArrayList<TokenTextElement> = ArrayList<>::new();
		let content: String = comment.get_content();
		 {
			let i: i32 = 0;
			while i < content.length() {
				{
					if content.charAt(i) == '/' {
						let comment_start: i32 = i;
						while i < content.length() - 1 {
							if content.charAt(i + 1) == '\n' || content.charAt(i + 1) == '\r' {
								break;
							}
							i += 1;
						}
						tokens.add(TokenTextElement::new(, &content.substring(comment_start, i + 1)));
					} else if content.charAt(i) == '\r' {
						if i < content.length() - 1 && content.charAt(i + 1) == '\n' {
							tokens.add(TokenTextElement::new(, "\r\n"));
							i += 1;
						} else {
							tokens.add(TokenTextElement::new(, "\r"));
						}
					} else if Character::isWhitespace(&content.charAt(i)) {
						tokens.add(TokenTextElement::new(, &Character::toString(&content.charAt(i))));
					} else {
						return Err(IllegalArgumentException::new("Expected Markdown comment content format, but got " + comment));
					}
				}
				i += 1;
			 }
		 }
	
		return tokens;
	}

	fn make_comment_tokens(&self, new_comment: &com::github::javaparser::ast::comments::comment::Comment) /* thrown(java.lang.UnsupportedOperationException | java.lang.IllegalStateException | java.lang.IllegalArgumentException) */ -> /* Java */ java::util::List /**/ {
		let tokens: List<TokenTextElement> = ArrayList<>::new();
		if new_comment.is_javadoc_comment() {
			let t: TokenTextElement = TokenTextElement::new(, new_comment.get_header() + new_comment.get_content() + new_comment.get_footer());
			tokens.add(t);
		} else if new_comment.is_line_comment() {
			let t: TokenTextElement = TokenTextElement::new(, new_comment.get_header() + new_comment.get_content());
			tokens.add(t);
		} else if new_comment.is_block_comment() {
			let t: TokenTextElement = TokenTextElement::new(, new_comment.get_header() + new_comment.get_content() + new_comment.get_footer());
			tokens.add(t);
		} else if new_comment.is_markdown_comment() {
			tokens.addAll(&self.convert_markdown_comment_content_to_tokens(&new_comment.as_markdown_comment()?)?);
		} else {
			return Err(UnsupportedOperationException::new("Unknown type of comment: " + new_comment.getClass().getSimpleName()));
		}
		return tokens;
	}

	fn get_index_and_count_of_comment_tokens(&self, old_value: &com::github::javaparser::ast::comments::comment::Comment, node_text: &com::github::javaparser::printer::lexicalpreservation::node_text::NodeText) -> com::github::javaparser::utils::pair::Pair {
		let matching_tokens: List<TokenTextElement> = self.find_token_text_element_for_comment(old_value, node_text);
		if !matching_tokens.isEmpty() {
			let matching_element: TextElement = matching_tokens.get(0);
			return Pair<>::new(&node_text.find_element(&matching_element.and(&matching_element.match_by_range())), &matching_tokens.size());
		}
		// If no matching TokenTextElements were found, we try searching through ChildTextElements as well
		let matching_childs: List<ChildTextElement> = self.find_child_text_element_for_comment(old_value, node_text)?;
		let matching_child: ChildTextElement = matching_childs.get(0);
		return Pair<>::new(&node_text.find_element(&matching_child.and(&matching_child.match_by_range())), &matching_childs.size());
	}

	fn find_child_text_element_for_comment(&self, old_value: &com::github::javaparser::ast::comments::comment::Comment, node_text: &com::github::javaparser::printer::lexicalpreservation::node_text::NodeText) /* thrown(java.lang.IllegalStateException) */ -> /* Java */ java::util::List /**/ {
		let matching_child_elements: List<ChildTextElement>;
		matching_child_elements = self.select_matching_child_elements(old_value, node_text);
		if matching_child_elements.size() > 1 {
			// Duplicate child nodes found, refine the result
			matching_child_elements = matching_child_elements.stream().filter(|t|t.get_child().has_range() && old_value.has_range()).filter(|t|t.get_child().get_range().get().equals(&old_value.get_range().get()) || (t.get_child().get_comment().isPresent() && t.get_child().get_comment().get().has_range() && t.get_child().get_comment().get().get_range().get().equals(&old_value.get_range().get()))).collect(&/* Java */ java::util::stream::Collectors /**/::toList());
		}
		if matching_child_elements.size() != 1 {
			return Err(IllegalStateException::new("The matching child text element for the comment to be removed could not be found."));
		}
		return matching_child_elements;
	}

	fn select_matching_child_elements(&self, old_value: &com::github::javaparser::ast::comments::comment::Comment, node_text: &com::github::javaparser::printer::lexicalpreservation::node_text::NodeText) -> /* Java */ java::util::List /**/ {
		let result: List<ChildTextElement> = ArrayList<>::new();
		let child_text_elements: List<ChildTextElement> = node_text.get_elements().stream().filter(|e|e.is_child()).map(|c|c as ChildTextElement).collect(&/* Java */ java::util::stream::Collectors /**/::toList());
		let iterator: ListIterator<ChildTextElement> = child_text_elements.listIterator();
		while iterator.hasNext() {
			let text_element: ChildTextElement = iterator.next();
			if text_element.is_comment() && self.is_same_comment((text_element.get_child() as Comment), old_value) {
				result.add(text_element);
				continue;
			}
			let node: Node = text_element.get_child();
			if node.get_comment().isPresent() && self.is_same_comment(&node.get_comment().get(), old_value) {
				result.add(text_element);
				continue;
			}
		}
		return result;
	}

	fn is_same_comment(&self, child_value: &com::github::javaparser::ast::comments::comment::Comment, old_value: &com::github::javaparser::ast::comments::comment::Comment) -> bool {
		return child_value.get_content().equals(&old_value.get_content());
	}

	fn find_token_text_element_for_comment(&self, old_value: &com::github::javaparser::ast::comments::comment::Comment, node_text: &com::github::javaparser::printer::lexicalpreservation::node_text::NodeText) -> /* Java */ java::util::List /**/ {
		let matching_tokens: List<TokenTextElement>;
		if old_value instanceof TraditionalJavadocComment {
			matching_tokens = node_text.get_elements().stream().filter(|e|e.is_token()).map(|e|e as TokenTextElement).filter(|t|t.get_text().equals(&old_value.as_string())).collect(&/* Java */ java::util::stream::Collectors /**/::toList());
		} else if old_value instanceof BlockComment {
			matching_tokens = node_text.get_elements().stream().filter(|e|e.is_token()).map(|e|e as TokenTextElement).filter(|t|t.get_text().equals(&old_value.as_string())).collect(&/* Java */ java::util::stream::Collectors /**/::toList());
		} else if old_value instanceof MarkdownComment {
			// Because a MarkdownComment consists of a sequence of tokens (as opposed to the other comment types
			// which consist of a single token), all the tokens making up the MarkdownComment need to be found to
			// be able to correctly replace or delete it.
			matching_tokens = ArrayList<>::new();
			let maybe_matching_tokens: ArrayList<TextElement> = ArrayList<>::new();
			let in_match: bool = false;
			let old_content: String = old_value.as_markdown_comment()?.get_content();
			let text_elements: List<TextElement> = node_text.get_elements();
			for text_element in text_elements {
				if in_match {
					// If a matching start has been found, then add all following tokens to maybeMatchingTokens
					// until either a matching end is found, at which point the token range is added to
					// matchingTokens, or a non-whitespace, non-comment token is found at which point we know the
					// maybeMatchingTokens do not actually match the markdown comment (just some prefix of it), so
					// maybeMatchingTokens is cleared.
					maybe_matching_tokens.add(text_element);
					if text_element.is_token() && old_content.endsWith(&text_element.expand()) {
						// We have a matching start and end, so check that the full text matches.
						let sb: StringBuilder = StringBuilder::new();
						for elem in maybe_matching_tokens {
							sb.append(&(elem as TokenTextElement).get_text());
						}
						if sb.toString().equals(old_content) {
							matching_tokens.addAll(&maybe_matching_tokens.stream().map(|e|e as TokenTextElement).collect(&/* Java */ java::util::stream::Collectors /**/::toList()));
							// Clear and continue, since multiple markdown comments may have the same content
							maybe_matching_tokens.clear();
							in_match = false;
						} else {
							maybe_matching_tokens.clear();
							in_match = false;
						}
					}
				} else if text_element.is_token() && old_content.startsWith(&(text_element as TokenTextElement).get_text()) {
					// Found a line comment that matches the first line of the markdown comment, so start looking
					// for the rest of the comment.
					maybe_matching_tokens.add(text_element);
					in_match = true;
				}
			}
		} else {
			matching_tokens = node_text.get_elements().stream().filter(|e|e.is_token()).map(|e|e as TokenTextElement).filter(|t|t.get_text().trim().equals(&(old_value.as_string()).trim())).collect(&/* Java */ java::util::stream::Collectors /**/::toList());
		}
		// as comments with the same content may exist on different lines.
		return matching_tokens.stream().filter(|t|(!t.get_token().has_range() && !old_value.has_range()) || (t.get_token().has_range() && old_value.has_range() && old_value.get_range().get().contains(&t.get_token().get_range().get()))).collect(&/* Java */ java::util::stream::Collectors /**/::toList());
	}

	fn fix_indent_of_added_node(&self, node_text: &com::github::javaparser::printer::lexicalpreservation::node_text::NodeText, index: i32) {
		if index <= 0 {
			return;
		}
		let current_space_candidate: TextElement = null;
		 {
			let i: i32 = index;
			while i >= 0 {
				{
					let space_candidate: TextElement = node_text.get_text_element(i);
					if space_candidate.is_space_or_tab() {
						// save the current indentation char
						current_space_candidate = node_text.get_text_element(i);
					}
					if !space_candidate.is_space_or_tab() {
						if space_candidate.is_newline() && i != index {
							let number_of_indentation_characters: i32 = index - i;
							 {
								let j: i32 = 0;
								while j < number_of_indentation_characters {
									{
										if current_space_candidate != null {
											// use the current (or last) indentation character
											node_text.add_element(index, TokenTextElement::new(&JavaToken::com::github::javaparser::java_token::Kind::SPACE.get_kind(), &current_space_candidate.expand()));
										} else {
											// use the default indentation character
											node_text.add_element(index, TokenTextElement::new(&JavaToken::com::github::javaparser::java_token::Kind::SPACE.get_kind()));
										}
									}
									j += 1;
								 }
							 }
	
						}
						break;
					}
				}
				i -= 1;
			 }
		 }
	
	}

	pub fn concrete_list_change(&self, changed_list: &com::github::javaparser::ast::node_list::NodeList, type: &com::github::javaparser::ast::observer::ast_observer::ListChangeType, index: i32, node_added_or_removed: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.UnsupportedOperationException | java.lang.IllegalStateException | java.lang.RuntimeException | java.lang.IllegalArgumentException) */ {
		let node_text: NodeText = com::github::javaparser::printer::lexicalpreservation::lexical_preserving_printer::LexicalPreservingPrinter::get_or_create_node_text(&changed_list.get_parent_node_for_children())?;
		/* final */ let difference_elements: List<DifferenceElement>;
		if type == AstObserver::com::github::javaparser::ast::observer::ast_observer::ListChangeType::REMOVAL {
			difference_elements = .calculate_list_removal_difference(&com::github::javaparser::printer::lexicalpreservation::lexical_preserving_printer::LexicalPreservingPrinter::find_node_list_name(changed_list)?, changed_list, index);
		} else if type == AstObserver::com::github::javaparser::ast::observer::ast_observer::ListChangeType::ADDITION {
			difference_elements = .calculate_list_addition_difference(&com::github::javaparser::printer::lexicalpreservation::lexical_preserving_printer::LexicalPreservingPrinter::find_node_list_name(changed_list)?, changed_list, index, node_added_or_removed);
		} else {
			return Err(UnsupportedOperationException::new("Unknown change type: " + type));
		}
		let difference: Difference = Difference::new(difference_elements, node_text, &changed_list.get_parent_node_for_children());
		difference.apply()?;
	}

	pub fn concrete_list_replacement(&self, changed_list: &com::github::javaparser::ast::node_list::NodeList, index: i32, old_value: &com::github::javaparser::ast::node::Node, new_value: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.UnsupportedOperationException) */ {
		let node_text: NodeText = com::github::javaparser::printer::lexicalpreservation::lexical_preserving_printer::LexicalPreservingPrinter::get_or_create_node_text(&changed_list.get_parent_node_for_children())?;
		let difference_elements: List<DifferenceElement> = .calculate_list_replacement_difference(&com::github::javaparser::printer::lexicalpreservation::lexical_preserving_printer::LexicalPreservingPrinter::find_node_list_name(changed_list)?, changed_list, index, new_value);
		let difference: Difference = Difference::new(difference_elements, node_text, &changed_list.get_parent_node_for_children());
		difference.apply()?;
	}
}

impl com::github::javaparser::ast::observer::ast_observer::AstObserver for Observer {}