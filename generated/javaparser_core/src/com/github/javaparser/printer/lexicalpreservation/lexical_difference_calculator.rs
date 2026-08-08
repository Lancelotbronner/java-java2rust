use crate::com::github::javaparser::GeneratedJavaParserConstants;
use crate::com::github::javaparser::ast::Modifier;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::expr::CharLiteralExpr;
use crate::com::github::javaparser::ast::expr::LambdaExpr;
use crate::com::github::javaparser::ast::expr::StringLiteralExpr;
use crate::com::github::javaparser::ast::expr::TextBlockLiteralExpr;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::stmt::ExpressionStmt;
use crate::com::github::javaparser::printer::ConcreteSyntaxModel;
use crate::com::github::javaparser::printer::SourcePrinter;
use crate::com::github::javaparser::printer::Stringable;
use crate::com::github::javaparser::printer::concretesyntaxmodel;
use crate::com::github::javaparser::printer::lexicalpreservation::changes;
use crate::com::github::javaparser::utils::LineSeparator;
use java::util;

struct LexicalDifferenceCalculator;

impl LexicalDifferenceCalculator {
	fn calculate_list_removal_difference(&self, observable_property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, node_list: &com::github::javaparser::ast::node_list::NodeList, index: i32) -> /* Java */ java::util::List /**/ {
		let container: Node = node_list.get_parent_node_for_children();
		let element: CsmElement = ConcreteSyntaxModel::for_class(&container.getClass())?;
		let original: CalculatedSyntaxModel = self.calculated_syntax_model_for_node(element, container);
		let after: CalculatedSyntaxModel = self.calculated_syntax_model_after_list_removal(element, observable_property, node_list, index);
		return DifferenceElementCalculator::new().calculate(original, after);
	}

	fn calculate_list_addition_difference(&self, observable_property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, node_list: &com::github::javaparser::ast::node_list::NodeList, index: i32, node_added: &com::github::javaparser::ast::node::Node) -> /* Java */ java::util::List /**/ {
		let container: Node = node_list.get_parent_node_for_children();
		let element: CsmElement = ConcreteSyntaxModel::for_class(&container.getClass())?;
		let original: CalculatedSyntaxModel = self.calculated_syntax_model_for_node(element, container);
		let after: CalculatedSyntaxModel = self.calculated_syntax_model_after_list_addition(element, observable_property, node_list, index, node_added);
		let difference_elements: List<DifferenceElement> = DifferenceElementCalculator::new().calculate(original, after);
		// Set the line separator character tokens
		let line_separator: LineSeparator = container.get_line_ending_style_or_default(LineSeparator::SYSTEM);
		self.replace_eol_tokens(difference_elements, line_separator);
		return difference_elements;
	}

	fn replace_eol_tokens(&self, difference_elements: &/* Java */ java::util::List /**/, line_separator: &com::github::javaparser::utils::line_separator::LineSeparator) {
		let eol: CsmElement = self.get_new_line_token(line_separator);
		 {
			let i: i32 = 0;
			while i < difference_elements.size() {
				{
					let difference_element: DifferenceElement = difference_elements.get(i);
					difference_elements.set(i, &difference_element.replace_eol_tokens(eol));
				}
				i += 1;
			 }
		 }
	
	}

	fn get_new_line_token(&self, line_separator: &com::github::javaparser::utils::line_separator::LineSeparator) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement {
		return CsmElement::newline(line_separator)?;
	}

	fn calculate_list_replacement_difference(&self, observable_property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, node_list: &com::github::javaparser::ast::node_list::NodeList, index: i32, new_value: &com::github::javaparser::ast::node::Node) -> /* Java */ java::util::List /**/ {
		let container: Node = node_list.get_parent_node_for_children();
		let element: CsmElement = ConcreteSyntaxModel::for_class(&container.getClass())?;
		let original: CalculatedSyntaxModel = self.calculated_syntax_model_for_node(element, container);
		let after: CalculatedSyntaxModel = self.calculated_syntax_model_after_list_replacement(element, observable_property, node_list, index, new_value);
		return DifferenceElementCalculator::new().calculate(original, after);
	}

	fn calculate_property_change(&self, node_text: &com::github::javaparser::printer::lexicalpreservation::node_text::NodeText, observed_node: &com::github::javaparser::ast::node::Node, property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, old_value: &/* Java */ java::lang::Object /**/, new_value: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.NullPointerException | java.lang.UnsupportedOperationException) */ {
		if node_text == null {
			return Err(NullPointerException::new());
		}
		let element: CsmElement = ConcreteSyntaxModel::for_class(&observed_node.getClass())?;
		let original: CalculatedSyntaxModel = self.calculated_syntax_model_for_node(element, observed_node);
		let after: CalculatedSyntaxModel = self.calculated_syntax_model_after_property_change(element, observed_node, property, old_value, new_value);
		let difference_elements: List<DifferenceElement> = DifferenceElementCalculator::new().calculate(original, after);
		let difference: Difference = Difference::new(difference_elements, node_text, observed_node);
		difference.apply()?;
	}

	fn calculated_syntax_model_for_node(&self, csm: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement, node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.UnsupportedOperationException | java.lang.IllegalStateException) */ -> com::github::javaparser::printer::lexicalpreservation::lexical_difference_calculator::CalculatedSyntaxModel {
		let elements: List<CsmElement> = LinkedList<>::new();
		self.calculated_syntax_model_for_node(csm, node, elements, NoChange::new())?;
		return CalculatedSyntaxModel::new(elements);
	}

	fn calculated_syntax_model_for_node(&self, node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.UnsupportedOperationException | java.lang.IllegalStateException) */ -> com::github::javaparser::printer::lexicalpreservation::lexical_difference_calculator::CalculatedSyntaxModel {
		return self.calculated_syntax_model_for_node(&ConcreteSyntaxModel::for_class(&node.getClass())?, node)?;
	}

	fn calculated_syntax_model_for_node(&self, csm: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement, node: &com::github::javaparser::ast::node::Node, elements: &/* Java */ java::util::List /**/, change: &com::github::javaparser::printer::lexicalpreservation::changes::change::Change) /* thrown(java.lang.UnsupportedOperationException | java.lang.IllegalStateException | java.lang.RuntimeException) */ {
		if csm instanceof CsmSequence {
			let csm_sequence: CsmSequence = csm as CsmSequence;
			csm_sequence.get_elements().forEach(|e|self.calculated_syntax_model_for_node(e, node, elements, change)?);
		} else if csm instanceof CsmComment {
		// nothing to do
		} else if csm instanceof CsmSingleReference {
			let csm_single_reference: CsmSingleReference = csm as CsmSingleReference;
			let child: Node;
			if change instanceof PropertyChange && (change as PropertyChange).get_property() == csm_single_reference.get_property() {
				child = (change as PropertyChange).get_new_value() as Node;
				if node instanceof LambdaExpr && child instanceof ExpressionStmt {
					// Same edge-case as in DefaultPrettyPrinterVisitor.visit(LambdaExpr, Void)
					child = (child as ExpressionStmt).get_expression();
				}
			} else {
				child = csm_single_reference.get_property().get_value_as_single_reference(node)?;
			}
			if child != null {
				elements.add(CsmChild::new(child));
			}
		} else if csm instanceof CsmNone {
		// nothing to do
		} else if csm instanceof CsmToken {
			elements.add(csm);
		} else if csm instanceof CsmOrphanCommentsEnding {
		// nothing to do
		} else if csm instanceof CsmList {
			let csm_list: CsmList = csm as CsmList;
			if csm_list.get_property().is_about_nodes() {
				let raw_value: Object = change.get_value(&csm_list.get_property(), node);
				let node_list: NodeList<?>;
				if raw_value instanceof Optional {
					let optional: Optional<?> = raw_value as Optional<?>;
					if optional.isPresent() {
						if !(optional.get() instanceof NodeList) {
							return Err(IllegalStateException::new("Expected NodeList, found " + optional.get().getClass().getCanonicalName()));
						}
						node_list = optional.get() as NodeList<?>;
					} else {
						node_list = NodeList<>::new();
					}
				} else {
					if !(raw_value instanceof NodeList) {
						return Err(IllegalStateException::new("Expected NodeList, found " + raw_value.getClass().getCanonicalName()));
					}
					node_list = raw_value as NodeList<?>;
				}
				if !node_list.is_empty() {
					self.calculated_syntax_model_for_node(&csm_list.get_preceeding(), node, elements, change)?;
					 {
						let i: i32 = 0;
						while i < node_list.size() {
							{
								if i != 0 {
									self.calculated_syntax_model_for_node(&csm_list.get_separator_pre(), node, elements, change)?;
								}
								elements.add(CsmChild::new(&node_list.get(i)));
								if i != (node_list.size() - 1) {
									self.calculated_syntax_model_for_node(&csm_list.get_separator_post(), node, elements, change)?;
								}
							}
							i += 1;
						 }
					 }
	
					self.calculated_syntax_model_for_node(&csm_list.get_following(), node, elements, change)?;
				}
			} else {
				let collection: Collection<?> = change.get_value(&csm_list.get_property(), node) as Collection<?>;
				if !collection.isEmpty() {
					self.calculated_syntax_model_for_node(&csm_list.get_preceeding(), node, elements, change)?;
					let first: bool = true;
					 {
						let it: Iterator<?> = collection.iterator();
						while it.hasNext(){
							if !first {
								self.calculated_syntax_model_for_node(&csm_list.get_separator_pre(), node, elements, change)?;
							}
							let value: Object = it.next();
							if value instanceof Modifier {
								let modifier: Modifier = value as Modifier;
								elements.add(CsmToken::new(&com::github::javaparser::printer::lexicalpreservation::lexical_difference_calculator::LexicalDifferenceCalculator::to_token(modifier)?));
							} else {
								return Err(UnsupportedOperationException::new("Not supported value found: " + it.next().getClass().getSimpleName()));
							}
							if it.hasNext() {
								self.calculated_syntax_model_for_node(&csm_list.get_separator_post(), node, elements, change)?;
							}
							first = false;
						}
					 }
	
					self.calculated_syntax_model_for_node(&csm_list.get_following(), node, elements, change)?;
				}
			}
		} else if csm instanceof CsmConditional {
			let csm_conditional: CsmConditional = csm as CsmConditional;
			let satisfied: bool = change.evaluate(csm_conditional, node)?;
			if satisfied {
				self.calculated_syntax_model_for_node(&csm_conditional.get_then_element(), node, elements, change)?;
			} else {
				self.calculated_syntax_model_for_node(&csm_conditional.get_else_element(), node, elements, change)?;
			}
		} else if csm instanceof CsmIndent {
			elements.add(csm);
		} else if csm instanceof CsmUnindent {
			elements.add(csm);
		} else if csm instanceof CsmAttribute {
			let csm_attribute: CsmAttribute = csm as CsmAttribute;
			let value: Object = change.get_value(&csm_attribute.get_property(), node);
			let text: String = value.toString();
			if value instanceof Stringable {
				text = (value as Stringable).as_string();
			}
			elements.add(CsmToken::new(&csm_attribute.get_token_type(node, &value.toString(), text)?, text));
		} else if (csm instanceof CsmString) && (node instanceof StringLiteralExpr) {
			// contain the new value, otherwise the original/current value should be used.
			if change instanceof PropertyChange {
				elements.add(CsmToken::new(GeneratedJavaParserConstants.STRING_LITERAL, "\"" + (change as PropertyChange).get_new_value() + "\""));
			} else {
				elements.add(CsmToken::new(GeneratedJavaParserConstants.STRING_LITERAL, "\"" + (node as StringLiteralExpr).get_value() + "\""));
			}
		} else if (csm instanceof CsmString) && (node instanceof TextBlockLiteralExpr) {
			// Per https://openjdk.java.net/jeps/378#1--Line-terminators, any 'CRLF' and 'CR' are turned into 'LF'
			// before interpreting the text
			let eol: String = node.get_line_ending_style()?.to_string();
			// FIXME: csm should be CsmTextBlock -- See also #2677
			if change instanceof PropertyChange {
				elements.add(CsmToken::new(GeneratedJavaParserConstants.TEXT_BLOCK_LITERAL, "\"\"\"" + eol + (change as PropertyChange).get_new_value() + "\"\"\""));
			} else {
				elements.add(CsmToken::new(GeneratedJavaParserConstants.TEXT_BLOCK_LITERAL, "\"\"\"" + eol + (node as TextBlockLiteralExpr).get_value() + "\"\"\""));
			}
		} else if (csm instanceof CsmChar) && (node instanceof CharLiteralExpr) {
			if change instanceof PropertyChange {
				elements.add(CsmToken::new(GeneratedJavaParserConstants.CHAR, "'" + (change as PropertyChange).get_new_value() + "'"));
			} else {
				elements.add(CsmToken::new(GeneratedJavaParserConstants.CHAR, "'" + (node as CharLiteralExpr).get_value() + "'"));
			}
		} else if csm instanceof CsmMix {
			let csm_mix: CsmMix = csm as CsmMix;
			let mix_elements: List<CsmElement> = LinkedList<>::new();
			csm_mix.get_elements().forEach(|e|self.calculated_syntax_model_for_node(e, node, mix_elements, change)?);
			elements.add(CsmMix::new(mix_elements));
		} else if csm instanceof CsmChild {
			elements.add(csm);
		} else {
			return Err(UnsupportedOperationException::new("Not supported element type: " + csm.getClass().getSimpleName() + " " + csm));
		}
	}

	pub fn to_token(&self, modifier: &com::github::javaparser::ast::modifier::Modifier) /* thrown(java.lang.UnsupportedOperationException) */ -> i32 {
		match modifier.get_keyword() {
			PUBLIC =>  {
				return GeneratedJavaParserConstants.PUBLIC;
			}
			PRIVATE =>  {
				return GeneratedJavaParserConstants.PRIVATE;
			}
			PROTECTED =>  {
				return GeneratedJavaParserConstants.PROTECTED;
			}
			STATIC =>  {
				return GeneratedJavaParserConstants.STATIC;
			}
			FINAL =>  {
				return GeneratedJavaParserConstants.FINAL;
			}
			ABSTRACT =>  {
				return GeneratedJavaParserConstants.ABSTRACT;
			}
			TRANSIENT =>  {
				return GeneratedJavaParserConstants.TRANSIENT;
			}
			SYNCHRONIZED =>  {
				return GeneratedJavaParserConstants.SYNCHRONIZED;
			}
			VOLATILE =>  {
				return GeneratedJavaParserConstants.VOLATILE;
			}
			NATIVE =>  {
				return GeneratedJavaParserConstants.NATIVE;
			}
			STRICTFP =>  {
				return GeneratedJavaParserConstants.STRICTFP;
			}
			TRANSITIVE =>  {
				return GeneratedJavaParserConstants.TRANSITIVE;
			}
			_ =>  {
				return Err(UnsupportedOperationException::new("Not supported keyword" + modifier.get_keyword().name()));
			}
		}
	}

	fn calculated_syntax_model_after_property_change(&self, node: &com::github::javaparser::ast::node::Node, property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, old_value: &/* Java */ java::lang::Object /**/, new_value: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.UnsupportedOperationException) */ -> com::github::javaparser::printer::lexicalpreservation::lexical_difference_calculator::CalculatedSyntaxModel {
		return self.calculated_syntax_model_after_property_change(&ConcreteSyntaxModel::for_class(&node.getClass())?, node, property, old_value, new_value);
	}

	fn calculated_syntax_model_after_property_change(&self, csm: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement, node: &com::github::javaparser::ast::node::Node, property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, old_value: &/* Java */ java::lang::Object /**/, new_value: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.UnsupportedOperationException | java.lang.IllegalStateException | java.lang.RuntimeException) */ -> com::github::javaparser::printer::lexicalpreservation::lexical_difference_calculator::CalculatedSyntaxModel {
		let elements: List<CsmElement> = LinkedList<>::new();
		self.calculated_syntax_model_for_node(csm, node, elements, PropertyChange::new(property, old_value, new_value))?;
		return CalculatedSyntaxModel::new(elements);
	}

	fn calculated_syntax_model_after_list_removal(&self, csm: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement, observable_property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, node_list: &com::github::javaparser::ast::node_list::NodeList, index: i32) /* thrown(java.lang.UnsupportedOperationException | java.lang.IllegalStateException | java.lang.RuntimeException) */ -> com::github::javaparser::printer::lexicalpreservation::lexical_difference_calculator::CalculatedSyntaxModel {
		let elements: List<CsmElement> = LinkedList<>::new();
		let container: Node = node_list.get_parent_node_for_children();
		self.calculated_syntax_model_for_node(csm, container, elements, ListRemovalChange::new(observable_property, index))?;
		return CalculatedSyntaxModel::new(elements);
	}

	fn calculated_syntax_model_after_list_addition(&self, csm: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement, observable_property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, node_list: &com::github::javaparser::ast::node_list::NodeList, index: i32, node_added: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.UnsupportedOperationException | java.lang.IllegalStateException | java.lang.RuntimeException) */ -> com::github::javaparser::printer::lexicalpreservation::lexical_difference_calculator::CalculatedSyntaxModel {
		let elements: List<CsmElement> = LinkedList<>::new();
		let container: Node = node_list.get_parent_node_for_children();
		self.calculated_syntax_model_for_node(csm, container, elements, ListAdditionChange::new(observable_property, index, node_added))?;
		return CalculatedSyntaxModel::new(elements);
	}

	fn calculated_syntax_model_after_list_addition(&self, container: &com::github::javaparser::ast::node::Node, observable_property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, index: i32, node_added: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.UnsupportedOperationException | java.lang.IllegalStateException | java.lang.RuntimeException) */ -> com::github::javaparser::printer::lexicalpreservation::lexical_difference_calculator::CalculatedSyntaxModel {
		let csm: CsmElement = ConcreteSyntaxModel::for_class(&container.getClass())?;
		let raw_value: Object = observable_property.get_raw_value(container)?;
		if !(raw_value instanceof NodeList) {
			return Err(IllegalStateException::new("Expected NodeList, found " + raw_value.getClass().getCanonicalName()));
		}
		let node_list: NodeList<?> = raw_value as NodeList<?>;
		return self.calculated_syntax_model_after_list_addition(csm, observable_property, node_list, index, node_added)?;
	}

	fn calculated_syntax_model_after_list_removal(&self, container: &com::github::javaparser::ast::node::Node, observable_property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, index: i32) /* thrown(java.lang.UnsupportedOperationException | java.lang.IllegalStateException | java.lang.RuntimeException) */ -> com::github::javaparser::printer::lexicalpreservation::lexical_difference_calculator::CalculatedSyntaxModel {
		let csm: CsmElement = ConcreteSyntaxModel::for_class(&container.getClass())?;
		let raw_value: Object = observable_property.get_raw_value(container)?;
		if !(raw_value instanceof NodeList) {
			return Err(IllegalStateException::new("Expected NodeList, found " + raw_value.getClass().getCanonicalName()));
		}
		let node_list: NodeList<?> = raw_value as NodeList<?>;
		return self.calculated_syntax_model_after_list_removal(csm, observable_property, node_list, index)?;
	}

	fn calculated_syntax_model_after_list_replacement(&self, csm: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement, observable_property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, node_list: &com::github::javaparser::ast::node_list::NodeList, index: i32, new_value: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.UnsupportedOperationException | java.lang.IllegalStateException | java.lang.RuntimeException) */ -> com::github::javaparser::printer::lexicalpreservation::lexical_difference_calculator::CalculatedSyntaxModel {
		let elements: List<CsmElement> = LinkedList<>::new();
		let container: Node = node_list.get_parent_node_for_children();
		self.calculated_syntax_model_for_node(csm, container, elements, ListReplacementChange::new(observable_property, index, new_value))?;
		return CalculatedSyntaxModel::new(elements);
	}
}

struct CalculatedSyntaxModel {
	elements: /* Java */ java::util::List /**/,
}

impl CalculatedSyntaxModel {
	fn new(elements: &/* Java */ java::util::List /**/) -> com::github::javaparser::printer::lexicalpreservation::lexical_difference_calculator::CalculatedSyntaxModel {
		self.elements = elements;
	}

	pub fn from(&self, index: i32) -> com::github::javaparser::printer::lexicalpreservation::lexical_difference_calculator::CalculatedSyntaxModel {
		return CalculatedSyntaxModel::new(ArrayList<>::new(&self.elements.subList(index, &self.elements.size())));
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "CalculatedSyntaxModel{" + "elements=" + self.elements + '}';
	}

	fn sub(&self, start: i32, end: i32) -> com::github::javaparser::printer::lexicalpreservation::lexical_difference_calculator::CalculatedSyntaxModel {
		return CalculatedSyntaxModel::new(&self.elements.subList(start, end));
	}

	fn remove_indentation_elements(&self) {
		self.elements.removeIf(|el|el instanceof CsmIndent || el instanceof CsmUnindent);
	}

	pub fn hash_code(&self) -> i32 {
		return self.elements.hashCode();
	}

	pub fn equals(&self, other: &/* Java */ java::lang::Object /**/) -> bool {
		if other == null {
			return false;
		}
	
		if !(other instanceof CalculatedSyntaxModel) {
			return false;
		}
	
		return self.elements.equals(other);
	}
}

pub struct CsmChild {
	child: com::github::javaparser::ast::node::Node,
}

impl CsmChild {
	pub fn get_child(&self) -> com::github::javaparser::ast::node::Node {
		return self.child;
	}

	fn new(child: &com::github::javaparser::ast::node::Node) -> com::github::javaparser::printer::lexicalpreservation::lexical_difference_calculator::CsmChild {
		self.child = child;
	}

	pub fn pretty_print(&self, node: &com::github::javaparser::ast::node::Node, printer: &com::github::javaparser::printer::source_printer::SourcePrinter) /* thrown(java.lang.UnsupportedOperationException) */ {
		return Err(UnsupportedOperationException::new("The prettyPrint method is not supported or implemented"));
	}

	pub fn is_corresponding_element(&self, text_element: &com::github::javaparser::printer::lexicalpreservation::text_element::TextElement) -> bool {
		return (text_element instanceof ChildTextElement) && (text_element as ChildTextElement).get_child() == self.get_child();
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "child(" + self.child.getClass().getSimpleName() + ")";
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if self == o {
			return true;
		}
	
		if o == null || self.getClass() != o.getClass() {
			return false;
		}
	
		let csm_child: CsmChild = o as CsmChild;
		return self.child.equals(csm_child.child);
	}

	pub fn hash_code(&self) -> i32 {
		return self.child.hash_code();
	}
}

impl com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement for CsmChild {}