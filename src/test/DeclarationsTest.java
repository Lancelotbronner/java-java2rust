package test;

import java2rust.Java2Rust;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

/**
 * Created by aschoerk on 15.05.16.
 */
public class DeclarationsTest {
	@Test
	public void canConvertFieldDeclaration() {
		assertEquals(
			"""
				struct A {
					i: i32,
				}
				""", Java2Rust.test("""
				class A { int i; }
				"""));
		assertEquals(
			"""
				struct A {
					i: i32 = 1,
				}
				""", Java2Rust.test("""
				class A { int i = 1; }
				"""));
	}

	@Test
	public void canConvertGenerics() {
		assertEquals(
			"""
				struct D;
				
				struct A<B, C: crate::D, E> {
					i: C,
				}
				""", Java2Rust.test("""
				class D {}
				class A<B, C extends D, E> { C i; }
				"""));
	}

	@Test
	public void canConvertMethodParameter() {
		assertEquals(
			"""
				struct A;
				
				impl A {
					fn m(i: i32) {}
				}
				""", Java2Rust.test("class A { void m(int i) { }; }"));
	}

	@Test
	public void canConvertInheritance() {
		assertEquals(
			"""
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
				""", Java2Rust.test("""
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
				"""));
		assertEquals(
			"""
				struct A {
					i: i32 = 1,
				}
				""", Java2Rust.test("""
				class A { int i = 1; }
				"""));
	}

	@Test
	public void canConvertClassWithExtends() {
		assertEquals(
			"""
				struct A {
					b: i32,
				}
				
				struct B {
					base: A,
					c: i32 = 5,
				}
				""", Java2Rust.test("""
				class A { int b; }
				class B extends A { int c = 5; }
				"""));
	}

	@Test
	public void canConvertInterfaceWithImplements() {
		assertEquals(
			"""
				trait B {}
				
				struct A;
				
				impl B for A {}
				""", Java2Rust.test("interface B {}; class A implements B { }"));
	}

	@Test
	public void canConvertClassWithMultipleImplementations() {
		assertEquals(
			"""
				struct A;
				
				impl B for A {}
				
				impl C for A {}
				""", Java2Rust.test("interface B {}; interface C {}; class A implements B, C { }"));
	}

	@Test
	public void canConvertClassWithConstructors() {
		assertEquals(
			"""
				struct A {
					x: i32 = 10,
				}
				
				struct B extends A {
					y: i32 = 20,
				}
				""", Java2Rust.test("""
				class A { int x; }
				class B extends A { int y; }
				"""));
	}

	@Test
	public void canConvertClassWithStaticMethods() {
		assertEquals(
			"""
				struct A {
					static fn m() {}
				}
				
				struct B extends A {
					static fn n() {}
				}
				""", Java2Rust.test("""
				class A { void static m(); }
				class B extends A { void static n(); }
				"""));
	}

	@Test
	public void canConvertClassWithFinalVariables() {
		assertEquals(
			"""
				struct A {
					final i32 a: i32 = 10,
				}
				""", Java2Rust.test("""
				class A { final int a; }
				"""));
	}

	@Test
	public void canConvertClassWithDefaultMethods() {
		assertEquals(
			"""
				struct A {
					fn f() {}
				}
				
				struct B extends A {
					void g() {}
				}
				""", Java2Rust.test("""
				class A { void f(); }
				class B extends A { void g(); }
				"""));
	}
}

//	@Test
//	public void canConvertVariableDeclaration() {
//		assertThat(call("class A { void m() { int i; }; }"), containsString("i: i32"));
//		assertThat(call("class A { void m() { int i = 2; }; }"), containsString("i: i32 = 2;"));
//	}
//
//	@Test
//	public void canConvertArrayDeclaration() {
//		int[] a;
//		int[] b = { 1, 2 };
//		int[][] c = new int[1][2];
//		int[][] d = { { 1, 4, 5, 6 }, { 1, 6 } };
//
//		assertThat(
//			call("int[] a;"),
//			containsString("let a: i32[];"));   // no reasonable conversion possible here
//		assertThat(call("int b[] = { 1, 2};"), containsString("let b: [i32; 2] = [1, 2, ]"));
//		assertThat(
//			call("int c[][] = new int[1][2];"),
//			containsString("let c: [[i32; 2]; 1] = [[0; 2]; 1];"));
//		assertThat(
//			call("int d[][] = { {1 , 4, 5, 6}, { 1, 6 }};"),
//			containsString("let d: [[i32; 4]; 2] = [[1, 4, 5, 6, ]\n" + "    , [1, 6, ]\n" + "    , ]\n" + "    ;"));
//
//	}
//
//	@Test
//	public void putSelfAsParam() {
//		assertThat(call("void method() { }"), containsString("method(&self)"));
//		assertThat(call("static void staticMethod() { }"), containsString("static_method()"));
//	}
//
//	@Test
//	public void enumDeclarationCreatesNewBlock() {
//		assertThat(
//			call("class X {\n" + " enum A { AA; private final int id; }\n" + " enum B { BB; private final int id; }\n" + "}"),
//			containsString("let id: i32"));
//	}
