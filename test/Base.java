package test;

import java2rust.JavaConverter;

/**
 * Created by aschoerk on 15.05.16.
 */
public class Base {
	protected String call(String s) {
		return JavaConverter.convert2Rust(s);
	}
}
