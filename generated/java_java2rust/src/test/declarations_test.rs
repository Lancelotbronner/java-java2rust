use crate::java2rust::Java2Rust;
use commons_lang3::org::junit::jupiter::api::Test;

pub struct DeclarationsTest;

impl DeclarationsTest {
	pub fn can_convert_field_declaration(&self) {
		Java2Rust::assert_conversion(r#"
		class A { int i; }
		"#, r#"
		struct A {
			i: i32,
		}
		"#);
		Java2Rust::assert_conversion(r#"
		class A { int i = 1; }
		"#, r#"
		struct A {
			i: i32 = 1,
		}
		"#);
	}

	pub fn can_convert_generics(&self) {
		Java2Rust::assert_conversion(r#"
		class D {}
		class A<B, C extends D, E> { C i; }
		"#, r#"
		struct D;
		
		struct A<B, C: crate::D, E> {
			i: C,
		}
		"#);
	}

	pub fn can_convert_method_parameter(&self) {
		Java2Rust::assert_conversion("class A { void m(int i) { }; }", r#"
		struct A;
		
		impl A {
			fn m(i: i32) {}
		}
		"#);
	}

	pub fn can_convert_inheritance(&self) {
		Java2Rust::assert_conversion(r#"
		class A {
			int x;
			int getX() { return x; }
			void setX(int x) { this.x = x; }
		}
		
		class B extends A {
			int y;
			@Override
			int getX() { return 0: }
			int getY() { return y; }
			void setY(int y) { this.y = y; }
		}
		
		class C extends B {
			int z;
			int getZ() { return z; }
			void setZ(int z) { this.z = z; }
		}
		"#, r#"
		struct A {
			x: i32,
		}
		
		impl A {
			fn get_x(&self) -> i32 { self.x };
			fn set_x(&mut self, x: i32) { self.x = x };
		}
		
		struct B {
			a: A,
			y: i32,
		}
		
		impl B {
			fn get_x(&self) -> i32 { 0 };
			fn set_x(&mut self, x: i32) { self.a.set_x(x) };
			fn get_y(&self) -> i32 { self.y }
			fn set_y(&mut self, y: i32) { self.y = y }
		}
		
		struct C {
			b: B,
			z: i32,
		}
		
		impl C {
			fn get_x(&self) -> i32 { self.b.get_x() };
			fn set_x(&mut self, x: i32) { self.b.set_x(x) };
			fn get_y(&self) -> i32 { self.b.get_y() };
			fn set_y(&mut self, y: i32) { self.b.set_y(x) };
			fn get_z(&self) -> i32 { self.z }
			fn set_z(&mut self, z: i32) { self.z = z }
		}
		"#);
	}

	pub fn can_convert_class_with_extends(&self) {
		Java2Rust::assert_conversion(r#"
		class A { int b; }
		class B extends A { int c = 5; }
		"#, r#"
		struct A {
			b: i32,
		}
		
		struct B {
			base: A,
			c: i32 = 5,
		}
		"#);
	}

	pub fn can_convert_interface_with_implements(&self) {
		Java2Rust::assert_conversion("interface B {}; class A implements B { }", r#"
		trait B;
		
		struct A;
		
		impl B for A {}
		"#);
	}

	pub fn can_convert_class_with_multiple_implementations(&self) {
		Java2Rust::assert_conversion("interface B {}; interface C {}; class A implements B, C { }", r#"
		trait B;
		
		trait C;
		
		struct A;
		
		impl B for A {}
		
		impl C for A {}
		"#);
	}

	pub fn can_convert_class_with_constructors(&self) {
		Java2Rust::assert_conversion(r#"
		class A { int x; }
		class B extends A { int y; }
		"#, r#"
		struct A {
			x: i32 = 10,
		}
		
		struct B {
			base: A,
			y: i32 = 20,
		}
		"#);
	}

	pub fn can_convert_class_with_static_methods(&self) {
		Java2Rust::assert_conversion(r#"
		class A { void static m(); }
		class B extends A { void static n(); }
		"#, r#"
		struct A {
			static fn m() {}
		}
		
		struct B extends A {
			static fn n() {}
		}
		"#);
	}

	pub fn can_convert_class_with_final_variables(&self) {
		Java2Rust::assert_conversion(r#"
		class A { final int a; }
		"#, r#"
		struct A {
			final i32 a: i32 = 10,
		}
		"#);
	}

	pub fn can_convert_class_with_default_methods(&self) {
		Java2Rust::assert_conversion(r#"
		class A { void f(); }
		class B extends A { void g(); }
		"#, r#"
		struct A {
			fn f() {}
		}
		
		struct B extends A {
			void g() {}
		}
		"#);
	}

	pub fn can_convert_variable_declaration(&self) {
		Java2Rust::assert_conversion("class A { void m() { int i; }; }", "i: i32");
		Java2Rust::assert_conversion("class A { void m() { int i = 2; }; }", "i: i32 = 2");
	}
}