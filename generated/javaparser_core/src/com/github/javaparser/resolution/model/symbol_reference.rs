use crate::com::github::javaparser::quality::Nullable;
use crate::com::github::javaparser::resolution::UnsolvedSymbolException;
use crate::com::github::javaparser::resolution::declarations::ResolvedDeclaration;
use java::util::Optional;

pub struct SymbolReference<S: com::github::javaparser::resolution::declarations::resolved_declaration::ResolvedDeclaration> {
	corresponding_declaration: S,
}

impl<S: com::github::javaparser::resolution::declarations::resolved_declaration::ResolvedDeclaration> SymbolReference {
	pub fn solved<S: com::github::javaparser::resolution::declarations::resolved_declaration::ResolvedDeclaration, S2: S>(&self, symbol_declaration: &S2) -> com::github::javaparser::resolution::model::symbol_reference::SymbolReference {
		return SymbolReference<>::new(symbol_declaration);
	}

	pub fn unsolved<S: com::github::javaparser::resolution::declarations::resolved_declaration::ResolvedDeclaration>(&self) -> com::github::javaparser::resolution::model::symbol_reference::SymbolReference {
		return SymbolReference<>::new(null);
	}

	pub fn unsolved<S: com::github::javaparser::resolution::declarations::resolved_declaration::ResolvedDeclaration, S2: S>(&self, clazz: &/* Java */ java::lang::Class /**/) -> com::github::javaparser::resolution::model::symbol_reference::SymbolReference {
		return com::github::javaparser::resolution::model::symbol_reference::SymbolReference::unsolved();
	}

	pub fn adapt<I: com::github::javaparser::resolution::declarations::resolved_declaration::ResolvedDeclaration, O: com::github::javaparser::resolution::declarations::resolved_declaration::ResolvedDeclaration>(&self, ref: &com::github::javaparser::resolution::model::symbol_reference::SymbolReference, clazz: &/* Java */ java::lang::Class /**/) -> com::github::javaparser::resolution::model::symbol_reference::SymbolReference {
		let declaration: Optional<I> = ref.get_declaration();
		if declaration.isPresent() {
			let symbol: I = declaration.get();
			if clazz.isInstance(symbol) {
				return com::github::javaparser::resolution::model::symbol_reference::SymbolReference::solved(&clazz.cast(symbol));
			}
		}
		return com::github::javaparser::resolution::model::symbol_reference::SymbolReference::unsolved();
	}

	fn new(corresponding_declaration: &S) -> com::github::javaparser::resolution::model::symbol_reference::SymbolReference {
		self.correspondingDeclaration = corresponding_declaration;
	}

	pub fn get_declaration(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.corresponding_declaration);
	}

	pub fn get_corresponding_declaration(&self) -> S {
		return self.get_declaration().orElseThrow(|()|UnsolvedSymbolException::new("Corresponding declaration not available for unsolved symbol."));
	}

	pub fn is_solved(&self) -> bool {
		return self.get_declaration().isPresent();
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "SymbolReference{" + self.corresponding_declaration + "}";
	}
}