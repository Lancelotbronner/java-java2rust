use java::io;
use java::util;
use crate::com::github::javaparser::ast;
use crate::com::github::javaparser::ast::body;
use crate::com::github::javaparser::ast::comments;
use crate::com::github::javaparser::ast::modules;
use crate::com::github::javaparser::ast::expr;
use crate::com::github::javaparser::ast::stmt;
use crate::com::github::javaparser::ast::type;
use crate::com::github::javaparser::utils;
use crate::com::github::javaparser::JavaToken::INVALID;
use crate::com::github::javaparser::ast::Node::Parsedness::UNPARSABLE;
use crate::com::github::javaparser::utils::Utils;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::GeneratedJavaParser;
use crate::com::github::javaparser::Range;
use crate::com::github::javaparser::Position;
use crate::com::github::javaparser::ast::type::ArrayType;
use crate::com::github::javaparser::GeneratedJavaParserTokenManagerBase;
use crate::com::github::javaparser::ast::stmt::SwitchEntry::Type;

struct GeneratedJavaParser {
	token_source: com::github::javaparser::generated_java_parser_token_manager::GeneratedJavaParserTokenManager,
	jj_input_stream: com::github::javaparser::simple_char_stream::SimpleCharStream,
	token: com::github::javaparser::token::Token,
	jj_nt: com::github::javaparser::token::Token,
	jj_ntk: i32,
	jj_scanpos: com::github::javaparser::token::Token,
	jj_lastpos: com::github::javaparser::token::Token,
	jj_la: i32,
	jj_looking_ahead: bool = false,
	jj_semla: bool,
	jj_gen: i32,
	jj_la1: &[i32] = : [i32; 190] = [0; 190],
	jj_2_rtns: &[com::github::javaparser::generated_java_parser::JJCalls] = : [Option<JJCalls>; 79] = [None; 79],
	jj_rescan: bool = false,
	jj_gc: i32 = 0,
	jj_ls: com::github::javaparser::generated_java_parser::LookaheadSuccess = LookaheadSuccess::new(),
	jj_expentries: /* Java */ java::util::List /**/ = java.util.ArrayList<>::new(),
	jj_expentry: &[i32],
	jj_kind: i32 = -1,
	jj_lasttokens: &[i32] = : [i32; 100] = [0; 100],
	jj_endpos: i32,
}

impl GeneratedJavaParser {
	static jj_la1_0: &[i32];

	static jj_la1_1: &[i32];

	static jj_la1_2: &[i32];

	static jj_la1_3: &[i32];

	static jj_la1_4: &[i32];

	fn token(&self) -> com::github::javaparser::java_token::JavaToken {
		return self.token.javaToken;
	}

	fn get_current_token(&self) -> com::github::javaparser::token::Token {
		return self.token;
	}

	fn set_tab_size(&self, size: i32) {
		self.jj_input_stream.set_tab_size(size);
	}

	fn get_token_source(&self) -> com::github::javaparser::generated_java_parser_token_manager::GeneratedJavaParserTokenManager {
		return self.token_source;
	}

	pub fn compilation_unit(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		let package_declaration: PackageDeclaration = null;
		let imports: NodeList<ImportDeclaration> = self.empty_node_list();
		let in: ImportDeclaration = null;
		let body_declarations: NodeList<BodyDeclaration<?>> = self.empty_node_list();
		let modifier: ModifierHolder;
		let body_declaration: BodyDeclaration<?> = null;
		let module: ModuleDeclaration = null;
		let r0 = 'try0: {
			'label_1: while true {
				if self.jj_2_1(2) {
				} else {
					break 'label_1;
				}
				match self.jj_consume_token() {
					Err(e) => break 'try0 Err(e),
					Ok(s) => s,
				};
			}
			if self.jj_2_2(2147483647) {
				package_declaration = match self.package_declaration() {
					Err(e) => break 'try0 Err(e),
					Ok(s) => s,
				};
			} else {
				;
			}
			'label_2: while true {
				match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
						{
							break;
						}
					}
					_ =>  {
						self.jj_la1[0] = self.jj_gen;
						break 'label_2;
					}
				}
				match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
					 =>  {
						{
							in = match self.import_declaration() {
								Err(e) => break 'try0 Err(e),
								Ok(s) => s,
							};
							imports = self.add(imports, in);
							break;
						}
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
						{
							modifier = match self.modifiers() {
								Err(e) => break 'try0 Err(e),
								Ok(s) => s,
							};
							match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
								 =>  {
								}
								 =>  {
									{
										body_declaration = match self.class_or_interface_declaration(modifier) {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										body_declarations = self.add(body_declarations, body_declaration);
										break;
									}
								}
								 =>  {
									{
										body_declaration = match self.record_declaration(modifier) {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										body_declarations = self.add(body_declarations, body_declaration);
										break;
									}
								}
								 =>  {
									{
										body_declaration = match self.enum_declaration(modifier) {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										body_declarations = self.add(body_declarations, body_declaration);
										break;
									}
								}
								 =>  {
									{
										body_declaration = match self.annotation_type_declaration(modifier) {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										body_declarations = self.add(body_declarations, body_declaration);
										break;
									}
								}
								 =>  {
								}
								 =>  {
									{
										module = match self.module_declaration(modifier) {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
								}
								 =>  {
								}
								 =>  {
								}
								 =>  {
								}
								 =>  {
								}
								 =>  {
								}
								 =>  {
								}
								 =>  {
								}
								 =>  {
								}
								 =>  {
								}
								 =>  {
								}
								 =>  {
								}
								 =>  {
								}
								 =>  {
								}
								 =>  {
								}
								 =>  {
								}
								 =>  {
								}
								 =>  {
								}
								 =>  {
								}
								 =>  {
								}
								 =>  {
								}
								 =>  {
								}
								 =>  {
								}
								 =>  {
								}
								 =>  {
								}
								 =>  {
								}
								 =>  {
									{
										body_declaration = match self.compact_class_member(modifier) {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										body_declarations = self.add(body_declarations, body_declaration);
										break;
									}
								}
								 =>  {
									{
										match self.jj_consume_token() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
								}
								_ =>  {
									self.jj_la1[1] = self.jj_gen;
									match self.jj_consume_token(-1) {
										Err(e) => break 'try0 Err(e),
										Ok(s) => s,
									};
									break 'try0 Err(ParseException::new());
								}
							}
							break;
						}
					}
					_ =>  {
						self.jj_la1[2] = self.jj_gen;
						self.jj_consume_token(-1)?;
						return Err(ParseException::new());
					}
				}
			}
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				0 =>  {
					{
						self.jj_consume_token(0)?;
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						break;
					}
				}
				_ =>  {
					self.jj_la1[3] = self.jj_gen;
					self.jj_consume_token(-1)?;
					return Err(ParseException::new());
				}
			}
			let types: NodeList<TypeDeclaration<?>> = self.type_declarations_for_cu(body_declarations);
			{
				if "" != null {
					return CompilationUnit::new(&self.range(&self.token_source.get_home_token(), &self.token()), package_declaration, imports, types, module);
				}
	
			}
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ ParseException) => {
				self.recover(, e);
				/* final */ let compilation_unit: CompilationUnit = CompilationUnit::new(&self.range(&self.token_source.get_home_token(), &self.token()), null, NodeList<ImportDeclaration>::new(), NodeList<TypeDeclaration<?>>::new(), null);
				compilation_unit.set_parsed(UNPARSABLE);
				{
					if "" != null {
						return compilation_unit;
					}
	
				}
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn compact_class_member(&mut self, modifier: &com::github::javaparser::modifier_holder::ModifierHolder) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::body_declaration::BodyDeclaration {
		let member: BodyDeclaration<?>;
		if self.jj_2_3(2147483647) {
			member = self.field_declaration(modifier)?;
		} else {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
					{
						member = self.method_declaration(modifier)?;
						break;
					}
				}
				_ =>  {
					self.jj_la1[4] = self.jj_gen;
					self.jj_consume_token(-1)?;
					return Err(ParseException::new());
				}
			}
		}
		{
			if "" != null {
				return member;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn package_declaration(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::package_declaration::PackageDeclaration {
		let annotations: NodeList<AnnotationExpr> = NodeList<AnnotationExpr>::new();
		let name: Name;
		let begin: JavaToken = ;
		let ann: AnnotationExpr;
		'label_3: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[5] = self.jj_gen;
					break 'label_3;
				}
			}
			ann = self.annotation()?;
			annotations = self.add(annotations, ann);
			begin = self.or_if_invalid(begin, ann);
		}
		self.jj_consume_token()?;
		begin = self.or_if_invalid(begin, &self.token());
		name = self.name()?;
		self.jj_consume_token()?;
		{
			if "" != null {
				return PackageDeclaration::new(&self.range(begin, &self.token()), annotations, name);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn import_declaration(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::import_declaration::ImportDeclaration {
		let name: Name;
		let is_static: bool = false;
		let is_asterisk: bool = false;
		let is_module: bool = false;
		let begin: JavaToken;
		self.jj_consume_token()?;
		begin = self.token();
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					self.jj_consume_token()?;
					is_static = true;
					break;
				}
			}
			_ =>  {
				self.jj_la1[6] = self.jj_gen;
				;
			}
		}
		if self.jj_2_4(3) {
			self.jj_consume_token()?;
			is_module = true;
			name = self.name()?;
		} else {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
					{
						name = self.name()?;
						break;
					}
				}
				_ =>  {
					self.jj_la1[7] = self.jj_gen;
					self.jj_consume_token(-1)?;
					return Err(ParseException::new());
				}
			}
		}
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					self.jj_consume_token()?;
					self.jj_consume_token()?;
					is_asterisk = true;
					break;
				}
			}
			_ =>  {
				self.jj_la1[8] = self.jj_gen;
				;
			}
		}
		self.jj_consume_token()?;
		{
			if "" != null {
				return ImportDeclaration::new(&self.range(begin, &self.token()), name, is_static, is_asterisk, is_module);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn modifiers(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::modifier_holder::ModifierHolder {
		let begin: JavaToken = ;
		let modifiers: NodeList<Modifier> = NodeList<Modifier>::new();
		let annotations: NodeList<AnnotationExpr> = NodeList<AnnotationExpr>::new();
		let ann: AnnotationExpr;
		'label_4: while true {
			if self.jj_2_5(2) {
			} else {
				break 'label_4;
			}
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						self.jj_consume_token()?;
						self.add(modifiers, Modifier::new(&self.token_range(), Modifier.Keyword::PUBLIC));
						begin = self.or_if_invalid(begin, &self.token());
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						self.add(modifiers, Modifier::new(&self.token_range(), Modifier.Keyword::STATIC));
						begin = self.or_if_invalid(begin, &self.token());
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						self.add(modifiers, Modifier::new(&self.token_range(), Modifier.Keyword::PROTECTED));
						begin = self.or_if_invalid(begin, &self.token());
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						self.add(modifiers, Modifier::new(&self.token_range(), Modifier.Keyword::PRIVATE));
						begin = self.or_if_invalid(begin, &self.token());
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						self.add(modifiers, Modifier::new(&self.token_range(), Modifier.Keyword::FINAL));
						begin = self.or_if_invalid(begin, &self.token());
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						self.add(modifiers, Modifier::new(&self.token_range(), Modifier.Keyword::ABSTRACT));
						begin = self.or_if_invalid(begin, &self.token());
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						self.add(modifiers, Modifier::new(&self.token_range(), Modifier.Keyword::SYNCHRONIZED));
						begin = self.or_if_invalid(begin, &self.token());
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						self.add(modifiers, Modifier::new(&self.token_range(), Modifier.Keyword::NATIVE));
						begin = self.or_if_invalid(begin, &self.token());
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						self.add(modifiers, Modifier::new(&self.token_range(), Modifier.Keyword::TRANSIENT));
						begin = self.or_if_invalid(begin, &self.token());
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						self.add(modifiers, Modifier::new(&self.token_range(), Modifier.Keyword::VOLATILE));
						begin = self.or_if_invalid(begin, &self.token());
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						self.add(modifiers, Modifier::new(&self.token_range(), Modifier.Keyword::STRICTFP));
						begin = self.or_if_invalid(begin, &self.token());
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						self.add(modifiers, Modifier::new(&self.token_range(), Modifier.Keyword::TRANSITIVE));
						begin = self.or_if_invalid(begin, &self.token());
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						self.add(modifiers, Modifier::new(&self.token_range(), Modifier.Keyword::DEFAULT));
						begin = self.or_if_invalid(begin, &self.token());
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						self.add(modifiers, Modifier::new(&self.token_range(), Modifier.Keyword::SEALED));
						begin = self.or_if_invalid(begin, &self.token());
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						self.add(modifiers, Modifier::new(&self.token_range(), Modifier.Keyword::NON_SEALED));
						begin = self.or_if_invalid(begin, &self.token());
						break;
					}
				}
				 =>  {
					{
						ann = self.annotation()?;
						annotations = self.add(annotations, ann);
						begin = self.or_if_invalid(begin, ann);
						break;
					}
				}
				_ =>  {
					self.jj_la1[9] = self.jj_gen;
					self.jj_consume_token(-1)?;
					return Err(ParseException::new());
				}
			}
		}
		{
			if "" != null {
				return ModifierHolder::new(begin, modifiers, annotations);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn class_or_interface_declaration(&mut self, modifier: &com::github::javaparser::modifier_holder::ModifierHolder) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration {
		let is_interface: bool;
		let name: SimpleName;
		let type_par: RangedList<TypeParameter> = RangedList<TypeParameter>::new(&self.empty_node_list());
		let ext_list: NodeList<ClassOrInterfaceType> = self.empty_node_list();
		let imp_list: NodeList<ClassOrInterfaceType> = self.empty_node_list();
		let permits_list: NodeList<ClassOrInterfaceType> = self.empty_node_list();
		let members: NodeList<BodyDeclaration<?>> = self.empty_node_list();
		let begin: JavaToken = modifier.begin;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					self.jj_consume_token()?;
					is_interface = false;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					is_interface = true;
					break;
				}
			}
			_ =>  {
				self.jj_la1[10] = self.jj_gen;
				self.jj_consume_token(-1)?;
				return Err(ParseException::new());
			}
		}
		begin = self.or_if_invalid(begin, &self.token());
		name = self.simple_name()?;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					type_par = self.type_parameters()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[11] = self.jj_gen;
				;
			}
		}
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					ext_list = self.extends_list()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[12] = self.jj_gen;
				;
			}
		}
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					imp_list = self.implements_list()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[13] = self.jj_gen;
				;
			}
		}
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					permits_list = self.permits_list()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[14] = self.jj_gen;
				;
			}
		}
		members = self.class_or_interface_body()?;
		{
			if "" != null {
				return ClassOrInterfaceDeclaration::new(&self.range(begin, &self.token()), modifier.modifiers, modifier.annotations, is_interface, name, type_par.list, ext_list, imp_list, permits_list, members);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn record_declaration(&mut self, modifier: &com::github::javaparser::modifier_holder::ModifierHolder) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::record_declaration::RecordDeclaration {
		let name: SimpleName;
		let parameters: Pair<NodeList<Parameter>, ReceiverParameter>;
		let type_par: RangedList<TypeParameter> = RangedList<TypeParameter>::new(&self.empty_node_list());
		let imp_list: NodeList<ClassOrInterfaceType> = self.empty_node_list();
		let members: NodeList<BodyDeclaration<?>> = self.empty_node_list();
		let begin: JavaToken = modifier.begin;
		self.jj_consume_token()?;
		begin = self.or_if_invalid(begin, &self.token());
		name = self.simple_name()?;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					type_par = self.type_parameters()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[15] = self.jj_gen;
				;
			}
		}
		parameters = self.parameters()?;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					imp_list = self.implements_list()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[16] = self.jj_gen;
				;
			}
		}
		members = self.record_body()?;
		{
			if "" != null {
				return RecordDeclaration::new(&self.range(begin, &self.token()), modifier.modifiers, modifier.annotations, name, parameters.a, type_par.list, imp_list, members, parameters.b);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn extends_list(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::node_list::NodeList {
		let extends_more_than_one: bool = false;
		let ret: NodeList<ClassOrInterfaceType> = NodeList<ClassOrInterfaceType>::new();
		let cit: ClassOrInterfaceType;
		self.jj_consume_token()?;
		cit = self.annotated_class_or_interface_type()?;
		ret.add(cit);
		'label_5: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[17] = self.jj_gen;
					break 'label_5;
				}
			}
			self.jj_consume_token()?;
			cit = self.annotated_class_or_interface_type()?;
			ret.add(cit);
			extends_more_than_one = true;
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn implements_list(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::node_list::NodeList {
		let ret: NodeList<ClassOrInterfaceType> = NodeList<ClassOrInterfaceType>::new();
		let cit: ClassOrInterfaceType;
		self.jj_consume_token()?;
		cit = self.annotated_class_or_interface_type()?;
		ret.add(cit);
		'label_6: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[18] = self.jj_gen;
					break 'label_6;
				}
			}
			self.jj_consume_token()?;
			cit = self.annotated_class_or_interface_type()?;
			ret.add(cit);
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn permits_list(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::node_list::NodeList {
		let ret: NodeList<ClassOrInterfaceType> = NodeList<ClassOrInterfaceType>::new();
		let cit: ClassOrInterfaceType;
		self.jj_consume_token()?;
		cit = self.annotated_class_or_interface_type()?;
		ret.add(cit);
		'label_7: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[19] = self.jj_gen;
					break 'label_7;
				}
			}
			self.jj_consume_token()?;
			cit = self.annotated_class_or_interface_type()?;
			ret.add(cit);
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn enum_declaration(&mut self, modifier: &com::github::javaparser::modifier_holder::ModifierHolder) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::enum_declaration::EnumDeclaration {
		let name: SimpleName;
		let imp_list: NodeList<ClassOrInterfaceType> = self.empty_node_list();
		let entry: EnumConstantDeclaration;
		let entries: NodeList<EnumConstantDeclaration> = self.empty_node_list();
		let member: BodyDeclaration<?>;
		let members: NodeList<BodyDeclaration<?>> = self.empty_node_list();
		let begin: JavaToken = modifier.begin;
		self.jj_consume_token()?;
		begin = self.or_if_invalid(begin, &self.token());
		name = self.simple_name()?;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					imp_list = self.implements_list()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[20] = self.jj_gen;
				;
			}
		}
		self.jj_consume_token()?;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
				{
					entry = self.enum_constant_declaration()?;
					entries.add(entry);
					'label_8: while true {
						if self.jj_2_6(2) {
						} else {
							break 'label_8;
						}
						self.jj_consume_token()?;
						entry = self.enum_constant_declaration()?;
						entries.add(entry);
					}
					break;
				}
			}
			_ =>  {
				self.jj_la1[21] = self.jj_gen;
				;
			}
		}
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					self.jj_consume_token()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[22] = self.jj_gen;
				;
			}
		}
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					self.jj_consume_token()?;
					'label_9: while true {
						match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
								{
									break;
								}
							}
							_ =>  {
								self.jj_la1[23] = self.jj_gen;
								break 'label_9;
							}
						}
						match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
								{
									member = self.class_or_interface_body_declaration()?;
									members = self.add(members, member);
									break;
								}
							}
							 =>  {
								{
									self.jj_consume_token()?;
									break;
								}
							}
							_ =>  {
								self.jj_la1[24] = self.jj_gen;
								self.jj_consume_token(-1)?;
								return Err(ParseException::new());
							}
						}
					}
					break;
				}
			}
			_ =>  {
				self.jj_la1[25] = self.jj_gen;
				;
			}
		}
		self.jj_consume_token()?;
		{
			if "" != null {
				return EnumDeclaration::new(&self.range(begin, &self.token()), modifier.modifiers, modifier.annotations, name, imp_list, entries, members);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn enum_constant_declaration(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::enum_constant_declaration::EnumConstantDeclaration {
		let annotations: NodeList<AnnotationExpr> = NodeList<AnnotationExpr>::new();
		let ann: AnnotationExpr;
		let name: SimpleName;
		let args: NodeList<Expression> = self.empty_node_list();
		let class_body: NodeList<BodyDeclaration<?>> = self.empty_node_list();
		let begin: JavaToken = ;
		'label_10: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[26] = self.jj_gen;
					break 'label_10;
				}
			}
			ann = self.annotation()?;
			annotations = self.add(annotations, ann);
			begin = self.or_if_invalid(begin, ann);
		}
		name = self.simple_name()?;
		begin = self.or_if_invalid(begin, &self.token());
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					args = self.arguments()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[27] = self.jj_gen;
				;
			}
		}
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					class_body = self.class_or_interface_body()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[28] = self.jj_gen;
				;
			}
		}
		{
			if "" != null {
				return EnumConstantDeclaration::new(&self.range(begin, &self.token()), annotations, name, args, class_body);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn type_parameters(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ranged_list::RangedList {
		let ret: RangedList<TypeParameter> = RangedList<TypeParameter>::new(NodeList<TypeParameter>::new());
		let tp: TypeParameter;
		let annotations: NodeList<AnnotationExpr>;
		self.jj_consume_token()?;
		ret.begin_at(&self.token());
		annotations = self.annotations()?;
		tp = self.type_parameter(annotations)?;
		ret.add(tp);
		annotations = null;
		'label_11: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[29] = self.jj_gen;
					break 'label_11;
				}
			}
			self.jj_consume_token()?;
			annotations = self.annotations()?;
			tp = self.type_parameter(annotations)?;
			ret.add(tp);
			annotations = null;
		}
		self.jj_consume_token()?;
		ret.end_at(&self.token());
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn type_parameter(&mut self, annotations: &com::github::javaparser::ast::node_list::NodeList) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::type::type_parameter::TypeParameter {
		let name: SimpleName;
		let type_bound: NodeList<ClassOrInterfaceType> = self.empty_node_list();
		let begin: JavaToken;
		// Annotations are passed as a parameter to this grammar entry.
		name = self.simple_name()?;
		begin = self.token();
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					type_bound = self.type_bound()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[30] = self.jj_gen;
				;
			}
		}
		{
			if "" != null {
				return TypeParameter::new(&self.range(begin, &self.token()), name, type_bound, annotations);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn type_bound(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::node_list::NodeList {
		let ret: NodeList<ClassOrInterfaceType> = self.empty_node_list();
		let cit: ClassOrInterfaceType;
		self.jj_consume_token()?;
		cit = self.annotated_class_or_interface_type()?;
		ret.add(cit);
		'label_12: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[31] = self.jj_gen;
					break 'label_12;
				}
			}
			self.jj_consume_token()?;
			cit = self.annotated_class_or_interface_type()?;
			ret.add(cit);
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn class_or_interface_body(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::node_list::NodeList {
		let ret: NodeList<BodyDeclaration<?>> = self.empty_node_list();
		let member: BodyDeclaration;
		self.jj_consume_token()?;
		'label_13: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[32] = self.jj_gen;
					break 'label_13;
				}
			}
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
					{
						member = self.class_or_interface_body_declaration()?;
						ret.add(member);
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						break;
					}
				}
				_ =>  {
					self.jj_la1[33] = self.jj_gen;
					self.jj_consume_token(-1)?;
					return Err(ParseException::new());
				}
			}
		}
		self.jj_consume_token()?;
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn record_body(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::node_list::NodeList {
		let ret: NodeList<BodyDeclaration<?>> = self.empty_node_list();
		let member: BodyDeclaration;
		self.jj_consume_token()?;
		'label_14: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[34] = self.jj_gen;
					break 'label_14;
				}
			}
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
					{
						member = self.record_body_declaration()?;
						ret.add(member);
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						break;
					}
				}
				_ =>  {
					self.jj_la1[35] = self.jj_gen;
					self.jj_consume_token(-1)?;
					return Err(ParseException::new());
				}
			}
		}
		self.jj_consume_token()?;
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn record_body_declaration(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::body_declaration::BodyDeclaration {
		let modifiers: ModifierHolder;
		let ret: BodyDeclaration<?>;
		if self.jj_2_13(2) {
			ret = self.initializer_declaration()?;
		} else {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
					{
						// Just get all the modifiers out of the way. If you want to do
						// more checks, pass the modifiers down to the member
						modifiers = self.modifiers()?;
						match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
							 =>  {
							}
							 =>  {
								{
									ret = self.class_or_interface_declaration(modifiers)?;
									break;
								}
							}
							_ =>  {
								self.jj_la1[36] = self.jj_gen;
								if self.jj_2_7(2147483647) {
									ret = self.enum_declaration(modifiers)?;
								} else if self.jj_2_8(2147483647) {
									ret = self.record_declaration(modifiers)?;
								} else if self.jj_2_9(2147483647) {
									ret = self.annotation_type_declaration(modifiers)?;
								} else if self.jj_2_10(2147483647) {
									ret = self.compact_constructor_declaration(modifiers)?;
								} else if self.jj_2_11(2147483647) {
									ret = self.constructor_declaration(modifiers)?;
								} else if self.jj_2_12(2147483647) {
									ret = self.field_declaration(modifiers)?;
								} else {
									match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
											{
												ret = self.method_declaration(modifiers)?;
												break;
											}
										}
										_ =>  {
											self.jj_la1[37] = self.jj_gen;
											self.jj_consume_token(-1)?;
											return Err(ParseException::new());
										}
									}
								}
							}
						}
						break;
					}
				}
				_ =>  {
					self.jj_la1[38] = self.jj_gen;
					self.jj_consume_token(-1)?;
					return Err(ParseException::new());
				}
			}
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn compact_constructor_declaration(&mut self, modifier: &com::github::javaparser::modifier_holder::ModifierHolder) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::compact_constructor_declaration::CompactConstructorDeclaration {
		let type_parameters: RangedList<TypeParameter> = RangedList<TypeParameter>::new(&self.empty_node_list());
		let name: SimpleName;
		let parameters: Pair<NodeList<Parameter>, ReceiverParameter> = Pair<NodeList<Parameter>, ReceiverParameter>::new(&self.empty_node_list(), null);
		let throws_: NodeList<ReferenceType> = self.empty_node_list();
		let stmts: NodeList<Statement> = self.empty_node_list();
		let begin: JavaToken = modifier.begin;
		let block_begin: JavaToken = ;
		let throw_type: ReferenceType;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					type_parameters = self.type_parameters()?;
					begin = self.or_if_invalid(begin, &type_parameters.range.get_begin());
					break;
				}
			}
			_ =>  {
				self.jj_la1[39] = self.jj_gen;
				;
			}
		}
		// Modifiers matched in the caller
		name = self.simple_name()?;
		begin = self.or_if_invalid(begin, &type_parameters.range.get_begin());
		begin = self.or_if_invalid(begin, &self.token());
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					self.jj_consume_token()?;
					throw_type = self.annotated_reference_type()?;
					throws_ = self.add(throws_, throw_type);
					'label_15: while true {
						match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
							 =>  {
								{
									break;
								}
							}
							_ =>  {
								self.jj_la1[40] = self.jj_gen;
								break 'label_15;
							}
						}
						self.jj_consume_token()?;
						throw_type = self.annotated_reference_type()?;
						throws_ = self.add(throws_, throw_type);
					}
					break;
				}
			}
			_ =>  {
				self.jj_la1[41] = self.jj_gen;
				;
			}
		}
		self.jj_consume_token()?;
		block_begin = self.token();
		stmts = self.statements()?;
		self.jj_consume_token()?;
		{
			if "" != null {
				return CompactConstructorDeclaration::new(&self.range(begin, &self.token()), modifier.modifiers, modifier.annotations, type_parameters.list, name, throws_, BlockStmt::new(&self.range(block_begin, &self.token()), stmts));
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn class_or_interface_body_declaration(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::body_declaration::BodyDeclaration {
		let modifiers: ModifierHolder;
		let ret: BodyDeclaration<?>;
		if self.jj_2_19(2) {
			ret = self.initializer_declaration()?;
		} else {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
					{
						// Just get all the modifiers out of the way. If you want to do
						// more checks, pass the modifiers down to the member
						modifiers = self.modifiers()?;
						match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
							 =>  {
							}
							 =>  {
								{
									ret = self.class_or_interface_declaration(modifiers)?;
									break;
								}
							}
							_ =>  {
								self.jj_la1[42] = self.jj_gen;
								if self.jj_2_14(2147483647) {
									ret = self.record_declaration(modifiers)?;
								} else if self.jj_2_15(2147483647) {
									ret = self.enum_declaration(modifiers)?;
								} else if self.jj_2_16(2147483647) {
									ret = self.annotation_type_declaration(modifiers)?;
								} else if self.jj_2_17(2147483647) {
									ret = self.constructor_declaration(modifiers)?;
								} else if self.jj_2_18(2147483647) {
									ret = self.field_declaration(modifiers)?;
								} else {
									match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
											{
												ret = self.method_declaration(modifiers)?;
												break;
											}
										}
										_ =>  {
											self.jj_la1[43] = self.jj_gen;
											self.jj_consume_token(-1)?;
											return Err(ParseException::new());
										}
									}
								}
							}
						}
						break;
					}
				}
				_ =>  {
					self.jj_la1[44] = self.jj_gen;
					self.jj_consume_token(-1)?;
					return Err(ParseException::new());
				}
			}
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn field_declaration(&mut self, modifier: &com::github::javaparser::modifier_holder::ModifierHolder) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::field_declaration::FieldDeclaration {
		let partial_type: Type;
		let variables: NodeList<VariableDeclarator> = NodeList<VariableDeclarator>::new();
		let val: VariableDeclarator;
		// Modifiers are already matched in the caller
		partial_type = self.type(&self.empty_node_list())?;
		val = self.variable_declarator(partial_type)?;
		variables.add(val);
		'label_16: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[45] = self.jj_gen;
					break 'label_16;
				}
			}
			self.jj_consume_token()?;
			val = self.variable_declarator(partial_type)?;
			variables.add(val);
		}
		self.jj_consume_token()?;
		let begin: JavaToken = self.or_if_invalid(modifier.begin, partial_type);
		{
			if "" != null {
				return FieldDeclaration::new(&self.range(begin, &self.token()), modifier.modifiers, modifier.annotations, variables);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn variable_declarator(&mut self, partial_type: &com::github::javaparser::ast::type::type::Type) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::variable_declarator::VariableDeclarator {
		let id: Pair<SimpleName, List<ArrayBracketPair>>;
		let init: Expression = null;
		id = self.variable_declarator_id()?;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					self.jj_consume_token()?;
					init = self.variable_initializer()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[46] = self.jj_gen;
				;
			}
		}
		{
			if "" != null {
				return VariableDeclarator::new(&self.range(id.a, &self.token()), &self.juggle_array_type(partial_type, id.b), id.a, init);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn variable_declarator_id(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.AssertionError | java.lang.IllegalStateException) */ -> com::github::javaparser::utils::pair::Pair {
		let name: SimpleName;
		let begin: JavaToken;
		let array_bracket_pair: ArrayBracketPair;
		let array_bracket_pairs: List<ArrayBracketPair> = ArrayList<ArrayBracketPair>::new(0);
		name = self.simple_name()?;
		begin = self.token();
		'label_17: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[47] = self.jj_gen;
					break 'label_17;
				}
			}
			array_bracket_pair = self.array_bracket_pair(Origin.NAME)?;
			array_bracket_pairs = self.add(array_bracket_pairs, array_bracket_pair);
		}
		if  {
			name.set_token_range(&name.get_token_range().get().with_end(&self.token())?);
		}
		{
			if "" != null {
				return Pair::new(name, array_bracket_pairs);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn variable_initializer(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					ret = self.array_initializer()?;
					break;
				}
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
				{
					ret = self.expression()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[48] = self.jj_gen;
				self.jj_consume_token(-1)?;
				return Err(ParseException::new());
			}
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn array_initializer(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr {
		let values: NodeList<Expression> = self.empty_node_list();
		let val: Expression;
		let begin: JavaToken;
		self.jj_consume_token()?;
		begin = self.token();
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
				{
					val = self.variable_initializer()?;
					values = self.add(values, val);
					'label_18: while true {
						if self.jj_2_20(2) {
						} else {
							break 'label_18;
						}
						self.jj_consume_token()?;
						val = self.variable_initializer()?;
						values = self.add(values, val);
					}
					break;
				}
			}
			_ =>  {
				self.jj_la1[49] = self.jj_gen;
				;
			}
		}
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					self.jj_consume_token()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[50] = self.jj_gen;
				;
			}
		}
		self.jj_consume_token()?;
		{
			if "" != null {
				return ArrayInitializerExpr::new(&self.range(begin, &self.token()), values);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn method_declaration(&mut self, modifier: &com::github::javaparser::modifier_holder::ModifierHolder) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::method_declaration::MethodDeclaration {
		let type_parameters: RangedList<TypeParameter> = RangedList<TypeParameter>::new(&self.empty_node_list());
		let type: Type;
		let name: SimpleName;
		let parameters: Pair<NodeList<Parameter>, ReceiverParameter> = Pair<NodeList<Parameter>, ReceiverParameter>::new(&self.empty_node_list(), null);
		let array_bracket_pair: ArrayBracketPair;
		let array_bracket_pairs: List<ArrayBracketPair> = ArrayList<ArrayBracketPair>::new(0);
		let throws_: NodeList<ReferenceType> = self.empty_node_list();
		let body: BlockStmt = null;
		let annotations: NodeList<AnnotationExpr>;
		let begin: JavaToken = modifier.begin;
		let throw_type: ReferenceType;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					type_parameters = self.type_parameters()?;
					begin = self.or_if_invalid(begin, &type_parameters.range.get_begin());
					break;
				}
			}
			_ =>  {
				self.jj_la1[51] = self.jj_gen;
				;
			}
		}
		annotations = self.annotations()?;
		modifier.annotations.add_all(annotations);
		begin = self.or_if_invalid(begin, &self.node_list_begin(annotations));
		type = self.result_type(&self.empty_node_list())?;
		begin = self.or_if_invalid(begin, type);
		name = self.simple_name()?;
		parameters = self.parameters()?;
		'label_19: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[52] = self.jj_gen;
					break 'label_19;
				}
			}
			array_bracket_pair = self.array_bracket_pair(Origin.NAME)?;
			array_bracket_pairs = self.add(array_bracket_pairs, array_bracket_pair);
		}
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					self.jj_consume_token()?;
					throw_type = self.annotated_reference_type()?;
					throws_ = self.add(throws_, throw_type);
					'label_20: while true {
						match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
							 =>  {
								{
									break;
								}
							}
							_ =>  {
								self.jj_la1[53] = self.jj_gen;
								break 'label_20;
							}
						}
						self.jj_consume_token()?;
						throw_type = self.annotated_reference_type()?;
						throws_ = self.add(throws_, throw_type);
					}
					break;
				}
			}
			_ =>  {
				self.jj_la1[54] = self.jj_gen;
				;
			}
		}
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					body = self.block()?;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[55] = self.jj_gen;
				self.jj_consume_token(-1)?;
				return Err(ParseException::new());
			}
		}
		type = self.juggle_array_type(type, array_bracket_pairs);
		{
			if "" != null {
				return MethodDeclaration::new(&self.range(begin, &self.token()), modifier.modifiers, modifier.annotations, type_parameters.list, type, name, parameters.a, throws_, body, parameters.b);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn annotated_reference_type(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::type::reference_type::ReferenceType {
		let annotations: NodeList<AnnotationExpr>;
		let type: ReferenceType;
		annotations = self.annotations()?;
		type = self.reference_type(annotations)?;
		{
			if "" != null {
				return type;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn annotated_type(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::type::type::Type {
		let annotations: NodeList<AnnotationExpr>;
		let type: Type;
		annotations = self.annotations()?;
		type = self.type(annotations)?;
		{
			if "" != null {
				return type;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn parameters(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::utils::pair::Pair {
		let ret: NodeList<Parameter> = self.empty_node_list();
		let par: Parameter;
		let rp: ReceiverParameter = null;
		self.jj_consume_token()?;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
				{
					if self.jj_2_21(2147483647) {
						rp = self.receiver_parameter()?;
					} else {
						match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
								{
									par = self.parameter()?;
									ret = self.add(ret, par);
									break;
								}
							}
							_ =>  {
								self.jj_la1[56] = self.jj_gen;
								self.jj_consume_token(-1)?;
								return Err(ParseException::new());
							}
						}
					}
					'label_21: while true {
						match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
							 =>  {
								{
									break;
								}
							}
							_ =>  {
								self.jj_la1[57] = self.jj_gen;
								break 'label_21;
							}
						}
						self.jj_consume_token()?;
						par = self.parameter()?;
						ret = self.add(ret, par);
					}
					break;
				}
			}
			_ =>  {
				self.jj_la1[58] = self.jj_gen;
				;
			}
		}
		self.jj_consume_token()?;
		{
			if "" != null {
				return Pair::new(ret, rp);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn lambda_parameters(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::node_list::NodeList {
		let ret: NodeList<Parameter> = null;
		let par: Parameter;
		par = self.parameter()?;
		ret = self.add(ret, par);
		'label_22: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[59] = self.jj_gen;
					break 'label_22;
				}
			}
			self.jj_consume_token()?;
			par = self.parameter()?;
			ret = self.add(ret, par);
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn inferred_lambda_parameters(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.AssertionError | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::node_list::NodeList {
		let ret: NodeList<Parameter> = null;
		let id: Pair<SimpleName, List<ArrayBracketPair>>;
		id = self.variable_declarator_id()?;
		ret = self.add(ret, Parameter::new(&self.range(id.a, id.a), NodeList<Modifier>::new(), &self.empty_node_list(), UnknownType::new(), false, &self.empty_node_list(), id.a));
		'label_23: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[60] = self.jj_gen;
					break 'label_23;
				}
			}
			self.jj_consume_token()?;
			id = self.variable_declarator_id()?;
			ret = self.add(ret, Parameter::new(&self.range(id.a, id.a), NodeList<Modifier>::new(), &self.empty_node_list(), UnknownType::new(), false, &self.empty_node_list(), id.a));
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn parameter(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.AssertionError | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::parameter::Parameter {
		let modifier: ModifierHolder;
		let partial_type: Type;
		let is_var_arg: bool = false;
		let id: Pair<SimpleName, List<ArrayBracketPair>>;
		let var_arg_annotations: NodeList<AnnotationExpr> = self.empty_node_list();
		modifier = self.modifiers()?;
		partial_type = self.type(&self.empty_node_list())?;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
			}
			 =>  {
				{
					var_arg_annotations = self.annotations()?;
					self.jj_consume_token()?;
					is_var_arg = true;
					break;
				}
			}
			_ =>  {
				self.jj_la1[61] = self.jj_gen;
				;
			}
		}
		id = self.variable_declarator_id()?;
		let begin: JavaToken = self.or_if_invalid(modifier.begin, partial_type);
		{
			if "" != null {
				return Parameter::new(&self.range(begin, &self.token()), modifier.modifiers, modifier.annotations, &self.juggle_array_type(partial_type, id.b), is_var_arg, var_arg_annotations, id.a);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn receiver_parameter(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter {
		let partial_type: Type;
		let id: Name;
		let annotations: NodeList<AnnotationExpr> = self.empty_node_list();
		annotations = self.annotations()?;
		partial_type = self.type(&self.empty_node_list())?;
		id = self.receiver_parameter_id()?;
		{
			if "" != null {
				return ReceiverParameter::new(&self.range(partial_type, &self.token()), annotations, partial_type, id);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn receiver_parameter_id(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::name::Name {
		let ret: Name = null;
		if self.jj_2_22(2147483647) {
			ret = self.name()?;
			self.jj_consume_token()?;
		} else {
			;
		}
		self.jj_consume_token()?;
		{
			if "" != null {
				return Name::new(&self.token_range(), ret, self.token.image);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn constructor_declaration(&mut self, modifier: &com::github::javaparser::modifier_holder::ModifierHolder) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration {
		let type_parameters: RangedList<TypeParameter> = RangedList<TypeParameter>::new(&self.empty_node_list());
		let name: SimpleName;
		let parameters: Pair<NodeList<Parameter>, ReceiverParameter> = Pair<NodeList<Parameter>, ReceiverParameter>::new(&self.empty_node_list(), null);
		let throws_: NodeList<ReferenceType> = self.empty_node_list();
		let stmts: NodeList<Statement> = self.empty_node_list();
		let begin: JavaToken = modifier.begin;
		let block_begin: JavaToken = ;
		let throw_type: ReferenceType;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					type_parameters = self.type_parameters()?;
					begin = self.or_if_invalid(begin, &type_parameters.range.get_begin());
					break;
				}
			}
			_ =>  {
				self.jj_la1[62] = self.jj_gen;
				;
			}
		}
		// Modifiers matched in the caller
		name = self.simple_name()?;
		begin = self.or_if_invalid(begin, &type_parameters.range.get_begin());
		begin = self.or_if_invalid(begin, &self.token());
		parameters = self.parameters()?;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					self.jj_consume_token()?;
					throw_type = self.annotated_reference_type()?;
					throws_ = self.add(throws_, throw_type);
					'label_24: while true {
						match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
							 =>  {
								{
									break;
								}
							}
							_ =>  {
								self.jj_la1[63] = self.jj_gen;
								break 'label_24;
							}
						}
						self.jj_consume_token()?;
						throw_type = self.annotated_reference_type()?;
						throws_ = self.add(throws_, throw_type);
					}
					break;
				}
			}
			_ =>  {
				self.jj_la1[64] = self.jj_gen;
				;
			}
		}
		self.jj_consume_token()?;
		block_begin = self.token();
		stmts = self.statements()?;
		self.jj_consume_token()?;
		{
			if "" != null {
				return ConstructorDeclaration::new(&self.range(begin, &self.token()), modifier.modifiers, modifier.annotations, type_parameters.list, name, parameters.a, throws_, BlockStmt::new(&self.range(block_begin, &self.token()), stmts), parameters.b);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn explicit_constructor_invocation(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::explicit_constructor_invocation_stmt::ExplicitConstructorInvocationStmt {
		let is_this: bool = false;
		let args: NodeList<Expression>;
		let expr: Expression = null;
		let type_args: RangedList<Type> = RangedList<Type>::new(null);
		let begin: JavaToken = ;
		if self.jj_2_24(2147483647) {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						type_args = self.type_arguments()?;
						begin = type_args.range.get_begin();
						break;
					}
				}
				_ =>  {
					self.jj_la1[65] = self.jj_gen;
					;
				}
			}
			self.jj_consume_token()?;
			begin = self.or_if_invalid(begin, &self.token());
			is_this = true;
			args = self.arguments()?;
			self.jj_consume_token()?;
		} else {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
					{
						if self.jj_2_23(2147483647) {
							expr = self.primary_expression_without_super_suffix()?;
							self.jj_consume_token()?;
							begin = self.or_if_invalid(begin, expr);
						} else {
							;
						}
						match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
							 =>  {
								{
									type_args = self.type_arguments()?;
									begin = self.or_if_invalid(begin, &type_args.range.get_begin());
									break;
								}
							}
							_ =>  {
								self.jj_la1[66] = self.jj_gen;
								;
							}
						}
						self.jj_consume_token()?;
						begin = self.or_if_invalid(begin, &self.token());
						args = self.arguments()?;
						self.jj_consume_token()?;
						break;
					}
				}
				_ =>  {
					self.jj_la1[67] = self.jj_gen;
					self.jj_consume_token(-1)?;
					return Err(ParseException::new());
				}
			}
		}
		{
			if "" != null {
				return ExplicitConstructorInvocationStmt::new(&self.range(begin, &self.token()), type_args.list, is_this, expr, args);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn statements(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::node_list::NodeList {
		let ret: NodeList<Statement> = self.empty_node_list();
		let stmt: Statement;
		'label_25: while true {
			if self.jj_2_25(2) {
			} else {
				break 'label_25;
			}
			stmt = self.block_statement()?;
			ret = self.add(ret, stmt);
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn initializer_declaration(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::initializer_declaration::InitializerDeclaration {
		let body: BlockStmt;
		let begin: JavaToken = ;
		let is_static: bool = false;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					self.jj_consume_token()?;
					is_static = true;
					begin = self.token();
					break;
				}
			}
			_ =>  {
				self.jj_la1[68] = self.jj_gen;
				;
			}
		}
		body = self.block()?;
		begin = self.or_if_invalid(begin, body);
		{
			if "" != null {
				return InitializerDeclaration::new(&self.range(begin, &self.token()), is_static, body);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn type(&mut self, annotations: &com::github::javaparser::ast::node_list::NodeList) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::type::type::Type {
		let ret: Type;
		if self.jj_2_26(2147483647) {
			ret = self.reference_type(annotations)?;
		} else {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
					{
						ret = self.primitive_type(annotations)?;
						break;
					}
				}
				_ =>  {
					self.jj_la1[69] = self.jj_gen;
					self.jj_consume_token(-1)?;
					return Err(ParseException::new());
				}
			}
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn reference_type(&mut self, annotations: &com::github::javaparser::ast::node_list::NodeList) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::type::reference_type::ReferenceType {
		let type: Type;
		let array_bracket_pair: ArrayBracketPair;
		let array_bracket_pairs: List<ArrayBracketPair> = ArrayList<ArrayBracketPair>::new(0);
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
				{
					// Note that an array of primitives is considered a reference type.
					type = self.primitive_type(annotations)?;
					'label_26: while true {
						array_bracket_pair = self.array_bracket_pair(Origin.TYPE)?;
						array_bracket_pairs = self.add(array_bracket_pairs, array_bracket_pair);
						if self.jj_2_27(2147483647) {
						} else {
							break 'label_26;
						}
					}
					break;
				}
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
				{
					type = self.class_or_interface_type(annotations)?;
					'label_27: while true {
						if self.jj_2_28(2147483647) {
						} else {
							break 'label_27;
						}
						array_bracket_pair = self.array_bracket_pair(Origin.TYPE)?;
						array_bracket_pairs = self.add(array_bracket_pairs, array_bracket_pair);
					}
					break;
				}
			}
			_ =>  {
				self.jj_la1[70] = self.jj_gen;
				self.jj_consume_token(-1)?;
				return Err(ParseException::new());
			}
		}
		{
			if "" != null {
				return com::github::javaparser::ast::type::array_type::ArrayType::wrap_in_array_types(type, array_bracket_pairs) as ReferenceType;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn array_bracket_pair(&self, origin: &com::github::javaparser::ast::type::array_type::Origin) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::type::array_type::ArrayBracketPair {
		let annotations: NodeList<AnnotationExpr>;
		let begin: JavaToken = ;
		annotations = self.annotations()?;
		self.jj_consume_token()?;
		begin = self.or_if_invalid(begin, &self.token());
		self.jj_consume_token()?;
		{
			if "" != null {
				return ArrayBracketPair::new(&self.range(begin, &self.token()), origin, annotations);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn intersection_type(&mut self, annotations: &com::github::javaparser::ast::node_list::NodeList) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::type::intersection_type::IntersectionType {
		let begin: JavaToken = ;
		let element_type: ReferenceType;
		let elements: NodeList<ReferenceType> = self.empty_node_list();
		element_type = self.reference_type(annotations)?;
		begin = self.or_if_invalid(begin, element_type);
		elements = self.add(elements, element_type);
		self.jj_consume_token()?;
		'label_28: while true {
			element_type = self.annotated_reference_type()?;
			elements = self.add(elements, element_type);
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[71] = self.jj_gen;
					break 'label_28;
				}
			}
		}
		{
			if "" != null {
				return IntersectionType::new(&self.range(begin, &self.token()), elements);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn annotated_class_or_interface_type(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType {
		let annotations: NodeList<AnnotationExpr> = NodeList<AnnotationExpr>::new();
		let cit: ClassOrInterfaceType;
		annotations = self.annotations()?;
		cit = self.class_or_interface_type(annotations)?;
		{
			if "" != null {
				return cit;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn class_or_interface_type(&self, first_annotations: &com::github::javaparser::ast::node_list::NodeList) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType {
		let ret: ClassOrInterfaceType;
		let name: SimpleName;
		let type_args: RangedList<Type> = RangedList<Type>::new(null);
		let begin: JavaToken;
		let annotations: NodeList<AnnotationExpr> = NodeList<AnnotationExpr>::new();
		name = self.simple_name()?;
		begin = self.token();
		if self.jj_2_29(2) {
			type_args = self.type_arguments()?;
		} else {
			;
		}
		ret = ClassOrInterfaceType::new(&self.range(begin, &self.token()), null, name, type_args.list, first_annotations);
		type_args = RangedList<Type>::new(null);
		'label_29: while true {
			if self.jj_2_30(2) {
			} else {
				break 'label_29;
			}
			self.jj_consume_token()?;
			annotations = self.annotations()?;
			name = self.simple_name()?;
			if self.jj_2_31(2) {
				type_args = self.type_arguments()?;
			} else {
				;
			}
			ret = ClassOrInterfaceType::new(&self.range(begin, &self.token()), ret, name, type_args.list, annotations);
			type_args = RangedList<Type>::new(null);
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn type_arguments(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ranged_list::RangedList {
		let ret: RangedList<Type> = RangedList<Type>::new(NodeList<Type>::new());
		let type: Type;
		self.jj_consume_token()?;
		ret.begin_at(&self.token());
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
				{
					type = self.type_argument()?;
					ret.add(type);
					'label_30: while true {
						match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
							 =>  {
								{
									break;
								}
							}
							_ =>  {
								self.jj_la1[72] = self.jj_gen;
								break 'label_30;
							}
						}
						self.jj_consume_token()?;
						type = self.type_argument()?;
						ret.add(type);
					}
					break;
				}
			}
			_ =>  {
				self.jj_la1[73] = self.jj_gen;
				;
			}
		}
		self.jj_consume_token()?;
		ret.end_at(&self.token());
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn type_argument(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::type::type::Type {
		let ret: Type;
		let annotations: NodeList<AnnotationExpr>;
		annotations = self.annotations()?;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
				{
					ret = self.type(annotations)?;
					break;
				}
			}
			 =>  {
				{
					ret = self.wildcard(annotations)?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[74] = self.jj_gen;
				self.jj_consume_token(-1)?;
				return Err(ParseException::new());
			}
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn wildcard(&mut self, first_annotations: &com::github::javaparser::ast::node_list::NodeList) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::type::wildcard_type::WildcardType {
		let ext: ReferenceType = null;
		let sup: ReferenceType = null;
		let begin: JavaToken;
		let annotations: NodeList<AnnotationExpr> = NodeList<AnnotationExpr>::new();
		self.jj_consume_token()?;
		begin = self.token();
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
			}
			 =>  {
				{
					match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
						 =>  {
							{
								self.jj_consume_token()?;
								annotations = self.annotations()?;
								ext = self.reference_type(annotations)?;
								break;
							}
						}
						 =>  {
							{
								self.jj_consume_token()?;
								annotations = self.annotations()?;
								sup = self.reference_type(annotations)?;
								break;
							}
						}
						_ =>  {
							self.jj_la1[75] = self.jj_gen;
							self.jj_consume_token(-1)?;
							return Err(ParseException::new());
						}
					}
					break;
				}
			}
			_ =>  {
				self.jj_la1[76] = self.jj_gen;
				;
			}
		}
		{
			if "" != null {
				return WildcardType::new(&self.range(begin, &self.token()), ext, sup, first_annotations);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn primitive_type(&mut self, annotations: &com::github::javaparser::ast::node_list::NodeList) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::type::primitive_type::PrimitiveType {
		let ret: PrimitiveType;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					self.jj_consume_token()?;
					ret = PrimitiveType::new(&self.token_range(), PrimitiveType.Primitive::BOOLEAN, annotations);
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					ret = PrimitiveType::new(&self.token_range(), PrimitiveType.Primitive::CHAR, annotations);
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					ret = PrimitiveType::new(&self.token_range(), PrimitiveType.Primitive::BYTE, annotations);
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					ret = PrimitiveType::new(&self.token_range(), PrimitiveType.Primitive::SHORT, annotations);
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					ret = PrimitiveType::new(&self.token_range(), PrimitiveType.Primitive::INT, annotations);
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					ret = PrimitiveType::new(&self.token_range(), PrimitiveType.Primitive::LONG, annotations);
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					ret = PrimitiveType::new(&self.token_range(), PrimitiveType.Primitive::FLOAT, annotations);
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					ret = PrimitiveType::new(&self.token_range(), PrimitiveType.Primitive::DOUBLE, annotations);
					break;
				}
			}
			_ =>  {
				self.jj_la1[77] = self.jj_gen;
				self.jj_consume_token(-1)?;
				return Err(ParseException::new());
			}
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn result_type(&mut self, annotations: &com::github::javaparser::ast::node_list::NodeList) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::type::type::Type {
		let ret: Type;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					self.jj_consume_token()?;
					ret = VoidType::new(&self.token_range());
					break;
				}
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
				{
					ret = self.type(annotations)?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[78] = self.jj_gen;
				self.jj_consume_token(-1)?;
				return Err(ParseException::new());
			}
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn name(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::name::Name {
		let ret: Name;
		self.identifier()?;
		ret = Name::new(&self.token_range(), null, self.token.image);
		'label_31: while true {
			if self.jj_2_32(2) {
			} else {
				break 'label_31;
			}
			self.jj_consume_token()?;
			self.identifier()?;
			ret = Name::new(&self.range(ret, &self.token()), ret, self.token.image);
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn simple_name(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::simple_name::SimpleName {
		let ret: SimpleName;
		self.identifier()?;
		ret = SimpleName::new(&self.token_range(), self.token.image);
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn identifier(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> /* Java */ java::lang::String /**/ {
		let ret: String;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					self.jj_consume_token()?;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[79] = self.jj_gen;
				self.jj_consume_token(-1)?;
				return Err(ParseException::new());
			}
		}
		ret = self.token.image;
		self.set_token_kind();
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn expression(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		let op: AssignExpr.Operator;
		let value: Expression;
		let lambda_body: Statement = null;
		let type_args: RangedList<Type> = RangedList<Type>::new(null);
		ret = self.conditional_expression()?;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
				{
					if self.jj_2_33(2) {
						op = self.assignment_operator()?;
						value = self.expression()?;
						ret = AssignExpr::new(&self.range(ret, &self.token()), ret, value, op);
					} else {
						match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
							 =>  {
								{
									self.jj_consume_token()?;
									lambda_body = self.lambda_body()?;
									ret = self.generate_lambda(ret, lambda_body);
									break;
								}
							}
							 =>  {
								{
									self.jj_consume_token()?;
									match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
										 =>  {
											{
												type_args = self.type_arguments()?;
												break;
											}
										}
										_ =>  {
											self.jj_la1[80] = self.jj_gen;
											;
										}
									}
									match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
											{
												self.identifier()?;
												break;
											}
										}
										 =>  {
											{
												self.jj_consume_token()?;
												break;
											}
										}
										_ =>  {
											self.jj_la1[81] = self.jj_gen;
											self.jj_consume_token(-1)?;
											return Err(ParseException::new());
										}
									}
									ret = MethodReferenceExpr::new(&self.range(ret, &self.token()), ret, type_args.list, self.token.image);
									break;
								}
							}
							_ =>  {
								self.jj_la1[82] = self.jj_gen;
								self.jj_consume_token(-1)?;
								return Err(ParseException::new());
							}
						}
					}
					break;
				}
			}
			_ =>  {
				self.jj_la1[83] = self.jj_gen;
				;
			}
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn assignment_operator(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::assign_expr::Operator {
		let ret: AssignExpr.Operator;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					self.jj_consume_token()?;
					ret = AssignExpr.Operator::ASSIGN;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					ret = AssignExpr.Operator::MULTIPLY;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					ret = AssignExpr.Operator::DIVIDE;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					ret = AssignExpr.Operator::REMAINDER;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					ret = AssignExpr.Operator::PLUS;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					ret = AssignExpr.Operator::MINUS;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					ret = AssignExpr.Operator::LEFT_SHIFT;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					ret = AssignExpr.Operator::SIGNED_RIGHT_SHIFT;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					ret = AssignExpr.Operator::UNSIGNED_RIGHT_SHIFT;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					ret = AssignExpr.Operator::BINARY_AND;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					ret = AssignExpr.Operator::XOR;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					ret = AssignExpr.Operator::BINARY_OR;
					break;
				}
			}
			_ =>  {
				self.jj_la1[84] = self.jj_gen;
				self.jj_consume_token(-1)?;
				return Err(ParseException::new());
			}
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn conditional_expression(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		let left: Expression;
		let right: Expression;
		ret = self.conditional_or_expression()?;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					self.jj_consume_token()?;
					left = self.expression()?;
					self.jj_consume_token()?;
					right = self.expression()?;
					ret = ConditionalExpr::new(&self.range(ret, &self.token()), ret, left, right);
					break;
				}
			}
			_ =>  {
				self.jj_la1[85] = self.jj_gen;
				;
			}
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn conditional_or_expression(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		let right: Expression;
		// First consider a higher-precedence operator, before considering the token as the left-hand part of this expression.
		ret = self.conditional_and_expression()?;
		'label_32: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[86] = self.jj_gen;
					break 'label_32;
				}
			}
			self.jj_consume_token()?;
			right = self.conditional_and_expression()?;
			ret = BinaryExpr::new(&self.range(ret, &self.token()), ret, right, BinaryExpr.Operator::OR);
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn conditional_and_expression(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		let right: Expression;
		// First consider a higher-precedence operator, before considering the token as the left-hand part of this expression.
		ret = self.inclusive_or_expression()?;
		'label_33: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[87] = self.jj_gen;
					break 'label_33;
				}
			}
			self.jj_consume_token()?;
			right = self.inclusive_or_expression()?;
			ret = BinaryExpr::new(&self.range(ret, &self.token()), ret, right, BinaryExpr.Operator::AND);
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn inclusive_or_expression(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		let right: Expression;
		// First consider a higher-precedence operator, before considering the token as the left-hand part of this expression.
		ret = self.exclusive_or_expression()?;
		'label_34: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[88] = self.jj_gen;
					break 'label_34;
				}
			}
			self.jj_consume_token()?;
			right = self.exclusive_or_expression()?;
			ret = BinaryExpr::new(&self.range(ret, &self.token()), ret, right, BinaryExpr.Operator::BINARY_OR);
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn exclusive_or_expression(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		let right: Expression;
		// First consider a higher-precedence operator, before considering the token as the left-hand part of this expression.
		ret = self.and_expression()?;
		'label_35: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[89] = self.jj_gen;
					break 'label_35;
				}
			}
			self.jj_consume_token()?;
			right = self.and_expression()?;
			ret = BinaryExpr::new(&self.range(ret, &self.token()), ret, right, BinaryExpr.Operator::XOR);
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn and_expression(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		let right: Expression;
		// First consider a higher-precedence operator, before considering the token as the left-hand part of this expression.
		ret = self.equality_expression()?;
		'label_36: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[90] = self.jj_gen;
					break 'label_36;
				}
			}
			self.jj_consume_token()?;
			right = self.equality_expression()?;
			ret = BinaryExpr::new(&self.range(ret, &self.token()), ret, right, BinaryExpr.Operator::BINARY_AND);
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn equality_expression(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		let right: Expression;
		let op: BinaryExpr.Operator;
		// First consider a higher-precedence operator, before considering the token as the left-hand part of this expression.
		// Note that instanceof is a {@code RelationalExpression} within the JLS, which differs from JavaParser
		ret = self.instance_of_expression()?;
		'label_37: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[91] = self.jj_gen;
					break 'label_37;
				}
			}
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						self.jj_consume_token()?;
						op = BinaryExpr.Operator::EQUALS;
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						op = BinaryExpr.Operator::NOT_EQUALS;
						break;
					}
				}
				_ =>  {
					self.jj_la1[92] = self.jj_gen;
					self.jj_consume_token(-1)?;
					return Err(ParseException::new());
				}
			}
			right = self.instance_of_expression()?;
			ret = BinaryExpr::new(&self.range(ret, &self.token()), ret, right, op);
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn pattern_expression(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::pattern_expr::PatternExpr {
		let ret: PatternExpr;
		if self.jj_2_34(2147483647) {
			ret = self.type_pattern_expression()?;
		} else {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
					{
						ret = self.record_pattern_expression()?;
						break;
					}
				}
				_ =>  {
					self.jj_la1[93] = self.jj_gen;
					self.jj_consume_token(-1)?;
					return Err(ParseException::new());
				}
			}
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn component_pattern_expression(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::component_pattern_expr::ComponentPatternExpr {
		let ret: ComponentPatternExpr;
		if self.jj_2_35(2147483647) {
			ret = self.match_all_pattern_expression()?;
		} else {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
					{
						ret = self.pattern_expression()?;
						break;
					}
				}
				_ =>  {
					self.jj_la1[94] = self.jj_gen;
					self.jj_consume_token(-1)?;
					return Err(ParseException::new());
				}
			}
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn type_pattern_expression(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::type_pattern_expr::TypePatternExpr {
		let modifier: ModifierHolder;
		let type: Type;
		let name: SimpleName;
		modifier = self.modifiers()?;
		type = self.type(modifier.annotations)?;
		name = self.simple_name()?;
		{
			if "" != null {
				return TypePatternExpr::new(&self.range(type, &self.token()), modifier.modifiers, type, name);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn record_pattern_expression(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::record_pattern_expr::RecordPatternExpr {
		let modifier: ModifierHolder;
		let type: ReferenceType;
		let pattern_list: NodeList<ComponentPatternExpr>;
		modifier = self.modifiers()?;
		type = self.reference_type(modifier.annotations)?;
		pattern_list = self.pattern_list()?;
		{
			if "" != null {
				return RecordPatternExpr::new(&self.range(type, &self.token()), modifier.modifiers, type, pattern_list);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn match_all_pattern_expression(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::match_all_pattern_expr::MatchAllPatternExpr {
		let modifier: ModifierHolder;
		modifier = self.modifiers()?;
		self.jj_consume_token()?;
		{
			if "" != null {
				return MatchAllPatternExpr::new(&self.range(&self.token(), &self.token()), modifier.modifiers);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn pattern_list(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::node_list::NodeList {
		let pattern: ComponentPatternExpr;
		let ret: NodeList<ComponentPatternExpr> = NodeList<>::new();
		self.jj_consume_token()?;
		pattern = self.component_pattern_expression()?;
		ret.add(pattern);
		'label_38: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[95] = self.jj_gen;
					break 'label_38;
				}
			}
			self.jj_consume_token()?;
			pattern = self.component_pattern_expression()?;
			ret.add(pattern);
		}
		self.jj_consume_token()?;
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn instance_of_expression(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		let type: ReferenceType;
		let annotations: NodeList<AnnotationExpr>;
		let pattern: PatternExpr;
		ret = self.relational_expression()?;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					self.jj_consume_token()?;
					if self.jj_2_36(2147483647) {
						pattern = self.pattern_expression()?;
						ret = InstanceOfExpr::new(&self.range(ret, &self.token()), ret, &pattern.get_type().as_reference_type()?, pattern);
					} else {
						match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
								{
									type = self.annotated_reference_type()?;
									ret = InstanceOfExpr::new(&self.range(ret, &self.token()), ret, type, null);
									break;
								}
							}
							_ =>  {
								self.jj_la1[96] = self.jj_gen;
								self.jj_consume_token(-1)?;
								return Err(ParseException::new());
							}
						}
					}
					break;
				}
			}
			_ =>  {
				self.jj_la1[97] = self.jj_gen;
				;
			}
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn relational_expression(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		let right: Expression;
		let op: BinaryExpr.Operator;
		ret = self.shift_expression()?;
		'label_39: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[98] = self.jj_gen;
					break 'label_39;
				}
			}
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						self.jj_consume_token()?;
						op = BinaryExpr.Operator::LESS;
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						op = BinaryExpr.Operator::GREATER;
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						op = BinaryExpr.Operator::LESS_EQUALS;
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						op = BinaryExpr.Operator::GREATER_EQUALS;
						break;
					}
				}
				_ =>  {
					self.jj_la1[99] = self.jj_gen;
					self.jj_consume_token(-1)?;
					return Err(ParseException::new());
				}
			}
			right = self.shift_expression()?;
			ret = BinaryExpr::new(&self.range(ret, &self.token()), ret, right, op);
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn shift_expression(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		let right: Expression;
		let op: BinaryExpr.Operator;
		ret = self.additive_expression()?;
		'label_40: while true {
			if self.jj_2_37(1) {
			} else {
				break 'label_40;
			}
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						self.jj_consume_token()?;
						op = BinaryExpr.Operator::LEFT_SHIFT;
						break;
					}
				}
				_ =>  {
					self.jj_la1[100] = self.jj_gen;
					if self.jj_2_38(1) {
						self.rsignedshift()?;
						op = BinaryExpr.Operator::SIGNED_RIGHT_SHIFT;
					} else if self.jj_2_39(1) {
						self.runsignedshift()?;
						op = BinaryExpr.Operator::UNSIGNED_RIGHT_SHIFT;
					} else {
						self.jj_consume_token(-1)?;
						return Err(ParseException::new());
					}
				}
			}
			right = self.additive_expression()?;
			ret = BinaryExpr::new(&self.range(ret, &self.token()), ret, right, op);
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn additive_expression(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		let right: Expression;
		let op: BinaryExpr.Operator;
		ret = self.multiplicative_expression()?;
		'label_41: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[101] = self.jj_gen;
					break 'label_41;
				}
			}
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						self.jj_consume_token()?;
						op = BinaryExpr.Operator::PLUS;
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						op = BinaryExpr.Operator::MINUS;
						break;
					}
				}
				_ =>  {
					self.jj_la1[102] = self.jj_gen;
					self.jj_consume_token(-1)?;
					return Err(ParseException::new());
				}
			}
			right = self.multiplicative_expression()?;
			ret = BinaryExpr::new(&self.range(ret, &self.token()), ret, right, op);
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn multiplicative_expression(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		let right: Expression;
		let op: BinaryExpr.Operator;
		ret = self.unary_expression()?;
		'label_42: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
				}
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[103] = self.jj_gen;
					break 'label_42;
				}
			}
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						self.jj_consume_token()?;
						op = BinaryExpr.Operator::MULTIPLY;
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						op = BinaryExpr.Operator::DIVIDE;
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						op = BinaryExpr.Operator::REMAINDER;
						break;
					}
				}
				_ =>  {
					self.jj_la1[104] = self.jj_gen;
					self.jj_consume_token(-1)?;
					return Err(ParseException::new());
				}
			}
			right = self.unary_expression()?;
			ret = BinaryExpr::new(&self.range(ret, &self.token()), ret, right, op);
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn unary_expression(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		let op: UnaryExpr.Operator;
		let begin: JavaToken = ;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					ret = self.pre_increment_expression()?;
					break;
				}
			}
			 =>  {
				{
					ret = self.pre_decrement_expression()?;
					break;
				}
			}
			 =>  {
			}
			 =>  {
				{
					match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
						 =>  {
							{
								self.jj_consume_token()?;
								op = UnaryExpr.Operator::PLUS;
								begin = self.token();
								break;
							}
						}
						 =>  {
							{
								self.jj_consume_token()?;
								op = UnaryExpr.Operator::MINUS;
								begin = self.token();
								break;
							}
						}
						_ =>  {
							self.jj_la1[105] = self.jj_gen;
							self.jj_consume_token(-1)?;
							return Err(ParseException::new());
						}
					}
					ret = self.unary_expression()?;
					ret = UnaryExpr::new(&self.range(begin, &self.token()), ret, op);
					break;
				}
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
				{
					ret = self.unary_expression_not_plus_minus()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[106] = self.jj_gen;
				self.jj_consume_token(-1)?;
				return Err(ParseException::new());
			}
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn pre_increment_expression(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		let begin: JavaToken = ;
		self.jj_consume_token()?;
		begin = self.token();
		ret = self.unary_expression()?;
		ret = UnaryExpr::new(&self.range(begin, &self.token()), ret, UnaryExpr.Operator::PREFIX_INCREMENT);
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn pre_decrement_expression(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		let begin: JavaToken;
		self.jj_consume_token()?;
		begin = self.token();
		ret = self.unary_expression()?;
		ret = UnaryExpr::new(&self.range(begin, &self.token()), ret, UnaryExpr.Operator::PREFIX_DECREMENT);
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn unary_expression_not_plus_minus(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		let op: UnaryExpr.Operator;
		let begin: JavaToken = ;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
			}
			 =>  {
				{
					match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
						 =>  {
							{
								self.jj_consume_token()?;
								op = UnaryExpr.Operator::BITWISE_COMPLEMENT;
								begin = self.token();
								break;
							}
						}
						 =>  {
							{
								self.jj_consume_token()?;
								op = UnaryExpr.Operator::LOGICAL_COMPLEMENT;
								begin = self.token();
								break;
							}
						}
						_ =>  {
							self.jj_la1[107] = self.jj_gen;
							self.jj_consume_token(-1)?;
							return Err(ParseException::new());
						}
					}
					ret = self.unary_expression()?;
					ret = UnaryExpr::new(&self.range(begin, &self.token()), ret, op);
					break;
				}
			}
			_ =>  {
				self.jj_la1[108] = self.jj_gen;
				if self.jj_2_40(2147483647) {
					ret = self.cast_expression()?;
				} else {
					match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
							{
								ret = self.postfix_expression()?;
								break;
							}
						}
						 =>  {
							{
								ret = self.switch_expression()?;
								break;
							}
						}
						_ =>  {
							self.jj_la1[109] = self.jj_gen;
							self.jj_consume_token(-1)?;
							return Err(ParseException::new());
						}
					}
				}
			}
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn postfix_expression(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		let op: UnaryExpr.Operator;
		ret = self.primary_expression()?;
		if self.jj_2_41(2) {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						self.jj_consume_token()?;
						op = UnaryExpr.Operator::POSTFIX_INCREMENT;
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						op = UnaryExpr.Operator::POSTFIX_DECREMENT;
						break;
					}
				}
				_ =>  {
					self.jj_la1[110] = self.jj_gen;
					self.jj_consume_token(-1)?;
					return Err(ParseException::new());
				}
			}
			ret = UnaryExpr::new(&self.range(ret, &self.token()), ret, op);
		} else {
			;
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn cast_expression(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		let reference_type: ReferenceType;
		let primitive_type: PrimitiveType;
		let begin: JavaToken = ;
		let annotations: NodeList<AnnotationExpr>;
		let types_of_multi_cast: NodeList<ReferenceType> = self.empty_node_list();
		self.jj_consume_token()?;
		begin = self.token();
		annotations = self.annotations()?;
		if self.jj_2_42(2) {
			primitive_type = self.primitive_type(annotations)?;
			self.jj_consume_token()?;
			ret = self.unary_expression()?;
			ret = CastExpr::new(&self.range(begin, &self.token()), primitive_type, ret);
		} else {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
					{
						// ( ReferenceType {AdditionalBound} ) UnaryExpressionNotPlusMinus
						// ( ReferenceType {AdditionalBound} ) LambdaExpression
						reference_type = self.reference_type(annotations)?;
						types_of_multi_cast = self.add(types_of_multi_cast, reference_type);
						'label_43: while true {
							match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
								 =>  {
									{
										break;
									}
								}
								_ =>  {
									self.jj_la1[111] = self.jj_gen;
									break 'label_43;
								}
							}
							self.jj_consume_token()?;
							reference_type = self.annotated_reference_type()?;
							types_of_multi_cast = self.add(types_of_multi_cast, reference_type);
						}
						self.jj_consume_token()?;
						ret = self.unary_expression_not_plus_minus()?;
						if types_of_multi_cast.size() > 1 {
							ret = CastExpr::new(&self.range(begin, &self.token()), IntersectionType::new(&self.range(&types_of_multi_cast.get(0), &types_of_multi_cast.get(types_of_multi_cast.size() - 1)), types_of_multi_cast), ret);
						} else {
							ret = CastExpr::new(&self.range(begin, &self.token()), reference_type, ret);
						}
						break;
					}
				}
				_ =>  {
					self.jj_la1[112] = self.jj_gen;
					self.jj_consume_token(-1)?;
					return Err(ParseException::new());
				}
			}
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn primary_expression(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		ret = self.primary_prefix()?;
		'label_44: while true {
			if self.jj_2_43(2) {
			} else {
				break 'label_44;
			}
			ret = self.primary_suffix(ret)?;
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn primary_expression_without_super_suffix(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		ret = self.primary_prefix()?;
		'label_45: while true {
			if self.jj_2_44(2147483647) {
			} else {
				break 'label_45;
			}
			ret = self.primary_suffix_without_super(ret)?;
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn primary_prefix(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.AssertionError | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression = null;
		let name: SimpleName;
		let type_args: RangedList<Type> = RangedList<Type>::new(null);
		let args: NodeList<Expression> = self.empty_node_list();
		let params: NodeList<Parameter> = self.empty_node_list();
		let has_args: bool = false;
		let is_lambda: bool = false;
		let type: Type;
		let begin: JavaToken;
		let p: Parameter = null;
		let id: SimpleName = null;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
				{
					ret = self.literal()?;
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					ret = ThisExpr::new(&self.token_range(), null);
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					ret = SuperExpr::new(&self.token_range(), null);
					match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
						 =>  {
							{
								self.jj_consume_token()?;
								match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
									 =>  {
										{
											type_args = self.type_arguments()?;
											break;
										}
									}
									_ =>  {
										self.jj_la1[113] = self.jj_gen;
										;
									}
								}
								name = self.simple_name()?;
								match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
									 =>  {
										{
											args = self.arguments()?;
											has_args = true;
											break;
										}
									}
									_ =>  {
										self.jj_la1[114] = self.jj_gen;
										;
									}
								}
								if has_args {
									ret = MethodCallExpr::new(&self.range(ret, &self.token()), ret, type_args.list, name, args);
								} else {
									ret = FieldAccessExpr::new(&self.range(ret, &self.token()), ret, &self.empty_node_list(), name);
								}
								break;
							}
						}
						 =>  {
							{
								self.jj_consume_token()?;
								match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
									 =>  {
										{
											type_args = self.type_arguments()?;
											break;
										}
									}
									_ =>  {
										self.jj_la1[115] = self.jj_gen;
										;
									}
								}
								match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
										{
											self.identifier()?;
											break;
										}
									}
									 =>  {
										{
											self.jj_consume_token()?;
											break;
										}
									}
									_ =>  {
										self.jj_la1[116] = self.jj_gen;
										self.jj_consume_token(-1)?;
										return Err(ParseException::new());
									}
								}
								ret = MethodReferenceExpr::new(&self.range(ret, &self.token()), ret, type_args.list, self.token.image);
								break;
							}
						}
						_ =>  {
							self.jj_la1[117] = self.jj_gen;
							self.jj_consume_token(-1)?;
							return Err(ParseException::new());
						}
					}
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					begin = self.token();
					match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
						 =>  {
							{
								self.jj_consume_token()?;
								ret = LambdaExpr::new(&self.range(begin, &self.token()), params, BlockStmt::new(), true);
								break;
							}
						}
						_ =>  {
							self.jj_la1[118] = self.jj_gen;
							if self.jj_2_45(2147483647) {
								params = self.lambda_parameters()?;
								self.jj_consume_token()?;
								ret = LambdaExpr::new(&self.range(begin, &self.token()), params, BlockStmt::new(), true);
							} else if self.jj_2_46(2147483647) {
								params = self.inferred_lambda_parameters()?;
								self.jj_consume_token()?;
								ret = LambdaExpr::new(&self.range(begin, &self.token()), params, BlockStmt::new(), true);
							} else {
								match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
									}
									 =>  {
										{
											// This could still be a lambda expression, but this is handled after matching -> elsewhere
											ret = self.expression()?;
											self.jj_consume_token()?;
											ret = EnclosedExpr::new(&self.range(begin, &self.token()), ret);
											break;
										}
									}
									_ =>  {
										self.jj_la1[119] = self.jj_gen;
										self.jj_consume_token(-1)?;
										return Err(ParseException::new());
									}
								}
							}
						}
					}
					break;
				}
			}
			 =>  {
				{
					ret = self.allocation_expression(null)?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[123] = self.jj_gen;
				if self.jj_2_47(2147483647) {
					type = self.result_type(&self.empty_node_list())?;
					self.jj_consume_token()?;
					self.jj_consume_token()?;
					ret = ClassExpr::new(&self.range(type, &self.token()), type);
				} else if self.jj_2_48(2147483647) {
					type = self.annotated_type()?;
					self.jj_consume_token()?;
					match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
						 =>  {
							{
								type_args = self.type_arguments()?;
								break;
							}
						}
						_ =>  {
							self.jj_la1[120] = self.jj_gen;
							;
						}
					}
					match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
							{
								self.identifier()?;
								break;
							}
						}
						 =>  {
							{
								self.jj_consume_token()?;
								break;
							}
						}
						_ =>  {
							self.jj_la1[121] = self.jj_gen;
							self.jj_consume_token(-1)?;
							return Err(ParseException::new());
						}
					}
					ret = TypeExpr::new(&self.range(type, type), type);
					ret = MethodReferenceExpr::new(&self.range(ret, &self.token()), ret, type_args.list, self.token.image);
				} else {
					match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
							{
								name = self.simple_name()?;
								begin = self.token();
								match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
									 =>  {
										{
											args = self.arguments()?;
											has_args = true;
											break;
										}
									}
									_ =>  {
										self.jj_la1[122] = self.jj_gen;
										;
									}
								}
								if has_args {
									ret = MethodCallExpr::new(&self.range(begin, &self.token()), null, null, name, args);
								} else {
									ret = NameExpr::new(name);
								}
								break;
							}
						}
						_ =>  {
							self.jj_la1[124] = self.jj_gen;
							self.jj_consume_token(-1)?;
							return Err(ParseException::new());
						}
					}
				}
			}
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn primary_suffix(&mut self, scope: &com::github::javaparser::ast::expr::expression::Expression) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		if self.jj_2_49(2) {
			ret = self.primary_suffix_without_super(scope)?;
		} else {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						self.jj_consume_token()?;
						self.jj_consume_token()?;
						ret = SuperExpr::new(&self.range(scope, &self.token()), &self.scope_to_name(scope)?);
						break;
					}
				}
				_ =>  {
					self.jj_la1[125] = self.jj_gen;
					self.jj_consume_token(-1)?;
					return Err(ParseException::new());
				}
			}
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn primary_suffix_without_super(&mut self, scope: &com::github::javaparser::ast::expr::expression::Expression) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		let type_args: RangedList<Type> = RangedList<Type>::new(null);
		let args: NodeList<Expression> = self.empty_node_list();
		let has_args: bool = false;
		let name: SimpleName;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					self.jj_consume_token()?;
					match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
						 =>  {
							{
								self.jj_consume_token()?;
								ret = ThisExpr::new(&self.range(scope, &self.token()), &self.scope_to_name(scope)?);
								break;
							}
						}
						 =>  {
							{
								ret = self.allocation_expression(scope)?;
								break;
							}
						}
						_ =>  {
							self.jj_la1[128] = self.jj_gen;
							if self.jj_2_50(2147483647) {
								match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
									 =>  {
										{
											type_args = self.type_arguments()?;
											break;
										}
									}
									_ =>  {
										self.jj_la1[126] = self.jj_gen;
										;
									}
								}
								name = self.simple_name()?;
								match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
									 =>  {
										{
											args = self.arguments()?;
											has_args = true;
											break;
										}
									}
									_ =>  {
										self.jj_la1[127] = self.jj_gen;
										;
									}
								}
								if has_args {
									ret = MethodCallExpr::new(&self.range(scope, &self.token()), scope, type_args.list, name, args);
								} else {
									ret = FieldAccessExpr::new(&self.range(scope, &self.token()), scope, type_args.list, name);
								}
							} else {
								self.jj_consume_token(-1)?;
								return Err(ParseException::new());
							}
						}
					}
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					ret = self.expression()?;
					self.jj_consume_token()?;
					ret = ArrayAccessExpr::new(&self.range(scope, &self.token()), scope, ret);
					break;
				}
			}
			_ =>  {
				self.jj_la1[129] = self.jj_gen;
				self.jj_consume_token(-1)?;
				return Err(ParseException::new());
			}
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn literal(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					self.jj_consume_token()?;
					ret = IntegerLiteralExpr::new(&self.token_range(), self.token.image);
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					ret = LongLiteralExpr::new(&self.token_range(), self.token.image);
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					ret = DoubleLiteralExpr::new(&self.token_range(), self.token.image);
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					ret = CharLiteralExpr::new(&self.token_range(), &self.unquote(self.token.image));
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					ret = StringLiteralExpr::new(&self.token_range(), &self.unquote(self.token.image));
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					ret = TextBlockLiteralExpr::new(&self.token_range(), &self.un_triple_quote(self.token.image));
					break;
				}
			}
			 =>  {
			}
			 =>  {
				{
					ret = self.boolean_literal()?;
					break;
				}
			}
			 =>  {
				{
					ret = self.null_literal()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[130] = self.jj_gen;
				self.jj_consume_token(-1)?;
				return Err(ParseException::new());
			}
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn boolean_literal(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					self.jj_consume_token()?;
					ret = BooleanLiteralExpr::new(&self.token_range(), true);
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					ret = BooleanLiteralExpr::new(&self.token_range(), false);
					break;
				}
			}
			_ =>  {
				self.jj_la1[131] = self.jj_gen;
				self.jj_consume_token(-1)?;
				return Err(ParseException::new());
			}
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn null_literal(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		self.jj_consume_token()?;
		{
			if "" != null {
				return NullLiteralExpr::new(&self.token_range());
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn arguments(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::node_list::NodeList {
		let ret: NodeList<Expression> = self.empty_node_list();
		self.jj_consume_token()?;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
				{
					ret = self.argument_list()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[132] = self.jj_gen;
				;
			}
		}
		self.jj_consume_token()?;
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn argument_list(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::node_list::NodeList {
		let ret: NodeList<Expression> = self.empty_node_list();
		let expr: Expression;
		expr = self.expression()?;
		ret.add(expr);
		'label_46: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[133] = self.jj_gen;
					break 'label_46;
				}
			}
			self.jj_consume_token()?;
			expr = self.expression()?;
			ret.add(expr);
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn allocation_expression(&mut self, scope: &com::github::javaparser::ast::expr::expression::Expression) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		let type: Type;
		let type_args: RangedList<Type> = RangedList<Type>::new(null);
		let anonymous_body: NodeList<BodyDeclaration<?>> = null;
		let args: NodeList<Expression>;
		let begin: JavaToken = ;
		let annotations: NodeList<AnnotationExpr> = NodeList<AnnotationExpr>::new();
		self.jj_consume_token()?;
		if scope == null {
			begin = self.token();
		} else {
			begin = self.or_if_invalid(begin, scope);
		}
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					type_args = self.type_arguments()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[134] = self.jj_gen;
				;
			}
		}
		annotations = self.annotations()?;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
				{
					// new array of primitives
					type = self.primitive_type(annotations)?;
					ret = self.array_creation(begin, type)?;
					break;
				}
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
				{
					// new reference type e.g. class (or an array of those)
					type = self.class_or_interface_type(annotations)?;
					match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
						 =>  {
						}
						 =>  {
							{
								// new Integer[6] -- the array doesn't have parameters.
								ret = self.array_creation(begin, type)?;
								break;
							}
						}
						 =>  {
							{
								// new Integer(6) -- not an array if parameters being passed.
								args = self.arguments()?;
								if self.jj_2_51(2) {
									anonymous_body = self.class_or_interface_body()?;
								} else {
									;
								}
								ret = ObjectCreationExpr::new(&self.range(begin, &self.token()), scope, type as ClassOrInterfaceType, type_args.list, args, anonymous_body);
								break;
							}
						}
						_ =>  {
							self.jj_la1[135] = self.jj_gen;
							self.jj_consume_token(-1)?;
							return Err(ParseException::new());
						}
					}
					break;
				}
			}
			_ =>  {
				self.jj_la1[136] = self.jj_gen;
				self.jj_consume_token(-1)?;
				return Err(ParseException::new());
			}
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn array_creation(&mut self, begin: &com::github::javaparser::java_token::JavaToken, type: &com::github::javaparser::ast::type::type::Type) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::array_creation_expr::ArrayCreationExpr {
		let expr: Expression = null;
		let array_initializer_expr: ArrayInitializerExpr = null;
		let inits: NodeList<Expression> = self.empty_node_list();
		let accum: List<NodeList<AnnotationExpr>> = ArrayList<NodeList<AnnotationExpr>>::new();
		let annotations: NodeList<AnnotationExpr> = NodeList<AnnotationExpr>::new();
		let array_creation_level_start: JavaToken = ;
		let level_ranges: List<TokenRange> = ArrayList<TokenRange>::new();
		'label_47: while true {
			annotations = self.annotations()?;
			self.jj_consume_token()?;
			array_creation_level_start =  if annotations.is_empty() { self.token() } else { self.or_if_invalid(array_creation_level_start, &annotations.get(0)) };
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
					{
						expr = self.expression()?;
						break;
					}
				}
				_ =>  {
					self.jj_la1[137] = self.jj_gen;
					;
				}
			}
			accum = self.add(accum, annotations);
			inits = self.add(inits, expr);
			annotations = null;
			expr = null;
			self.jj_consume_token()?;
			level_ranges.add(&self.range(array_creation_level_start, &self.token()));
			if self.jj_2_52(2) {
			} else {
				break 'label_47;
			}
		}
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					array_initializer_expr = self.array_initializer()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[138] = self.jj_gen;
				;
			}
		}
		{
			if "" != null {
				return self.juggle_array_creation(&self.range(begin, &self.token()), level_ranges, type, inits, accum, array_initializer_expr);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn statement(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::statement::Statement {
		let ret: Statement;
		let r0 = 'try0: {
			if self.jj_2_53(2) {
				ret = match self.labeled_statement() {
					Err(e) => break 'try0 Err(e),
					Ok(s) => s,
				};
			} else if self.jj_2_54(3) {
				ret = match self.assert_statement() {
					Err(e) => break 'try0 Err(e),
					Ok(s) => s,
				};
			} else if self.jj_2_55(3) {
				ret = match self.yield_statement() {
					Err(e) => break 'try0 Err(e),
					Ok(s) => s,
				};
			} else {
				match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
					 =>  {
						{
							ret = match self.block() {
								Err(e) => break 'try0 Err(e),
								Ok(s) => s,
							};
							break;
						}
					}
					 =>  {
						{
							ret = match self.empty_statement() {
								Err(e) => break 'try0 Err(e),
								Ok(s) => s,
							};
							break;
						}
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
						{
							ret = match self.statement_expression() {
								Err(e) => break 'try0 Err(e),
								Ok(s) => s,
							};
							break;
						}
					}
					 =>  {
						{
							ret = match self.switch_statement() {
								Err(e) => break 'try0 Err(e),
								Ok(s) => s,
							};
							break;
						}
					}
					 =>  {
						{
							ret = match self.if_statement() {
								Err(e) => break 'try0 Err(e),
								Ok(s) => s,
							};
							break;
						}
					}
					 =>  {
						{
							ret = match self.while_statement() {
								Err(e) => break 'try0 Err(e),
								Ok(s) => s,
							};
							break;
						}
					}
					 =>  {
						{
							ret = match self.do_statement() {
								Err(e) => break 'try0 Err(e),
								Ok(s) => s,
							};
							break;
						}
					}
					 =>  {
						{
							ret = match self.for_statement() {
								Err(e) => break 'try0 Err(e),
								Ok(s) => s,
							};
							break;
						}
					}
					 =>  {
						{
							ret = match self.break_statement() {
								Err(e) => break 'try0 Err(e),
								Ok(s) => s,
							};
							break;
						}
					}
					 =>  {
						{
							ret = match self.continue_statement() {
								Err(e) => break 'try0 Err(e),
								Ok(s) => s,
							};
							break;
						}
					}
					 =>  {
						{
							ret = match self.return_statement() {
								Err(e) => break 'try0 Err(e),
								Ok(s) => s,
							};
							break;
						}
					}
					 =>  {
						{
							ret = match self.throw_statement() {
								Err(e) => break 'try0 Err(e),
								Ok(s) => s,
							};
							break;
						}
					}
					 =>  {
						{
							ret = match self.synchronized_statement() {
								Err(e) => break 'try0 Err(e),
								Ok(s) => s,
							};
							break;
						}
					}
					 =>  {
						{
							ret = match self.try_statement() {
								Err(e) => break 'try0 Err(e),
								Ok(s) => s,
							};
							break;
						}
					}
					_ =>  {
						self.jj_la1[139] = self.jj_gen;
						match self.jj_consume_token(-1) {
							Err(e) => break 'try0 Err(e),
							Ok(s) => s,
						};
						break 'try0 Err(ParseException::new());
					}
				}
			}
			{
				if "" != null {
					return ret;
				}
	
			}
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ ParseException) => {
				let error_range: TokenRange = self.recover_statement(, , , e);
				{
					if "" != null {
						return UnparsableStmt::new(error_range);
					}
	
				}
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn assert_statement(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::assert_stmt::AssertStmt {
		let check: Expression;
		let msg: Expression = null;
		let begin: JavaToken;
		self.jj_consume_token()?;
		begin = self.token();
		check = self.expression()?;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					self.jj_consume_token()?;
					msg = self.expression()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[140] = self.jj_gen;
				;
			}
		}
		self.jj_consume_token()?;
		{
			if "" != null {
				return AssertStmt::new(&self.range(begin, &self.token()), check, msg);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn labeled_statement(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::labeled_stmt::LabeledStmt {
		let label: SimpleName;
		let stmt: Statement;
		let begin: JavaToken;
		label = self.simple_name()?;
		begin = self.token();
		self.jj_consume_token()?;
		stmt = self.statement()?;
		{
			if "" != null {
				return LabeledStmt::new(&self.range(begin, &self.token()), label, stmt);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn block(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::block_stmt::BlockStmt {
		let stmts: NodeList<Statement> = self.empty_node_list();
		let begin: JavaToken;
		self.jj_consume_token()?;
		begin = self.token();
		let r0 = 'try0: {
			stmts = match self.statements() {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			match self.jj_consume_token() {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			{
				if "" != null {
					return BlockStmt::new(&self.range(begin, &self.token()), stmts);
				}
	
			}
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ ParseException) => {
				self.recover(, e);
				let block: BlockStmt = BlockStmt::new(&self.range(begin, &self.token()), NodeList<Statement>::new());
				block.set_parsed(UNPARSABLE);
				{
					if "" != null {
						return block;
					}
	
				}
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		break 'try0 Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn block_statement(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::statement::Statement {
		let ret: Statement;
		let expr: Expression;
		let type_decl: ClassOrInterfaceDeclaration;
		let record_declaration: RecordDeclaration;
		let modifier: ModifierHolder;
		let r0 = 'try0: {
			if self.jj_2_56(2147483647) {
				// JLS specifies local class. Since java 16 local enum or interface are allowed.
				modifier = match self.modifiers() {
					Err(e) => break 'try0 Err(e),
					Ok(s) => s,
				};
				type_decl = match self.class_or_interface_declaration(modifier) {
					Err(e) => break 'try0 Err(e),
					Ok(s) => s,
				};
				ret = LocalClassDeclarationStmt::new(&self.range(type_decl, &self.token()), type_decl);
			} else if self.jj_2_57(2147483647) {
				modifier = match self.modifiers() {
					Err(e) => break 'try0 Err(e),
					Ok(s) => s,
				};
				record_declaration = match self.record_declaration(modifier) {
					Err(e) => break 'try0 Err(e),
					Ok(s) => s,
				};
				ret = LocalRecordDeclarationStmt::new(&self.range(record_declaration, &self.token()), record_declaration);
			} else if self.jj_2_58(2147483647) {
				ret = match self.yield_statement() {
					Err(e) => break 'try0 Err(e),
					Ok(s) => s,
				};
			} else if self.jj_2_59(2147483647) {
				ret = match self.assert_statement() {
					Err(e) => break 'try0 Err(e),
					Ok(s) => s,
				};
			} else if self.jj_2_60(2147483647) {
				expr = match self.variable_declaration_expression() {
					Err(e) => break 'try0 Err(e),
					Ok(s) => s,
				};
				match self.jj_consume_token() {
					Err(e) => break 'try0 Err(e),
					Ok(s) => s,
				};
				ret = ExpressionStmt::new(&self.range(expr, &self.token()), expr);
			} else if self.jj_2_61(2147483647) {
				ret = match self.explicit_constructor_invocation() {
					Err(e) => break 'try0 Err(e),
					Ok(s) => s,
				};
			} else {
				match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
					}
					 =>  {
						{
							ret = match self.statement() {
								Err(e) => break 'try0 Err(e),
								Ok(s) => s,
							};
							break;
						}
					}
					_ =>  {
						self.jj_la1[141] = self.jj_gen;
						match self.jj_consume_token(-1) {
							Err(e) => break 'try0 Err(e),
							Ok(s) => s,
						};
						break 'try0 Err(ParseException::new());
					}
				}
			}
			{
				if "" != null {
					return ret;
				}
	
			}
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ ParseException) => {
				let error_range: TokenRange = self.recover_statement(, , , e);
				{
					if "" != null {
						return UnparsableStmt::new(error_range);
					}
	
				}
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn variable_declaration_expression(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr {
		let modifier: ModifierHolder;
		let partial_type: Type;
		let variables: NodeList<VariableDeclarator> = NodeList<VariableDeclarator>::new();
		let var: VariableDeclarator;
		modifier = self.modifiers()?;
		partial_type = self.type(&self.empty_node_list())?;
		var = self.variable_declarator(partial_type)?;
		variables.add(var);
		'label_48: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[142] = self.jj_gen;
					break 'label_48;
				}
			}
			self.jj_consume_token()?;
			var = self.variable_declarator(partial_type)?;
			variables.add(var);
		}
		let begin: JavaToken = self.or_if_invalid(modifier.begin, partial_type);
		{
			if "" != null {
				return VariableDeclarationExpr::new(&self.range(begin, &self.token()), modifier.modifiers, modifier.annotations, variables);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn empty_statement(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::empty_stmt::EmptyStmt {
		self.jj_consume_token()?;
		{
			if "" != null {
				return EmptyStmt::new(&self.token_range());
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn lambda_body(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::statement::Statement {
		let expr: Expression;
		let n: Statement = null;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
				{
					expr = self.expression()?;
					n = ExpressionStmt::new(&self.range(expr, &self.token()), expr);
					break;
				}
			}
			 =>  {
				{
					n = self.block()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[143] = self.jj_gen;
				self.jj_consume_token(-1)?;
				return Err(ParseException::new());
			}
		}
		{
			if "" != null {
				return n;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn statement_expression(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::expression_stmt::ExpressionStmt {
		let expr: Expression;
		let op: AssignExpr.Operator;
		let value: Expression;
		let type_args: RangedList<Type> = RangedList<Type>::new(null);
		let lambda_body: Statement;
		if self.jj_2_62(2) {
			expr = self.pre_increment_expression()?;
		} else {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						expr = self.pre_decrement_expression()?;
						break;
					}
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
					{
						expr = self.primary_expression()?;
						match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
								{
									match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
										 =>  {
											{
												self.jj_consume_token()?;
												expr = UnaryExpr::new(&self.range(expr, &self.token()), expr, UnaryExpr.Operator::POSTFIX_INCREMENT);
												break;
											}
										}
										 =>  {
											{
												self.jj_consume_token()?;
												expr = UnaryExpr::new(&self.range(expr, &self.token()), expr, UnaryExpr.Operator::POSTFIX_DECREMENT);
												break;
											}
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
										}
										 =>  {
											{
												op = self.assignment_operator()?;
												value = self.expression()?;
												expr = AssignExpr::new(&self.range(expr, &self.token()), expr, value, op);
												break;
											}
										}
										_ =>  {
											self.jj_la1[144] = self.jj_gen;
											self.jj_consume_token(-1)?;
											return Err(ParseException::new());
										}
									}
									break;
								}
							}
							_ =>  {
								self.jj_la1[145] = self.jj_gen;
								;
							}
						}
						break;
					}
				}
				_ =>  {
					self.jj_la1[146] = self.jj_gen;
					self.jj_consume_token(-1)?;
					return Err(ParseException::new());
				}
			}
		}
		self.jj_consume_token()?;
		{
			if "" != null {
				return ExpressionStmt::new(&self.range(expr, &self.token()), expr);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn switch_statement(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::switch_stmt::SwitchStmt {
		let selector: Expression;
		let entry: SwitchEntry;
		let entries: NodeList<SwitchEntry> = self.empty_node_list();
		let begin: JavaToken;
		self.jj_consume_token()?;
		begin = self.token();
		self.jj_consume_token()?;
		selector = self.expression()?;
		self.jj_consume_token()?;
		self.jj_consume_token()?;
		'label_49: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[147] = self.jj_gen;
					break 'label_49;
				}
			}
			entry = self.switch_entry()?;
			entries = self.add(entries, entry);
		}
		self.jj_consume_token()?;
		{
			if "" != null {
				return SwitchStmt::new(&self.range(begin, &self.token()), selector, entries);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn switch_expression(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::switch_expr::SwitchExpr {
		let selector: Expression;
		let entry: SwitchEntry;
		let entries: NodeList<SwitchEntry> = self.empty_node_list();
		let begin: JavaToken;
		self.jj_consume_token()?;
		begin = self.token();
		self.jj_consume_token()?;
		selector = self.expression()?;
		self.jj_consume_token()?;
		self.jj_consume_token()?;
		'label_50: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[148] = self.jj_gen;
					break 'label_50;
				}
			}
			entry = self.switch_entry()?;
			entries = self.add(entries, entry);
		}
		self.jj_consume_token()?;
		{
			if "" != null {
				return SwitchExpr::new(&self.range(begin, &self.token()), selector, entries);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn switch_entry(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::switch_entry::SwitchEntry {
		let label: Expression = null;
		let labels: NodeList<Expression> = self.empty_node_list();
		let stmts: NodeList<Statement> = self.empty_node_list();
		let begin: JavaToken;
		let ret: SwitchEntry;
		let stmt: Statement = null;
		let expr: Expression = null;
		let is_default: bool = false;
		let guard: Expression = null;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					self.jj_consume_token()?;
					begin = self.token();
					if self.jj_2_63(3) {
						label = self.null_literal()?;
						labels = self.add(labels, label);
						self.jj_consume_token()?;
						self.jj_consume_token()?;
						is_default = true;
					} else if self.jj_2_64(2147483647) {
						label = self.pattern_expression()?;
						labels = self.add(labels, label);
						match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
							 =>  {
								{
									self.jj_consume_token()?;
									guard = self.conditional_expression()?;
									break;
								}
							}
							_ =>  {
								self.jj_la1[149] = self.jj_gen;
								;
							}
						}
					} else {
						match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
								{
									label = self.conditional_expression()?;
									labels = self.add(labels, label);
									'label_51: while true {
										match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
											 =>  {
												{
													break;
												}
											}
											_ =>  {
												self.jj_la1[150] = self.jj_gen;
												break 'label_51;
											}
										}
										self.jj_consume_token()?;
										label = self.conditional_expression()?;
										labels = self.add(labels, label);
									}
									break;
								}
							}
							_ =>  {
								self.jj_la1[151] = self.jj_gen;
								self.jj_consume_token(-1)?;
								return Err(ParseException::new());
							}
						}
					}
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					begin = self.token();
					is_default = true;
					break;
				}
			}
			_ =>  {
				self.jj_la1[152] = self.jj_gen;
				self.jj_consume_token(-1)?;
				return Err(ParseException::new());
			}
		}
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					self.jj_consume_token()?;
					stmts = self.statements()?;
					ret = SwitchEntry::new(&self.range(begin, &self.token()), labels, STATEMENT_GROUP, stmts, is_default, guard);
					break;
				}
			}
			 =>  {
				{
					self.jj_consume_token()?;
					match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
						}
						 =>  {
							{
								stmt = self.switch_entry_expression()?;
								let r: TokenRange = self.range(begin, &self.token());
								stmts.add(stmt);
								ret = SwitchEntry::new(r, labels, EXPRESSION, stmts, is_default, guard);
								break;
							}
						}
						 =>  {
							{
								stmt = self.block()?;
								let r: TokenRange = self.range(begin, &self.token());
								stmts.add(stmt);
								ret = SwitchEntry::new(r, labels, BLOCK, stmts, is_default, guard);
								break;
							}
						}
						 =>  {
							{
								stmt = self.throw_statement()?;
								let r: TokenRange = self.range(begin, &self.token());
								stmts.add(stmt);
								ret = SwitchEntry::new(r, labels, THROWS_STATEMENT, stmts, is_default, guard);
								break;
							}
						}
						_ =>  {
							self.jj_la1[153] = self.jj_gen;
							self.jj_consume_token(-1)?;
							return Err(ParseException::new());
						}
					}
					break;
				}
			}
			_ =>  {
				self.jj_la1[154] = self.jj_gen;
				self.jj_consume_token(-1)?;
				return Err(ParseException::new());
			}
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn switch_entry_expression(&self) /* thrown(com.github.javaparser.ParseException | java.lang.AssertionError | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::expression_stmt::ExpressionStmt {
		let expr: Expression;
		expr = self.expression()?;
		self.jj_consume_token()?;
		let r: TokenRange = expr.get_token_range().orElse(null);
		if r != null {
			r = r.with_end(&self.token())?;
		}
		{
			if "" != null {
				return ExpressionStmt::new(r, expr);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn if_statement(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::if_stmt::IfStmt {
		let condition: Expression;
		let then_stmt: Statement;
		let else_stmt: Statement = null;
		let begin: JavaToken;
		self.jj_consume_token()?;
		begin = self.token();
		self.jj_consume_token()?;
		condition = self.expression()?;
		self.jj_consume_token()?;
		then_stmt = self.statement()?;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					self.jj_consume_token()?;
					else_stmt = self.statement()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[155] = self.jj_gen;
				;
			}
		}
		{
			if "" != null {
				return IfStmt::new(&self.range(begin, &self.token()), condition, then_stmt, else_stmt);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn while_statement(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::while_stmt::WhileStmt {
		let condition: Expression;
		let body: Statement;
		let begin: JavaToken;
		self.jj_consume_token()?;
		begin = self.token();
		self.jj_consume_token()?;
		condition = self.expression()?;
		self.jj_consume_token()?;
		body = self.statement()?;
		{
			if "" != null {
				return WhileStmt::new(&self.range(begin, &self.token()), condition, body);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn do_statement(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::do_stmt::DoStmt {
		let condition: Expression;
		let body: Statement;
		let begin: JavaToken;
		self.jj_consume_token()?;
		begin = self.token();
		body = self.statement()?;
		self.jj_consume_token()?;
		self.jj_consume_token()?;
		condition = self.expression()?;
		self.jj_consume_token()?;
		self.jj_consume_token()?;
		{
			if "" != null {
				return DoStmt::new(&self.range(begin, &self.token()), body, condition);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn for_statement(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::statement::Statement {
		let var_expr: VariableDeclarationExpr = null;
		let expr: Expression = null;
		let init: NodeList<Expression> = self.empty_node_list();
		let update: NodeList<Expression> = self.empty_node_list();
		let body: Statement;
		let begin: JavaToken;
		self.jj_consume_token()?;
		begin = self.token();
		self.jj_consume_token()?;
		if self.jj_2_65(2147483647) {
			var_expr = self.variable_declaration_expression()?;
			self.jj_consume_token()?;
			expr = self.expression()?;
		} else {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
					{
						match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
								{
									init = self.for_init()?;
									break;
								}
							}
							_ =>  {
								self.jj_la1[156] = self.jj_gen;
								;
							}
						}
						self.jj_consume_token()?;
						match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
								{
									expr = self.expression()?;
									break;
								}
							}
							_ =>  {
								self.jj_la1[157] = self.jj_gen;
								;
							}
						}
						self.jj_consume_token()?;
						match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
								{
									update = self.for_update()?;
									break;
								}
							}
							_ =>  {
								self.jj_la1[158] = self.jj_gen;
								;
							}
						}
						break;
					}
				}
				_ =>  {
					self.jj_la1[159] = self.jj_gen;
					self.jj_consume_token(-1)?;
					return Err(ParseException::new());
				}
			}
		}
		self.jj_consume_token()?;
		body = self.statement()?;
		if var_expr != null {
			{
				if "" != null {
					return ForEachStmt::new(&self.range(begin, &self.token()), var_expr, expr, body);
				}
	
			}
		}
		{
			if "" != null {
				return ForStmt::new(&self.range(begin, &self.token()), init, expr, update, body);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn for_init(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::node_list::NodeList {
		let ret: NodeList<Expression>;
		let expr: Expression;
		if self.jj_2_66(2147483647) {
			expr = self.variable_declaration_expression()?;
			ret = NodeList<Expression>::new();
			ret.add(expr);
		} else {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
					{
						ret = self.expression_list()?;
						break;
					}
				}
				_ =>  {
					self.jj_la1[160] = self.jj_gen;
					self.jj_consume_token(-1)?;
					return Err(ParseException::new());
				}
			}
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn expression_list(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::node_list::NodeList {
		let ret: NodeList<Expression> = NodeList<Expression>::new();
		let expr: Expression;
		expr = self.expression()?;
		ret.add(expr);
		'label_52: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[161] = self.jj_gen;
					break 'label_52;
				}
			}
			self.jj_consume_token()?;
			expr = self.expression()?;
			ret.add(expr);
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn for_update(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::node_list::NodeList {
		let ret: NodeList<Expression>;
		ret = self.expression_list()?;
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn break_statement(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::break_stmt::BreakStmt {
		let label: SimpleName = null;
		let begin: JavaToken;
		self.jj_consume_token()?;
		begin = self.token();
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
				{
					label = self.simple_name()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[162] = self.jj_gen;
				;
			}
		}
		self.jj_consume_token()?;
		{
			if "" != null {
				return BreakStmt::new(&self.range(begin, &self.token()), label);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn yield_statement(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::yield_stmt::YieldStmt {
		let value: Expression;
		let begin: JavaToken;
		self.jj_consume_token()?;
		begin = self.token();
		value = self.expression()?;
		self.jj_consume_token()?;
		{
			if "" != null {
				return YieldStmt::new(&self.range(begin, &self.token()), value);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn continue_statement(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::continue_stmt::ContinueStmt {
		let label: SimpleName = null;
		let begin: JavaToken;
		self.jj_consume_token()?;
		begin = self.token();
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
				{
					label = self.simple_name()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[163] = self.jj_gen;
				;
			}
		}
		self.jj_consume_token()?;
		{
			if "" != null {
				return ContinueStmt::new(&self.range(begin, &self.token()), label);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn return_statement(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::return_stmt::ReturnStmt {
		let expr: Expression = null;
		let begin: JavaToken;
		self.jj_consume_token()?;
		begin = self.token();
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
				{
					expr = self.expression()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[164] = self.jj_gen;
				;
			}
		}
		self.jj_consume_token()?;
		{
			if "" != null {
				return ReturnStmt::new(&self.range(begin, &self.token()), expr);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn throw_statement(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::throw_stmt::ThrowStmt {
		let expr: Expression;
		let begin: JavaToken;
		self.jj_consume_token()?;
		begin = self.token();
		expr = self.expression()?;
		self.jj_consume_token()?;
		{
			if "" != null {
				return ThrowStmt::new(&self.range(begin, &self.token()), expr);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn synchronized_statement(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::synchronized_stmt::SynchronizedStmt {
		let expr: Expression;
		let body: BlockStmt;
		let begin: JavaToken;
		self.jj_consume_token()?;
		begin = self.token();
		self.jj_consume_token()?;
		expr = self.expression()?;
		self.jj_consume_token()?;
		body = self.block()?;
		{
			if "" != null {
				return SynchronizedStmt::new(&self.range(begin, &self.token()), expr, body);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn try_statement(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.AssertionError | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::try_stmt::TryStmt {
		let resources: NodeList<Expression> = self.empty_node_list();
		let try_block: BlockStmt;
		let finally_block: BlockStmt = null;
		let catchs: NodeList<CatchClause> = self.empty_node_list();
		let catch_block: BlockStmt;
		let except_modifier: ModifierHolder;
		let exception_type: ReferenceType;
		let exception_types: NodeList<ReferenceType> = self.empty_node_list();
		let except_id: Pair<SimpleName, List<ArrayBracketPair>>;
		let begin: JavaToken;
		let catch_begin: JavaToken;
		let types_begin: JavaToken;
		let param_end: JavaToken;
		let type: Type;
		self.jj_consume_token()?;
		begin = self.token();
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					resources = self.resource_specification()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[165] = self.jj_gen;
				;
			}
		}
		try_block = self.block()?;
		'label_53: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[166] = self.jj_gen;
					break 'label_53;
				}
			}
			self.jj_consume_token()?;
			catch_begin = self.token();
			self.jj_consume_token()?;
			except_modifier = self.modifiers()?;
			types_begin = except_modifier.begin;
			exception_type = self.reference_type(&self.empty_node_list())?;
			exception_types.add(exception_type);
			types_begin = self.or_if_invalid(types_begin, &self.token());
			'label_54: while true {
				match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
					 =>  {
						{
							break;
						}
					}
					_ =>  {
						self.jj_la1[167] = self.jj_gen;
						break 'label_54;
					}
				}
				self.jj_consume_token()?;
				exception_type = self.annotated_reference_type()?;
				exception_types.add(exception_type);
			}
			except_id = self.variable_declarator_id()?;
			param_end = self.token();
			self.jj_consume_token()?;
			catch_block = self.block()?;
			if exception_types.size() > 1 {
				type = UnionType::new(&self.range(&exception_types.get(0), &exception_types.get(exception_types.size() - 1)), exception_types);
			} else {
				type = exception_types.get(0) as Type;
			}
			let catch_type: Parameter = Parameter::new(&self.range(types_begin, param_end), except_modifier.modifiers, except_modifier.annotations, type, false, &self.empty_node_list(), except_id.a);
			catchs = self.add(catchs, CatchClause::new(&self.range(catch_begin, &self.token()), catch_type, catch_block));
			exception_types = self.empty_node_list();
		}
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					self.jj_consume_token()?;
					finally_block = self.block()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[168] = self.jj_gen;
				;
			}
		}
		{
			if "" != null {
				return TryStmt::new(&self.range(begin, &self.token()), resources, try_block, catchs, finally_block);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn resource_specification(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::node_list::NodeList {
		let variables: NodeList<Expression>;
		self.jj_consume_token()?;
		variables = self.resources()?;
		if self.jj_2_67(2) {
			self.jj_consume_token()?;
		} else {
			;
		}
		self.jj_consume_token()?;
		{
			if "" != null {
				return variables;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn resources(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::node_list::NodeList {
		let expressions: NodeList<Expression> = NodeList<Expression>::new();
		let expr: Expression;
		expr = self.resource()?;
		expressions.add(expr);
		'label_55: while true {
			if self.jj_2_68(2) {
			} else {
				break 'label_55;
			}
			self.jj_consume_token()?;
			expr = self.resource()?;
			expressions.add(expr);
		}
		{
			if "" != null {
				return expressions;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn resource(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let expr: Expression;
		if self.jj_2_69(2147483647) {
			/* this is a bit more lenient than we need to be, e.g. allowing access modifiers like private*/ 
			expr = self.variable_declaration_expression()?;
		} else {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
					{
						expr = self.primary_expression()?;
						break;
					}
				}
				_ =>  {
					self.jj_la1[169] = self.jj_gen;
					self.jj_consume_token(-1)?;
					return Err(ParseException::new());
				}
			}
		}
		{
			if "" != null {
				return expr;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn runsignedshift(&self) /* thrown(com.github.javaparser.ParseException) */ {
		if self.get_token(1).kind ==  && self.get_token(1).realKind ==  {
		} else {
			self.jj_consume_token(-1)?;
			return Err(ParseException::new());
		}
		self.jj_consume_token()?;
		self.jj_consume_token()?;
		self.jj_consume_token()?;
	}

	pub fn rsignedshift(&self) /* thrown(com.github.javaparser.ParseException) */ {
		if self.get_token(1).kind ==  && self.get_token(1).realKind ==  {
		} else {
			self.jj_consume_token(-1)?;
			return Err(ParseException::new());
		}
		self.jj_consume_token()?;
		self.jj_consume_token()?;
	}

	pub fn annotations(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::node_list::NodeList {
		let annotations: NodeList<AnnotationExpr> = NodeList<AnnotationExpr>::new();
		let annotation: AnnotationExpr;
		'label_56: while true {
			if self.jj_2_70(2147483647) {
			} else {
				break 'label_56;
			}
			annotation = self.annotation()?;
			annotations = self.add(annotations, annotation);
		}
		{
			if "" != null {
				return annotations;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn annotation(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::annotation_expr::AnnotationExpr {
		let ret: AnnotationExpr;
		let name: Name;
		let pairs: NodeList<MemberValuePair> = self.empty_node_list();
		let begin: JavaToken;
		let member_val: Expression;
		self.jj_consume_token()?;
		begin = self.token();
		name = self.name()?;
		if self.jj_2_71(2147483647) {
			self.jj_consume_token()?;
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
					{
						pairs = self.member_value_pairs()?;
						break;
					}
				}
				_ =>  {
					self.jj_la1[170] = self.jj_gen;
					;
				}
			}
			self.jj_consume_token()?;
			ret = NormalAnnotationExpr::new(&self.range(begin, &self.token()), name, pairs);
		} else if self.jj_2_72(2147483647) {
			self.jj_consume_token()?;
			member_val = self.member_value()?;
			self.jj_consume_token()?;
			ret = SingleMemberAnnotationExpr::new(&self.range(begin, &self.token()), name, member_val);
		} else {
			ret = MarkerAnnotationExpr::new(&self.range(begin, &self.token()), name);
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn member_value_pairs(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::node_list::NodeList {
		let ret: NodeList<MemberValuePair> = NodeList<MemberValuePair>::new();
		let pair: MemberValuePair;
		pair = self.member_value_pair()?;
		ret.add(pair);
		'label_57: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[171] = self.jj_gen;
					break 'label_57;
				}
			}
			self.jj_consume_token()?;
			pair = self.member_value_pair()?;
			ret.add(pair);
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn member_value_pair(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::member_value_pair::MemberValuePair {
		let name: SimpleName;
		let value: Expression;
		let begin: JavaToken;
		name = self.simple_name()?;
		begin = self.token();
		self.jj_consume_token()?;
		value = self.member_value()?;
		{
			if "" != null {
				return MemberValuePair::new(&self.range(begin, &self.token()), name, value);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn member_value(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		if self.jj_2_73(2147483647) {
			ret = self.annotation()?;
		} else {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						ret = self.member_value_array_initializer()?;
						break;
					}
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
					{
						ret = self.conditional_expression()?;
						break;
					}
				}
				_ =>  {
					self.jj_la1[172] = self.jj_gen;
					self.jj_consume_token(-1)?;
					return Err(ParseException::new());
				}
			}
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn member_value_array_initializer(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: NodeList<Expression> = self.empty_node_list();
		let member: Expression;
		let begin: JavaToken;
		self.jj_consume_token()?;
		begin = self.token();
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
				{
					member = self.member_value()?;
					ret.add(member);
					'label_58: while true {
						if self.jj_2_74(2) {
						} else {
							break 'label_58;
						}
						self.jj_consume_token()?;
						member = self.member_value()?;
						ret.add(member);
					}
					break;
				}
			}
			_ =>  {
				self.jj_la1[173] = self.jj_gen;
				;
			}
		}
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					self.jj_consume_token()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[174] = self.jj_gen;
				;
			}
		}
		self.jj_consume_token()?;
		{
			if "" != null {
				return ArrayInitializerExpr::new(&self.range(begin, &self.token()), ret);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn annotation_type_declaration(&self, modifier: &com::github::javaparser::modifier_holder::ModifierHolder) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::annotation_declaration::AnnotationDeclaration {
		let name: SimpleName;
		let members: NodeList<BodyDeclaration<?>> = self.empty_node_list();
		let begin: JavaToken = modifier.begin;
		self.jj_consume_token()?;
		begin = self.or_if_invalid(begin, &self.token());
		self.jj_consume_token()?;
		name = self.simple_name()?;
		members = self.annotation_type_body()?;
		{
			if "" != null {
				return AnnotationDeclaration::new(&self.range(begin, &self.token()), modifier.modifiers, modifier.annotations, name, members);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn annotation_type_body(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::node_list::NodeList {
		let ret: NodeList<BodyDeclaration<?>> = self.empty_node_list();
		let member: BodyDeclaration;
		self.jj_consume_token()?;
		'label_59: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[175] = self.jj_gen;
					break 'label_59;
				}
			}
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
					{
						member = self.annotation_body_declaration()?;
						ret = self.add_when_not_null(ret, member);
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						break;
					}
				}
				_ =>  {
					self.jj_la1[176] = self.jj_gen;
					self.jj_consume_token(-1)?;
					return Err(ParseException::new());
				}
			}
		}
		self.jj_consume_token()?;
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn annotation_body_declaration(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::body_declaration::BodyDeclaration {
		let modifier: ModifierHolder;
		let ret: BodyDeclaration;
		modifier = self.modifiers()?;
		if self.jj_2_75(2147483647) {
			ret = self.record_declaration(modifier)?;
		} else if self.jj_2_76(2147483647) {
			ret = self.annotation_type_member_declaration(modifier)?;
		} else {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
					{
						ret = self.class_or_interface_declaration(modifier)?;
						break;
					}
				}
				_ =>  {
					self.jj_la1[177] = self.jj_gen;
					if self.jj_2_77(2147483647) {
						ret = self.enum_declaration(modifier)?;
					} else {
						match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
							 =>  {
								{
									ret = self.annotation_type_declaration(modifier)?;
									break;
								}
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
							}
							 =>  {
								{
									ret = self.field_declaration(modifier)?;
									break;
								}
							}
							_ =>  {
								self.jj_la1[178] = self.jj_gen;
								self.jj_consume_token(-1)?;
								return Err(ParseException::new());
							}
						}
					}
				}
			}
		}
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn annotation_type_member_declaration(&mut self, modifier: &com::github::javaparser::modifier_holder::ModifierHolder) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::annotation_member_declaration::AnnotationMemberDeclaration {
		let type: Type;
		let name: SimpleName;
		let default_val: Expression = null;
		// TODO/FIXME: Consider missing `[Dims] (present in the JLS, but not the JavaParser grammar)
		// TODO/FIXME: {AnnotationTypeElementModifier} UnannType Identifier ( ) [Dims] [DefaultValue] ;
		type = self.type(&self.empty_node_list())?;
		name = self.simple_name()?;
		self.jj_consume_token()?;
		self.jj_consume_token()?;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					default_val = self.default_value()?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[179] = self.jj_gen;
				;
			}
		}
		self.jj_consume_token()?;
		let begin: JavaToken = self.or_if_invalid(modifier.begin, type);
		{
			if "" != null {
				return AnnotationMemberDeclaration::new(&self.range(begin, &self.token()), modifier.modifiers, modifier.annotations, type, name, default_val);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn default_value(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		self.jj_consume_token()?;
		ret = self.member_value()?;
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn module_directive(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::modules::module_directive::ModuleDirective {
		let modifiers: ModifierHolder;
		let name: Name;
		let tmp_name: Name;
		let names: NodeList<Name> = self.empty_node_list();
		let begin: JavaToken;
		let directive: ModuleDirective;
		let transitive_exceptional_token: JavaToken;
		if self.jj_2_78(2147483647) {
			self.jj_consume_token()?;
			begin = self.token();
			self.jj_consume_token()?;
			transitive_exceptional_token = self.token();
			self.set_token_kind();
			self.jj_consume_token()?;
			directive = ModuleRequiresDirective::new(&self.range(begin, &self.token()), NodeList<Modifier>::new(), Name::new(&self.range(transitive_exceptional_token, transitive_exceptional_token), null, &transitive_exceptional_token.get_text()));
		} else {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
					{
						self.jj_consume_token()?;
						begin = self.token();
						modifiers = self.modifiers()?;
						name = self.name()?;
						self.jj_consume_token()?;
						directive = ModuleRequiresDirective::new(&self.range(begin, &self.token()), modifiers.modifiers, name);
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						begin = self.token();
						name = self.name()?;
						match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
							 =>  {
								{
									self.jj_consume_token()?;
									tmp_name = self.name()?;
									names.add(tmp_name);
									'label_60: while true {
										match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
											 =>  {
												{
													break;
												}
											}
											_ =>  {
												self.jj_la1[180] = self.jj_gen;
												break 'label_60;
											}
										}
										self.jj_consume_token()?;
										tmp_name = self.name()?;
										names.add(tmp_name);
									}
									break;
								}
							}
							_ =>  {
								self.jj_la1[181] = self.jj_gen;
								;
							}
						}
						self.jj_consume_token()?;
						directive = ModuleExportsDirective::new(&self.range(begin, &self.token()), name, names);
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						begin = self.token();
						name = self.name()?;
						match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
							 =>  {
								{
									self.jj_consume_token()?;
									tmp_name = self.name()?;
									names.add(tmp_name);
									'label_61: while true {
										match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
											 =>  {
												{
													break;
												}
											}
											_ =>  {
												self.jj_la1[182] = self.jj_gen;
												break 'label_61;
											}
										}
										self.jj_consume_token()?;
										tmp_name = self.name()?;
										names.add(tmp_name);
									}
									break;
								}
							}
							_ =>  {
								self.jj_la1[183] = self.jj_gen;
								;
							}
						}
						self.jj_consume_token()?;
						directive = ModuleOpensDirective::new(&self.range(begin, &self.token()), name, names);
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						begin = self.token();
						name = self.name()?;
						self.jj_consume_token()?;
						directive = ModuleUsesDirective::new(&self.range(begin, &self.token()), name);
						break;
					}
				}
				 =>  {
					{
						self.jj_consume_token()?;
						begin = self.token();
						name = self.name()?;
						self.jj_consume_token()?;
						tmp_name = self.name()?;
						names.add(tmp_name);
						'label_62: while true {
							match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
								 =>  {
									{
										break;
									}
								}
								_ =>  {
									self.jj_la1[184] = self.jj_gen;
									break 'label_62;
								}
							}
							self.jj_consume_token()?;
							tmp_name = self.name()?;
							names.add(tmp_name);
						}
						self.jj_consume_token()?;
						directive = ModuleProvidesDirective::new(&self.range(begin, &self.token()), name, names);
						break;
					}
				}
				_ =>  {
					self.jj_la1[185] = self.jj_gen;
					self.jj_consume_token(-1)?;
					return Err(ParseException::new());
				}
			}
		}
		{
			if "" != null {
				return directive;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn module_declaration(&mut self, modifier: &com::github::javaparser::modifier_holder::ModifierHolder) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration {
		let directives: NodeList<ModuleDirective> = NodeList<ModuleDirective>::new();
		let open: bool = false;
		let directive: ModuleDirective;
		let name: Name;
		let begin: JavaToken = modifier.begin;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
				{
					self.jj_consume_token()?;
					open = true;
					begin = self.or_if_invalid(begin, &self.token());
					break;
				}
			}
			_ =>  {
				self.jj_la1[186] = self.jj_gen;
				;
			}
		}
		self.jj_consume_token()?;
		begin = self.or_if_invalid(begin, &self.token());
		name = self.name()?;
		self.jj_consume_token()?;
		'label_63: while true {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
					{
						break;
					}
				}
				_ =>  {
					self.jj_la1[187] = self.jj_gen;
					break 'label_63;
				}
			}
			directive = self.module_directive()?;
			directives = self.add(directives, directive);
		}
		self.jj_consume_token()?;
		{
			if "" != null {
				return ModuleDeclaration::new(&self.range(begin, &self.token()), modifier.annotations, name, open, directives);
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn block_parse_start(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::block_stmt::BlockStmt {
		let ret: BlockStmt;
		ret = self.block()?;
		self.jj_consume_token(0)?;
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn block_statement_parse_start(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::statement::Statement {
		let ret: Statement;
		if self.jj_2_79(3) {
			ret = self.block_statement()?;
		} else {
			match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
				}
				 =>  {
					{
						ret = self.explicit_constructor_invocation()?;
						break;
					}
				}
				_ =>  {
					self.jj_la1[188] = self.jj_gen;
					self.jj_consume_token(-1)?;
					return Err(ParseException::new());
				}
			}
		}
		self.jj_consume_token(0)?;
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn import_declaration_parse_start(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::import_declaration::ImportDeclaration {
		let ret: ImportDeclaration;
		ret = self.import_declaration()?;
		self.jj_consume_token(0)?;
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn expression_parse_start(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::expression::Expression {
		let ret: Expression;
		ret = self.expression()?;
		self.jj_consume_token(0)?;
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn annotation_parse_start(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::annotation_expr::AnnotationExpr {
		let ret: AnnotationExpr;
		ret = self.annotation()?;
		self.jj_consume_token(0)?;
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn annotation_body_declaration_parse_start(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::body_declaration::BodyDeclaration {
		let ret: BodyDeclaration<?>;
		ret = self.annotation_body_declaration()?;
		self.jj_consume_token(0)?;
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn class_or_interface_body_declaration_parse_start(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::body_declaration::BodyDeclaration {
		let ret: BodyDeclaration<?>;
		ret = self.class_or_interface_body_declaration()?;
		self.jj_consume_token(0)?;
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn class_or_interface_type_parse_start(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType {
		let ret: ClassOrInterfaceType;
		ret = self.annotated_class_or_interface_type()?;
		self.jj_consume_token(0)?;
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn result_type_parse_start(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::type::type::Type {
		let annotations: NodeList<AnnotationExpr>;
		let ret: Type;
		annotations = self.annotations()?;
		ret = self.result_type(annotations)?;
		self.jj_consume_token(0)?;
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn variable_declaration_expression_parse_start(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr {
		let ret: VariableDeclarationExpr;
		ret = self.variable_declaration_expression()?;
		self.jj_consume_token(0)?;
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn explicit_constructor_invocation_parse_start(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::explicit_constructor_invocation_stmt::ExplicitConstructorInvocationStmt {
		let ret: ExplicitConstructorInvocationStmt;
		ret = self.explicit_constructor_invocation()?;
		self.jj_consume_token(0)?;
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn name_parse_start(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::name::Name {
		let ret: Name;
		ret = self.name()?;
		self.jj_consume_token(0)?;
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn simple_name_parse_start(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::simple_name::SimpleName {
		let ret: SimpleName;
		ret = self.simple_name()?;
		self.jj_consume_token(0)?;
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn parameter_parse_start(&self) /* thrown(com.github.javaparser.ParseException | java.lang.AssertionError | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::parameter::Parameter {
		let ret: Parameter;
		ret = self.parameter()?;
		self.jj_consume_token(0)?;
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn package_declaration_parse_start(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::package_declaration::PackageDeclaration {
		let ret: PackageDeclaration;
		ret = self.package_declaration()?;
		self.jj_consume_token(0)?;
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn type_declaration_parse_start(&mut self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::type_declaration::TypeDeclaration {
		let ret: TypeDeclaration<?>;
		let modifier: ModifierHolder;
		modifier = self.modifiers()?;
		match  if self.jj_ntk == -1 { self.jj_ntk_f() } else { self.jj_ntk } {
			 =>  {
			}
			 =>  {
				{
					ret = self.class_or_interface_declaration(modifier)?;
					break;
				}
			}
			 =>  {
				{
					ret = self.enum_declaration(modifier)?;
					break;
				}
			}
			 =>  {
				{
					ret = self.annotation_type_declaration(modifier)?;
					break;
				}
			}
			_ =>  {
				self.jj_la1[189] = self.jj_gen;
				self.jj_consume_token(-1)?;
				return Err(ParseException::new());
			}
		}
		self.jj_consume_token(0)?;
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn module_declaration_parse_start(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration {
		let ret: ModuleDeclaration;
		let modifiers: ModifierHolder;
		modifiers = self.modifiers()?;
		ret = self.module_declaration(modifiers)?;
		self.jj_consume_token(0)?;
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn module_directive_parse_start(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::modules::module_directive::ModuleDirective {
		let ret: ModuleDirective;
		ret = self.module_directive()?;
		self.jj_consume_token(0)?;
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn type_parameter_parse_start(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::type::type_parameter::TypeParameter {
		let ret: TypeParameter;
		let annotations: NodeList<AnnotationExpr>;
		annotations = self.annotations()?;
		ret = self.type_parameter(annotations)?;
		self.jj_consume_token(0)?;
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	pub fn method_declaration_parse_start(&self) /* thrown(com.github.javaparser.ParseException | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::method_declaration::MethodDeclaration {
		let ret: MethodDeclaration;
		let modifier: ModifierHolder;
		modifier = self.modifiers()?;
		ret = self.method_declaration(modifier)?;
		self.jj_consume_token(0)?;
		{
			if "" != null {
				return ret;
			}
	
		}
		return Err(IllegalStateException::new("Missing return statement in function"));
	}

	fn jj_2_1(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_1());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(0, xla);
	
	}

	fn jj_2_2(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_2());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(1, xla);
	
	}

	fn jj_2_3(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_3());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(2, xla);
	
	}

	fn jj_2_4(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_4());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(3, xla);
	
	}

	fn jj_2_5(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_5());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(4, xla);
	
	}

	fn jj_2_6(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_6());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(5, xla);
	
	}

	fn jj_2_7(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_7());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(6, xla);
	
	}

	fn jj_2_8(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_8());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(7, xla);
	
	}

	fn jj_2_9(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_9());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(8, xla);
	
	}

	fn jj_2_10(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_10());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(9, xla);
	
	}

	fn jj_2_11(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_11());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(10, xla);
	
	}

	fn jj_2_12(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_12());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(11, xla);
	
	}

	fn jj_2_13(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_13());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(12, xla);
	
	}

	fn jj_2_14(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_14());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(13, xla);
	
	}

	fn jj_2_15(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_15());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(14, xla);
	
	}

	fn jj_2_16(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_16());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(15, xla);
	
	}

	fn jj_2_17(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_17());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(16, xla);
	
	}

	fn jj_2_18(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_18());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(17, xla);
	
	}

	fn jj_2_19(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_19());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(18, xla);
	
	}

	fn jj_2_20(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_20());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(19, xla);
	
	}

	fn jj_2_21(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_21());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(20, xla);
	
	}

	fn jj_2_22(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_22());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(21, xla);
	
	}

	fn jj_2_23(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_23());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(22, xla);
	
	}

	fn jj_2_24(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_24());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(23, xla);
	
	}

	fn jj_2_25(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_25());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(24, xla);
	
	}

	fn jj_2_26(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_26());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(25, xla);
	
	}

	fn jj_2_27(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_27());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(26, xla);
	
	}

	fn jj_2_28(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_28());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(27, xla);
	
	}

	fn jj_2_29(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_29());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(28, xla);
	
	}

	fn jj_2_30(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_30());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(29, xla);
	
	}

	fn jj_2_31(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_31());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(30, xla);
	
	}

	fn jj_2_32(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_32());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(31, xla);
	
	}

	fn jj_2_33(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_33());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(32, xla);
	
	}

	fn jj_2_34(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_34());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(33, xla);
	
	}

	fn jj_2_35(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_35());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(34, xla);
	
	}

	fn jj_2_36(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_36());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(35, xla);
	
	}

	fn jj_2_37(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_37());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(36, xla);
	
	}

	fn jj_2_38(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_38());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(37, xla);
	
	}

	fn jj_2_39(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_39());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(38, xla);
	
	}

	fn jj_2_40(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_40());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(39, xla);
	
	}

	fn jj_2_41(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_41());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(40, xla);
	
	}

	fn jj_2_42(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_42());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(41, xla);
	
	}

	fn jj_2_43(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_43());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(42, xla);
	
	}

	fn jj_2_44(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_44());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(43, xla);
	
	}

	fn jj_2_45(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_45());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(44, xla);
	
	}

	fn jj_2_46(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_46());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(45, xla);
	
	}

	fn jj_2_47(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_47());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(46, xla);
	
	}

	fn jj_2_48(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_48());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(47, xla);
	
	}

	fn jj_2_49(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_49());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(48, xla);
	
	}

	fn jj_2_50(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_50());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(49, xla);
	
	}

	fn jj_2_51(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_51());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(50, xla);
	
	}

	fn jj_2_52(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_52());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(51, xla);
	
	}

	fn jj_2_53(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_53());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(52, xla);
	
	}

	fn jj_2_54(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_54());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(53, xla);
	
	}

	fn jj_2_55(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_55());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(54, xla);
	
	}

	fn jj_2_56(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_56());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(55, xla);
	
	}

	fn jj_2_57(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_57());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(56, xla);
	
	}

	fn jj_2_58(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_58());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(57, xla);
	
	}

	fn jj_2_59(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_59());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(58, xla);
	
	}

	fn jj_2_60(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_60());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(59, xla);
	
	}

	fn jj_2_61(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_61());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(60, xla);
	
	}

	fn jj_2_62(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_62());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(61, xla);
	
	}

	fn jj_2_63(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_63());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(62, xla);
	
	}

	fn jj_2_64(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_64());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(63, xla);
	
	}

	fn jj_2_65(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_65());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(64, xla);
	
	}

	fn jj_2_66(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_66());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(65, xla);
	
	}

	fn jj_2_67(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_67());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(66, xla);
	
	}

	fn jj_2_68(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_68());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(67, xla);
	
	}

	fn jj_2_69(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_69());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(68, xla);
	
	}

	fn jj_2_70(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_70());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(69, xla);
	
	}

	fn jj_2_71(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_71());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(70, xla);
	
	}

	fn jj_2_72(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_72());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(71, xla);
	
	}

	fn jj_2_73(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_73());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(72, xla);
	
	}

	fn jj_2_74(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_74());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(73, xla);
	
	}

	fn jj_2_75(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_75());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(74, xla);
	
	}

	fn jj_2_76(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_76());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(75, xla);
	
	}

	fn jj_2_77(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_77());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(76, xla);
	
	}

	fn jj_2_78(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_78());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(77, xla);
	
	}

	fn jj_2_79(&mut self, xla: i32) -> bool {
		self.jj_la = xla;
		self.jj_scanpos = self.token;
		self.jj_lastpos = self.token;
		let r0 = 'try0: {
			return (!self.jj_3_79());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ LookaheadSuccess) => {
				return true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.jj_save(78, xla);
	
	}

	fn jj_3_r_191(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_299(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_19() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_356() {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_19(&self) -> bool {
		if self.jj_3_r_89() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_468(&mut self) -> bool {
		if self.jj_3_r_105() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_515() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_356(&mut self) -> bool {
		if self.jj_3_r_128() {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_383() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_384() {
				self.jj_scanpos = xsp;
				if self.jj_3_r_385() {
					self.jj_scanpos = xsp;
					if self.jj_3_r_386() {
						self.jj_scanpos = xsp;
						if self.jj_3_r_387() {
							self.jj_scanpos = xsp;
							if self.jj_3_r_388() {
								self.jj_scanpos = xsp;
								if self.jj_3_r_389() {
									return true;
								}
	
							}
						}
					}
				}
			}
		}
		return false;
	}

	fn jj_3_r_85(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_scan_token(75)? {
			self.jj_scanpos = xsp;
			if self.jj_scan_token(69)? {
				self.jj_scanpos = xsp;
				if self.jj_scan_token(70)? {
					self.jj_scanpos = xsp;
					if self.jj_scan_token(71)? {
						self.jj_scanpos = xsp;
						if self.jj_scan_token(72)? {
							self.jj_scanpos = xsp;
							if self.jj_scan_token(73)? {
								self.jj_scanpos = xsp;
								if self.jj_scan_token(74)? {
									self.jj_scanpos = xsp;
									if self.jj_scan_token(76)? {
										self.jj_scanpos = xsp;
										if self.jj_scan_token(77)? {
											self.jj_scanpos = xsp;
											if self.jj_scan_token(78)? {
												self.jj_scanpos = xsp;
												if self.jj_scan_token(26)? {
													self.jj_scanpos = xsp;
													if self.jj_scan_token(55)? {
														self.jj_scanpos = xsp;
														if self.jj_scan_token(68)? {
															self.jj_scanpos = xsp;
															if self.jj_scan_token(50)? {
																self.jj_scanpos = xsp;
																if self.jj_scan_token(46)? {
																	self.jj_scanpos = xsp;
																	if self.jj_scan_token(52)? {
																		self.jj_scanpos = xsp;
																		if self.jj_scan_token(79)? {
																			self.jj_scanpos = xsp;
																			if self.jj_scan_token(152)? {
																				self.jj_scanpos = xsp;
																				if self.jj_scan_token(12)? {
																					self.jj_scanpos = xsp;
																					if self.jj_scan_token(98)? {
																						return true;
																					}
	
																				}
																			}
																		}
																	}
																}
															}
														}
													}
												}
											}
										}
									}
								}
							}
						}
					}
				}
			}
		}
		return false;
	}

	fn jj_3_r_117(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_192() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_193() {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_515(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_105() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_383(&self) -> bool {
		if self.jj_3_r_230() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_384(&self) -> bool {
		if self.jj_3_r_138() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_192(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_237() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_238() {
				self.jj_scanpos = xsp;
				if self.jj_3_r_239() {
					return true;
				}
	
			}
		}
		return false;
	}

	fn jj_3_r_385(&self) -> bool {
		if self.jj_3_r_408() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_386(&self) -> bool {
		if self.jj_3_r_409() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_237(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_387(&self) -> bool {
		if self.jj_3_r_410() {
			return true;
		}
	
		return false;
	}

	fn jj_3_14(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_90() {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_91() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_3_r_92() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_238(&self) -> bool {
		if self.jj_3_r_267() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_388(&self) -> bool {
		if self.jj_3_r_65() {
			return true;
		}
	
		return false;
	}

	fn jj_3_15(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_239(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_297() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_3_r_90() {
			return true;
		}
	
		xsp = self.jj_scanpos;
		if self.jj_3_r_298() {
			self.jj_scanpos = xsp;
		}
	
		return false;
	}

	fn jj_3_r_389(&self) -> bool {
		if self.jj_3_r_411() {
			return true;
		}
	
		return false;
	}

	fn jj_3_16(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_297(&self) -> bool {
		if self.jj_3_r_103() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_298(&self) -> bool {
		if self.jj_3_r_243() {
			return true;
		}
	
		return false;
	}

	fn jj_3_17(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_93() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_3_r_85()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_513(&self) -> bool {
		if self.jj_3_r_468() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_93(&self) -> bool {
		if self.jj_3_r_143() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_193(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_105() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_18(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_87() {
			return true;
		}
	
		if self.jj_3_r_85()? {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_94() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		xsp = self.jj_scanpos;
		if self.jj_scan_token(108)? {
			self.jj_scanpos = xsp;
			if self.jj_scan_token(113)? {
				self.jj_scanpos = xsp;
				if self.jj_scan_token(107)? {
					return true;
				}
	
			}
		}
		return false;
	}

	fn jj_3_50(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_122() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_3_r_85()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_122(&self) -> bool {
		if self.jj_3_r_103() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_105(&mut self) -> bool {
		if self.jj_3_r_177() {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_241() {
			self.jj_scanpos = xsp;
		}
	
		return false;
	}

	fn jj_3_r_91(&self) -> bool {
		if self.jj_3_r_143() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_241(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_33() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_301() {
				self.jj_scanpos = xsp;
				if self.jj_3_r_302() {
					return true;
				}
	
			}
		}
		return false;
	}

	fn jj_3_r_65(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_87() {
			return true;
		}
	
		if self.jj_3_r_135() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_140() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_342(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_375() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_33(&self) -> bool {
		if self.jj_3_r_104() {
			return true;
		}
	
		if self.jj_3_r_105() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_375(&self) -> bool {
		if self.jj_3_r_90() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_140(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_135() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_94(&self) -> bool {
		if self.jj_3_r_146() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_260(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_318() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_319() {
				self.jj_scanpos = xsp;
				if self.jj_3_r_320() {
					self.jj_scanpos = xsp;
					if self.jj_3_r_321() {
						self.jj_scanpos = xsp;
						if self.jj_3_r_322() {
							self.jj_scanpos = xsp;
							if self.jj_3_r_323() {
								self.jj_scanpos = xsp;
								if self.jj_3_r_324() {
									self.jj_scanpos = xsp;
									if self.jj_3_r_325() {
										return true;
									}
	
								}
							}
						}
					}
				}
			}
		}
		return false;
	}

	fn jj_3_r_301(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_358() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_318(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_302(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_359() {
			self.jj_scanpos = xsp;
		}
	
		xsp = self.jj_scanpos;
		if self.jj_3_r_360() {
			self.jj_scanpos = xsp;
			if self.jj_scan_token(42)? {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_319(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_359(&self) -> bool {
		if self.jj_3_r_103() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_320(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_360(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_85()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_321(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_127(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_105() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_322(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_323(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_324(&self) -> bool {
		if self.jj_3_r_367() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_325(&self) -> bool {
		if self.jj_3_r_133() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_135(&mut self) -> bool {
		if self.jj_3_r_119() {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_206() {
			self.jj_scanpos = xsp;
		}
	
		return false;
	}

	fn jj_3_r_206(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_95() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_104(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_165() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_166() {
				self.jj_scanpos = xsp;
				if self.jj_3_r_167() {
					self.jj_scanpos = xsp;
					if self.jj_3_r_168() {
						self.jj_scanpos = xsp;
						if self.jj_3_r_169() {
							self.jj_scanpos = xsp;
							if self.jj_3_r_170() {
								self.jj_scanpos = xsp;
								if self.jj_3_r_171() {
									self.jj_scanpos = xsp;
									if self.jj_3_r_172() {
										self.jj_scanpos = xsp;
										if self.jj_3_r_173() {
											self.jj_scanpos = xsp;
											if self.jj_3_r_174() {
												self.jj_scanpos = xsp;
												if self.jj_3_r_175() {
													self.jj_scanpos = xsp;
													if self.jj_3_r_176() {
														return true;
													}
	
												}
											}
										}
									}
								}
							}
						}
					}
				}
			}
		}
		return false;
	}

	fn jj_3_r_343(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_376() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_367(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_400() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_401() {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_165(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_166(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_376(&self) -> bool {
		if self.jj_3_r_90() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_167(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_168(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_169(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_400(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_170(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_171(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_401(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_172(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_173(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_174(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_175(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_176(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_133(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_344(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_377() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_377(&self) -> bool {
		if self.jj_3_r_105() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_119(&mut self) -> bool {
		if self.jj_3_r_90() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_195() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_177(&mut self) -> bool {
		if self.jj_3_r_233() {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_300() {
			self.jj_scanpos = xsp;
		}
	
		return false;
	}

	fn jj_3_r_195(&self) -> bool {
		if self.jj_3_r_146() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_300(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_105() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_105() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_345(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_105() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_243(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_303() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_303(&self) -> bool {
		if self.jj_3_r_361() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_95(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_150() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_151() {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_346(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_105() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_148() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_150(&self) -> bool {
		if self.jj_3_r_220() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_361(&mut self) -> bool {
		if self.jj_3_r_105() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_393() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_151(&self) -> bool {
		if self.jj_3_r_105() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_233(&mut self) -> bool {
		if self.jj_3_r_290() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_357() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_393(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_105() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_357(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_290() {
			return true;
		}
	
		return false;
	}

	fn jj_3_1(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_220(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_394() {
			self.jj_scanpos = xsp;
		}
	
		xsp = self.jj_scanpos;
		if self.jj_scan_token(108)? {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_394(&mut self) -> bool {
		if self.jj_3_r_95() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_20() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_20(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_95() {
			return true;
		}
	
		return false;
	}

	fn jj_3_2(&self) -> bool {
		if self.jj_3_r_64() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_290(&mut self) -> bool {
		if self.jj_3_r_349() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_390() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_390(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_349() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_347(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_378() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_3_r_148() {
			return true;
		}
	
		xsp = self.jj_scanpos;
		if self.jj_3_r_502() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_503() {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_378(&self) -> bool {
		if self.jj_3_r_405() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_267(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_332() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_3_r_102() {
			return true;
		}
	
		xsp = self.jj_scanpos;
		if self.jj_3_r_333() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_334() {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_502(&mut self) -> bool {
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_510() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		xsp = self.jj_scanpos;
		if self.jj_3_r_511() {
			self.jj_scanpos = xsp;
		}
	
		return false;
	}

	fn jj_3_r_332(&self) -> bool {
		if self.jj_3_r_103() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_510(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_128() {
			return true;
		}
	
		if self.jj_3_r_215() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_514() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		if self.jj_3_r_119() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_148() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_333(&self) -> bool {
		if self.jj_3_r_115() {
			return true;
		}
	
		if self.jj_3_r_370() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_411(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_434() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_3_r_102() {
			return true;
		}
	
		if self.jj_3_r_120() {
			return true;
		}
	
		if self.jj_3_r_90() {
			return true;
		}
	
		if self.jj_3_r_92() {
			return true;
		}
	
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_453() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		xsp = self.jj_scanpos;
		if self.jj_3_r_454() {
			self.jj_scanpos = xsp;
		}
	
		xsp = self.jj_scanpos;
		if self.jj_3_r_455() {
			self.jj_scanpos = xsp;
			if self.jj_scan_token(107)? {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_434(&self) -> bool {
		if self.jj_3_r_143() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_334(&mut self) -> bool {
		if self.jj_3_r_162() {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_371() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_372() {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_349(&mut self) -> bool {
		if self.jj_3_r_379() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_412() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_453(&self) -> bool {
		if self.jj_3_r_146() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_514(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_292() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_371(&self) -> bool {
		if self.jj_3_r_370() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_454(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_292() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_467() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_412(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_379() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_372(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_243()? {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_51() {
			self.jj_scanpos = xsp;
		}
	
		return false;
	}

	fn jj_3_r_467(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_292() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_455(&self) -> bool {
		if self.jj_3_r_148() {
			return true;
		}
	
		return false;
	}

	fn jj_3_3(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_65()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_51(&self) -> bool {
		if self.jj_3_r_123() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_503(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_148() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_511(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_148() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_64(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_139() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_66() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_139(&self) -> bool {
		if self.jj_3_r_141() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_292(&self) -> bool {
		if self.jj_3_r_102() {
			return true;
		}
	
		if self.jj_3_r_215() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_379(&mut self) -> bool {
		if self.jj_3_r_406() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_435() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_121(&self) -> bool {
		if self.jj_3_r_102() {
			return true;
		}
	
		if self.jj_3_r_87() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_435(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_406() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_370(&mut self) -> bool {
		let xsp: Token;
		if self.jj_3_52() {
			return true;
		}
	
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_52() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		xsp = self.jj_scanpos;
		if self.jj_3_r_402() {
			self.jj_scanpos = xsp;
		}
	
		return false;
	}

	fn jj_3_52(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_102() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_124() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_405(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_438() {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_67() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_402(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_220()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_124(&self) -> bool {
		if self.jj_3_r_105() {
			return true;
		}
	
		return false;
	}

	fn jj_3_67(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_4(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_66() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_406(&mut self) -> bool {
		if self.jj_3_r_428() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_445() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_92(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_149() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_445(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_428() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_149(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_217() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_218() {
				return true;
			}
	
		}
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_219() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_217(&self) -> bool {
		if self.jj_3_r_96() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_438(&mut self) -> bool {
		if self.jj_3_r_134() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_68() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_68(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_134() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_218(&self) -> bool {
		if self.jj_3_r_118() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_219(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_118() {
			return true;
		}
	
		return false;
	}

	fn jj_3_21(&self) -> bool {
		if self.jj_3_r_96() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_231(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_53() {
			self.jj_scanpos = xsp;
			if self.jj_3_54() {
				self.jj_scanpos = xsp;
				if self.jj_3_55() {
					self.jj_scanpos = xsp;
					if self.jj_3_r_274() {
						self.jj_scanpos = xsp;
						if self.jj_3_r_275() {
							self.jj_scanpos = xsp;
							if self.jj_3_r_276() {
								self.jj_scanpos = xsp;
								if self.jj_3_r_277() {
									self.jj_scanpos = xsp;
									if self.jj_3_r_278() {
										self.jj_scanpos = xsp;
										if self.jj_3_r_279() {
											self.jj_scanpos = xsp;
											if self.jj_3_r_280() {
												self.jj_scanpos = xsp;
												if self.jj_3_r_281() {
													self.jj_scanpos = xsp;
													if self.jj_3_r_282() {
														self.jj_scanpos = xsp;
														if self.jj_3_r_283() {
															self.jj_scanpos = xsp;
															if self.jj_3_r_284() {
																self.jj_scanpos = xsp;
																if self.jj_3_r_285() {
																	self.jj_scanpos = xsp;
																	if self.jj_3_r_286() {
																		self.jj_scanpos = xsp;
																		if self.jj_3_r_287() {
																			return true;
																		}
	
																	}
																}
															}
														}
													}
												}
											}
										}
									}
								}
							}
						}
					}
				}
			}
		}
		return false;
	}

	fn jj_3_r_428(&mut self) -> bool {
		if self.jj_3_r_439() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_462() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_53(&self) -> bool {
		if self.jj_3_r_125() {
			return true;
		}
	
		return false;
	}

	fn jj_3_54(&self) -> bool {
		if self.jj_3_r_126() {
			return true;
		}
	
		return false;
	}

	fn jj_3_55(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_127()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_128(&mut self) -> bool {
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_5() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_274(&self) -> bool {
		if self.jj_3_r_148() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_462(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_471() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_472() {
				return true;
			}
	
		}
		if self.jj_3_r_439() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_275(&self) -> bool {
		if self.jj_3_r_335() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_276(&self) -> bool {
		if self.jj_3_r_336() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_277(&self) -> bool {
		if self.jj_3_r_337() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_278(&self) -> bool {
		if self.jj_3_r_338() {
			return true;
		}
	
		return false;
	}

	fn jj_3_5(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_67() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_68() {
				self.jj_scanpos = xsp;
				if self.jj_3_r_69() {
					self.jj_scanpos = xsp;
					if self.jj_3_r_70() {
						self.jj_scanpos = xsp;
						if self.jj_3_r_71() {
							self.jj_scanpos = xsp;
							if self.jj_3_r_72() {
								self.jj_scanpos = xsp;
								if self.jj_3_r_73() {
									self.jj_scanpos = xsp;
									if self.jj_3_r_74() {
										self.jj_scanpos = xsp;
										if self.jj_3_r_75() {
											self.jj_scanpos = xsp;
											if self.jj_3_r_76() {
												self.jj_scanpos = xsp;
												if self.jj_3_r_77() {
													self.jj_scanpos = xsp;
													if self.jj_3_r_78() {
														self.jj_scanpos = xsp;
														if self.jj_3_r_79() {
															self.jj_scanpos = xsp;
															if self.jj_3_r_80() {
																self.jj_scanpos = xsp;
																if self.jj_3_r_81() {
																	self.jj_scanpos = xsp;
																	if self.jj_3_r_82() {
																		return true;
																	}
	
																}
															}
														}
													}
												}
											}
										}
									}
								}
							}
						}
					}
				}
			}
		}
		return false;
	}

	fn jj_3_r_279(&self) -> bool {
		if self.jj_3_r_339() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_471(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_280(&self) -> bool {
		if self.jj_3_r_340() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_134(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_204() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_205() {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_281(&self) -> bool {
		if self.jj_3_r_341() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_472(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_282(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_342()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_283(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_343()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_284(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_344()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_67(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_285(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_345()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_204(&self) -> bool {
		if self.jj_3_r_130() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_286(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_346()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_330(&mut self) -> bool {
		if self.jj_3_r_118() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_368() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_68(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_287(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_347()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_69(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_368(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_118() {
			return true;
		}
	
		return false;
	}

	fn jj_3_69(&self) -> bool {
		if self.jj_3_r_128() {
			return true;
		}
	
		if self.jj_3_r_87() {
			return true;
		}
	
		if self.jj_3_r_135() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_70(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_108(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_178() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_179() {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_71(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_178(&self) -> bool {
		if self.jj_3_r_106() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_205(&self) -> bool {
		if self.jj_3_r_250() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_72(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_73(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_74(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_179(&self) -> bool {
		if self.jj_3_r_234() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_75(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_126(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_105() {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_199() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_76(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_111(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		self.jj_looking_ahead = true;
		self.jj_sem_l_a = self.get_token(1).kind ==  && self.get_token(1).realKind == ;
		self.jj_looking_ahead = false;
		if !self.jj_sem_l_a || self.jj_3_r_181() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_34(&self) -> bool {
		if self.jj_3_r_106() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_77(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_199(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_105() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_78(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_79(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_80(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_110(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		self.jj_looking_ahead = true;
		self.jj_sem_l_a = self.get_token(1).kind ==  && self.get_token(1).realKind == ;
		self.jj_looking_ahead = false;
		if !self.jj_sem_l_a || self.jj_3_r_180() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_81(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_181(&self) -> bool {
		return false;
	}

	fn jj_3_r_350(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_380() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_381() {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_82(&self) -> bool {
		if self.jj_3_r_141() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_380(&self) -> bool {
		if self.jj_3_r_107() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_180(&self) -> bool {
		return false;
	}

	fn jj_3_r_125(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_90() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_231() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_381(&self) -> bool {
		if self.jj_3_r_108() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_331(&mut self) -> bool {
		if self.jj_3_r_119() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_369() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_35(&self) -> bool {
		if self.jj_3_r_107() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_102(&mut self) -> bool {
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_163() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_369(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_119() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_163(&self) -> bool {
		if self.jj_3_r_141() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_106(&self) -> bool {
		if self.jj_3_r_128() {
			return true;
		}
	
		if self.jj_3_r_87() {
			return true;
		}
	
		if self.jj_3_r_90() {
			return true;
		}
	
		return false;
	}

	fn jj_3_70(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_148(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_216() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_118(&mut self) -> bool {
		if self.jj_3_r_128() {
			return true;
		}
	
		if self.jj_3_r_87() {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_194() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_3_r_119() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_230(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_272() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_273() {
				return true;
			}
	
		}
		if self.jj_3_r_90() {
			return true;
		}
	
		xsp = self.jj_scanpos;
		if self.jj_3_r_310() {
			self.jj_scanpos = xsp;
		}
	
		xsp = self.jj_scanpos;
		if self.jj_3_r_311() {
			self.jj_scanpos = xsp;
		}
	
		xsp = self.jj_scanpos;
		if self.jj_3_r_312() {
			self.jj_scanpos = xsp;
		}
	
		xsp = self.jj_scanpos;
		if self.jj_3_r_313() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_3_r_123() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_234(&self) -> bool {
		if self.jj_3_r_128() {
			return true;
		}
	
		if self.jj_3_r_215() {
			return true;
		}
	
		if self.jj_3_r_291() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_194(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_102() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_272(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_273(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_310(&self) -> bool {
		if self.jj_3_r_143() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_311(&self) -> bool {
		if self.jj_3_r_363() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_312(&self) -> bool {
		if self.jj_3_r_252() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_313(&self) -> bool {
		if self.jj_3_r_364() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_107(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_128() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_96(&self) -> bool {
		if self.jj_3_r_102() {
			return true;
		}
	
		if self.jj_3_r_87() {
			return true;
		}
	
		if self.jj_3_r_152() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_141(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_66() {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_254() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_255() {
				self.jj_scanpos = xsp;
				if self.jj_3_r_256() {
					return true;
				}
	
			}
		}
		return false;
	}

	fn jj_3_r_291(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_350() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_351() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_99(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_155() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_156() {
				self.jj_scanpos = xsp;
				if self.jj_3_r_157() {
					self.jj_scanpos = xsp;
					if self.jj_3_r_158() {
						self.jj_scanpos = xsp;
						if self.jj_3_r_159() {
							self.jj_scanpos = xsp;
							if self.jj_3_r_160() {
								self.jj_scanpos = xsp;
								if self.jj_3_r_161() {
									return true;
								}
	
							}
						}
					}
				}
			}
		}
		return false;
	}

	fn jj_3_r_254(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_314() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_314(&self) -> bool {
		if self.jj_3_r_365() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_351(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_350() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_255(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_137() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_152(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_221() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_155(&self) -> bool {
		if self.jj_3_r_128() {
			return true;
		}
	
		if self.jj_3_r_230() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_221(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_66() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_71(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_136() {
			self.jj_scanpos = xsp;
			if self.jj_scan_token(102)? {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_256(&self) -> bool {
		return false;
	}

	fn jj_3_r_156(&self) -> bool {
		if self.jj_3_r_128() {
			return true;
		}
	
		if self.jj_3_r_138() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_136(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_85()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_72(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_22(&self) -> bool {
		if self.jj_3_r_66() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_138(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_90() {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_210() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_3_r_92()? {
			return true;
		}
	
		xsp = self.jj_scanpos;
		if self.jj_3_r_211() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_3_r_212() {
			return true;
		}
	
		return false;
	}

	fn jj_3_56(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_128() {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_scan_token(19)? {
			self.jj_scanpos = xsp;
			if self.jj_scan_token(39)? {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_157(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_127()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_210(&self) -> bool {
		if self.jj_3_r_143() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_158(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_126()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_57(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_128() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_90() {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_129() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_3_r_92()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_211(&self) -> bool {
		if self.jj_3_r_252() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_159(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_130() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_439(&mut self) -> bool {
		if self.jj_3_r_458() {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_470() {
			self.jj_scanpos = xsp;
		}
	
		return false;
	}

	fn jj_3_58(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_127()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_365(&mut self) -> bool {
		if self.jj_3_r_398() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_399() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_59(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_126()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_470(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_481() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_482() {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_399(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_398() {
			return true;
		}
	
		return false;
	}

	fn jj_3_60(&self) -> bool {
		if self.jj_3_r_130() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_160(&self) -> bool {
		if self.jj_3_r_131() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_161(&self) -> bool {
		if self.jj_3_r_231() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_481(&self) -> bool {
		if self.jj_3_r_108() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_482(&self) -> bool {
		if self.jj_3_r_292() {
			return true;
		}
	
		return false;
	}

	fn jj_3_61(&self) -> bool {
		if self.jj_3_r_131() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_410(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_433() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_3_r_90() {
			return true;
		}
	
		if self.jj_3_r_92()? {
			return true;
		}
	
		xsp = self.jj_scanpos;
		if self.jj_3_r_452() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_216() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_130(&mut self) -> bool {
		if self.jj_3_r_128() {
			return true;
		}
	
		if self.jj_3_r_87() {
			return true;
		}
	
		if self.jj_3_r_135() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_200() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_36(&self) -> bool {
		if self.jj_3_r_108() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_433(&self) -> bool {
		if self.jj_3_r_143() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_363(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_307() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_473() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_200(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_135() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_452(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_292() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_466() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_473(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_307() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_129(&self) -> bool {
		if self.jj_3_r_143() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_466(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_292() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_398(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_90() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_137() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_335(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_458(&mut self) -> bool {
		if self.jj_3_r_469() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_480() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_480(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_485() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_486() {
				self.jj_scanpos = xsp;
				if self.jj_3_r_487() {
					self.jj_scanpos = xsp;
					if self.jj_3_r_488() {
						return true;
					}
	
				}
			}
		}
		if self.jj_3_r_469() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_137(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_207() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_208() {
				self.jj_scanpos = xsp;
				if self.jj_3_r_209() {
					return true;
				}
	
			}
		}
		return false;
	}

	fn jj_3_r_252(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_307() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_308() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_485(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_308(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_307() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_358(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_391() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_392() {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_486(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_207(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_141()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_131(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_201() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_202() {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_487(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_208(&self) -> bool {
		if self.jj_3_r_251() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_488(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_391(&self) -> bool {
		if self.jj_3_r_105() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_209(&self) -> bool {
		if self.jj_3_r_177() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_201(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_242() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_243()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_392(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_148()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_242(&self) -> bool {
		if self.jj_3_r_103() {
			return true;
		}
	
		return false;
	}

	fn jj_3_73(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_202(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_244() {
			self.jj_scanpos = xsp;
		}
	
		xsp = self.jj_scanpos;
		if self.jj_3_r_245() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_243()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_24(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_98() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_244(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_97() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_98(&self) -> bool {
		if self.jj_3_r_103() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_245(&self) -> bool {
		if self.jj_3_r_103() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_336(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_62() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_373() {
				self.jj_scanpos = xsp;
				if self.jj_3_r_374() {
					return true;
				}
	
			}
		}
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_469(&mut self) -> bool {
		if self.jj_3_r_479() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_37() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_251(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_422() {
			self.jj_scanpos = xsp;
		}
	
		xsp = self.jj_scanpos;
		if self.jj_scan_token(108)? {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_62(&self) -> bool {
		if self.jj_3_r_132() {
			return true;
		}
	
		return false;
	}

	fn jj_3_37(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_109() {
			self.jj_scanpos = xsp;
			if self.jj_3_38() {
				self.jj_scanpos = xsp;
				if self.jj_3_39() {
					return true;
				}
	
			}
		}
		if self.jj_3_r_479() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_364(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_307() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_474() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_23(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_97() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_373(&self) -> bool {
		if self.jj_3_r_304() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_422(&mut self) -> bool {
		if self.jj_3_r_137() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_74() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_474(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_307() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_374(&mut self) -> bool {
		if self.jj_3_r_250() {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_404() {
			self.jj_scanpos = xsp;
		}
	
		return false;
	}

	fn jj_3_r_109(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_38(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_110()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_74(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_137() {
			return true;
		}
	
		return false;
	}

	fn jj_3_39(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_111()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_404(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_425() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_426() {
				self.jj_scanpos = xsp;
				if self.jj_3_r_427() {
					return true;
				}
	
			}
		}
		return false;
	}

	fn jj_3_r_425(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_426(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_427(&self) -> bool {
		if self.jj_3_r_104() {
			return true;
		}
	
		if self.jj_3_r_105() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_479(&mut self) -> bool {
		if self.jj_3_r_484() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_495() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_409(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_90() {
			return true;
		}
	
		if self.jj_3_r_449() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_495(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_498() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_499() {
				return true;
			}
	
		}
		if self.jj_3_r_484() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_216(&mut self) -> bool {
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_25() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_25(&self) -> bool {
		if self.jj_3_r_99() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_498(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_499(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_408(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_90() {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_446() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		xsp = self.jj_scanpos;
		if self.jj_3_r_447() {
			self.jj_scanpos = xsp;
		}
	
		xsp = self.jj_scanpos;
		if self.jj_scan_token(108)? {
			self.jj_scanpos = xsp;
		}
	
		xsp = self.jj_scanpos;
		if self.jj_3_r_448() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_446(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_252()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_447(&mut self) -> bool {
		if self.jj_3_r_83() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_6() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_337(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_105() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_500() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_6(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_83() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_448(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_463() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_484(&mut self) -> bool {
		if self.jj_3_r_203() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_497() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_89(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_147() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_3_r_148()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_449(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_464() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_147(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_463(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_477() {
			self.jj_scanpos = xsp;
			if self.jj_scan_token(107)? {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_477(&self) -> bool {
		if self.jj_3_r_299() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_500(&self) -> bool {
		if self.jj_3_r_407() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_497(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_505() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_506() {
				self.jj_scanpos = xsp;
				if self.jj_3_r_507() {
					return true;
				}
	
			}
		}
		if self.jj_3_r_203() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_464(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_478() {
			self.jj_scanpos = xsp;
			if self.jj_scan_token(107)? {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_478(&self) -> bool {
		if self.jj_3_r_483() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_505(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_506(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_507(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_87(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_144() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_145() {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_144(&self) -> bool {
		if self.jj_3_r_215() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_83(&mut self) -> bool {
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_142() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		if self.jj_3_r_90() {
			return true;
		}
	
		xsp = self.jj_scanpos;
		if self.jj_3_r_475() {
			self.jj_scanpos = xsp;
		}
	
		xsp = self.jj_scanpos;
		if self.jj_3_r_476() {
			self.jj_scanpos = xsp;
		}
	
		return false;
	}

	fn jj_3_r_145(&self) -> bool {
		if self.jj_3_r_115() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_142(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_141()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_475(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_243()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_476(&self) -> bool {
		if self.jj_3_r_123() {
			return true;
		}
	
		return false;
	}

	fn jj_3_26(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_100() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_101() {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_100(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_115() {
			return true;
		}
	
		if self.jj_3_r_102() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_355(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_105() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_382() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_203(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_246() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_247() {
				self.jj_scanpos = xsp;
				if self.jj_3_r_248() {
					self.jj_scanpos = xsp;
					if self.jj_3_r_249() {
						return true;
					}
	
				}
			}
		}
		return false;
	}

	fn jj_3_r_483(&mut self) -> bool {
		if self.jj_3_r_128() {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_489() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_490() {
				self.jj_scanpos = xsp;
				if self.jj_3_r_491() {
					self.jj_scanpos = xsp;
					if self.jj_3_r_492() {
						self.jj_scanpos = xsp;
						if self.jj_3_r_493() {
							self.jj_scanpos = xsp;
							if self.jj_3_r_494() {
								return true;
							}
	
						}
					}
				}
			}
		}
		return false;
	}

	fn jj_3_r_246(&self) -> bool {
		if self.jj_3_r_132() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_247(&self) -> bool {
		if self.jj_3_r_304() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_382(&self) -> bool {
		if self.jj_3_r_407() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_248(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_305() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_306() {
				return true;
			}
	
		}
		if self.jj_3_r_203() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_489(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_138()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_305(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_490(&self) -> bool {
		if self.jj_3_r_496() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_306(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_249(&self) -> bool {
		if self.jj_3_r_236() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_491(&self) -> bool {
		if self.jj_3_r_230() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_492(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_408()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_75(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_138()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_493(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_409()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_76(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_87() {
			return true;
		}
	
		if self.jj_3_r_85()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_494(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_65()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_215(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_258() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_259() {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_77(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_132(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_203() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_143(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_102() {
			return true;
		}
	
		if self.jj_3_r_213() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_214() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_258(&mut self) -> bool {
		if self.jj_3_r_115() {
			return true;
		}
	
		let xsp: Token;
		if self.jj_3_r_316() {
			return true;
		}
	
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_316() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_259(&mut self) -> bool {
		if self.jj_3_r_162() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_317() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_316(&self) -> bool {
		if self.jj_3_r_146() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_214(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_102() {
			return true;
		}
	
		if self.jj_3_r_213() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_101(&self) -> bool {
		if self.jj_3_r_162() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_317(&self) -> bool {
		if self.jj_3_r_146() {
			return true;
		}
	
		return false;
	}

	fn jj_3_27(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_102() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_304(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_203() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_407(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_429() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_430() {
				return true;
			}
	
		}
		xsp = self.jj_scanpos;
		if self.jj_3_r_431() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_432() {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_496(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_87() {
			return true;
		}
	
		if self.jj_3_r_90() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_504() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_28(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_102() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_429(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_63() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_440() {
				self.jj_scanpos = xsp;
				if self.jj_3_r_441() {
					return true;
				}
	
			}
		}
		return false;
	}

	fn jj_3_r_146(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_102() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_504(&self) -> bool {
		if self.jj_3_r_512() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_213(&mut self) -> bool {
		if self.jj_3_r_90() {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_257() {
			self.jj_scanpos = xsp;
		}
	
		return false;
	}

	fn jj_3_r_257(&self) -> bool {
		if self.jj_3_r_315() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_236(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_293() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_294() {
				self.jj_scanpos = xsp;
				if self.jj_3_r_295() {
					self.jj_scanpos = xsp;
					if self.jj_3_r_296() {
						return true;
					}
	
				}
			}
		}
		return false;
	}

	fn jj_3_63(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_133()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_293(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_352() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_353() {
				return true;
			}
	
		}
		if self.jj_3_r_203() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_352(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_440(&mut self) -> bool {
		if self.jj_3_r_108() {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_459() {
			self.jj_scanpos = xsp;
		}
	
		return false;
	}

	fn jj_3_r_294(&self) -> bool {
		if self.jj_3_r_112() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_353(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_295(&self) -> bool {
		if self.jj_3_r_354() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_459(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_177() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_296(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_355()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_307(&self) -> bool {
		if self.jj_3_r_102() {
			return true;
		}
	
		if self.jj_3_r_162() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_441(&mut self) -> bool {
		if self.jj_3_r_177() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_460() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_512(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_137() {
			return true;
		}
	
		return false;
	}

	fn jj_3_64(&self) -> bool {
		if self.jj_3_r_108() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_430(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_40(&self) -> bool {
		if self.jj_3_r_112() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_431(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_216() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_460(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_177() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_432(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_442() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_443() {
				self.jj_scanpos = xsp;
				if self.jj_3_r_444() {
					return true;
				}
	
			}
		}
		return false;
	}

	fn jj_3_r_315(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_307() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_366() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_162(&mut self) -> bool {
		if self.jj_3_r_90() {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_29() {
			self.jj_scanpos = xsp;
		}
	
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_30() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_442(&self) -> bool {
		if self.jj_3_r_461() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_354(&mut self) -> bool {
		if self.jj_3_r_250() {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_41() {
			self.jj_scanpos = xsp;
		}
	
		return false;
	}

	fn jj_3_29(&self) -> bool {
		if self.jj_3_r_103() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_366(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_307() {
			return true;
		}
	
		return false;
	}

	fn jj_3_41(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_113() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_114() {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_443(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_148()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_30(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_102() {
			return true;
		}
	
		if self.jj_3_r_90() {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_31() {
			self.jj_scanpos = xsp;
		}
	
		return false;
	}

	fn jj_3_31(&self) -> bool {
		if self.jj_3_r_103() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_113(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_444(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_345()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_114(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_103(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_164() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_78(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_461(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_105() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_112(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_102() {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_42() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_182() {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_164(&mut self) -> bool {
		if self.jj_3_r_232() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_271() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_123(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_198() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_271(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_232() {
			return true;
		}
	
		return false;
	}

	fn jj_3_42(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_115() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_203() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_198(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_240() {
			self.jj_scanpos = xsp;
			if self.jj_scan_token(107)? {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_240(&self) -> bool {
		if self.jj_3_r_299() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_182(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_215() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_235() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_236() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_235(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_292() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_232(&mut self) -> bool {
		if self.jj_3_r_102() {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_288() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_289() {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_212(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_253() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_288(&self) -> bool {
		if self.jj_3_r_87() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_289(&self) -> bool {
		if self.jj_3_r_348() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_338(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_105() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_231() {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_501() {
			self.jj_scanpos = xsp;
		}
	
		return false;
	}

	fn jj_3_r_253(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_309() {
			self.jj_scanpos = xsp;
			if self.jj_scan_token(107)? {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_309(&self) -> bool {
		if self.jj_3_r_362() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_501(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_231() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_348(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_403() {
			self.jj_scanpos = xsp;
		}
	
		return false;
	}

	fn jj_3_r_250(&mut self) -> bool {
		if self.jj_3_r_153() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_43() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_403(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_423() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_424() {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_423(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_102() {
			return true;
		}
	
		if self.jj_3_r_215() {
			return true;
		}
	
		return false;
	}

	fn jj_3_43(&self) -> bool {
		if self.jj_3_r_116() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_424(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_102() {
			return true;
		}
	
		if self.jj_3_r_215() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_339(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_105() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_231() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_97(&mut self) -> bool {
		if self.jj_3_r_153() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_154() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_154(&self) -> bool {
		if self.jj_3_r_117() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_362(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_13() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_395() {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_13(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_89()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_79(&self) -> bool {
		if self.jj_3_r_99() {
			return true;
		}
	
		return false;
	}

	fn jj_3_44(&self) -> bool {
		if self.jj_3_r_117() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_115(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_183() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_184() {
				self.jj_scanpos = xsp;
				if self.jj_3_r_185() {
					self.jj_scanpos = xsp;
					if self.jj_3_r_186() {
						self.jj_scanpos = xsp;
						if self.jj_3_r_187() {
							self.jj_scanpos = xsp;
							if self.jj_3_r_188() {
								self.jj_scanpos = xsp;
								if self.jj_3_r_189() {
									self.jj_scanpos = xsp;
									if self.jj_3_r_190() {
										return true;
									}
	
								}
							}
						}
					}
				}
			}
		}
		return false;
	}

	fn jj_3_r_340(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_231() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_105() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_395(&mut self) -> bool {
		if self.jj_3_r_128() {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_413() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_414() {
				self.jj_scanpos = xsp;
				if self.jj_3_r_415() {
					self.jj_scanpos = xsp;
					if self.jj_3_r_416() {
						self.jj_scanpos = xsp;
						if self.jj_3_r_417() {
							self.jj_scanpos = xsp;
							if self.jj_3_r_418() {
								self.jj_scanpos = xsp;
								if self.jj_3_r_419() {
									self.jj_scanpos = xsp;
									if self.jj_3_r_420() {
										return true;
									}
	
								}
							}
						}
					}
				}
			}
		}
		return false;
	}

	fn jj_3_r_183(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_413(&self) -> bool {
		if self.jj_3_r_230() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_184(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_414(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_408()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_153(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_222() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_223() {
				self.jj_scanpos = xsp;
				if self.jj_3_r_224() {
					self.jj_scanpos = xsp;
					if self.jj_3_r_225() {
						self.jj_scanpos = xsp;
						if self.jj_3_r_226() {
							self.jj_scanpos = xsp;
							if self.jj_3_r_227() {
								self.jj_scanpos = xsp;
								if self.jj_3_r_228() {
									self.jj_scanpos = xsp;
									if self.jj_3_r_229() {
										return true;
									}
	
								}
							}
						}
					}
				}
			}
		}
		return false;
	}

	fn jj_3_r_185(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_186(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_415(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_138()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_187(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_222(&self) -> bool {
		if self.jj_3_r_260() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_416(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_409()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_188(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_223(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_189(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_7(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_224(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_261() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_262() {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_417(&self) -> bool {
		if self.jj_3_r_436() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_190(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_8(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_418(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_410()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_9(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_261(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_326() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_3_r_90() {
			return true;
		}
	
		xsp = self.jj_scanpos;
		if self.jj_3_r_327() {
			self.jj_scanpos = xsp;
		}
	
		return false;
	}

	fn jj_3_r_419(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_65()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_326(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_103()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_420(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_411()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_10(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_84() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_3_r_85()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_327(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_243()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_84(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_143()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_341(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_396() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_397() {
				return true;
			}
	
		}
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_231() {
			return true;
		}
	
		return false;
	}

	fn jj_3_11(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_86() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_3_r_85()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_120(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_196() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_197() {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_86(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_143()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_12(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_87() {
			return true;
		}
	
		if self.jj_3_r_85()? {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_88() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		xsp = self.jj_scanpos;
		if self.jj_scan_token(108)? {
			self.jj_scanpos = xsp;
			if self.jj_scan_token(113)? {
				self.jj_scanpos = xsp;
				if self.jj_scan_token(107)? {
					return true;
				}
	
			}
		}
		return false;
	}

	fn jj_3_r_262(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_328() {
			self.jj_scanpos = xsp;
		}
	
		xsp = self.jj_scanpos;
		if self.jj_3_r_329() {
			self.jj_scanpos = xsp;
			if self.jj_scan_token(42)? {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_196(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_328(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_103()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_329(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_85()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_197(&self) -> bool {
		if self.jj_3_r_87() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_225(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_263() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_264() {
				self.jj_scanpos = xsp;
				if self.jj_3_r_265() {
					self.jj_scanpos = xsp;
					if self.jj_3_r_266() {
						return true;
					}
	
				}
			}
		}
		return false;
	}

	fn jj_3_r_396(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_130() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_105() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_263(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_397(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_421() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		xsp = self.jj_scanpos;
		if self.jj_3_r_508() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		xsp = self.jj_scanpos;
		if self.jj_3_r_509() {
			self.jj_scanpos = xsp;
		}
	
		return false;
	}

	fn jj_3_r_436(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_450() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_3_r_90() {
			return true;
		}
	
		xsp = self.jj_scanpos;
		if self.jj_3_r_451() {
			self.jj_scanpos = xsp;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_216() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_421(&self) -> bool {
		if self.jj_3_r_437() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_264(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_330() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_450(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_143()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_65(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_130() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_508(&self) -> bool {
		if self.jj_3_r_105() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_509(&self) -> bool {
		if self.jj_3_r_513() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_265(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_331() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_66(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_85()? {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_32() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_451(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_292() {
			return true;
		}
	
		let xsp: Token;
		while true {
			xsp = self.jj_scanpos;
			if self.jj_3_r_465() {
				self.jj_scanpos = xsp;
				break;
			}
		}
		return false;
	}

	fn jj_3_r_88(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_146()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_45(&self) -> bool {
		if self.jj_3_r_118() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_465(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_292() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_266(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_105() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_32(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_3_r_85()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_226(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_267()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_227(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_120() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_46(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_119() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_228(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_121() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_268() {
			self.jj_scanpos = xsp;
		}
	
		xsp = self.jj_scanpos;
		if self.jj_3_r_269() {
			self.jj_scanpos = xsp;
			if self.jj_scan_token(42)? {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_268(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_103()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_47(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_120() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_269(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_85()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_90(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_85()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_229(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_90()? {
			return true;
		}
	
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_270() {
			self.jj_scanpos = xsp;
		}
	
		return false;
	}

	fn jj_3_48(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_121() {
			return true;
		}
	
		if self.jj_scan_token()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_270(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_243()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_437(&mut self) -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_r_456() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_457() {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_r_456(&self) -> bool {
		if self.jj_3_r_130() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_457(&self) -> bool {
		if self.jj_3_r_468() {
			return true;
		}
	
		return false;
	}

	fn jj_3_r_116(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		let xsp: Token;
		xsp = self.jj_scanpos;
		if self.jj_3_49() {
			self.jj_scanpos = xsp;
			if self.jj_3_r_191()? {
				return true;
			}
	
		}
		return false;
	}

	fn jj_3_66(&self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ -> bool {
		if self.jj_3_r_128() {
			return true;
		}
	
		if self.jj_3_r_87() {
			return true;
		}
	
		if self.jj_3_r_85()? {
			return true;
		}
	
		return false;
	}

	fn jj_3_49(&self) -> bool {
		if self.jj_3_r_117() {
			return true;
		}
	
		return false;
	}

	init {
	    jj_la1_init_0();
	    jj_la1_init_1();
	    jj_la1_init_2();
	    jj_la1_init_3();
	    jj_la1_init_4();
	}

	fn jj_la1_init_0(&mut self) {
		self.jj_la1_0 = : [i32; ] = [0; ];
	}

	fn jj_la1_init_1(&mut self) {
		self.jj_la1_1 = : [i32; ] = [0; ];
	}

	fn jj_la1_init_2(&mut self) {
		self.jj_la1_2 = : [i32; ] = [0; ];
	}

	fn jj_la1_init_3(&mut self) {
		self.jj_la1_3 = : [i32; ] = [0; ];
	}

	fn jj_la1_init_4(&mut self) {
		self.jj_la1_4 = : [i32; ] = [0; ];
	}

	pub fn new(stream: &com::github::javaparser::provider::Provider) -> com::github::javaparser::generated_java_parser::GeneratedJavaParser {
		self.jj_input_stream = SimpleCharStream::new(stream, 1, 1);
		self.token_source = GeneratedJavaParserTokenManager::new(self.jj_input_stream);
		self.token = Token::new();
		self.jj_ntk = -1;
		self.jj_gen = 0;
		 {
			let i: i32 = 0;
			while i < 190 {
				{
					self.jj_la1[i] = -1;
				}
				i += 1;
			 }
		 }
	
		 {
			let i: i32 = 0;
			while i < self.jj_2_rtns.length {
				{
					self.jj_2_rtns[i] = JJCalls::new();
				}
				i += 1;
			 }
		 }
	
	}

	pub fn new(sdsl: &/* Java */ java::lang::String /**/) -> com::github::javaparser::generated_java_parser::GeneratedJavaParser {
		this(StringProvider::new(sdsl));
	}

	pub fn re_init(&self, sdsl: &/* Java */ java::lang::String /**/) {
		self.re_init(StringProvider::new(sdsl));
	}

	pub fn re_init(&mut self, stream: &com::github::javaparser::provider::Provider) {
		if self.jj_input_stream == null {
			self.jj_input_stream = SimpleCharStream::new(stream, 1, 1);
		} else {
			self.jj_input_stream.re_init(stream, 1, 1);
		}
		if self.token_source == null {
			self.token_source = GeneratedJavaParserTokenManager::new(self.jj_input_stream);
		}
		self.token_source.re_init(self.jj_input_stream);
		self.token = Token::new();
		self.jj_ntk = -1;
		self.jj_gen = 0;
		 {
			let i: i32 = 0;
			while i < 190 {
				{
					self.jj_la1[i] = -1;
				}
				i += 1;
			 }
		 }
	
		 {
			let i: i32 = 0;
			while i < self.jj_2_rtns.length {
				{
					self.jj_2_rtns[i] = JJCalls::new();
				}
				i += 1;
			 }
		 }
	
	}

	pub fn new(tm: &com::github::javaparser::generated_java_parser_token_manager::GeneratedJavaParserTokenManager) -> com::github::javaparser::generated_java_parser::GeneratedJavaParser {
		self.token_source = tm;
		self.token = Token::new();
		self.jj_ntk = -1;
		self.jj_gen = 0;
		 {
			let i: i32 = 0;
			while i < 190 {
				{
					self.jj_la1[i] = -1;
				}
				i += 1;
			 }
		 }
	
		 {
			let i: i32 = 0;
			while i < self.jj_2_rtns.length {
				{
					self.jj_2_rtns[i] = JJCalls::new();
				}
				i += 1;
			 }
		 }
	
	}

	pub fn re_init(&mut self, tm: &com::github::javaparser::generated_java_parser_token_manager::GeneratedJavaParserTokenManager) {
		self.token_source = tm;
		self.token = Token::new();
		self.jj_ntk = -1;
		self.jj_gen = 0;
		 {
			let i: i32 = 0;
			while i < 190 {
				{
					self.jj_la1[i] = -1;
				}
				i += 1;
			 }
		 }
	
		 {
			let i: i32 = 0;
			while i < self.jj_2_rtns.length {
				{
					self.jj_2_rtns[i] = JJCalls::new();
				}
				i += 1;
			 }
		 }
	
	}

	fn jj_consume_token(&mut self, kind: i32) /* thrown(com.github.javaparser.ParseException | com.github.javaparser.TokenMgrException) */ -> com::github::javaparser::token::Token {
		/* final */ let old_token: Token = self.token;
		if self.token.next != null {
			self.token = self.token.next;
		}
		else {
			self.token.next = self.token_source.get_next_token()?;
			self.token = self.token.next;
		}
		self.jj_ntk = -1;
		if self.token.kind == kind {
			self.jj_gen += 1;
			if self.jj_gc += 1 > 100 {
				self.jj_gc = 0;
				 {
					let i: i32 = 0;
					while i < self.jj_2_rtns.length {
						{
							let c: JJCalls = self.jj_2_rtns[i];
							while c != null {
								if c.gen < self.jj_gen {
									c.first = null;
								}
	
								c = c.next;
							}
						}
						i += 1;
					 }
				 }
	
			}
			return self.token;
		}
		self.token = old_token;
		self.jj_kind = kind;
		return Err(self.generate_parse_exception());
	}

	fn jj_scan_token(&mut self, kind: i32) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess | com.github.javaparser.TokenMgrException) */ -> bool {
		if self.jj_scanpos == self.jj_lastpos {
			self.jj_la -= 1;
			if self.jj_scanpos.next == null {
				self.jj_lastpos = self.jj_scanpos = self.jj_scanpos.next = self.token_source.get_next_token()?;
			} else {
				self.jj_lastpos = self.jj_scanpos = self.jj_scanpos.next;
			}
		} else {
			self.jj_scanpos = self.jj_scanpos.next;
		}
		if self.jj_rescan {
			let i: i32 = 0;
			let tok: Token = self.token;
			while tok != null && tok != self.jj_scanpos {
				i += 1;
				tok = tok.next;
			}
			if tok != null {
				self.jj_add_error_token(kind, i);
			}
	
		}
		if self.jj_scanpos.kind != kind {
			return true;
		}
	
		if self.jj_la == 0 && self.jj_scanpos == self.jj_lastpos {
			return Err(self.jj_ls);
		}
	
		return false;
	}

	pub fn get_next_token(&mut self) /* thrown(com.github.javaparser.TokenMgrException) */ -> com::github::javaparser::token::Token {
		if self.token.next != null {
			self.token = self.token.next;
		}
		else {self.token = self.token.next = self.token_source.get_next_token()?;
		}
	
		self.jj_ntk = -1;
		self.jj_gen += 1;
		return self.token;
	}

	pub fn get_token(&self, index: i32) /* thrown(com.github.javaparser.TokenMgrException) */ -> com::github::javaparser::token::Token {
		let t: Token =  if self.jj_looking_ahead { self.jj_scanpos } else { self.token };
		 {
			let i: i32 = 0;
			while i < index {
				{
					if t.next == null {
						t.next = self.token_source.get_next_token()?;
					}
	
					t = t.next;
				}
				i += 1;
			 }
		 }
	
		return t;
	}

	fn jj_ntk_f(&mut self) /* thrown(com.github.javaparser.TokenMgrException) */ -> i32 {
		/* final */ let nt: Token = self.jj_nt = self.token.next;
		/* final */ let ret: i32;
		if nt == null {
			self.token.next = self.token_source.get_next_token()?;
			ret = self.jj_ntk = self.token.next.kind;
		} else {ret = self.jj_ntk = nt.kind;
		}
	
		return ret;
	}

	fn jj_add_error_token(&mut self, kind: i32, pos: i32) {
		if pos >= 100 {
			return;
		}
		if pos == self.jj_endpos + 1 {
			self.jj_lasttokens[self.jj_endpos += 1 !!!check!!! post increment] = kind;
		} else if self.jj_endpos != 0 {
			self.jj_expentry = : [i32; self.jj_endpos] = [0; self.jj_endpos];
			 {
				let i: i32 = 0;
				while i < self.jj_endpos {
					{
						self.jj_expentry[i] = self.jj_lasttokens[i];
					}
					i += 1;
				 }
			 }
	
			for /* final */ oldentry in self.jj_expentries {
				if oldentry.length == self.jj_expentry.length {
					let is_matched: bool = true;
					 {
						let i: i32 = 0;
						while i < self.jj_expentry.length {
							{
								if oldentry[i] != self.jj_expentry[i] {
									is_matched = false;
									break;
								}
							}
							i += 1;
						 }
					 }
	
					if is_matched {
						self.jj_expentries.add(self.jj_expentry);
						break;
					}
				}
			}
			if pos != 0 {
				self.jj_endpos = pos;
				self.jj_lasttokens[self.jj_endpos - 1] = kind;
			}
		}
	}

	pub fn generate_parse_exception(&mut self) -> com::github::javaparser::parse_exception::ParseException {
		self.jj_expentries.clear();
		let la1tokens: [bool; 153] = [false; 153];
		if self.jj_kind >= 0 {
			la1tokens[self.jj_kind] = true;
			self.jj_kind = -1;
		}
		 {
			let i: i32 = 0;
			while i < 190 {
				{
					if self.jj_la1[i] == self.jj_gen {
						 {
							let j: i32 = 0;
							while j < 32 {
								{
									if (self.jj_la1_0[i] & (1 << j)) != 0 {
										la1tokens[j] = true;
									}
									if (self.jj_la1_1[i] & (1 << j)) != 0 {
										la1tokens[32 + j] = true;
									}
									if (self.jj_la1_2[i] & (1 << j)) != 0 {
										la1tokens[64 + j] = true;
									}
									if (self.jj_la1_3[i] & (1 << j)) != 0 {
										la1tokens[96 + j] = true;
									}
									if (self.jj_la1_4[i] & (1 << j)) != 0 {
										la1tokens[128 + j] = true;
									}
								}
								j += 1;
							 }
						 }
	
					}
				}
				i += 1;
			 }
		 }
	
		 {
			let i: i32 = 0;
			while i < 153 {
				{
					if la1tokens[i] {
						self.jj_expentry = : [i32; 1] = [0; 1];
						self.jj_expentry[0] = i;
						self.jj_expentries.add(self.jj_expentry);
					}
				}
				i += 1;
			 }
		 }
	
		self.jj_endpos = 0;
		self.jj_rescan_token();
		self.jj_add_error_token(0, 0);
		let exptokseq: [[i32; ]; self.jj_expentries.size()] = [[0; ]; self.jj_expentries.size()];
		 {
			let i: i32 = 0;
			while i < self.jj_expentries.size() {
				{
					exptokseq[i] = self.jj_expentries.get(i);
				}
				i += 1;
			 }
		 }
	
		return ParseException::new(self.token, exptokseq, ,  if self.token_source == null { null } else { GeneratedJavaParserTokenManager.lexStateNames[self.token_source.curLexState] });
	}

	pub fn trace_enabled(&self) -> bool {
		return false;
	}

	pub fn enable_tracing(&self) {
	}

	pub fn disable_tracing(&self) {
	}

	fn jj_rescan_token(&mut self) /* thrown(com.github.javaparser.GeneratedJavaParser.LookaheadSuccess) */ {
		self.jj_rescan = true;
		 {
			let i: i32 = 0;
			while i < 79 {
				{
					let r0 = 'try0: {
						let p: JJCalls = self.jj_2_rtns[i];
						loop { {
							if p.gen > self.jj_gen {
								self.jj_la = p.arg;
								self.jj_scanpos = p.first;
								self.jj_lastpos = p.first;
								match i {
									0 =>  {
										match self.jj_3_1() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									1 =>  {
										self.jj_3_2();
										break;
									}
									2 =>  {
										match self.jj_3_3() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									3 =>  {
										match self.jj_3_4() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									4 =>  {
										self.jj_3_5();
										break;
									}
									5 =>  {
										match self.jj_3_6() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									6 =>  {
										match self.jj_3_7() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									7 =>  {
										match self.jj_3_8() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									8 =>  {
										match self.jj_3_9() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									9 =>  {
										match self.jj_3_10() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									10 =>  {
										match self.jj_3_11() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									11 =>  {
										match self.jj_3_12() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									12 =>  {
										match self.jj_3_13() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									13 =>  {
										match self.jj_3_14() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									14 =>  {
										match self.jj_3_15() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									15 =>  {
										match self.jj_3_16() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									16 =>  {
										match self.jj_3_17() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									17 =>  {
										match self.jj_3_18() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									18 =>  {
										self.jj_3_19();
										break;
									}
									19 =>  {
										match self.jj_3_20() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									20 =>  {
										self.jj_3_21();
										break;
									}
									21 =>  {
										self.jj_3_22();
										break;
									}
									22 =>  {
										match self.jj_3_23() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									23 =>  {
										match self.jj_3_24() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									24 =>  {
										self.jj_3_25();
										break;
									}
									25 =>  {
										self.jj_3_26();
										break;
									}
									26 =>  {
										match self.jj_3_27() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									27 =>  {
										match self.jj_3_28() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									28 =>  {
										self.jj_3_29();
										break;
									}
									29 =>  {
										match self.jj_3_30() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									30 =>  {
										self.jj_3_31();
										break;
									}
									31 =>  {
										match self.jj_3_32() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									32 =>  {
										self.jj_3_33();
										break;
									}
									33 =>  {
										self.jj_3_34();
										break;
									}
									34 =>  {
										self.jj_3_35();
										break;
									}
									35 =>  {
										self.jj_3_36();
										break;
									}
									36 =>  {
										self.jj_3_37();
										break;
									}
									37 =>  {
										match self.jj_3_38() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									38 =>  {
										match self.jj_3_39() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									39 =>  {
										self.jj_3_40();
										break;
									}
									40 =>  {
										self.jj_3_41();
										break;
									}
									41 =>  {
										match self.jj_3_42() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									42 =>  {
										self.jj_3_43();
										break;
									}
									43 =>  {
										self.jj_3_44();
										break;
									}
									44 =>  {
										self.jj_3_45();
										break;
									}
									45 =>  {
										match self.jj_3_46() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									46 =>  {
										match self.jj_3_47() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									47 =>  {
										match self.jj_3_48() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									48 =>  {
										self.jj_3_49();
										break;
									}
									49 =>  {
										match self.jj_3_50() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									50 =>  {
										self.jj_3_51();
										break;
									}
									51 =>  {
										match self.jj_3_52() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									52 =>  {
										self.jj_3_53();
										break;
									}
									53 =>  {
										self.jj_3_54();
										break;
									}
									54 =>  {
										match self.jj_3_55() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									55 =>  {
										match self.jj_3_56() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									56 =>  {
										match self.jj_3_57() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									57 =>  {
										match self.jj_3_58() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									58 =>  {
										match self.jj_3_59() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									59 =>  {
										self.jj_3_60();
										break;
									}
									60 =>  {
										self.jj_3_61();
										break;
									}
									61 =>  {
										self.jj_3_62();
										break;
									}
									62 =>  {
										match self.jj_3_63() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									63 =>  {
										self.jj_3_64();
										break;
									}
									64 =>  {
										match self.jj_3_65() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									65 =>  {
										match self.jj_3_66() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									66 =>  {
										match self.jj_3_67() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									67 =>  {
										match self.jj_3_68() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									68 =>  {
										self.jj_3_69();
										break;
									}
									69 =>  {
										match self.jj_3_70() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									70 =>  {
										match self.jj_3_71() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									71 =>  {
										match self.jj_3_72() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									72 =>  {
										match self.jj_3_73() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									73 =>  {
										match self.jj_3_74() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									74 =>  {
										match self.jj_3_75() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									75 =>  {
										match self.jj_3_76() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									76 =>  {
										match self.jj_3_77() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									77 =>  {
										match self.jj_3_78() {
											Err(e) => break 'try0 Err(e),
											Ok(s) => s,
										};
										break;
									}
									78 =>  {
										self.jj_3_79();
										break;
									}
								}
							}
							p = p.next;
						}if !(p != null) break;}
						break 'try0 Ok(());
					};
					match r0 {
						Err(e @ LookaheadSuccess) => {
						/*  ignore */ 
						},
						Err(e) => Err(e)?,
						Ok => (),
					}
				}
				i += 1;
			 }
		 }
	
		self.jj_rescan = false;
	}

	fn jj_save(&self, index: i32, xla: i32) {
		let p: JJCalls = self.jj_2_rtns[index];
		while p.gen > self.jj_gen {
			if p.next == null {
				p.next = JJCalls::new();
				p = p.next;
				break;
			}
			p = p.next;
		}
		p.gen = self.jj_gen + xla - self.jj_la;
		p.first = self.token;
		p.arg = xla;
	}
}

impl com::github::javaparser::generated_java_parser_constants::GeneratedJavaParserConstants for GeneratedJavaParser {}

struct LookaheadSuccess;

impl /* Java */ java::io::Serializable /**/ for LookaheadSuccess {}

struct JJCalls {
	gen: i32,
	first: com::github::javaparser::token::Token,
	arg: i32,
	next: com::github::javaparser::generated_java_parser::JJCalls,
}