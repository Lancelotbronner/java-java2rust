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
	public void canConvertMethodParameter() {
		assertEquals(
			"""
				struct A;
				
				impl A {
					fn m(i: i32) {}
				}
				""", Java2Rust.test("class A { void m(int i) { }; }"));
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

}
