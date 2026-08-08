use crate::com::github::javaparser::ast::Node::Parsedness::UNPARSABLE;
use crate::com::github::javaparser::utils::PositionUtils::sortByBeginPosition;
use crate::com::github::javaparser::utils::Utils;
use java::util::stream::Collectors::joining;
use crate::com::github::javaparser::ast;
use crate::com::github::javaparser::ast::body;
use crate::com::github::javaparser::ast::comments;
use crate::com::github::javaparser::ast::expr;
use crate::com::github::javaparser::ast::modules;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithTraversableScope;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithTypeArguments;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithVariables;
use crate::com::github::javaparser::ast::nodeTypes::SwitchNode;
use crate::com::github::javaparser::ast::stmt;
use crate::com::github::javaparser::ast::type;
use crate::com::github::javaparser::ast::visitor::Visitable;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::printer::configuration::ConfigurationOption;
use crate::com::github::javaparser::printer::configuration::DefaultConfigurationOption;
use crate::com::github::javaparser::printer::configuration::DefaultPrinterConfiguration::ConfigOption;
use crate::com::github::javaparser::printer::configuration::ImportOrderingStrategy;
use crate::com::github::javaparser::printer::configuration::PrinterConfiguration;
use crate::com::github::javaparser::printer::configuration::imports::DefaultImportOrderingStrategy;
use java::util;
use java::util::concurrent::atomic::AtomicBoolean;
use java::util::regex::Pattern;

pub struct DefaultPrettyPrinterVisitor {
	configuration: com::github::javaparser::printer::configuration::printer_configuration::PrinterConfiguration,
	printer: com::github::javaparser::printer::source_printer::SourcePrinter,
}

impl DefaultPrettyPrinterVisitor {
	static RTRIM: /* Java */ java::util::regex::Pattern /**/ = Pattern::compile("\\s+$");

	pub fn new(configuration: &com::github::javaparser::printer::configuration::printer_configuration::PrinterConfiguration) -> com::github::javaparser::printer::default_pretty_printer_visitor::DefaultPrettyPrinterVisitor {
		this(configuration, SourcePrinter::new(configuration));
	}

	pub fn new(configuration: &com::github::javaparser::printer::configuration::printer_configuration::PrinterConfiguration, printer: &com::github::javaparser::printer::source_printer::SourcePrinter) -> com::github::javaparser::printer::default_pretty_printer_visitor::DefaultPrettyPrinterVisitor {
		self.configuration = configuration;
		self.printer = printer;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.printer.to_string();
	}

	fn print_modifiers(&self, modifiers: &com::github::javaparser::ast::node_list::NodeList) {
		if modifiers.size() > 0 {
			self.printer.print(modifiers.stream().map(Modifier::getKeyword).map(Modifier.Keyword::asString).collect(&/* Java */ java::util::stream::Collectors /**/::joining(" ")) + " ");
		}
	}

	fn print_members(&self, members: &com::github::javaparser::ast::node_list::NodeList, arg: &/* Java */ java::lang::Void /**/) {
		for /* final */ member in members {
			self.printer.println();
			member.accept(self, arg);
			self.printer.println();
		}
	}

	fn print_compact_class_members(&self, members: &com::github::javaparser::ast::node_list::NodeList, arg: &/* Java */ java::lang::Void /**/) {
		let member: BodyDeclaration<?>;
		let size: i32 = members.size();
		 {
			let i: i32 = 0;
			while i < size {
				{
					member = members.get(i);
					if i > 0 {
						// Only print the preceding line if this is not the first member in the list
						self.printer.println();
					}
					member.accept(self, arg);
					if i < size - 1 {
						// Only print the following line if this is not the last member in the list
						self.printer.println();
					}
				}
				i += 1;
			 }
		 }
	
	}

	fn print_member_annotations(&self, annotations: &com::github::javaparser::ast::node_list::NodeList, arg: &/* Java */ java::lang::Void /**/) {
		if annotations.is_empty() {
			return;
		}
		for /* final */ a in annotations {
			a.accept(self, arg);
			self.printer.println();
		}
	}

	fn print_annotations(&self, annotations: &com::github::javaparser::ast::node_list::NodeList, prefix_witha_space: bool, arg: &/* Java */ java::lang::Void /**/) {
		if annotations.is_empty() {
			return;
		}
		if prefix_with_a_space {
			self.printer.print(" ");
		}
		for annotation in annotations {
			annotation.accept(self, arg);
			self.printer.print(" ");
		}
	}

	fn print_type_args(&self, node_with_type_arguments: &com::github::javaparser::ast::node_types::node_with_type_arguments::NodeWithTypeArguments, arg: &/* Java */ java::lang::Void /**/) {
		let type_arguments: NodeList<Type> = node_with_type_arguments.get_type_arguments().orElse(null);
		if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(type_arguments) {
			self.printer.print("<");
			 {
				/* final */ let i: Iterator<Type> = type_arguments.iterator();
				while i.hasNext(){
					/* final */ let t: Type = i.next();
					t.accept(self, arg);
					if i.hasNext() {
						self.printer.print(", ");
					}
				}
			 }
	
			self.printer.print(">");
		}
	}

	fn print_type_parameters(&self, args: &com::github::javaparser::ast::node_list::NodeList, arg: &/* Java */ java::lang::Void /**/) {
		if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(args) {
			self.printer.print("<");
			 {
				/* final */ let i: Iterator<TypeParameter> = args.iterator();
				while i.hasNext(){
					/* final */ let t: TypeParameter = i.next();
					t.accept(self, arg);
					if i.hasNext() {
						self.printer.print(", ");
					}
				}
			 }
	
			self.printer.print(">");
		}
	}

	fn print_arguments<T: com::github::javaparser::ast::expr::expression::Expression>(&self, args: &com::github::javaparser::ast::node_list::NodeList, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.IllegalStateException) */ {
		self.printer.print("(");
		if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(args) {
			let column_align_parameters: bool = (args.size() > 1) && self.get_option(ConfigOption::COLUMN_ALIGN_PARAMETERS).isPresent();
			if column_align_parameters {
				self.printer.indent_with_align_to(self.printer.get_cursor().column);
			}
			 {
				/* final */ let i: Iterator<T> = args.iterator();
				while i.hasNext(){
					/* final */ let e: T = i.next();
					e.accept(self, arg);
					if i.hasNext() {
						self.printer.print(",");
						if column_align_parameters {
							self.printer.println();
						} else {
							self.printer.print(" ");
						}
					}
				}
			 }
	
			if column_align_parameters {
				self.printer.unindent()?;
			}
		}
		self.printer.print(")");
	}

	fn print_pre_post_fix_optional_list(&self, args: &com::github::javaparser::ast::node_list::NodeList, arg: &/* Java */ java::lang::Void /**/, prefix: &/* Java */ java::lang::String /**/, separator: &/* Java */ java::lang::String /**/, postfix: &/* Java */ java::lang::String /**/) {
		if !args.is_empty() {
			self.printer.print(prefix);
			 {
				/* final */ let i: Iterator<? extends Visitable> = args.iterator();
				while i.hasNext(){
					/* final */ let v: Visitable = i.next();
					v.accept(self, arg);
					if i.hasNext() {
						self.printer.print(separator);
					}
				}
			 }
	
			self.printer.print(postfix);
		}
	}

	fn print_pre_post_fix_required_list(&self, args: &com::github::javaparser::ast::node_list::NodeList, arg: &/* Java */ java::lang::Void /**/, prefix: &/* Java */ java::lang::String /**/, separator: &/* Java */ java::lang::String /**/, postfix: &/* Java */ java::lang::String /**/) {
		self.printer.print(prefix);
		if !args.is_empty() {
			 {
				/* final */ let i: Iterator<? extends Visitable> = args.iterator();
				while i.hasNext(){
					/* final */ let v: Visitable = i.next();
					v.accept(self, arg);
					if i.hasNext() {
						self.printer.print(separator);
					}
				}
			 }
	
		}
		self.printer.print(postfix);
	}

	fn print_comment(&self, comment: &/* Java */ java::util::Optional /**/, arg: &/* Java */ java::lang::Void /**/) {
		comment.ifPresent(|c|c.accept(self, arg));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::compilation_unit::CompilationUnit, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		if n.get_parsed() == UNPARSABLE {
			self.printer.println("???");
			return;
		}
		if n.get_package_declaration().isPresent() {
			n.get_package_declaration().get().accept(self, arg);
		}
		self.print_imports(&n.get_imports(), arg);
		 {
			/* final */ let i: Iterator<TypeDeclaration<?>> = n.get_types().iterator();
			while i.hasNext(){
				i.next().accept(self, arg);
				self.printer.println();
				if i.hasNext() {
					self.printer.println();
				}
			}
		 }
	
		n.get_module().ifPresent(|m|m.accept(self, arg));
		self.print_orphan_comments_ending(n);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::package_declaration::PackageDeclaration, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.print_member_annotations(&n.get_annotations(), arg);
		self.printer.print("package ");
		n.get_name().accept(self, arg);
		self.printer.println(";");
		self.printer.println();
		self.print_orphan_comments_ending(n);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name_expr::NameExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		n.get_name().accept(self, arg);
		self.print_orphan_comments_ending(n);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name::Name, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		if n.get_qualifier().isPresent() {
			n.get_qualifier().get().accept(self, arg);
			self.printer.print(".");
		}
		self.printer.print(&n.get_identifier());
		self.print_orphan_comments_ending(n);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::simple_name::SimpleName, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print(&n.get_identifier());
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.IllegalStateException | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		if !n.is_compact() {
			self.print_member_annotations(&n.get_annotations(), arg);
			self.print_modifiers(&n.get_modifiers());
			if n.is_interface() {
				self.printer.print("interface ");
			} else {
				self.printer.print("class ");
			}
			n.get_name().accept(self, arg);
			self.print_type_parameters(&n.get_type_parameters(), arg);
			if !n.get_extended_types().is_empty() {
				self.printer.print(" extends ");
				 {
					/* final */ let i: Iterator<ClassOrInterfaceType> = n.get_extended_types().iterator();
					while i.hasNext(){
						/* final */ let c: ClassOrInterfaceType = i.next();
						c.accept(self, arg);
						if i.hasNext() {
							self.printer.print(", ");
						}
					}
				 }
	
			}
			if !n.get_implemented_types().is_empty() {
				self.printer.print(" implements ");
				 {
					/* final */ let i: Iterator<ClassOrInterfaceType> = n.get_implemented_types().iterator();
					while i.hasNext(){
						/* final */ let c: ClassOrInterfaceType = i.next();
						c.accept(self, arg);
						if i.hasNext() {
							self.printer.print(", ");
						}
					}
				 }
	
			}
			if !n.get_permitted_types().is_empty() {
				self.printer.print(" permits ");
				 {
					/* final */ let i: Iterator<ClassOrInterfaceType> = n.get_permitted_types().iterator();
					while i.hasNext(){
						/* final */ let c: ClassOrInterfaceType = i.next();
						c.accept(self, arg);
						if i.hasNext() {
							self.printer.print(", ");
						}
					}
				 }
	
			}
			self.printer.println(" {");
			self.printer.indent()?;
		}
		if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_members()) {
			if n.is_compact() {
				self.print_compact_class_members(&n.get_members(), arg);
			} else {
				self.print_members(&n.get_members(), arg);
			}
		}
		self.print_orphan_comments_ending(n);
		if !n.is_compact() {
			self.printer.unindent()?;
			self.printer.print("}");
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::record_declaration::RecordDeclaration, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.IllegalStateException | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.print_member_annotations(&n.get_annotations(), arg);
		self.print_modifiers(&n.get_modifiers());
		self.printer.print("record ");
		n.get_name().accept(self, arg);
		self.print_type_parameters(&n.get_type_parameters(), arg);
		self.printer.print("(");
		if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_parameters()) {
			 {
				/* final */ let i: Iterator<Parameter> = n.get_parameters().iterator();
				while i.hasNext(){
					/* final */ let p: Parameter = i.next();
					p.accept(self, arg);
					if i.hasNext() {
						self.printer.print(", ");
					}
				}
			 }
	
		}
		self.printer.print(")");
		if !n.get_implemented_types().is_empty() {
			self.printer.print(" implements ");
			 {
				/* final */ let i: Iterator<ClassOrInterfaceType> = n.get_implemented_types().iterator();
				while i.hasNext(){
					/* final */ let c: ClassOrInterfaceType = i.next();
					c.accept(self, arg);
					if i.hasNext() {
						self.printer.print(", ");
					}
				}
			 }
	
		}
		self.printer.println(" {");
		self.printer.indent()?;
		if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_members()) {
			self.print_members(&n.get_members(), arg);
		}
		self.print_orphan_comments_ending(n);
		self.printer.unindent()?;
		self.printer.print("}");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::traditional_javadoc_comment::TraditionalJavadocComment, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		if self.get_option(ConfigOption::PRINT_COMMENTS).isPresent() && self.get_option(ConfigOption::PRINT_JAVADOC).isPresent() {
			self.printer.println(&n.get_header());
			/* final */ let comment_content: String = com::github::javaparser::utils::utils::Utils::normalize_eol_in_text_block(&n.get_content(), &self.get_option(ConfigOption::END_OF_LINE_CHARACTER).get().as_string());
			let lines: Vec<String> = comment_content.split("\\R");
			let stripped_lines: List<String> = ArrayList<>::new();
			for line in lines {
				/* final */ let trimmed_line: String = line.trim();
				if trimmed_line.startsWith("*") {
					line = trimmed_line.substring(1);
				}
				line = com::github::javaparser::utils::utils::Utils::trim_trailing_spaces(line);
				stripped_lines.add(line);
			}
			let skipping_leading_empty_lines: bool = true;
			let prepend_empty_line: bool = false;
			let prepend_space: bool = stripped_lines.stream().anyMatch(|line|!line.isEmpty() && !line.startsWith(" "));
			for line in stripped_lines {
				if line.isEmpty() {
					if !skipping_leading_empty_lines {
						prepend_empty_line = true;
					}
				} else {
					skipping_leading_empty_lines = false;
					if prepend_empty_line {
						self.printer.println(" *");
						prepend_empty_line = false;
					}
					self.printer.print(" *");
					if prepend_space {
						self.printer.print(" ");
					}
					self.printer.println(line);
				}
			}
			self.printer.println(" " + n.get_footer());
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		if n.get_scope().isPresent() {
			n.get_scope().get().accept(self, arg);
			self.printer.print(".");
		}
		self.print_annotations(&n.get_annotations(), false, arg);
		n.get_name().accept(self, arg);
		if n.is_using_diamond_operator() {
			self.printer.print("<>");
		} else {
			self.print_type_args(n, arg);
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::type_parameter::TypeParameter, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.print_annotations(&n.get_annotations(), false, arg);
		n.get_name().accept(self, arg);
		if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_type_bound()) {
			self.printer.print(" extends ");
			 {
				/* final */ let i: Iterator<ClassOrInterfaceType> = n.get_type_bound().iterator();
				while i.hasNext(){
					/* final */ let c: ClassOrInterfaceType = i.next();
					c.accept(self, arg);
					if i.hasNext() {
						self.printer.print(" & ");
					}
				}
			 }
	
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::primitive_type::PrimitiveType, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.print_annotations(&n.get_annotations(), true, arg);
		self.printer.print(&n.get_type().as_string());
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::array_type::ArrayType, arg: &/* Java */ java::lang::Void /**/) {
		/* final */ let array_type_buffer: List<ArrayType> = LinkedList<>::new();
		let type: Type = n;
		while type instanceof ArrayType {
			/* final */ let array_type: ArrayType = type as ArrayType;
			array_type_buffer.add(array_type);
			type = array_type.get_component_type();
		}
		type.accept(self, arg);
		for array_type in array_type_buffer {
			self.print_annotations(&array_type.get_annotations(), true, arg);
			self.printer.print("[]");
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::array_creation_level::ArrayCreationLevel, arg: &/* Java */ java::lang::Void /**/) {
		self.print_annotations(&n.get_annotations(), true, arg);
		self.printer.print("[");
		if n.get_dimension().isPresent() {
			n.get_dimension().get().accept(self, arg);
		}
		self.printer.print("]");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::intersection_type::IntersectionType, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.print_annotations(&n.get_annotations(), false, arg);
		let is_first: bool = true;
		for element in n.get_elements() {
			if is_first {
				is_first = false;
			} else {
				self.printer.print(" & ");
			}
			element.accept(self, arg);
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::union_type::UnionType, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.print_annotations(&n.get_annotations(), true, arg);
		let is_first: bool = true;
		for element in n.get_elements() {
			if is_first {
				is_first = false;
			} else {
				self.printer.print(" | ");
			}
			element.accept(self, arg);
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::wildcard_type::WildcardType, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.print_annotations(&n.get_annotations(), false, arg);
		self.printer.print("?");
		if n.get_extended_type().isPresent() {
			self.printer.print(" extends ");
			n.get_extended_type().get().accept(self, arg);
		}
		if n.get_super_type().isPresent() {
			self.printer.print(" super ");
			n.get_super_type().get().accept(self, arg);
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::unknown_type::UnknownType, arg: &/* Java */ java::lang::Void /**/) {
	// Nothing to print
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::field_declaration::FieldDeclaration, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.print_member_annotations(&n.get_annotations(), arg);
		self.print_modifiers(&n.get_modifiers());
		if !n.get_variables().is_empty() {
			let maximum_common_type: Optional<Type> = n.get_maximum_common_type();
			maximum_common_type.ifPresent(|t|t.accept(self, arg));
			if !maximum_common_type.isPresent() {
				self.printer.print("???");
			}
		}
		self.printer.print(" ");
		 {
			/* final */ let i: Iterator<VariableDeclarator> = n.get_variables().iterator();
			while i.hasNext(){
				/* final */ let var: VariableDeclarator = i.next();
				var.accept(self, arg);
				if i.hasNext() {
					self.printer.print(", ");
				}
			}
		 }
	
		self.printer.print(";");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::variable_declarator::VariableDeclarator, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		n.get_name().accept(self, arg);
		n.findAncestor(NodeWithVariables.class).ifPresent(|ancestor|(ancestor as NodeWithVariables<?>).get_maximum_common_type().ifPresent(|common_type|{
			/* final */ let type: Type = n.get_type();
			let array_type: ArrayType = null;
			 {
				let i: i32 = common_type.get_array_level();
				while i < type.get_array_level() {
					{
						if array_type == null {
							array_type = type as ArrayType;
						} else {
							array_type = array_type.get_component_type() as ArrayType;
						}
						self.print_annotations(&array_type.get_annotations(), true, arg);
						self.printer.print("[]");
					}
					i += 1;
				 }
			 }
	
		}));
		if n.get_initializer().isPresent() {
			self.printer.print(" = ");
			n.get_initializer().get().accept(self, arg);
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.IllegalStateException | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print("{");
		if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_values()) {
			/* final */ let multi_line: bool = self.do_print_as_array_of_annotations(n);
			if multi_line {
				self.printer.println();
				self.printer.indent()?;
				self.printer.indent()?;
			} else {
				self.printer.print(" ");
			}
			 {
				/* final */ let i: Iterator<Expression> = n.get_values().iterator();
				while i.hasNext(){
					/* final */ let expr: Expression = i.next();
					expr.accept(self, arg);
					if i.hasNext() {
						self.printer.print( if multi_line { "," } else { ", " });
						if multi_line {
							self.printer.println();
						}
	
					}
				}
			 }
	
			if multi_line {
				self.printer.println();
				self.printer.unindent()?;
				self.printer.unindent()?;
			} else {
				self.printer.print(" ");
			}
		}
		self.print_orphan_comments_ending(n);
		self.printer.print("}");
	}

	fn do_print_as_array_of_annotations(&self, n: &com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr) -> bool {
		return self.get_option(ConfigOption::INDENT_PRINT_ARRAYS_OF_ANNOTATIONS).isPresent() && n.get_values().stream().allMatch(|s|s instanceof AnnotationExpr);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::void_type::VoidType, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.print_annotations(&n.get_annotations(), false, arg);
		self.printer.print("void");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::var_type::VarType, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.print_annotations(&n.get_annotations(), false, arg);
		self.printer.print("var");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modifier::Modifier, arg: &/* Java */ java::lang::Void /**/) {
		self.printer.print(&n.get_keyword().as_string());
		self.printer.print(" ");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_access_expr::ArrayAccessExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		n.get_name().accept(self, arg);
		self.printer.print("[");
		n.get_index().accept(self, arg);
		self.printer.print("]");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_creation_expr::ArrayCreationExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print("new ");
		n.get_element_type().accept(self, arg);
		for level in n.get_levels() {
			level.accept(self, arg);
		}
		if n.get_initializer().isPresent() {
			self.printer.print(" ");
			n.get_initializer().get().accept(self, arg);
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::assign_expr::AssignExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		n.get_target().accept(self, arg);
		if self.get_option(ConfigOption::SPACE_AROUND_OPERATORS).isPresent() {
			self.printer.print(" ");
		}
		self.printer.print(&n.get_operator().as_string());
		if self.get_option(ConfigOption::SPACE_AROUND_OPERATORS).isPresent() {
			self.printer.print(" ");
		}
		n.get_value().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::binary_expr::BinaryExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		n.get_left().accept(self, arg);
		if self.get_option(ConfigOption::SPACE_AROUND_OPERATORS).isPresent() {
			self.printer.print(" ");
		}
		self.printer.print(&n.get_operator().as_string());
		if self.get_option(ConfigOption::SPACE_AROUND_OPERATORS).isPresent() {
			self.printer.print(" ");
		}
		n.get_right().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::cast_expr::CastExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print("(");
		n.get_type().accept(self, arg);
		self.printer.print(") ");
		n.get_expression().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::class_expr::ClassExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		n.get_type().accept(self, arg);
		self.printer.print(".class");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::conditional_expr::ConditionalExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		n.get_condition().accept(self, arg);
		self.printer.print(" ? ");
		n.get_then_expr().accept(self, arg);
		self.printer.print(" : ");
		n.get_else_expr().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::enclosed_expr::EnclosedExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print("(");
		n.get_inner().accept(self, arg);
		self.printer.print(")");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::field_access_expr::FieldAccessExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		n.get_scope().accept(self, arg);
		self.printer.print(".");
		n.get_name().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::instance_of_expr::InstanceOfExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		n.get_expression().accept(self, arg);
		self.printer.print(" instanceof ");
		if n.get_pattern().isPresent() {
			n.get_pattern().get().accept(self, arg);
		} else {
			n.get_type().accept(self, arg);
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::type_pattern_expr::TypePatternExpr, arg: &/* Java */ java::lang::Void /**/) {
		self.print_modifiers(&n.get_modifiers());
		n.get_type().accept(self, arg);
		self.printer.print(" ");
		n.get_name().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::record_pattern_expr::RecordPatternExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.IllegalStateException | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		n.get_type()?.accept(self, arg);
		self.print_arguments(&n.get_pattern_list(), arg)?;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::match_all_pattern_expr::MatchAllPatternExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print(MatchAllPatternExpr::UNNAMED_PLACEHOLDER);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::char_literal_expr::CharLiteralExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print("'");
		self.printer.print(&n.get_value());
		self.printer.print("'");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::double_literal_expr::DoubleLiteralExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print(&n.get_value());
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::integer_literal_expr::IntegerLiteralExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print(&n.get_value());
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::long_literal_expr::LongLiteralExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print(&n.get_value());
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::string_literal_expr::StringLiteralExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print("\"");
		self.printer.print(&n.get_value());
		self.printer.print("\"");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::text_block_literal_expr::TextBlockLiteralExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.IllegalStateException | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print("\"\"\"");
		self.printer.indent()?;
		n.strip_indent_of_lines().forEach(|line|{
			self.printer.println();
			self.printer.print(line);
		});
		self.printer.print("\"\"\"");
		self.printer.unindent()?;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::boolean_literal_expr::BooleanLiteralExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print(&String::valueOf(&n.get_value()));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::null_literal_expr::NullLiteralExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print("null");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::this_expr::ThisExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		if n.get_type_name().isPresent() {
			n.get_type_name().get().accept(self, arg);
			self.printer.print(".");
		}
		self.printer.print("this");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::super_expr::SuperExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		if n.get_type_name().isPresent() {
			n.get_type_name().get().accept(self, arg);
			self.printer.print(".");
		}
		self.printer.print("super");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		// determine whether we do reindenting for aligmnent at all
		// - is it enabled?
		// - are we in a statement where we want the alignment?
		// - are we not directly in the argument list of a method call expression?
		let column_align_first_method_chain: AtomicBoolean = AtomicBoolean::new();
		if self.get_option(ConfigOption::COLUMN_ALIGN_FIRST_METHOD_CHAIN).isPresent() {
			// pick the kind of expressions where vertically aligning method calls is okay.
			if n.findAncestor(Statement.class).map(|p|p.isReturnStmt() || p.isThrowStmt() || p.isAssertStmt() || p.isExpressionStmt()).orElse(false) {
				// search for first parent that does not have its child as scope
				let c: Node = n;
				let p: Optional<Node> = c.get_parent_node();
				while p.isPresent() && p.filter(NodeWithTraversableScope.class::isInstance).map(NodeWithTraversableScope.class::cast).flatMap(NodeWithTraversableScope::traverseScope).map(c::equals).orElse(false) {
					c = p.get();
					p = c.get_parent_node();
				}
				// check if the parent is a method call and thus we are in an argument list
				column_align_first_method_chain.set(!p.filter(MethodCallExpr.class::isInstance).isPresent());
			}
		}
		// we are at the last method call of a call chain
		// this means we do not start reindenting for alignment or we undo it
		let last_method_in_call_chain: AtomicBoolean = AtomicBoolean::new(true);
		if column_align_first_method_chain.get() {
			let node: Node = n;
			while node.get_parent_node().filter(NodeWithTraversableScope.class::isInstance).map(NodeWithTraversableScope.class::cast).flatMap(NodeWithTraversableScope::traverseScope).map(node::equals).orElse(false) {
				node = node.get_parent_node().orElseThrow(AssertionError::new);
				if node instanceof MethodCallExpr {
					last_method_in_call_chain.set(false);
					break;
				}
			}
		}
		// search whether there is a method call with scope in the scope already
		// this means that we probably started reindenting for alignment there
		let method_call_with_scope_in_scope: AtomicBoolean = AtomicBoolean::new();
		if column_align_first_method_chain.get() {
			let s: Optional<Expression> = n.get_scope();
			while s.filter(NodeWithTraversableScope.class::isInstance).isPresent() {
				let parent_scope: Optional<Expression> = s.map(NodeWithTraversableScope.class::cast).flatMap(NodeWithTraversableScope::traverseScope);
				if s.filter(MethodCallExpr.class::isInstance).isPresent() && parent_scope.isPresent() {
					method_call_with_scope_in_scope.set(true);
					break;
				}
				s = parent_scope;
			}
		}
		// we have a scope
		// this means we are not the first method in the chain
		n.get_scope().ifPresent(|scope|{
			scope.accept(self, arg);
			if column_align_first_method_chain.get() {
				if method_call_with_scope_in_scope.get() {
					/*  We're a method call on the result of something (method call, property access, ...) that is not stand alone,
	                    and not the first one with scope, like:
	                    we're x() in a.b().x(), or in a=b().c[15].d.e().x().
	                    That means that the "else" has been executed by one of the methods in the scope chain, so that the alignment
	                    is set to the "." of that method.
	                    That means we will align to that "." when we start a new line: */ 
					self.printer.println();
				} else if !last_method_in_call_chain.get() {
					/*  We're the first method call on the result of something in the chain (method call, property access, ...),
	                    but we are not at the same time the last method call in that chain, like:
	                    we're x() in a().x().y(), or in Long.x().y.z(). That means we get to dictate the indent of following method
	                    calls in this chain by setting the cursor to where we are now: just before the "."
	                    that start this method call. */ 
					self.printer.reindent_with_align_to_cursor();
				}
			}
			self.printer.print(".");
		});
		self.print_type_args(n, arg);
		n.get_name().accept(self, arg);
		self.printer.duplicate_indent();
		self.print_arguments(&n.get_arguments(), arg)?;
		self.printer.unindent()?;
		if column_align_first_method_chain.get() && method_call_with_scope_in_scope.get() && last_method_in_call_chain.get() {
			// undo the aligning after the arguments of the last method call are printed
			self.printer.reindent_to_previous_level()?;
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::object_creation_expr::ObjectCreationExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.IllegalStateException | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		if n.has_scope() {
			n.get_scope().get().accept(self, arg);
			self.printer.print(".");
		}
		self.printer.print("new ");
		self.print_type_args(n, arg);
		if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_type_arguments().orElse(null)) {
			self.printer.print(" ");
		}
		n.get_type().accept(self, arg);
		self.print_arguments(&n.get_arguments(), arg)?;
		if n.get_anonymous_class_body().isPresent() {
			self.printer.println(" {");
			self.printer.indent()?;
			self.print_members(&n.get_anonymous_class_body().get(), arg);
			self.printer.unindent()?;
			self.printer.print("}");
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::unary_expr::UnaryExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		if n.get_operator().is_prefix() {
			self.printer.print(&n.get_operator().as_string());
		}
		n.get_expression().accept(self, arg);
		if n.get_operator().is_postfix() {
			self.printer.print(&n.get_operator().as_string());
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.print_member_annotations(&n.get_annotations(), arg);
		self.print_modifiers(&n.get_modifiers());
		self.print_type_parameters(&n.get_type_parameters(), arg);
		if n.is_generic() {
			self.printer.print(" ");
		}
		n.get_name().accept(self, arg);
		self.printer.print("(");
		n.get_receiver_parameter().ifPresent(|rp|{
			rp.accept(self, arg);
			if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_parameters()) {
				self.printer.print(", ");
			}
		});
		if !n.get_parameters().is_empty() {
			 {
				/* final */ let i: Iterator<Parameter> = n.get_parameters().iterator();
				while i.hasNext(){
					/* final */ let p: Parameter = i.next();
					p.accept(self, arg);
					if i.hasNext() {
						self.printer.print(", ");
					}
				}
			 }
	
		}
		self.printer.print(")");
		if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_thrown_exceptions()) {
			self.printer.print(" throws ");
			 {
				/* final */ let i: Iterator<ReferenceType> = n.get_thrown_exceptions().iterator();
				while i.hasNext(){
					/* final */ let name: ReferenceType = i.next();
					name.accept(self, arg);
					if i.hasNext() {
						self.printer.print(", ");
					}
				}
			 }
	
		}
		self.printer.print(" ");
		n.get_body().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::compact_constructor_declaration::CompactConstructorDeclaration, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.print_member_annotations(&n.get_annotations(), arg);
		self.print_modifiers(&n.get_modifiers());
		self.print_type_parameters(&n.get_type_parameters(), arg);
		if n.is_generic() {
			self.printer.print(" ");
		}
		n.get_name().accept(self, arg);
		if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_thrown_exceptions()) {
			self.printer.print(" throws ");
			 {
				/* final */ let i: Iterator<ReferenceType> = n.get_thrown_exceptions().iterator();
				while i.hasNext(){
					/* final */ let name: ReferenceType = i.next();
					name.accept(self, arg);
					if i.hasNext() {
						self.printer.print(", ");
					}
				}
			 }
	
		}
		self.printer.print(" ");
		n.get_body().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::method_declaration::MethodDeclaration, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.print_member_annotations(&n.get_annotations(), arg);
		self.print_modifiers(&n.get_modifiers());
		self.print_type_parameters(&n.get_type_parameters(), arg);
		if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_type_parameters()) {
			self.printer.print(" ");
		}
		n.get_type().accept(self, arg);
		self.printer.print(" ");
		n.get_name().accept(self, arg);
		self.printer.print("(");
		n.get_receiver_parameter().ifPresent(|rp|{
			rp.accept(self, arg);
			if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_parameters()) {
				self.printer.print(", ");
			}
		});
		if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_parameters()) {
			 {
				/* final */ let i: Iterator<Parameter> = n.get_parameters().iterator();
				while i.hasNext(){
					/* final */ let p: Parameter = i.next();
					p.accept(self, arg);
					if i.hasNext() {
						self.printer.print(", ");
					}
				}
			 }
	
		}
		self.printer.print(")");
		if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_thrown_exceptions()) {
			self.printer.print(" throws ");
			 {
				/* final */ let i: Iterator<ReferenceType> = n.get_thrown_exceptions().iterator();
				while i.hasNext(){
					/* final */ let name: ReferenceType = i.next();
					name.accept(self, arg);
					if i.hasNext() {
						self.printer.print(", ");
					}
				}
			 }
	
		}
		if !n.get_body().isPresent() {
			self.printer.print(";");
		} else {
			self.printer.print(" ");
			n.get_body().get().accept(self, arg);
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::parameter::Parameter, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.print_annotations(&n.get_annotations(), false, arg);
		self.print_modifiers(&n.get_modifiers());
		n.get_type().accept(self, arg);
		if n.is_var_args() {
			self.print_annotations(&n.get_var_args_annotations(), false, arg);
			self.printer.print("...");
		}
		if !(n.get_type().is_unknown_type()) {
			self.printer.print(" ");
		}
		n.get_name().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.print_annotations(&n.get_annotations(), false, arg);
		n.get_type().accept(self, arg);
		self.printer.print(" ");
		n.get_name().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::explicit_constructor_invocation_stmt::ExplicitConstructorInvocationStmt, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.IllegalStateException | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		if n.is_this() {
			self.print_type_args(n, arg);
			self.printer.print("this");
		} else {
			if n.get_expression().isPresent() {
				n.get_expression().get().accept(self, arg);
				self.printer.print(".");
			}
			self.print_type_args(n, arg);
			self.printer.print("super");
		}
		self.print_arguments(&n.get_arguments(), arg)?;
		self.printer.print(";");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		if n.get_parent_node().map(ExpressionStmt.class::isInstance).orElse(false) {
			self.print_member_annotations(&n.get_annotations(), arg);
		} else {
			self.print_annotations(&n.get_annotations(), false, arg);
		}
		self.print_modifiers(&n.get_modifiers());
		if !n.get_variables().is_empty() {
			n.get_maximum_common_type().ifPresent(|t|t.accept(self, arg));
		}
		self.printer.print(" ");
		 {
			/* final */ let i: Iterator<VariableDeclarator> = n.get_variables().iterator();
			while i.hasNext(){
				/* final */ let v: VariableDeclarator = i.next();
				v.accept(self, arg);
				if i.hasNext() {
					self.printer.print(", ");
				}
			}
		 }
	
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::local_class_declaration_stmt::LocalClassDeclarationStmt, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		n.get_class_declaration().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::local_record_declaration_stmt::LocalRecordDeclarationStmt, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		n.get_record_declaration().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::assert_stmt::AssertStmt, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print("assert ");
		n.get_check().accept(self, arg);
		if n.get_message().isPresent() {
			self.printer.print(" : ");
			n.get_message().get().accept(self, arg);
		}
		self.printer.print(";");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.IllegalStateException | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.println("{");
		if n.get_statements() != null {
			self.printer.indent()?;
			for /* final */ s in n.get_statements() {
				s.accept(self, arg);
				self.printer.println();
			}
		}
		self.print_orphan_comments_ending(n);
		self.printer.unindent()?;
		self.printer.print("}");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::labeled_stmt::LabeledStmt, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		n.get_label().accept(self, arg);
		self.printer.print(": ");
		n.get_statement().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::empty_stmt::EmptyStmt, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print(";");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::expression_stmt::ExpressionStmt, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		n.get_expression().accept(self, arg);
		self.printer.print(";");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::switch_stmt::SwitchStmt, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_switch_node(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::switch_expr::SwitchExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_switch_node(n, arg);
	}

	fn print_switch_node(&self, n: &com::github::javaparser::ast::node_types::switch_node::SwitchNode, arg: &/* Java */ java::lang::Void /**/) {
		self.print_comment(&n.get_comment(), arg);
		self.printer.print("switch(");
		n.get_selector().accept(self, arg);
		self.printer.println(") {");
		if n.get_entries() != null {
			self.indent_if(&self.get_option(ConfigOption::INDENT_CASE_IN_SWITCH).isPresent());
			for /* final */ e in n.get_entries() {
				e.accept(self, arg);
			}
			self.unindent_if(&self.get_option(ConfigOption::INDENT_CASE_IN_SWITCH).isPresent());
		}
		self.printer.print("}");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::switch_entry::SwitchEntry, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.IllegalStateException | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		// old/new switch
		/* final */ let separator: String =  if (n.get_type() == SwitchEntry::com::github::javaparser::ast::stmt::switch_entry::Type::STATEMENT_GROUP) { ":" } else { " ->" };
		if com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_labels()) {
			self.printer.print("default" + separator);
		} else {
			self.printer.print("case ");
			 {
				/* final */ let i: Iterator<Expression> = n.get_labels().iterator();
				while i.hasNext(){
					/* final */ let label: Expression = i.next();
					label.accept(self, arg);
					if i.hasNext() {
						self.printer.print(", ");
					}
				}
			 }
	
			// `case null, default -> ...` added in JEP 441
			if n.get_labels().is_non_empty() && n.is_default() {
				self.printer.print(", default");
			}
			if n.get_guard().isPresent() {
				self.printer.print(" when ");
				n.get_guard().get().accept(self, arg);
			}
			self.printer.print(separator);
		}
		self.printer.println();
		self.printer.indent()?;
		if n.get_statements() != null {
			for /* final */ s in n.get_statements() {
				s.accept(self, arg);
				self.printer.println();
			}
		}
		self.printer.unindent()?;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::break_stmt::BreakStmt, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print("break");
		n.get_label().ifPresent(|l|self.printer.print(" ").print(&l.get_identifier()));
		self.printer.print(";");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::yield_stmt::YieldStmt, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print("yield ");
		n.get_expression().accept(self, arg);
		self.printer.print(";");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::return_stmt::ReturnStmt, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print("return");
		if n.get_expression().isPresent() {
			self.printer.print(" ");
			n.get_expression().get().accept(self, arg);
		}
		self.printer.print(";");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::enum_declaration::EnumDeclaration, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.IllegalStateException | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.print_member_annotations(&n.get_annotations(), arg);
		self.print_modifiers(&n.get_modifiers());
		self.printer.print("enum ");
		n.get_name().accept(self, arg);
		if !n.get_implemented_types().is_empty() {
			self.printer.print(" implements ");
			 {
				/* final */ let i: Iterator<ClassOrInterfaceType> = n.get_implemented_types().iterator();
				while i.hasNext(){
					/* final */ let c: ClassOrInterfaceType = i.next();
					c.accept(self, arg);
					if i.hasNext() {
						self.printer.print(", ");
					}
				}
			 }
	
		}
		self.printer.println(" {");
		self.printer.indent()?;
		if n.get_entries().is_non_empty() {
			// Either we hit the constant amount limit in the configurations, or any of the constants has a comment
			/* final */ let align_vertically: bool = n.get_entries().size() > self.get_option(ConfigOption::MAX_ENUM_CONSTANTS_TO_ALIGN_HORIZONTALLY).get().as_integer() || n.get_entries().stream().anyMatch(|e|e.get_comment().isPresent());
			self.printer.println();
			 {
				/* final */ let i: Iterator<EnumConstantDeclaration> = n.get_entries().iterator();
				while i.hasNext(){
					/* final */ let e: EnumConstantDeclaration = i.next();
					e.accept(self, arg);
					if i.hasNext() {
						if align_vertically {
							self.printer.println(",");
						} else {
							self.printer.print(", ");
						}
					}
				}
			 }
	
		}
		if !n.get_members().is_empty() {
			self.printer.println(";");
			self.print_members(&n.get_members(), arg);
		} else {
			if !n.get_entries().is_empty() {
				self.printer.println();
			}
		}
		self.printer.unindent()?;
		self.printer.print("}");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::enum_constant_declaration::EnumConstantDeclaration, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.IllegalStateException | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.print_member_annotations(&n.get_annotations(), arg);
		n.get_name().accept(self, arg);
		if !n.get_arguments().is_empty() {
			self.print_arguments(&n.get_arguments(), arg)?;
		}
		if !n.get_class_body().is_empty() {
			self.printer.println(" {");
			self.printer.indent()?;
			self.print_members(&n.get_class_body(), arg);
			self.printer.unindent()?;
			self.printer.println("}");
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::initializer_declaration::InitializerDeclaration, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		if n.is_static() {
			self.printer.print("static ");
		}
		n.get_body().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::if_stmt::IfStmt, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.IllegalStateException | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print("if (");
		n.get_condition().accept(self, arg);
		/* final */ let then_block: bool = n.get_then_stmt() instanceof BlockStmt;
		if // block statement should start on the same line
		then_block {
			self.printer.print(") ");
		}
		else {
			self.printer.println(")");
			self.printer.indent()?;
		}
		n.get_then_stmt().accept(self, arg);
		if !then_block {
			self.printer.unindent()?;
		}
	
		if n.get_else_stmt().isPresent() {
			if then_block {
				self.printer.print(" ");
			}
			else {self.printer.println();
			}
	
			/* final */ let else_if: bool = n.get_else_stmt().orElse(null) instanceof IfStmt;
			/* final */ let else_block: bool = n.get_else_stmt().orElse(null) instanceof BlockStmt;
			if // put chained if and start of block statement on a same level
			else_if || else_block {
				self.printer.print("else ");
			}
			else {
				self.printer.println("else");
				self.printer.indent()?;
			}
			if n.get_else_stmt().isPresent() {
				n.get_else_stmt().get().accept(self, arg);
			}
	
			if !(else_if || else_block) {
				self.printer.unindent()?;
			}
	
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::while_stmt::WhileStmt, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print("while (");
		n.get_condition().accept(self, arg);
		self.printer.print(") ");
		n.get_body().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::continue_stmt::ContinueStmt, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print("continue");
		n.get_label().ifPresent(|l|self.printer.print(" ").print(&l.get_identifier()));
		self.printer.print(";");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::do_stmt::DoStmt, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print("do ");
		n.get_body().accept(self, arg);
		self.printer.print(" while (");
		n.get_condition().accept(self, arg);
		self.printer.print(");");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::for_each_stmt::ForEachStmt, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print("for (");
		n.get_variable().accept(self, arg);
		self.printer.print(" : ");
		n.get_iterable().accept(self, arg);
		self.printer.print(") ");
		n.get_body().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::for_stmt::ForStmt, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print("for (");
		if n.get_initialization() != null {
			 {
				/* final */ let i: Iterator<Expression> = n.get_initialization().iterator();
				while i.hasNext(){
					/* final */ let e: Expression = i.next();
					e.accept(self, arg);
					if i.hasNext() {
						self.printer.print(", ");
					}
				}
			 }
	
		}
		self.printer.print("; ");
		if n.get_compare().isPresent() {
			n.get_compare().get().accept(self, arg);
		}
		self.printer.print("; ");
		if n.get_update() != null {
			 {
				/* final */ let i: Iterator<Expression> = n.get_update().iterator();
				while i.hasNext(){
					/* final */ let e: Expression = i.next();
					e.accept(self, arg);
					if i.hasNext() {
						self.printer.print(", ");
					}
				}
			 }
	
		}
		self.printer.print(") ");
		n.get_body().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::throw_stmt::ThrowStmt, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print("throw ");
		n.get_expression().accept(self, arg);
		self.printer.print(";");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::synchronized_stmt::SynchronizedStmt, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print("synchronized (");
		n.get_expression().accept(self, arg);
		self.printer.print(") ");
		n.get_body().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::try_stmt::TryStmt, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.IllegalStateException | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print("try ");
		if !n.get_resources().is_empty() {
			self.printer.print("(");
			let resources: Iterator<Expression> = n.get_resources().iterator();
			let first: bool = true;
			while resources.hasNext() {
				resources.next().accept(self, arg);
				if resources.hasNext() {
					self.printer.print(";");
					self.printer.println();
					if first {
						self.printer.indent()?;
					}
				}
				first = false;
			}
			if n.get_resources().size() > 1 {
				self.printer.unindent()?;
			}
			self.printer.print(") ");
		}
		n.get_try_block().accept(self, arg);
		for /* final */ c in n.get_catch_clauses() {
			c.accept(self, arg);
		}
		if n.get_finally_block().isPresent() {
			self.printer.print(" finally ");
			n.get_finally_block().get().accept(self, arg);
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::catch_clause::CatchClause, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print(" catch (");
		n.get_parameter().accept(self, arg);
		self.printer.print(") ");
		n.get_body().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::annotation_declaration::AnnotationDeclaration, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.IllegalStateException | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.print_member_annotations(&n.get_annotations(), arg);
		self.print_modifiers(&n.get_modifiers());
		self.printer.print("@interface ");
		n.get_name().accept(self, arg);
		self.printer.println(" {");
		self.printer.indent()?;
		if n.get_members() != null {
			self.print_members(&n.get_members(), arg);
		}
		self.printer.unindent()?;
		self.printer.print("}");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::annotation_member_declaration::AnnotationMemberDeclaration, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.print_member_annotations(&n.get_annotations(), arg);
		self.print_modifiers(&n.get_modifiers());
		n.get_type().accept(self, arg);
		self.printer.print(" ");
		n.get_name().accept(self, arg);
		self.printer.print("()");
		if n.get_default_value().isPresent() {
			self.printer.print(" default ");
			n.get_default_value().get().accept(self, arg);
		}
		self.printer.print(";");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::marker_annotation_expr::MarkerAnnotationExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print("@");
		n.get_name().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::single_member_annotation_expr::SingleMemberAnnotationExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print("@");
		n.get_name().accept(self, arg);
		self.printer.print("(");
		n.get_member_value().accept(self, arg);
		self.printer.print(")");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::normal_annotation_expr::NormalAnnotationExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print("@");
		n.get_name().accept(self, arg);
		self.printer.print("(");
		if n.get_pairs() != null {
			 {
				/* final */ let i: Iterator<MemberValuePair> = n.get_pairs().iterator();
				while i.hasNext(){
					/* final */ let m: MemberValuePair = i.next();
					m.accept(self, arg);
					if i.hasNext() {
						self.printer.print(", ");
					}
				}
			 }
	
		}
		self.printer.print(")");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::member_value_pair::MemberValuePair, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		n.get_name().accept(self, arg);
		self.printer.print(" = ");
		n.get_value().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::line_comment::LineComment, arg: &/* Java */ java::lang::Void /**/) {
		if !self.get_option(ConfigOption::PRINT_COMMENTS).isPresent() {
			return;
		}
		self.printer.print(&n.get_header()).println(&com::github::javaparser::utils::utils::Utils::normalize_eol_in_text_block(&self.RTRIM.matcher(&n.get_content()).replaceAll(""), ""));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::block_comment::BlockComment, arg: &/* Java */ java::lang::Void /**/) {
		if !self.get_option(ConfigOption::PRINT_COMMENTS).isPresent() {
			return;
		}
		/* final */ let comment_content: String = com::github::javaparser::utils::utils::Utils::normalize_eol_in_text_block(&n.get_content(), &self.get_option(ConfigOption::END_OF_LINE_CHARACTER).get().as_string());
		// as BlockComment should not be formatted, -1 to preserve any trailing empty line if present
		let lines: Vec<String> = comment_content.split("\\R", -1);
		self.printer.print(&n.get_header());
		 {
			let i: i32 = 0;
			while i < (lines.length - 1) {
				{
					self.printer.print(lines[i]);
					// Avoids introducing indentation in blockcomments. ie: do not use println() as it would trigger indentation
					// at the next print call.
					self.printer.print(&self.get_option(ConfigOption::END_OF_LINE_CHARACTER).get().as_value());
				}
				i += 1;
			 }
		 }
	
		// last line is not followed by a newline, and simply terminated with `*/`
		self.printer.print(lines[lines.length - 1]);
		self.printer.println(&n.get_footer());
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::markdown_comment::MarkdownComment, arg: &/* Java */ java::lang::Void /**/) {
		if !self.get_option(ConfigOption::PRINT_COMMENTS).isPresent() {
			return;
		}
		/* final */ let comment_content: String = com::github::javaparser::utils::utils::Utils::normalize_eol_in_text_block(&n.get_content(), &self.get_option(ConfigOption::END_OF_LINE_CHARACTER).get().as_string());
		let lines: Vec<String> = comment_content.split("\\R");
		 {
			let i: i32 = 0;
			while i < (lines.length - 1) {
				{
					self.printer.print(&n.get_header());
					self.printer.print(lines[i]);
					// Avoids introducing indentation in markdown comments. ie: do not use println() as it would trigger
					// indentation
					// at the next print call.
					self.printer.print(&self.get_option(ConfigOption::END_OF_LINE_CHARACTER).get().as_value());
				}
				i += 1;
			 }
		 }
	
		self.printer.print(&n.get_header());
		self.printer.println(lines[lines.length - 1]);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::lambda_expr::LambdaExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		/* final */ let parameters: NodeList<Parameter> = n.get_parameters();
		/* final */ let print_par: bool = n.is_enclosing_parameters();
		if print_par {
			self.printer.print("(");
		}
		 {
			let i: Iterator<Parameter> = parameters.iterator();
			while i.hasNext(){
				let p: Parameter = i.next();
				p.accept(self, arg);
				if i.hasNext() {
					self.printer.print(", ");
				}
			}
		 }
	
		if print_par {
			self.printer.print(")");
		}
		self.printer.print(" -> ");
		/* final */ let body: Statement = n.get_body();
		if body instanceof ExpressionStmt {
			// Print the expression directly
			(body as ExpressionStmt).get_expression().accept(self, arg);
		} else {
			body.accept(self, arg);
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::method_reference_expr::MethodReferenceExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		let scope: Expression = n.get_scope();
		let identifier: String = n.get_identifier();
		if scope != null {
			n.get_scope().accept(self, arg);
		}
		self.printer.print("::");
		self.print_type_args(n, arg);
		if identifier != null {
			self.printer.print(identifier);
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::type_expr::TypeExpr, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		if n.get_type() != null {
			n.get_type().accept(self, arg);
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::node_list::NodeList, arg: &/* Java */ java::lang::Void /**/) {
		for node in n {
			(node as Node).accept(self, arg);
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::import_declaration::ImportDeclaration, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_comment(&n.get_comment(), arg);
		self.printer.print("import ");
		if n.is_static() {
			self.printer.print("static ");
		}
		if n.is_module() {
			self.printer.print("module ");
		}
		n.get_name().accept(self, arg);
		if n.is_asterisk() {
			self.printer.print(".*");
		}
		self.printer.println(";");
		self.print_orphan_comments_ending(n);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration, arg: &/* Java */ java::lang::Void /**/) /* thrown(java.lang.AssertionError | java.lang.IllegalStateException) */ {
		self.print_member_annotations(&n.get_annotations(), arg);
		if n.is_open() {
			self.printer.print("open ");
		}
		self.printer.print("module ");
		n.get_name().accept(self, arg);
		self.printer.println(" {").indent()?;
		n.get_directives().accept(self, arg);
		self.printer.unindent()?.println("}");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_requires_directive::ModuleRequiresDirective, arg: &/* Java */ java::lang::Void /**/) {
		self.printer.print("requires ");
		self.print_modifiers(&n.get_modifiers());
		n.get_name().accept(self, arg);
		self.printer.println(";");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_exports_directive::ModuleExportsDirective, arg: &/* Java */ java::lang::Void /**/) {
		self.printer.print("exports ");
		n.get_name().accept(self, arg);
		self.print_pre_post_fix_optional_list(&n.get_module_names(), arg, " to ", ", ", "");
		self.printer.println(";");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_provides_directive::ModuleProvidesDirective, arg: &/* Java */ java::lang::Void /**/) {
		self.printer.print("provides ");
		n.get_name().accept(self, arg);
		self.print_pre_post_fix_required_list(&n.get_with(), arg, " with ", ", ", "");
		self.printer.println(";");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_uses_directive::ModuleUsesDirective, arg: &/* Java */ java::lang::Void /**/) {
		self.printer.print("uses ");
		n.get_name().accept(self, arg);
		self.printer.println(";");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_opens_directive::ModuleOpensDirective, arg: &/* Java */ java::lang::Void /**/) {
		self.printer.print("opens ");
		n.get_name().accept(self, arg);
		self.print_pre_post_fix_optional_list(&n.get_module_names(), arg, " to ", ", ", "");
		self.printer.println(";");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::unparsable_stmt::UnparsableStmt, arg: &/* Java */ java::lang::Void /**/) {
		self.printer.print("???;");
	}

	fn print_imports(&self, imports: &com::github::javaparser::ast::node_list::NodeList, arg: &/* Java */ java::lang::Void /**/) {
		let strategy: ImportOrderingStrategy = DefaultImportOrderingStrategy::new();
		// Get Import strategy from configuration
		let optional_strategy: Optional<ConfigurationOption> = self.get_option(ConfigOption::SORT_IMPORTS_STRATEGY);
		if optional_strategy.isPresent() {
			let strategy_option: ConfigurationOption = optional_strategy.get();
			if strategy_option.has_value() {
				strategy = strategy_option.as_value();
			}
		}
		// Keep retro-compatibility with option ORDER_IMPORTS.
		let order_imports_option: Optional<ConfigurationOption> = self.get_option(ConfigOption::ORDER_IMPORTS);
		if order_imports_option.isPresent() {
			strategy.set_sort_imports_alphabetically(true);
		}
		// Sort the imports according to the strategy
		let group_ordered_imports: List<NodeList<ImportDeclaration>> = strategy.sort_imports(imports);
		for import_group in group_ordered_imports {
			import_group.accept(self, arg);
			if !import_group.is_empty() {
				self.printer.println();
			}
		}
	}

	fn print_orphan_comments_before_this_child_node(&self, node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		if !self.get_option(ConfigOption::PRINT_COMMENTS).isPresent() {
			return;
		}
	
		if node instanceof Comment {
			return;
		}
	
		let parent: Node = node.get_parent_node().orElse(null);
		if parent == null {
			return;
		}
	
		let everything: List<Node> = ArrayList<>::new(&parent.get_child_nodes());
		com::github::javaparser::utils::position_utils::PositionUtils::sort_by_begin_position(everything);
		let position_of_the_child: i32 = -1;
		 {
			let i: i32 = 0;
			while i < everything.size() {
				{
					// indexOf is by equality, so this is used to index by identity
					if everything.get(i) == node {
						position_of_the_child = i;
						break;
					}
				}
				i += 1;
			 }
		 }
	
		if position_of_the_child == -1 {
			return Err(AssertionError::new("I am not a child of my parent."));
		}
		let position_of_previous_child: i32 = -1;
		 {
			let i: i32 = position_of_the_child - 1;
			while i >= 0 && position_of_previous_child == -1 {
				{
					if !(everything.get(i) instanceof Comment) {
						position_of_previous_child = i;
					}
	
				}
				i -= 1;
			 }
		 }
	
		 {
			let i: i32 = position_of_previous_child + 1;
			while i < position_of_the_child {
				{
					let node_to_print: Node = everything.get(i);
					if !(node_to_print instanceof Comment) {
						return Err(RuntimeException::new("Expected comment, instead " + node_to_print.getClass() + ". Position of previous child: " + position_of_previous_child + ", position of child " + position_of_the_child));
					}
	
					node_to_print.accept(self, null);
				}
				i += 1;
			 }
		 }
	
	}

	fn print_orphan_comments_ending(&self, node: &com::github::javaparser::ast::node::Node) {
		if !self.get_option(ConfigOption::PRINT_COMMENTS).isPresent() {
			return;
		}
	
		let everything: List<Node> = ArrayList<>::new(&node.get_child_nodes());
		com::github::javaparser::utils::position_utils::PositionUtils::sort_by_begin_position(everything);
		if everything.isEmpty() {
			return;
		}
		let comments_at_end: i32 = 0;
		let finding_comments: bool = true;
		while finding_comments && comments_at_end < everything.size() {
			let last: Node = everything.get(everything.size() - 1 - comments_at_end);
			finding_comments = (last instanceof Comment);
			if finding_comments {
				comments_at_end += 1;
			}
		}
		 {
			let i: i32 = 0;
			while i < comments_at_end {
				{
					everything.get(everything.size() - comments_at_end + i).accept(self, null);
				}
				i += 1;
			 }
		 }
	
	}

	fn indent_if(&self, expr: bool) /* thrown(java.lang.AssertionError) */ {
		if expr {
			self.printer.indent()?;
		}
	
	}

	fn unindent_if(&self, expr: bool) /* thrown(java.lang.IllegalStateException) */ {
		if expr {
			self.printer.unindent()?;
		}
	
	}

	fn get_option(&self, c_option: &com::github::javaparser::printer::configuration::default_printer_configuration::ConfigOption) -> /* Java */ java::util::Optional /**/ {
		return self.configuration.get(DefaultConfigurationOption::new(c_option));
	}
}

impl com::github::javaparser::ast::visitor::void_visitor::VoidVisitor for DefaultPrettyPrinterVisitor {}