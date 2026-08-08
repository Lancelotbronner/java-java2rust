use crate::com::github::javaparser::JavaToken::Kind::EOF;
use crate::com::github::javaparser::Providers::UTF8;
use crate::com::github::javaparser::Providers::provider;
use crate::com::github::javaparser::Range::range;
use crate::com::github::javaparser::StaticJavaParser::parseName;
use crate::com::github::javaparser::ast::Modifier::createModifierList;
use crate::com::github::javaparser::utils::CodeGenerationUtils::subtractPaths;
use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser;
use crate::com::github::javaparser::ast::body;
use crate::com::github::javaparser::ast::comments::Comment;
use crate::com::github::javaparser::ast::comments::TraditionalJavadocComment;
use crate::com::github::javaparser::ast::expr::Name;
use crate::com::github::javaparser::ast::modules::ModuleDeclaration;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithName;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::CompilationUnitMetaModel;
use crate::com::github::javaparser::metamodel::InternalProperty;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::OptionalProperty;
use crate::com::github::javaparser::printer::ConfigurablePrinter;
use crate::com::github::javaparser::printer::Printer;
use crate::com::github::javaparser::printer::configuration::PrinterConfiguration;
use crate::com::github::javaparser::utils::ClassUtils;
use crate::com::github::javaparser::utils::CodeGenerationUtils;
use crate::com::github::javaparser::utils::Utils;
use java::io::IOException;
use java::nio::charset::Charset;
use java::nio::file::Files;
use java::nio::file::Path;
use java::nio::file::Paths;
use java::util::List;
use java::util::Objects;
use java::util::Optional;
use java::util::function::Function;
use java::util::stream::Collectors;

pub struct CompilationUnit {
	package_declaration: com::github::javaparser::ast::package_declaration::PackageDeclaration,
	imports: com::github::javaparser::ast::node_list::NodeList,
	types: com::github::javaparser::ast::node_list::NodeList,
	module: com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration,
	storage: com::github::javaparser::ast::compilation_unit::Storage,
}

impl CompilationUnit {
	static JAVA_LANG: /* Java */ java::lang::String /**/ = "java.lang";

	pub fn new() -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		this(null, null, NodeList<>::new(), NodeList<>::new(), null);
	}

	pub fn new(package_declaration: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		this(null, PackageDeclaration::new(Name::new(package_declaration)), NodeList<>::new(), NodeList<>::new(), null);
	}

	pub fn new(package_declaration: &com::github::javaparser::ast::package_declaration::PackageDeclaration, imports: &com::github::javaparser::ast::node_list::NodeList, types: &com::github::javaparser::ast::node_list::NodeList, module: &com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration) -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		this(null, package_declaration, imports, types, module);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, package_declaration: &com::github::javaparser::ast::package_declaration::PackageDeclaration, imports: &com::github::javaparser::ast::node_list::NodeList, types: &com::github::javaparser::ast::node_list::NodeList, module: &com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration) -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		super(token_range);
		self.set_package_declaration(package_declaration);
		self.set_imports(imports);
		self.set_types(types);
		self.set_module(module);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn printer(&self, printer: &com::github::javaparser::printer::printer::Printer) -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		self.set_data(, printer);
		return self;
	}

	fn get_printer(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::printer::printer::Printer {
		if !self.contains_data() {
			// create a default printer
			let printer: Printer = self.create_default_printer();
			self.printer(printer);
		}
		return self.get_data()?;
	}

	fn get_printer(&self, config: &com::github::javaparser::printer::configuration::printer_configuration::PrinterConfiguration) -> com::github::javaparser::printer::printer::Printer {
		let printer: Printer = self.get_printer()?;
		if printer instanceof ConfigurablePrinter {
			(printer as ConfigurablePrinter).set_configuration(config);
		}
		self.printer(printer);
		return printer;
	}

	pub fn get_comments(&self) -> /* Java */ java::util::List /**/ {
		let comments: List<Comment> = self.get_all_contained_comments();
		self.get_comment().ifPresent(comments::add);
		return comments;
	}

	pub fn get_all_comments(&self) -> /* Java */ java::util::List /**/ {
		let comments: List<Comment> = self.get_all_contained_comments();
		self.get_comment().ifPresent(comments::add);
		return comments;
	}

	pub fn get_imports(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.imports;
	}

	pub fn get_import(&self, i: i32) -> com::github::javaparser::ast::import_declaration::ImportDeclaration {
		return self.get_imports().get(i);
	}

	pub fn get_package_declaration(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.package_declaration);
	}

	pub fn get_types(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.types;
	}

	pub fn get_type(&self, i: i32) -> com::github::javaparser::ast::body::type_declaration::TypeDeclaration {
		return self.get_types().get(i);
	}

	pub fn set_imports(&mut self, imports: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		com::github::javaparser::utils::utils::Utils::assert_not_null(imports)?;
		if imports == self.imports {
			return self;
		}
		self.notify_property_change(ObservableProperty::IMPORTS, self.imports, imports);
		if self.imports != null {
			self.imports.set_parent_node(null);
		}
	
		self.imports = imports;
		self.set_as_parent_node_of(imports);
		return self;
	}

	pub fn set_import(&self, i: i32, imports: &com::github::javaparser::ast::import_declaration::ImportDeclaration) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		self.get_imports().set(i, imports)?;
		return self;
	}

	pub fn add_import(&self, import_declaration: &com::github::javaparser::ast::import_declaration::ImportDeclaration) -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		if import_declaration.is_asterisk() {
			self.get_imports().removeIf(|im|Objects::equals(&com::github::javaparser::ast::compilation_unit::CompilationUnit::get_import_package_name(im).get(), &com::github::javaparser::ast::compilation_unit::CompilationUnit::get_import_package_name(import_declaration).orElse(null)));
		}
		if !self.is_implicit_import(import_declaration) && self.get_imports().stream().noneMatch(|im|im.equals(import_declaration) || (im.is_asterisk() && Objects::equals(&com::github::javaparser::ast::compilation_unit::CompilationUnit::get_import_package_name(im).get(), &com::github::javaparser::ast::compilation_unit::CompilationUnit::get_import_package_name(import_declaration).orElse(null)))) {
			self.get_imports().add(import_declaration);
		}
		return self;
	}

	fn is_implicit_import(&self, import_declaration: &com::github::javaparser::ast::import_declaration::ImportDeclaration) -> bool {
		let import_package_name: Optional<Name> = com::github::javaparser::ast::compilation_unit::CompilationUnit::get_import_package_name(import_declaration);
		if import_package_name.isPresent() {
			if com::github::javaparser::static_java_parser::StaticJavaParser::parse_name(self.JAVA_LANG).equals(&import_package_name.get()) {
				// java.lang is implicitly imported
				return true;
			}
			if self.package_declaration != null {
				// the import is within the same package
				let current_package_name: Name = self.package_declaration.get_name();
				return current_package_name.equals(&import_package_name.get());
			}
			return false;
		}
		return true;
	}

	fn get_import_package_name(&self, import_declaration: &com::github::javaparser::ast::import_declaration::ImportDeclaration) -> /* Java */ java::util::Optional /**/ {
		return ( if import_declaration.is_asterisk() { Name::new(&import_declaration.get_name(), "*") } else { import_declaration.get_name() }).get_qualifier();
	}

	pub fn set_package_declaration(&mut self, package_declaration: &com::github::javaparser::ast::package_declaration::PackageDeclaration) -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		if package_declaration == self.packageDeclaration {
			return self;
		}
		self.notify_property_change(ObservableProperty::PACKAGE_DECLARATION, self.packageDeclaration, package_declaration);
		if self.packageDeclaration != null {
			self.packageDeclaration.set_parent_node(null);
		}
	
		self.packageDeclaration = package_declaration;
		self.set_as_parent_node_of(package_declaration);
		return self;
	}

	pub fn set_types(&mut self, types: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		com::github::javaparser::utils::utils::Utils::assert_not_null(types)?;
		if types == self.types {
			return self;
		}
		self.notify_property_change(ObservableProperty::TYPES, self.types, types);
		if self.types != null {
			self.types.set_parent_node(null);
		}
	
		self.types = types;
		self.set_as_parent_node_of(types);
		return self;
	}

	pub fn set_type(&self, i: i32, type: &com::github::javaparser::ast::body::type_declaration::TypeDeclaration) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		let copy: NodeList<TypeDeclaration<?>> = NodeList<>::new();
		copy.add_all(&self.get_types());
		self.get_types().set(i, type)?;
		self.notify_property_change(ObservableProperty::TYPES, copy, self.types);
		return self;
	}

	pub fn add_type(&self, type: &com::github::javaparser::ast::body::type_declaration::TypeDeclaration) -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		let copy: NodeList<TypeDeclaration<?>> = NodeList<>::new();
		copy.add_all(&self.get_types());
		self.get_types().add(type);
		self.notify_property_change(ObservableProperty::TYPES, copy, self.types);
		return self;
	}

	pub fn set_package_declaration(&self, name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		self.set_package_declaration(PackageDeclaration::new(&com::github::javaparser::static_java_parser::StaticJavaParser::parse_name(name)));
		return self;
	}

	pub fn add_import(&self, name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		return self.add_import(name, false, false, false);
	}

	pub fn add_import(&self, clazz: &/* Java */ java::lang::Class /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		if clazz.isArray() {
			return self.add_import(&clazz.getComponentType())?;
		}
		if ClassUtils::is_primitive_or_wrapper(clazz) || self.JAVA_LANG.equals(&clazz.getPackage().getName()) {
			return self;
		}
	
		if clazz.isAnonymousClass() || clazz.isLocalClass() {
			return Err(IllegalArgumentException::new(clazz.getName() + " is an anonymous or local class therefore it can't be added with addImport"));
		}
	
		return self.add_import(&clazz.getCanonicalName());
	}

	pub fn add_import(&self, name: &/* Java */ java::lang::String /**/, is_static: bool, is_asterisk: bool) -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		if name == null {
			return self;
		}
		return self.add_import(ImportDeclaration::new(name, is_static, is_asterisk, false));
	}

	pub fn add_import(&self, name: &/* Java */ java::lang::String /**/, is_static: bool, is_asterisk: bool, is_module: bool) -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		if name == null {
			return self;
		}
		return self.add_import(ImportDeclaration::new(name, is_static, is_asterisk, is_module));
	}

	pub fn add_class(&self, name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration {
		return self.add_class(name, Modifier::com::github::javaparser::ast::modifier::Keyword::PUBLIC);
	}

	pub fn add_class(&self, name: &/* Java */ java::lang::String /**/, modifiers: &com::github::javaparser::ast::modifier::Keyword) -> com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration {
		let class_or_interface_declaration: ClassOrInterfaceDeclaration = ClassOrInterfaceDeclaration::new(&com::github::javaparser::ast::modifier::Modifier::create_modifier_list(modifiers), false, name);
		self.get_types().add(class_or_interface_declaration);
		return class_or_interface_declaration;
	}

	pub fn add_interface(&self, name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration {
		return self.add_interface(name, Modifier::com::github::javaparser::ast::modifier::Keyword::PUBLIC);
	}

	pub fn add_interface(&self, name: &/* Java */ java::lang::String /**/, modifiers: &com::github::javaparser::ast::modifier::Keyword) -> com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration {
		let class_or_interface_declaration: ClassOrInterfaceDeclaration = ClassOrInterfaceDeclaration::new(&com::github::javaparser::ast::modifier::Modifier::create_modifier_list(modifiers), true, name);
		self.get_types().add(class_or_interface_declaration);
		return class_or_interface_declaration;
	}

	pub fn add_enum(&self, name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::body::enum_declaration::EnumDeclaration {
		return self.add_enum(name, Modifier::com::github::javaparser::ast::modifier::Keyword::PUBLIC);
	}

	pub fn add_enum(&self, name: &/* Java */ java::lang::String /**/, modifiers: &com::github::javaparser::ast::modifier::Keyword) -> com::github::javaparser::ast::body::enum_declaration::EnumDeclaration {
		let enum_declaration: EnumDeclaration = EnumDeclaration::new(&com::github::javaparser::ast::modifier::Modifier::create_modifier_list(modifiers), name);
		self.get_types().add(enum_declaration);
		return enum_declaration;
	}

	pub fn add_annotation_declaration(&self, name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::body::annotation_declaration::AnnotationDeclaration {
		return self.add_annotation_declaration(name, Modifier::com::github::javaparser::ast::modifier::Keyword::PUBLIC);
	}

	pub fn add_annotation_declaration(&self, name: &/* Java */ java::lang::String /**/, modifiers: &com::github::javaparser::ast::modifier::Keyword) -> com::github::javaparser::ast::body::annotation_declaration::AnnotationDeclaration {
		let annotation_declaration: AnnotationDeclaration = AnnotationDeclaration::new(&com::github::javaparser::ast::modifier::Modifier::create_modifier_list(modifiers), name);
		self.get_types().add(annotation_declaration);
		return annotation_declaration;
	}

	pub fn get_class_by_name(&self, class_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::util::Optional /**/ {
		return self.get_types().stream().filter(|type|type.get_name_as_string().equals(class_name) && type instanceof ClassOrInterfaceDeclaration && !(type as ClassOrInterfaceDeclaration).is_interface()).findFirst().map(|t|t as ClassOrInterfaceDeclaration);
	}

	pub fn get_local_declaration_from_classname(&self, class_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::util::List /**/ {
		return self.find_all(ClassOrInterfaceDeclaration.class).stream().filter(|cid|cid.get_fully_qualified_name().get().endsWith(class_name)).collect(&Collectors::toList());
	}

	pub fn get_interface_by_name(&self, interface_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::util::Optional /**/ {
		return self.get_types().stream().filter(|type|type.get_name_as_string().equals(interface_name) && type instanceof ClassOrInterfaceDeclaration && (type as ClassOrInterfaceDeclaration).is_interface()).findFirst().map(|t|t as ClassOrInterfaceDeclaration);
	}

	pub fn get_enum_by_name(&self, enum_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::util::Optional /**/ {
		return self.get_types().stream().filter(|type|type.get_name_as_string().equals(enum_name) && type instanceof EnumDeclaration).findFirst().map(|t|t as EnumDeclaration);
	}

	pub fn get_primary_type_name(&self) -> /* Java */ java::util::Optional /**/ {
		return self.get_storage().map(Storage::getFileName).map(Utils::removeFileExtension);
	}

	pub fn get_primary_type(&self) -> /* Java */ java::util::Optional /**/ {
		return self.get_primary_type_name().flatMap(|name|self.get_types().stream().filter(|t|t.get_name_as_string().equals(name)).findFirst());
	}

	pub fn get_annotation_declaration_by_name(&self, annotation_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::util::Optional /**/ {
		return self.get_types().stream().filter(|type|type.get_name_as_string().equals(annotation_name) && type instanceof AnnotationDeclaration).findFirst().map(|t|t as AnnotationDeclaration);
	}

	pub fn get_record_by_name(&self, record_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::util::Optional /**/ {
		return self.get_types().stream().filter(|type|type.get_name_as_string().equals(record_name) && type instanceof RecordDeclaration).findFirst().map(|t|t as RecordDeclaration);
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.imports.size() {
				{
					if self.imports.get(i) == node {
						self.imports.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		if self.module != null {
			if node == self.module {
				self.remove_module();
				return true;
			}
		}
		if self.package_declaration != null {
			if node == self.package_declaration {
				self.remove_package_declaration();
				return true;
			}
		}
		 {
			let i: i32 = 0;
			while i < self.types.size() {
				{
					if self.types.get(i) == node {
						self.types.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn remove_package_declaration(&self) -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		return self.set_package_declaration(null as PackageDeclaration);
	}

	pub fn get_module(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.module);
	}

	pub fn set_module(&mut self, module: &com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration) -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		if module == self.module {
			return self;
		}
		self.notify_property_change(ObservableProperty::MODULE, self.module, module);
		if self.module != null {
			self.module.set_parent_node(null);
		}
	
		self.module = module;
		self.set_as_parent_node_of(module);
		return self;
	}

	pub fn remove_module(&self) -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		return self.set_module(null as ModuleDeclaration);
	}

	pub fn get_storage(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.storage);
	}

	pub fn set_storage(&mut self, path: &/* Java */ java::nio::file::Path /**/) -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		self.storage = Storage::new(self, path);
		return self;
	}

	pub fn set_storage(&mut self, path: &/* Java */ java::nio::file::Path /**/, charset: &/* Java */ java::nio::charset::Charset /**/) -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		self.storage = Storage::new(self, path, charset);
		return self;
	}

	pub fn set_module(&self, name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration {
		/* final */ let module: ModuleDeclaration = ModuleDeclaration::new(&com::github::javaparser::static_java_parser::StaticJavaParser::parse_name(name), false);
		self.set_module(module);
		return module;
	}

	pub fn recalculate_positions(&self) /* thrown(java.lang.IllegalStateException) */ {
		if !self.get_token_range().isPresent() {
			return Err(IllegalStateException::new("Can't recalculate positions without tokens."));
		}
		let cursor: Position = Position::com::github::javaparser::position::Position::HOME;
		for t in self.get_token_range().get() {
			let token_length: i32 =  if t.get_kind() == EOF.get_kind() { 0 } else { t.get_text().length() - 1 };
			t.set_range(&com::github::javaparser::range::Range::range(cursor, &cursor.right(token_length)));
			if t.get_category().is_end_of_line() {
				cursor = cursor.next_line();
			} else {
				cursor = cursor.right(token_length + 1);
			}
		}
	}

	pub fn clone(&self) -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		return self.accept(CloneVisitor::new(), null) as CompilationUnit;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::compilation_unit_meta_model::CompilationUnitMetaModel {
		return JavaParserMetaModel::compilationUnitMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.imports.size() {
				{
					if self.imports.get(i) == node {
						self.imports.set(i, replacement_node as ImportDeclaration)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		if self.module != null {
			if node == self.module {
				self.set_module(replacement_node as ModuleDeclaration);
				return true;
			}
		}
		if self.package_declaration != null {
			if node == self.package_declaration {
				self.set_package_declaration(replacement_node as PackageDeclaration);
				return true;
			}
		}
		 {
			let i: i32 = 0;
			while i < self.types.size() {
				{
					if self.types.get(i) == node {
						self.types.set(i, replacement_node as TypeDeclaration)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.replace(node, replacement_node);
	}
}

impl /* Java */ java::lang::Cloneable /**/ for CompilationUnit {}

impl com::github::javaparser::has_parent_node::HasParentNode for CompilationUnit {}

impl com::github::javaparser::ast::observer::observable::Observable for CompilationUnit {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for CompilationUnit {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for CompilationUnit {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for CompilationUnit {}

pub struct Storage {
	compilation_unit: com::github::javaparser::ast::compilation_unit::CompilationUnit,
	path: /* Java */ java::nio::file::Path /**/,
	encoding: /* Java */ java::nio::charset::Charset /**/,
}

impl Storage {
	fn new(compilation_unit: &com::github::javaparser::ast::compilation_unit::CompilationUnit, path: &/* Java */ java::nio::file::Path /**/) -> com::github::javaparser::ast::compilation_unit::Storage {
		this(compilation_unit, path, );
	}

	fn new(compilation_unit: &com::github::javaparser::ast::compilation_unit::CompilationUnit, path: &/* Java */ java::nio::file::Path /**/, encoding: &/* Java */ java::nio::charset::Charset /**/) -> com::github::javaparser::ast::compilation_unit::Storage {
		self.compilationUnit = compilation_unit;
		self.path = path.toAbsolutePath();
		self.encoding = encoding;
	}

	pub fn get_path(&self) -> /* Java */ java::nio::file::Path /**/ {
		return self.path;
	}

	pub fn get_compilation_unit(&self) -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		return self.compilation_unit;
	}

	pub fn get_encoding(&self) -> /* Java */ java::nio::charset::Charset /**/ {
		return self.encoding;
	}

	pub fn get_source_root(&self) -> /* Java */ java::nio::file::Path /**/ {
		/* final */ let pkg_as_string: Optional<String> = self.compilation_unit.get_package_declaration().map(NodeWithName::getNameAsString);
		return pkg_as_string.map(|p|Paths::get(&CodeGenerationUtils::package_to_path(p))).map(|pkg|com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::subtract_paths(&self.get_directory(), pkg)?).orElseGet(|()|self.get_directory());
	}

	pub fn get_file_name(&self) -> /* Java */ java::lang::String /**/ {
		return self.path.getFileName().toString();
	}

	pub fn get_directory(&self) -> /* Java */ java::nio::file::Path /**/ {
		return self.path.getParent();
	}

	pub fn save(&self) {
		self.save(|cu|self.compilation_unit.get_printer()?.print(cu));
	}

	pub fn save(&self, make_output: &/* Java */ java::util::function::Function /**/) /* thrown(java.lang.RuntimeException) */ {
		self.save(make_output, self.encoding)?;
	}

	pub fn save(&self, make_output: &/* Java */ java::util::function::Function /**/, encoding: &/* Java */ java::nio::charset::Charset /**/) /* thrown(java.lang.RuntimeException) */ {
		let r0 = 'try0: {
			Files::createDirectories(&self.path.getParent());
			/* final */ let code: String = make_output.apply(&self.get_compilation_unit());
			Files::write(self.path, &code.getBytes(encoding));
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ IOException) => {
				break 'try0 Err(RuntimeException::new(e));
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn reparse(&self, java_parser: &com::github::javaparser::java_parser::JavaParser) /* thrown(java.lang.RuntimeException) */ -> com::github::javaparser::parse_result::ParseResult {
		let r0 = 'try0: {
			return java_parser.parse(ParseStart.com::github::javaparser::parse_start::ParseStart::COMPILATION_UNIT, &com::github::javaparser::providers::Providers::provider(&self.get_path()));
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ IOException) => {
				break 'try0 Err(RuntimeException::new(e));
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}
}