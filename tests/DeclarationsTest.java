import org.junit.jupiter.api.Test;

public class DeclarationsTest {
	@Test
	public void canConvertFieldDeclaration() {
		Java2Rust.assertConversion(
			"""
				class A { int i; }
				""", """
				struct A {
					i: i32,
				}
				""");
		Java2Rust.assertConversion(
			"""
				class A { int i = 1; }
				""", """
				struct A {
					i: i32 = 1,
				}
				""");
	}

	@Test
	public void canConvertGenerics() {
		Java2Rust.assertConversion(
			"""
				class D {}
				class A<B, C extends D, E> { C i; }
				""", """
				struct D;
				
				struct A<B, C: crate::D, E> {
					i: C,
				}
				""");
	}

	@Test
	public void canConvertMethodParameter() {
		Java2Rust.assertConversion(
			"class A { void m(int i) { }; }", """
				struct A;
				
				impl A {
					fn m(i: i32) {}
				}
				""");
	}

	@Test
	public void canConvertInheritance() {
		Java2Rust.assertConversion(
			"""
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
				""", """
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
				""");
	}

	@Test
	public void canConvertClassWithExtends() {
		Java2Rust.assertConversion(
			"""
				class A { int b; }
				class B extends A { int c = 5; }
				""", """
				struct A {
					b: i32,
				}
				
				struct B {
					base: A,
					c: i32 = 5,
				}
				""");
	}

	@Test
	public void canConvertInterfaceWithImplements() {
		Java2Rust.assertConversion(
			"interface B {}; class A implements B { }", """
				trait B;
				
				struct A;
				
				impl B for A {}
				""");
	}

	@Test
	public void canConvertClassWithMultipleImplementations() {
		Java2Rust.assertConversion(
			"interface B {}; interface C {}; class A implements B, C { }", """
				trait B;
				
				trait C;
				
				struct A;
				
				impl B for A {}
				
				impl C for A {}
				""");
	}

	@Test
	public void canConvertClassWithConstructors() {
		Java2Rust.assertConversion(
			"""
				class A { int x; }
				class B extends A { int y; }
				""", """
				struct A {
					x: i32 = 10,
				}
				
				struct B {
					base: A,
					y: i32 = 20,
				}
				""");
	}

	@Test
	public void canConvertClassWithStaticMethods() {
		Java2Rust.assertConversion(
			"""
				class A { void static m(); }
				class B extends A { void static n(); }
				""", """
				struct A {
					static fn m() {}
				}
				
				struct B extends A {
					static fn n() {}
				}
				""");
	}

	@Test
	public void canConvertClassWithFinalVariables() {
		Java2Rust.assertConversion(
			"""
				class A { final int a; }
				""", """
				struct A {
					final i32 a: i32 = 10,
				}
				""");
	}

	@Test
	public void canConvertClassWithDefaultMethods() {
		Java2Rust.assertConversion(
			"""
				class A { void f(); }
				class B extends A { void g(); }
				""", """
				struct A {
					fn f() {}
				}
				
				struct B extends A {
					void g() {}
				}
				""");
	}

	@Test
	public void canConvertVariableDeclaration() {
		Java2Rust.assertConversion("class A { void m() { int i; }; }", "i: i32");
		Java2Rust.assertConversion("class A { void m() { int i = 2; }; }", "i: i32 = 2");
	}

	//		@Test
	//		public void canConvertArrayDeclaration() {
	//			int[] a;
	//			int[] b = { 1, 2 };
	//			int[][] c = new int[1][2];
	//			int[][] d = { { 1, 4, 5, 6 }, { 1, 6 } };
	//
	//			assertThat(
	//				call("int[] a;"),
	//				containsString("let a: i32[];"));   // no reasonable conversion possible here
	//			assertThat(call("int b[] = { 1, 2};"), containsString("let b: [i32; 2] = [1, 2, ]"));
	//			assertThat(
	//				call("int c[][] = new int[1][2];"),
	//				containsString("let c: [[i32; 2]; 1] = [[0; 2]; 1];"));
	//			assertThat(
	//				call("int d[][] = { {1 , 4, 5, 6}, { 1, 6 }};"),
	//				containsString("let d: [[i32; 4]; 2] = [[1, 4, 5, 6, ]\n" + "    , [1, 6, ]\n" + "    , ]\n" + "    ;"));
	//		}

	//		@Test
	//		public void putSelfAsParam() {
	//			assertThat(call("void method() { }"), containsString("method(&self)"));
	//			assertThat(call("static void staticMethod() { }"), containsString("static_method()"));
	//		}

	//		@Test
	//		public void enumDeclarationCreatesNewBlock() {
	//			assertThat(
	//				call("class X {\n" + " enum A { AA; private final int id; }\n" + " enum B { BB; private final int id; }\n" + "}"),
	//				containsString("let id: i32"));
	//		}
}
