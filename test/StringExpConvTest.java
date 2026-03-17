package test;

import org.junit.jupiter.api.Test;

/**
 * Created by aschoerk on 03.07.16.
 */
public class StringExpConvTest extends Base {
	@Test
	public void testDecl() {
		String res = call(
			" class A { void m() { String s = \"5 choose \" + i + \"gdgahdgs\"; };  }  ");
		System.out.println(res);

	}

}
