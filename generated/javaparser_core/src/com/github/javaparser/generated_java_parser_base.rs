use crate::com::github::javaparser::ast;
use crate::com::github::javaparser::ast::body::BodyDeclaration;
use crate::com::github::javaparser::ast::body::ClassOrInterfaceDeclaration;
use crate::com::github::javaparser::ast::body::Parameter;
use crate::com::github::javaparser::ast::body::TypeDeclaration;
use crate::com::github::javaparser::ast::comments::CommentsCollection;
use crate::com::github::javaparser::ast::expr;
use crate::com::github::javaparser::ast::stmt::Statement;
use crate::com::github::javaparser::ast::type;
use crate::com::github::javaparser::utils::Pair;
use java::util;
use crate::com::github::javaparser::GeneratedJavaParserConstants::EOF;
use crate::com::github::javaparser::ast::type::ArrayType::unwrapArrayTypes;
use crate::com::github::javaparser::ast::type::ArrayType::wrapInArrayTypes;
use crate::com::github::javaparser::utils::Utils::assertNotNull;

struct GeneratedJavaParserBase {
	problems: /* Java */ java::util::List /**/ = ArrayList<>::new(),
	store_tokens: bool,
}

impl GeneratedJavaParserBase {
	fn get_token_source(&self) -> com::github::javaparser::generated_java_parser_token_manager::GeneratedJavaParserTokenManager ;

	fn re_init(&self, provider: &com::github::javaparser::provider::Provider) ;

	fn token(&self) -> com::github::javaparser::java_token::JavaToken ;

	fn get_next_token(&self) -> com::github::javaparser::token::Token ;

	fn get_token(&self, index: i32) -> com::github::javaparser::token::Token ;

	fn reset(&mut self, provider: &com::github::javaparser::provider::Provider) {
		self.re_init(provider);
		self.problems = ArrayList<>::new();
		self.get_token_source().reset();
	}

	pub fn get_tokens(&self) -> /* Java */ java::util::List /**/ {
		return self.get_token_source().get_tokens();
	}

	fn get_comments_collection(&self) -> com::github::javaparser::ast::comments::comments_collection::CommentsCollection {
		return self.get_token_source().get_comments_collection();
	}

	fn add_problem(&self, message: &/* Java */ java::lang::String /**/) {
		// TODO tokenRange only takes the final token. Need all the tokens.
		self.problems.add(Problem::new(message, &self.token_range(), null));
	}

	fn token_range(&self) -> com::github::javaparser::token_range::TokenRange {
		if self.store_tokens {
			return TokenRange::new(&self.token(), &self.token());
		}
		return null;
	}

	fn range(&self, begin: &com::github::javaparser::java_token::JavaToken, end: &com::github::javaparser::java_token::JavaToken) -> com::github::javaparser::token_range::TokenRange {
		if self.store_tokens {
			return TokenRange::new(begin, end);
		}
		return null;
	}

	fn range(&self, begin: &com::github::javaparser::ast::node::Node, end: &com::github::javaparser::java_token::JavaToken) -> com::github::javaparser::token_range::TokenRange {
		if self.store_tokens {
			return TokenRange::new(&begin.get_token_range().get().get_begin(), end);
		}
		return null;
	}

	fn range(&self, begin: &com::github::javaparser::java_token::JavaToken, end: &com::github::javaparser::ast::node::Node) -> com::github::javaparser::token_range::TokenRange {
		if self.store_tokens {
			return TokenRange::new(begin, &end.get_token_range().get().get_end());
		}
		return null;
	}

	fn range(&self, begin: &com::github::javaparser::ast::node::Node, end: &com::github::javaparser::ast::node::Node) -> com::github::javaparser::token_range::TokenRange {
		if self.store_tokens {
			return TokenRange::new(&begin.get_token_range().get().get_begin(), &end.get_token_range().get().get_end());
		}
		return null;
	}

	fn or_if_invalid(&self, first_choice: &com::github::javaparser::java_token::JavaToken, second_choice: &com::github::javaparser::java_token::JavaToken) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::java_token::JavaToken {
		if self.store_tokens {
			com::github::javaparser::utils::utils::Utils::assert_not_null(first_choice)?;
			com::github::javaparser::utils::utils::Utils::assert_not_null(second_choice)?;
			if first_choice.valid() || second_choice.invalid() {
				return first_choice;
			}
			return second_choice;
		}
		return null;
	}

	fn or_if_invalid(&self, first_choice: &com::github::javaparser::java_token::JavaToken, second_choice: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::java_token::JavaToken {
		if self.store_tokens {
			return self.or_if_invalid(first_choice, &second_choice.get_token_range().get().get_begin())?;
		}
		return null;
	}

	fn node_list_begin(&self, l: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::java_token::JavaToken {
		if !self.store_tokens || l.is_empty() {
			return JavaToken::com::github::javaparser::java_token::JavaToken::INVALID;
		}
		return l.get(0).get_token_range().get().get_begin();
	}

	fn set_token_kind(&self, new_kind: i32) {
		self.token().set_kind(new_kind);
	}

	fn set_store_tokens(&mut self, store_tokens: bool) {
		self.storeTokens = store_tokens;
		self.get_token_source().set_store_tokens(store_tokens);
	}

	fn recover(&self, recovery_token_type: i32, p: &com::github::javaparser::parse_exception::ParseException) -> com::github::javaparser::token_range::TokenRange {
		let begin: JavaToken = null;
		if p.currentToken != null {
			begin = self.token();
		}
		let t: Token;
		loop { {
			t = self.get_next_token();
		}if !(t.kind != recovery_token_type && t.kind != ) break;}
		let end: JavaToken = self.token();
		let token_range: TokenRange = null;
		if begin != null && end != null {
			token_range = self.range(begin, end);
		}
		self.problems.add(Problem::new(&self.make_message_for_parse_exception(p), token_range, p));
		return token_range;
	}

	fn recover_statement(&self, recovery_token_type: i32, l_brace_type: i32, r_brace_type: i32, p: &com::github::javaparser::parse_exception::ParseException) -> com::github::javaparser::token_range::TokenRange {
		let begin: JavaToken = null;
		if p.currentToken != null {
			begin = self.token();
		}
		let level: i32 = 0;
		let t: Token;
		loop { {
			let next_token: Token = self.get_token(1);
			if next_token != null && next_token.kind == r_brace_type && level == 0 {
				let token_range: TokenRange = self.range(begin, &self.token());
				self.problems.add(Problem::new(&self.make_message_for_parse_exception(p), token_range, p));
				return token_range;
			}
			t = self.get_next_token();
			if t.kind == l_brace_type {
				level += 1;
			} else if t.kind == r_brace_type {
				level -= 1;
			}
		}if !(!(t.kind == recovery_token_type && level == 0) && t.kind != ) break;}
		let end: JavaToken = self.token();
		let token_range: TokenRange = null;
		if begin != null && end != null {
			token_range = self.range(begin, end);
		}
		self.problems.add(Problem::new(&self.make_message_for_parse_exception(p), token_range, p));
		return token_range;
	}

	fn empty_node_list<T: com::github::javaparser::ast::node::Node>(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return NodeList<>::new();
	}

	fn add<T: com::github::javaparser::ast::node::Node>(&self, mut list: &com::github::javaparser::ast::node_list::NodeList, obj: &T) -> com::github::javaparser::ast::node_list::NodeList {
		if list == null {
			list = NodeList<>::new();
		}
		list.add(obj);
		return list;
	}

	fn add_when_not_null<T: com::github::javaparser::ast::node::Node>(&self, list: &com::github::javaparser::ast::node_list::NodeList, obj: &T) -> com::github::javaparser::ast::node_list::NodeList {
		if obj == null {
			return list;
		}
		return self.add(list, obj);
	}

	fn prepend<T: com::github::javaparser::ast::node::Node>(&self, mut list: &com::github::javaparser::ast::node_list::NodeList, obj: &T) -> com::github::javaparser::ast::node_list::NodeList {
		if list == null {
			list = NodeList<>::new();
		}
		list.add_first(obj);
		return list;
	}

	fn add<T>(&self, mut list: &/* Java */ java::util::List /**/, obj: &T) -> /* Java */ java::util::List /**/ {
		if list == null {
			list = LinkedList<>::new();
		}
		list.add(obj);
		return list;
	}

	fn propagate_range_growth_on_right(&self, node: &com::github::javaparser::ast::node::Node, end_node: &com::github::javaparser::ast::node::Node) {
		if self.store_tokens {
			node.get_parent_node().ifPresent(|node_parent|{
				let is_child_on_the_right_border_of_parent: bool = node.get_token_range().get().get_end().equals(&node_parent.get_token_range().get().get_end());
				if is_child_on_the_right_border_of_parent {
					self.propagate_range_growth_on_right(node_parent, end_node);
				}
			});
			node.set_token_range(&self.range(node, end_node));
		}
	}

	fn generate_lambda(&self, mut ret: &com::github::javaparser::ast::expr::expression::Expression, lambda_body: &com::github::javaparser::ast::stmt::statement::Statement) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::expression::Expression {
		if ret instanceof EnclosedExpr {
			let inner: Expression = (ret as EnclosedExpr).get_inner();
			let id: SimpleName = (inner as NameExpr).get_name();
			let params: NodeList<Parameter> = self.add(NodeList<>::new(), Parameter::new(&id.get_token_range().orElse(null), NodeList<>::new(), NodeList<>::new(), UnknownType::new(), false, NodeList<>::new(), id));
			ret = LambdaExpr::new(&self.range(ret, lambda_body), params, lambda_body, true);
		} else if ret instanceof NameExpr {
			let id: SimpleName = (ret as NameExpr).get_name();
			let params: NodeList<Parameter> = self.add(NodeList<>::new(), Parameter::new(&ret.get_token_range().orElse(null), NodeList<>::new(), NodeList<>::new(), UnknownType::new(), false, NodeList<>::new(), id));
			ret = LambdaExpr::new(&self.range(ret, lambda_body), params, lambda_body, false);
		} else if ret instanceof LambdaExpr {
			(ret as LambdaExpr).set_body(lambda_body)?;
			self.propagate_range_growth_on_right(ret, lambda_body);
		} else if ret instanceof CastExpr {
			let cast_expr: CastExpr = ret as CastExpr;
			let inner: Expression = self.generate_lambda(&cast_expr.get_expression(), lambda_body)?;
			cast_expr.set_expression(inner)?;
			self.propagate_range_growth_on_right(cast_expr, inner);
		} else {
			self.add_problem("Failed to parse lambda expression! Please create an issue at https://github.com/javaparser/javaparser/issues");
		}
		return ret;
	}

	fn juggle_array_creation(&self, range: &com::github::javaparser::token_range::TokenRange, level_ranges: &/* Java */ java::util::List /**/, type: &com::github::javaparser::ast::type::type::Type, dimensions: &com::github::javaparser::ast::node_list::NodeList, array_annotations: &/* Java */ java::util::List /**/, array_initializer_expr: &com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr) -> com::github::javaparser::ast::expr::array_creation_expr::ArrayCreationExpr {
		let levels: NodeList<ArrayCreationLevel> = NodeList<>::new();
		 {
			let i: i32 = 0;
			while i < array_annotations.size() {
				{
					levels.add(ArrayCreationLevel::new(&level_ranges.get(i), &dimensions.get(i), &array_annotations.get(i)));
				}
				i += 1;
			 }
		 }
	
		return ArrayCreationExpr::new(range, type, levels, array_initializer_expr);
	}

	fn juggle_array_type(&self, partial_type: &com::github::javaparser::ast::type::type::Type, additional_brackets: &/* Java */ java::util::List /**/) -> com::github::javaparser::ast::type::type::Type {
		let partial_parts: Pair<Type, List<ArrayType.ArrayBracketPair>> = com::github::javaparser::ast::type::array_type::ArrayType::unwrap_array_types(partial_type);
		let element_type: Type = partial_parts.a;
		let left_most_brackets: List<ArrayType.ArrayBracketPair> = partial_parts.b;
		return com::github::javaparser::ast::type::array_type::ArrayType::wrap_in_array_types(element_type, additional_brackets, left_most_brackets).clone();
	}

	fn make_message_for_parse_exception(&self, exception: &com::github::javaparser::parse_exception::ParseException) -> /* Java */ java::lang::String /**/ {
		/* final */ let sb: StringBuilder = StringBuilder::new("Parse error. Found ");
		/* final */ let expected: StringBuilder = StringBuilder::new();
		let max_expected_token_sequence_length: i32 = 0;
		let sorted_options: TreeSet<String> = TreeSet<>::new();
		 {
			let i: i32 = 0;
			while i < exception.expectedTokenSequences.length {
				{
					if max_expected_token_sequence_length < exception.expectedTokenSequences[i].length {
						max_expected_token_sequence_length = exception.expectedTokenSequences[i].length;
					}
					 {
						let j: i32 = 0;
						while j < exception.expectedTokenSequences[i].length {
							{
								sorted_options.add(exception.tokenImage[exception.expectedTokenSequences[i][j]]);
							}
							j += 1;
						 }
					 }
	
				}
				i += 1;
			 }
		 }
	
		for option in sorted_options {
			expected.append(" ").append(option);
		}
		let token: Token = exception.currentToken.next;
		 {
			let i: i32 = 0;
			while i < max_expected_token_sequence_length {
				{
					let token_text: String = token.image;
					let escaped_token_text: String = ParseException::add_escapes(token_text);
					if i != 0 {
						sb.append(" ");
					}
					if token.kind == 0 {
						sb.append(exception.tokenImage[0]);
						break;
					}
					escaped_token_text = "\"" + escaped_token_text + "\"";
					let image: String = exception.tokenImage[token.kind];
					if image.equals(escaped_token_text) {
						sb.append(image);
					} else {
						sb.append(" ").append(escaped_token_text).append(" ").append(image);
					}
					token = token.next;
				}
				i += 1;
			 }
		 }
	
		if exception.expectedTokenSequences.length != 0 {
			let num_expected_tokens: i32 = exception.expectedTokenSequences.length;
			sb.append(", expected").append( if num_expected_tokens == 1 { "" } else { " one of " }).append(&expected.toString());
		}
		return sb.toString();
	}

	fn scope_to_name(&self, scope: &com::github::javaparser::ast::expr::expression::Expression) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::name::Name {
		if scope.is_name_expr() {
			let simple_name: SimpleName = scope.as_name_expr()?.get_name();
			return Name::new(&simple_name.get_token_range().orElse(null), null, &simple_name.get_identifier());
		}
		if scope.is_field_access_expr() {
			let field_access_expr: FieldAccessExpr = scope.as_field_access_expr()?;
			return Name::new(&field_access_expr.get_token_range().orElse(null), &self.scope_to_name(&field_access_expr.get_scope())?, &field_access_expr.get_name().get_identifier());
		}
		return Err(IllegalStateException::new("Unexpected expression type: " + scope.getClass().getSimpleName()));
	}

	fn unquote(&self, s: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return s.substring(1, s.length() - 1);
	}

	fn un_triple_quote(&self, s: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		let start: i32 = 3;
		// Skip over the first end of line too:
		if s.charAt(start) == '\r' {
			start += 1;
		}
		if s.charAt(start) == '\n' {
			start += 1;
		}
		return s.substring(start, s.length() - 3);
	}

	fn set_yield_supported(&self) {
		self.get_token_source().set_yield_supported();
	}

	fn type_declarations_for_cu(&self, body_declarations: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::node_list::NodeList {
		let types: NodeList<TypeDeclaration<?>> = self.empty_node_list();
		// special needs to be done. Just return the declarations cast to type declarations.
		if body_declarations.stream().allMatch(BodyDeclaration::isTypeDeclaration) {
			for body_declaration in body_declarations {
				types.add(&body_declaration.as_type_declaration()?);
			}
			return types;
		}
		// If the above return wasn't hit, then this is a compact class declaration, so the type declaration for the
		// implicit class must be created manually and all the found body declarations added as children of that.
		// We also know at this point that at least one body declaration exists, otherwise allMatch would have matched
		// the empty list.
		let compact_class: ClassOrInterfaceDeclaration = ClassOrInterfaceDeclaration::new(NodeList<Modifier>::new(), false, "$COMPACT_CLASS");
		let maybe_starting_range: Optional<TokenRange> = body_declarations.get(0).get_token_range();
		let maybe_end_range: Optional<TokenRange> = body_declarations.get(body_declarations.size() - 1).get_token_range();
		if maybe_starting_range.isPresent() && maybe_end_range.isPresent() {
			let begin: JavaToken = maybe_starting_range.get().get_begin();
			let end: JavaToken = maybe_end_range.get().get_end();
			let token_range: TokenRange = TokenRange::new(begin, end);
			compact_class.set_token_range(token_range);
		}
		compact_class.set_compact(true);
		compact_class.add_modifier(Modifier::com::github::javaparser::ast::modifier::Keyword::FINAL);
		for body_declaration in body_declarations {
			compact_class.add_member(body_declaration);
		}
		types.add(compact_class);
		return types;
	}
}