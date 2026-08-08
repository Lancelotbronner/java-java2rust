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

pub struct GeneratedJavaParserTokenManager {
	tokens: /* Java */ java::util::List /**/ = ArrayList<JavaToken>::new(),
	comments_collection: com::github::javaparser::ast::comments::comments_collection::CommentsCollection = CommentsCollection::new(),
	home_token: com::github::javaparser::java_token::JavaToken,
	token_work_stack: /* Java */ java::util::Stack /**/ = Stack<Token>::new(),
	store_tokens: bool,
	yield_supported: bool = false,
	markdown_comment_tokens: /* Java */ java::util::ArrayDeque /**/ = ArrayDeque<Token>::new(),
	expect_markdown_comment: bool,
	expect_end_of_markdown_line: bool,
	cur_lex_state: i32 = 0,
	default_lex_state: i32 = 0,
	jjnew_state_cnt: i32,
	jjround: i32,
	jjmatched_pos: i32,
	jjmatched_kind: i32,
	input_stream: com::github::javaparser::simple_char_stream::SimpleCharStream,
	jjrounds: &[i32] = : [i32; 160] = [0; 160],
	jjstate_set: &[i32] = : [i32; 2 * 160] = [0; 2 * 160],
	jjimage: /* Java */ java::lang::StringBuilder /**/ = StringBuilder::new(),
	image: /* Java */ java::lang::StringBuilder /**/ = jjimage,
	jjimage_len: i32,
	length_of_match: i32,
	cur_char: i32,
}

impl GeneratedJavaParserTokenManager {
	static jjbitVec0: &[i64] = vec![0x0, 0x0, 0x100000020, 0x0, ]
	;

	static jjbitVec1: &[i64] = vec![0x0, 0x0, 0x1, 0x0, ]
	;

	static jjbitVec2: &[i64] = vec![0x4000, 0x0, 0x0, 0x0, ]
	;

	static jjbitVec3: &[i64] = vec![0x830000003fff, 0x180000000, 0x0, 0x0, ]
	;

	static jjbitVec4: &[i64] = vec![0x1, 0x0, 0x0, 0x0, ]
	;

	static jjbitVec5: &[i64] = vec![0x0, 0x0, 0x0, 0x8000000000000000, ]
	;

	static jjbitVec6: &[i64] = vec![0xfffffffffffffffe, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, ]
	;

	static jjbitVec8: &[i64] = vec![0x0, 0x0, 0xffffffffffffffff, 0xffffffffffffffff, ]
	;

	static jjbitVec9: &[i64] = vec![0xfff0000040220002, 0xffffffffffffdfff, 0xfffff02f7fffffff, 0x12000000ff7fffff, ]
	;

	static jjbitVec10: &[i64] = vec![0x0, 0x0, 0x420043c00000000, 0xff7fffffff7fffff, ]
	;

	static jjbitVec11: &[i64] = vec![0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0x501f0003ffc3, ]
	;

	static jjbitVec12: &[i64] = vec![0x0, 0xbcdf000000000000, 0xfffffffbffffd740, 0xffbfffffffffffff, ]
	;

	static jjbitVec13: &[i64] = vec![0xffffffffffffffff, 0xffffffffffffffff, 0xfffffffffffffc03, 0xffffffffffffffff, ]
	;

	static jjbitVec14: &[i64] = vec![0xfffeffffffffffff, 0xfffffffe027fffff, 0x80ff, 0x707ffffff0000, ]
	;

	static jjbitVec15: &[i64] = vec![0xffffffff00000800, 0xfffec000000007ff, 0xffffffffffffffff, 0x9c00c060002fffff, ]
	;

	static jjbitVec16: &[i64] = vec![0xfffffffd0000, 0xffffffffffffe000, 0x2003fffffffff, 0x43007fffffffc00, ]
	;

	static jjbitVec17: &[i64] = vec![0x110043fffff, 0x7ff01ffffff, 0x3fdfffff00000000, 0x0, ]
	;

	static jjbitVec18: &[i64] = vec![0x23fffffffffffff0, 0xfffe0003ff010000, 0x23c5fdfffff99fe1, 0x180f0003b0004000, ]
	;

	static jjbitVec19: &[i64] = vec![0x36dfdfffff987e0, 0x1c00005e000000, 0x23edfdfffffbbfe0, 0x202000300010000, ]
	;

	static jjbitVec20: &[i64] = vec![0x23edfdfffff99fe0, 0x20003b0000000, 0x3ffc718d63dc7e8, 0x200000000010000, ]
	;

	static jjbitVec21: &[i64] = vec![0x23fffdfffffddfe0, 0x307000000, 0x23effdfffffddfe1, 0x6000340000000, ]
	;

	static jjbitVec22: &[i64] = vec![0x27fffffffffddfe0, 0xfc00000380704000, 0x2ffbfffffc7fffe0, 0x7f, ]
	;

	static jjbitVec23: &[i64] = vec![0x800dfffffffffffe, 0x7f, 0x200decaefef02596, 0xf000005f, ]
	;

	static jjbitVec24: &[i64] = vec![0x1, 0x1ffffffffeff, 0x1f00, 0x0, ]
	;

	static jjbitVec25: &[i64] = vec![0x800007ffffffffff, 0xffe1c0623c3f0000, 0xffffffff00004003, 0xf7ffffffffff20bf, ]
	;

	static jjbitVec26: &[i64] = vec![0xffffffffffffffff, 0xffffffff3d7f3dff, 0x7f3dffffffff3dff, 0xffffffffff7fff3d, ]
	;

	static jjbitVec27: &[i64] = vec![0xffffffffff3dffff, 0x7ffffff, 0xffffffff0000ffff, 0x3f3fffffffffffff, ]
	;

	static jjbitVec28: &[i64] = vec![0xffffffffffffffff, 0xffff9fffffffffff, 0xffffffff07fffffe, 0x1ffc7ffffffffff, ]
	;

	static jjbitVec29: &[i64] = vec![0x3ffff0003dfff, 0x1dfff0003ffff, 0xfffffffffffff, 0x18800000, ]
	;

	static jjbitVec30: &[i64] = vec![0xffffffff00000000, 0xffffffffffffff, 0xffff05ffffffff9f, 0x3fffffffffffff, ]
	;

	static jjbitVec31: &[i64] = vec![0x7fffffff, 0x1f3fffffff0000, 0xffff0fffffffffff, 0x3ff, ]
	;

	static jjbitVec32: &[i64] = vec![0xffffffff007fffff, 0x1fffff, 0x8000000000, 0x0, ]
	;

	static jjbitVec33: &[i64] = vec![0xfffffffffffe0, 0xfe0, 0xfc00c001fffffff8, 0x3fffffffff, ]
	;

	static jjbitVec34: &[i64] = vec![0xfffffffff, 0x3ffffffffc00e000, 0x1ff, 0x63de0000000000, ]
	;

	static jjbitVec35: &[i64] = vec![0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0x0, ]
	;

	static jjbitVec36: &[i64] = vec![0xffffffff3f3fffff, 0x3fffffffaaff3f3f, 0x5fdfffffffffffff, 0x1fdc1fff0fcf1fdc, ]
	;

	static jjbitVec37: &[i64] = vec![0x8000000000000000, 0x8002000000100001, 0xffffffff1fff0000, 0x0, ]
	;

	static jjbitVec38: &[i64] = vec![0xf3ffbd503e2ffc84, 0xffffffff000043e0, 0x1ff, 0x0, ]
	;

	static jjbitVec39: &[i64] = vec![0xffff7fffffffffff, 0xffffffff7fffffff, 0xffffffffffffffff, 0xc781fffffffff, ]
	;

	static jjbitVec40: &[i64] = vec![0xffff20bfffffffff, 0x80ffffffffff, 0x7f7f7f7f007fffff, 0x7f7f7f7f, ]
	;

	static jjbitVec41: &[i64] = vec![0x800000000000, 0x0, 0x0, 0x0, ]
	;

	static jjbitVec42: &[i64] = vec![0x1f3e03fe000000e0, 0xfffffffffffffffe, 0xfffffffee07fffff, 0xf7ffffffffffffff, ]
	;

	static jjbitVec43: &[i64] = vec![0xfffe7fffffffffe0, 0xffffffffffffffff, 0x7ffffff00007fff, 0xffff000000000000, ]
	;

	static jjbitVec44: &[i64] = vec![0xffffffffffffffff, 0xffffffffffffffff, 0x3fffffffffffff, 0x0, ]
	;

	static jjbitVec45: &[i64] = vec![0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0x7ffffffffff, ]
	;

	static jjbitVec46: &[i64] = vec![0xffffffffffffffff, 0xffffffffffffffff, 0x1fff, 0x3fffffffffff0000, ]
	;

	static jjbitVec47: &[i64] = vec![0xc00ffff1fff, 0x80007fffffffffff, 0xffffffff3fffffff, 0xffffffffffff, ]
	;

	static jjbitVec48: &[i64] = vec![0xfffffffcff800000, 0xffffffffffffffff, 0xff7ffffffff9ff, 0xff80000000000000, ]
	;

	static jjbitVec49: &[i64] = vec![0x1000007fffff7bb, 0xfffffffffffff, 0xffffffffffffc, 0x28fc000000000000, ]
	;

	static jjbitVec50: &[i64] = vec![0xffff003ffffffc00, 0x1fffffff0000007f, 0x7fffffffffff0, 0x7c00ffdf00008000, ]
	;

	static jjbitVec51: &[i64] = vec![0x1ffffffffff, 0xc47fffff00000ff7, 0x3e62ffffffffffff, 0x1c07ff38000005, ]
	;

	static jjbitVec52: &[i64] = vec![0xffff7f7f007e7e7e, 0xffff003ff7ffffff, 0xffffffffffffffff, 0x7ffffffff, ]
	;

	static jjbitVec53: &[i64] = vec![0xffffffffffffffff, 0xffffffffffffffff, 0xffff000fffffffff, 0xffffffffffff87f, ]
	;

	static jjbitVec54: &[i64] = vec![0xffffffffffffffff, 0xffff3fffffffffff, 0xffffffffffffffff, 0x3ffffff, ]
	;

	static jjbitVec55: &[i64] = vec![0x5f7ffdffa0f8007f, 0xffffffffffffffdb, 0x3ffffffffffff, 0xfffffffffff80000, ]
	;

	static jjbitVec56: &[i64] = vec![0x3fffffffffffffff, 0xffffffffffff0000, 0xfffffffffffcffff, 0x1fff0000000000ff, ]
	;

	static jjbitVec57: &[i64] = vec![0x18000000000000, 0xffdf02000000e000, 0xffffffffffffffff, 0x1fffffffffffffff, ]
	;

	static jjbitVec58: &[i64] = vec![0x87fffffe00000010, 0xffffffc007fffffe, 0x7fffffffffffffff, 0x631cfcfcfc, ]
	;

	static jjbitVec59: &[i64] = vec![0x0, 0x0, 0x420243cffffffff, 0xff7fffffff7fffff, ]
	;

	static jjbitVec60: &[i64] = vec![0xffffffffffffffff, 0xbcdfffffffffffff, 0xfffffffbffffd740, 0xffbfffffffffffff, ]
	;

	static jjbitVec61: &[i64] = vec![0xffffffffffffffff, 0xffffffffffffffff, 0xfffffffffffffcfb, 0xffffffffffffffff, ]
	;

	static jjbitVec62: &[i64] = vec![0xfffeffffffffffff, 0xfffffffe027fffff, 0xbffffffffffe80ff, 0x707ffffff00b6, ]
	;

	static jjbitVec63: &[i64] = vec![0xffffffff17ff083f, 0xffffc3ffffffffff, 0xffffffffffffffff, 0x9ffffdffbfefffff, ]
	;

	static jjbitVec64: &[i64] = vec![0xffffffffffff8000, 0xffffffffffffe7ff, 0x3ffffffffffff, 0x43fffffffffffff, ]
	;

	static jjbitVec65: &[i64] = vec![0x3fffffffffff, 0x7ff0fffffff, 0x3fdfffff00000000, 0xfffffffffff00000, ]
	;

	static jjbitVec66: &[i64] = vec![0xffffffffffffffff, 0xfffeffcfffffffff, 0xf3c5fdfffff99fef, 0x180fffcfb080799f, ]
	;

	static jjbitVec67: &[i64] = vec![0xd36dfdfffff987ee, 0x3fffc05e023987, 0xf3edfdfffffbbfee, 0xfe02ffcf00013bbf, ]
	;

	static jjbitVec68: &[i64] = vec![0xf3edfdfffff99fee, 0x2ffcfb0c0399f, 0xc3ffc718d63dc7ec, 0x200ffc000813dc7, ]
	;

	static jjbitVec69: &[i64] = vec![0xe3fffdfffffddfef, 0xffcf07603ddf, 0xf3effdfffffddfef, 0x6ffcf40603ddf, ]
	;

	static jjbitVec70: &[i64] = vec![0xfffffffffffddfef, 0xfc00ffcf80f07ddf, 0x2ffbfffffc7fffec, 0xcffc0ff5f847f, ]
	;

	static jjbitVec71: &[i64] = vec![0x87fffffffffffffe, 0x3ff7fff, 0x3bffecaefef02596, 0xf3ff3f5f, ]
	;

	static jjbitVec72: &[i64] = vec![0xc2a003ff03000001, 0xfffe1ffffffffeff, 0x1ffffffffeffffdf, 0x40, ]
	;

	static jjbitVec73: &[i64] = vec![0xffffffffffffffff, 0xffffffffffff03ff, 0xffffffff3fffffff, 0xf7ffffffffff20bf, ]
	;

	static jjbitVec74: &[i64] = vec![0xffffffffff3dffff, 0xe7ffffff, 0xffffffff0000ffff, 0x3f3fffffffffffff, ]
	;

	static jjbitVec75: &[i64] = vec![0x1fffff001fdfff, 0xddfff000fffff, 0xffffffffffffffff, 0x3ff388fffff, ]
	;

	static jjbitVec76: &[i64] = vec![0xffffffff03ff7800, 0xffffffffffffff, 0xffff07ffffffffff, 0x3fffffffffffff, ]
	;

	static jjbitVec77: &[i64] = vec![0xfff0fff7fffffff, 0x1f3fffffffffc0, 0xffff0fffffffffff, 0x3ff03ff, ]
	;

	static jjbitVec78: &[i64] = vec![0xffffffff0fffffff, 0x9fffffff7fffffff, 0x3fff008003ff03ff, 0x0, ]
	;

	static jjbitVec79: &[i64] = vec![0xffffffffffffffff, 0xff80003ff0fff, 0xffffffffffffffff, 0xfffffffffffff, ]
	;

	static jjbitVec80: &[i64] = vec![0xffffffffffffff, 0x3fffffffffffe3ff, 0x1ff, 0x3fffffffff70000, ]
	;

	static jjbitVec81: &[i64] = vec![0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xfbffffffffffffff, ]
	;

	static jjbitVec82: &[i64] = vec![0x80007c000000f800, 0x8002ffdf00100001, 0xffffffff1fff0000, 0x1ffe21fff0000, ]
	;

	static jjbitVec83: &[i64] = vec![0xffff7fffffffffff, 0xffffffff7fffffff, 0xffffffffffffffff, 0xff81fffffffff, ]
	;

	static jjbitVec84: &[i64] = vec![0xffff20bfffffffff, 0x800080ffffffffff, 0x7f7f7f7f007fffff, 0xffffffff7f7f7f7f, ]
	;

	static jjbitVec85: &[i64] = vec![0x1f3efffe000000e0, 0xfffffffffffffffe, 0xfffffffee67fffff, 0xf7ffffffffffffff, ]
	;

	static jjbitVec86: &[i64] = vec![0xfffffff1fff, 0xbff0ffffffffffff, 0xffffffffffffffff, 0x3ffffffffffff, ]
	;

	static jjbitVec87: &[i64] = vec![0x10000ffffffffff, 0xfffffffffffff, 0xffffffffffffffff, 0x28ffffff03ff003f, ]
	;

	static jjbitVec88: &[i64] = vec![0xffff3fffffffffff, 0x1fffffff000fffff, 0xffffffffffffffff, 0x7fffffff03ff8001, ]
	;

	static jjbitVec89: &[i64] = vec![0x7fffffffffffff, 0xfc7fffff03ff3fff, 0xffffffffffffffff, 0x7cffff38000007, ]
	;

	static jjbitVec90: &[i64] = vec![0xffff7f7f007e7e7e, 0xffff003ff7ffffff, 0xffffffffffffffff, 0x3ff37ffffffffff, ]
	;

	static jjbitVec91: &[i64] = vec![0x5f7ffdffe0f8007f, 0xffffffffffffffdb, 0x3ffffffffffff, 0xfffffffffff80000, ]
	;

	static jjbitVec92: &[i64] = vec![0x18ffff0000ffff, 0xffdf02000000e000, 0xffffffffffffffff, 0x9fffffffffffffff, ]
	;

	static jjbitVec93: &[i64] = vec![0x87fffffe03ff0010, 0xffffffc007fffffe, 0x7fffffffffffffff, 0xe0000631cfcfcfc, ]
	;

	pub static jjstrLiteralImages: &[/* Java */ java::lang::String /**/] = vec!["", null, null, null, null, null, null, null, null, null, null, "\141\142\163\164\162\141\143\164", "\141\163\163\145\162\164", "\142\157\157\154\145\141\156", "\142\162\145\141\153", "\142\171\164\145", "\143\141\163\145", "\143\141\164\143\150", "\143\150\141\162", "\143\154\141\163\163", "\143\157\156\163\164", "\143\157\156\164\151\156\165\145", "\144\145\146\141\165\154\164", "\144\157", "\144\157\165\142\154\145", "\145\154\163\145", "\145\156\165\155", "\145\170\164\145\156\144\163", "\146\141\154\163\145", "\146\151\156\141\154", "\146\151\156\141\154\154\171", "\146\154\157\141\164", "\146\157\162", "\147\157\164\157", "\151\146", "\151\155\160\154\145\155\145\156\164\163", "\151\155\160\157\162\164", "\151\156\163\164\141\156\143\145\157\146", "\151\156\164", "\151\156\164\145\162\146\141\143\145", "\154\157\156\147", "\156\141\164\151\166\145", "\156\145\167", "\156\157\156\55\163\145\141\154\145\144", "\156\165\154\154", "\160\141\143\153\141\147\145", "\160\145\162\155\151\164\163", "\160\162\151\166\141\164\145", "\160\162\157\164\145\143\164\145\144", "\160\165\142\154\151\143", "\162\145\143\157\162\144", "\162\145\164\165\162\156", "\163\145\141\154\145\144", "\163\150\157\162\164", "\163\164\141\164\151\143", "\163\164\162\151\143\164\146\160", "\163\165\160\145\162", "\163\167\151\164\143\150", "\163\171\156\143\150\162\157\156\151\172\145\144", "\164\150\151\163", "\164\150\162\157\167", "\164\150\162\157\167\163", "\164\162\141\156\163\151\145\156\164", "\164\162\165\145", "\164\162\171", "\166\157\151\144", "\166\157\154\141\164\151\154\145", "\167\150\151\154\145", "\171\151\145\154\144", "\162\145\161\165\151\162\145\163", "\164\157", "\167\151\164\150", "\157\160\145\156", "\157\160\145\156\163", "\165\163\145\163", "\155\157\144\165\154\145", "\145\170\160\157\162\164\163", "\160\162\157\166\151\144\145\163", "\164\162\141\156\163\151\164\151\166\145", "\167\150\145\156", null, null, null, null, null, null, null, null, null, null, null, null, null, null, null, null, null, null, null, null, null, "\50", "\51", "\173", "\175", "\133", "\135", "\73", "\54", "\56", "\56\56\56", "\100", "\72\72", "\75", "\74", "\41", "\176", "\77", "\72", "\55\76", "\75\75", "\76\75", "\74\75", "\41\75", "\46\46", "\174\174", "\53\53", "\55\55", "\53", "\55", "\52", "\57", "\46", "\174", "\136", "\45", "\74\74", "\53\75", "\55\75", "\52\75", "\57\75", "\46\75", "\174\75", "\136\75", "\45\75", "\74\74\75", "\76\76\75", "\76\76\76\75", "\76\76\76", "\76\76", "\76", "\32", "\137", ]
	;

	static jjnextStates: &[i32] = vec![39, 40, 47, 48, 81, 82, 83, 84, 85, 86, 87, 88, 10, 94, 95, 96, 102, 103, 104, 75, 79, 13, 15, 21, 115, 119, 122, 126, 130, 133, 137, 150, 3, 4, 5, 10, 8, 10, 11, 7, 8, 10, 11, 39, 40, 50, 47, 48, 91, 10, 93, 90, 91, 10, 93, 99, 10, 101, 98, 99, 10, 101, 105, 108, 10, 106, 107, 108, 10, 111, 10, 113, 110, 111, 10, 113, 117, 118, 83, 120, 121, 83, 124, 125, 83, 139, 140, 141, 143, 144, 145, 148, 149, 10, 152, 153, 154, 155, 158, 159, 10, 41, 49, 51, 3, 4, 6, 7, 8, 9, 16, 17, 19, 32, 33, 81, 82, 84, 85, 86, 87, 89, 90, 91, 92, 94, 95, 97, 98, 99, 100, 102, 103, 106, 107, 109, 110, 111, 112, 120, 121, 124, 125, 131, 132, 135, 136, 146, 147, 148, 149, 156, 157, 158, 159, ]
	;

	pub static lexStateNames: &[/* Java */ java::lang::String /**/] = vec!["DEFAULT", "IN_JAVADOC_COMMENT", "IN_MULTI_LINE_COMMENT", "IN_TEXT_BLOCK", ]
	;

	pub static jjnewLexState: &[i32] = vec![-1, -1, -1, -1, -1, -1, 1, 2, 0, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 3, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, ]
	;

	static jjtoToken: &[i64] = vec![0xfffffffffffff801, 0xffffffe56043ffff, 0x1ffffff, ]
	;

	static jjtoSkip: &[i64] = vec![0x33e, 0x0, 0x0, ]
	;

	static jjtoSpecial: &[i64] = vec![0x33e, 0x0, 0x0, ]
	;

	static jjtoMore: &[i64] = vec![0x4c0, 0x280000000, 0x0, ]
	;

	fn reset(&mut self) {
		self.tokens = ArrayList<JavaToken>::new();
		self.comments_collection = CommentsCollection::new();
		self.home_token = null;
	}

	fn get_tokens(&self) -> /* Java */ java::util::List /**/ {
		if self.store_tokens {
			return self.tokens;
		}
		return null;
	}

	fn get_comments_collection(&self) -> com::github::javaparser::ast::comments::comments_collection::CommentsCollection {
		return self.comments_collection;
	}

	fn get_home_token(&self) -> com::github::javaparser::java_token::JavaToken {
		return self.home_token;
	}

	pub fn set_store_tokens(&mut self, store_tokens: bool) {
		self.storeTokens = store_tokens;
	}

	pub fn set_yield_supported(&mut self) {
		self.yield_supported = true;
	}

	fn create_markdown_comment(&self) {
		while !self.markdown_comment_tokens.isEmpty() && /* Java*/ TokenTypes/* */ .isWhitespace(self.markdown_comment_tokens.peekFirst().kind) {
			self.markdown_comment_tokens.removeFirst();
		}
		if !self.markdown_comment_tokens.isEmpty() {
			let comment: MarkdownComment = com::github::javaparser::generated_java_parser_token_manager_base::GeneratedJavaParserTokenManagerBase::create_markdown_comment_from_token_list(self.markdown_comment_tokens)?;
			self.markdown_comment_tokens.clear();
			self.expect_markdown_comment = true;
			self.expect_end_of_markdown_line = false;
			self.comments_collection.add_comment(comment);
		}
	}

	fn common_token_action(&mut self, mut token: &com::github::javaparser::token::Token) {
		// Use an intermediary stack to avoid recursion, see issue 1003
		loop { {
			self.token_work_stack.push(token);
			token = token.special_token;
		}if !(token != null) break;}
		// A /// sequence only indicates the start of a markdown comment if it is only preceded by whitespace characters
		// in the line. This variable is used to keep track of this.
		self.expect_markdown_comment = true;
		// A newline token only indicates the end of the markdown comment if it is not preceded by a line comment (e.g.
		// the second of two consecutive newlines would end the comment). This variable keeps track of whether the next
		// newline should end the markdown comment (if currently processing one).
		self.expect_end_of_markdown_line = false;
		// The stack is now filled with tokens in left-to-right order. Process them.
		while !self.token_work_stack.empty() {
			token = self.token_work_stack.pop();
			token.javaToken = JavaToken::new(token, self.tokens);
			if self.store_tokens {
				self.tokens.add(token.javaToken);
			}
			if self.home_token == null {
				self.home_token = token.javaToken;
			}
			//     comment node.
			if /* Java*/ TokenTypes/* */ .isEndOfLineToken(token.kind) {
				if self.expect_end_of_markdown_line {
					// A newline is processed, but it's the first newline after a markdown comment line
					// (expectEndOfMarkdownComment is still true), so it does not end the comment yet.
					self.markdown_comment_tokens.add(token);
					self.expect_end_of_markdown_line = false;
				} else {
					// A newline is processed, but it's not the first newline after a markdown comment line, so
					// create the comment now.
					self.create_markdown_comment();
				}
				// A new line is started, so until a non-whitespace non-line-comment token is processed, expect a new
				// markdown comment.
				self.expect_markdown_comment = true;
			} else if self.expect_markdown_comment && com::github::javaparser::generated_java_parser_token_manager_base::GeneratedJavaParserTokenManagerBase::is_markdown_comment_line_candidate(token) {
				// The next markdown line comment is processed, so add it to the buffer.
				self.expect_end_of_markdown_line = true;
				self.markdown_comment_tokens.add(token);
			} else if self.expect_markdown_comment && /* Java*/ TokenTypes/* */ .isWhitespaceButNotEndOfLine(token.kind) {
				// Non-newline whitespace characters are always included in the token range for the markdown comment, so
				// add them to the buffer
				self.markdown_comment_tokens.add(token);
			} else if /* Java*/ TokenTypes/* */ .isComment(token.kind) {
				// A comment token is found, but not one that is a markdown line candidate (those are handled in an
				// else above). This could be a line comment not starting with ///, or a block comment. At this point,
				// end the markdown comment and handle the other comment separately.
				self.create_markdown_comment();
				let comment: Comment = com::github::javaparser::generated_java_parser_token_manager_base::GeneratedJavaParserTokenManagerBase::create_comment_from_token(token)?;
				self.comments_collection.add_comment(comment);
			} else if !/* Java*/ TokenTypes/* */ .isWhitespace(token.kind) {
				// Any non-whitespace token ends the markdown comment. If the markdownCommentTokens buffer is empty or
				// only contains whitespace, it is simply cleared.
				self.expect_markdown_comment = false;
				self.create_markdown_comment();
			}
		}
	}

	fn jj_stop_string_literal_dfa_0(&mut self, pos: i32, active0: i64, active1: i64, active2: i64) -> i32 {
		match pos {
			0 =>  {
				if (active1 & 0x80000000) != 0x0 {
					return 160;
				}
	
				if (active0 & 0xfffffffffffff800) != 0x0 || (active1 & 0xffff) != 0x0 {
					self.jjmatched_kind = 98;
					return 161;
				}
				if (active0 & 0x80) != 0x0 || (active2 & 0x1008) != 0x0 {
					return 75;
				}
	
				if (active1 & 0x600000000000) != 0x0 {
					return 2;
				}
	
				if (active2 & 0x1000000) != 0x0 {
					return 162;
				}
	
				return -1;
			}
			1 =>  {
				if (active0 & 0x80) != 0x0 {
					return 77;
				}
	
				if (active1 & 0x80000000) != 0x0 {
					if self.jjmatched_pos != 1 {
						self.jjmatched_kind = 94;
						self.jjmatched_pos = 1;
					}
					return -1;
				}
				if (active0 & 0x401800000) != 0x0 || (active1 & 0x40) != 0x0 {
					return 161;
				}
	
				if (active0 & 0xfffffffbfe7ff800) != 0x0 || (active1 & 0xffbf) != 0x0 {
					if self.jjmatched_pos != 1 {
						self.jjmatched_kind = 98;
						self.jjmatched_pos = 1;
					}
					return 161;
				}
				return -1;
			}
			2 =>  {
				if (active0 & 0xfffffb3aff7ff800) != 0x0 || (active1 & 0xffbe) != 0x0 {
					if self.jjmatched_pos != 2 {
						self.jjmatched_kind = 98;
						self.jjmatched_pos = 2;
					}
					return 161;
				}
				if (active1 & 0x80000000) != 0x0 {
					if self.jjmatched_pos < 1 {
						self.jjmatched_kind = 94;
						self.jjmatched_pos = 1;
					}
					return -1;
				}
				if (active0 & 0x4c100000000) != 0x0 || (active1 & 0x1) != 0x0 {
					return 161;
				}
	
				return -1;
			}
			3 =>  {
				if (active0 & 0x8800110206058000) != 0x0 || (active1 & 0x8782) != 0x0 {
					return 161;
				}
	
				if (active0 & 0x77ffe2b8f97a7800) != 0x0 || (active1 & 0x783c) != 0x0 {
					if self.jjmatched_pos != 3 {
						self.jjmatched_kind = 98;
						self.jjmatched_pos = 3;
					}
					return 161;
				}
				if (active0 & 0x80000000000) != 0x0 {
					if self.jjmatched_pos < 2 {
						self.jjmatched_kind = 98;
						self.jjmatched_pos = 2;
					}
					return -1;
				}
				return -1;
			}
			4 =>  {
				if (active0 & 0x46dfe2b809603800) != 0x0 || (active1 & 0x7824) != 0x0 {
					if self.jjmatched_pos != 4 {
						self.jjmatched_kind = 98;
						self.jjmatched_pos = 4;
					}
					return 161;
				}
				if (active0 & 0x31200000f01a4000) != 0x0 || (active1 & 0x218) != 0x0 {
					return 161;
				}
	
				if (active0 & 0x80000000000) != 0x0 {
					if self.jjmatched_pos < 2 {
						self.jjmatched_kind = 98;
						self.jjmatched_pos = 2;
					}
					return -1;
				}
				return -1;
			}
			5 =>  {
				if (active0 & 0x4481e0a848602800) != 0x0 || (active1 & 0x7024) != 0x0 {
					self.jjmatched_kind = 98;
					self.jjmatched_pos = 5;
					return 161;
				}
				if (active0 & 0x225e021001001000) != 0x0 || (active1 & 0x800) != 0x0 {
					return 161;
				}
	
				if (active0 & 0x80000000000) != 0x0 {
					if self.jjmatched_pos < 2 {
						self.jjmatched_kind = 98;
						self.jjmatched_pos = 2;
					}
					return -1;
				}
				return -1;
			}
			6 =>  {
				if (active0 & 0xe00048402000) != 0x0 || (active1 & 0x1000) != 0x0 {
					return 161;
				}
	
				if (active0 & 0x80000000000) != 0x0 {
					if self.jjmatched_pos < 2 {
						self.jjmatched_kind = 98;
						self.jjmatched_pos = 2;
					}
					return -1;
				}
				if (active0 & 0x448100a800200800) != 0x0 || (active1 & 0x6024) != 0x0 {
					self.jjmatched_kind = 98;
					self.jjmatched_pos = 6;
					return 161;
				}
				return -1;
			}
			7 =>  {
				if (active0 & 0x440100a800000000) != 0x0 || (active1 & 0x4000) != 0x0 {
					self.jjmatched_kind = 98;
					self.jjmatched_pos = 7;
					return 161;
				}
				if (active0 & 0x80000000200800) != 0x0 || (active1 & 0x2024) != 0x0 {
					return 161;
				}
	
				if (active0 & 0x80000000000) != 0x0 {
					if self.jjmatched_pos < 2 {
						self.jjmatched_kind = 98;
						self.jjmatched_pos = 2;
					}
					return -1;
				}
				return -1;
			}
			8 =>  {
				if (active0 & 0x4001008000000000) != 0x0 {
					return 161;
				}
	
				if (active0 & 0x400002800000000) != 0x0 || (active1 & 0x4000) != 0x0 {
					self.jjmatched_kind = 98;
					self.jjmatched_pos = 8;
					return 161;
				}
				if (active0 & 0x80000000000) != 0x0 {
					if self.jjmatched_pos < 2 {
						self.jjmatched_kind = 98;
						self.jjmatched_pos = 2;
					}
					return -1;
				}
				return -1;
			}
			9 =>  {
				if (active0 & 0x2800000000) != 0x0 || (active1 & 0x4000) != 0x0 {
					return 161;
				}
	
				if (active0 & 0x400000000000000) != 0x0 {
					self.jjmatched_kind = 98;
					self.jjmatched_pos = 9;
					return 161;
				}
				if (active0 & 0x80000000000) != 0x0 {
					if self.jjmatched_pos < 2 {
						self.jjmatched_kind = 98;
						self.jjmatched_pos = 2;
					}
					return -1;
				}
				return -1;
			}
			10 =>  {
				if (active0 & 0x400000000000000) != 0x0 {
					self.jjmatched_kind = 98;
					self.jjmatched_pos = 10;
					return 161;
				}
				return -1;
			}
			_ =>  {
				return -1;
			}
		}
	}

	fn jj_start_nfa_0(&self, pos: i32, active0: i64, active1: i64, active2: i64) -> i32 {
		return self.jj_move_nfa_0(&self.jj_stop_string_literal_dfa_0(pos, active0, active1, active2), pos + 1);
	}

	fn jj_stop_at_pos(&mut self, pos: i32, kind: i32) -> i32 {
		self.jjmatched_kind = kind;
		self.jjmatched_pos = pos;
		return pos + 1;
	}

	fn jj_move_string_literal_dfa0_0(&mut self) -> i32 {
		match self.cur_char {
			10 =>  {
				return self.jj_stop_at_pos(0, 3);
			}
			13 =>  {
				self.jjmatched_kind = 4;
				return self.jj_move_string_literal_dfa1_0(0x4, 0x0, 0x0);
			}
			26 =>  {
				return self.jj_stop_at_pos(0, 151);
			}
			'!' =>  {
				self.jjmatched_kind = 115;
				return self.jj_move_string_literal_dfa1_0(0x0, 0x800000000000000, 0x0);
			}
			'"' =>  {
				return self.jj_move_string_literal_dfa1_0(0x0, 0x80000000, 0x0);
			}
			'%' =>  {
				self.jjmatched_kind = 135;
				return self.jj_move_string_literal_dfa1_0(0x0, 0x0, 0x10000);
			}
			'&' =>  {
				self.jjmatched_kind = 132;
				return self.jj_move_string_literal_dfa1_0(0x0, 0x1000000000000000, 0x2000);
			}
			'(' =>  {
				return self.jj_stop_at_pos(0, 101);
			}
			')' =>  {
				return self.jj_stop_at_pos(0, 102);
			}
			'*' =>  {
				self.jjmatched_kind = 130;
				return self.jj_move_string_literal_dfa1_0(0x0, 0x0, 0x800);
			}
			'+' =>  {
				self.jjmatched_kind = 128;
				return self.jj_move_string_literal_dfa1_0(0x0, 0x4000000000000000, 0x200);
			}
			',' =>  {
				return self.jj_stop_at_pos(0, 108);
			}
			'-' =>  {
				self.jjmatched_kind = 129;
				return self.jj_move_string_literal_dfa1_0(0x0, 0x8080000000000000, 0x400);
			}
			'.' =>  {
				self.jjmatched_kind = 109;
				return self.jj_move_string_literal_dfa1_0(0x0, 0x400000000000, 0x0);
			}
			'/' =>  {
				self.jjmatched_kind = 131;
				return self.jj_move_string_literal_dfa1_0(0x80, 0x0, 0x1000);
			}
			':' =>  {
				self.jjmatched_kind = 118;
				return self.jj_move_string_literal_dfa1_0(0x0, 0x1000000000000, 0x0);
			}
			';' =>  {
				return self.jj_stop_at_pos(0, 107);
			}
			'<' =>  {
				self.jjmatched_kind = 114;
				return self.jj_move_string_literal_dfa1_0(0x0, 0x400000000000000, 0x20100);
			}
			'=' =>  {
				self.jjmatched_kind = 113;
				return self.jj_move_string_literal_dfa1_0(0x0, 0x100000000000000, 0x0);
			}
			'>' =>  {
				self.jjmatched_kind = 150;
				return self.jj_move_string_literal_dfa1_0(0x0, 0x200000000000000, 0x3c0000);
			}
			'?' =>  {
				return self.jj_stop_at_pos(0, 117);
			}
			'@' =>  {
				return self.jj_stop_at_pos(0, 111);
			}
			'[' =>  {
				return self.jj_stop_at_pos(0, 105);
			}
			']' =>  {
				return self.jj_stop_at_pos(0, 106);
			}
			'^' =>  {
				self.jjmatched_kind = 134;
				return self.jj_move_string_literal_dfa1_0(0x0, 0x0, 0x8000);
			}
			'_' =>  {
				return self.jj_start_nfa_with_states_0(0, 152, 162);
			}
			'a' =>  {
				return self.jj_move_string_literal_dfa1_0(0x1800, 0x0, 0x0);
			}
			'b' =>  {
				return self.jj_move_string_literal_dfa1_0(0xe000, 0x0, 0x0);
			}
			'c' =>  {
				return self.jj_move_string_literal_dfa1_0(0x3f0000, 0x0, 0x0);
			}
			'd' =>  {
				return self.jj_move_string_literal_dfa1_0(0x1c00000, 0x0, 0x0);
			}
			'e' =>  {
				return self.jj_move_string_literal_dfa1_0(0xe000000, 0x1000, 0x0);
			}
			'f' =>  {
				return self.jj_move_string_literal_dfa1_0(0x1f0000000, 0x0, 0x0);
			}
			'g' =>  {
				return self.jj_move_string_literal_dfa1_0(0x200000000, 0x0, 0x0);
			}
			'i' =>  {
				return self.jj_move_string_literal_dfa1_0(0xfc00000000, 0x0, 0x0);
			}
			'l' =>  {
				return self.jj_move_string_literal_dfa1_0(0x10000000000, 0x0, 0x0);
			}
			'm' =>  {
				return self.jj_move_string_literal_dfa1_0(0x0, 0x800, 0x0);
			}
			'n' =>  {
				return self.jj_move_string_literal_dfa1_0(0x1e0000000000, 0x0, 0x0);
			}
			'o' =>  {
				return self.jj_move_string_literal_dfa1_0(0x0, 0x300, 0x0);
			}
			'p' =>  {
				return self.jj_move_string_literal_dfa1_0(0x3e00000000000, 0x2000, 0x0);
			}
			'r' =>  {
				return self.jj_move_string_literal_dfa1_0(0xc000000000000, 0x20, 0x0);
			}
			's' =>  {
				return self.jj_move_string_literal_dfa1_0(0x7f0000000000000, 0x0, 0x0);
			}
			't' =>  {
				return self.jj_move_string_literal_dfa1_0(0xf800000000000000, 0x4041, 0x0);
			}
			'u' =>  {
				return self.jj_move_string_literal_dfa1_0(0x0, 0x400, 0x0);
			}
			'v' =>  {
				return self.jj_move_string_literal_dfa1_0(0x0, 0x6, 0x0);
			}
			'w' =>  {
				return self.jj_move_string_literal_dfa1_0(0x0, 0x8088, 0x0);
			}
			'y' =>  {
				return self.jj_move_string_literal_dfa1_0(0x0, 0x10, 0x0);
			}
			'{' =>  {
				return self.jj_stop_at_pos(0, 103);
			}
			'|' =>  {
				self.jjmatched_kind = 133;
				return self.jj_move_string_literal_dfa1_0(0x0, 0x2000000000000000, 0x4000);
			}
			'}' =>  {
				return self.jj_stop_at_pos(0, 104);
			}
			'~' =>  {
				return self.jj_stop_at_pos(0, 116);
			}
			_ =>  {
				return self.jj_move_nfa_0(0, 0);
			}
		}
	}

	fn jj_move_string_literal_dfa1_0(&mut self, active0: i64, active1: i64, active2: i64) /* thrown(java.io.IOException | java.lang.IllegalStateException) */ -> i32 {
		let r0 = 'try0: {
			self.cur_char = match self.input_stream.read_char() {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ java.io.IOException) => {
				self.jj_stop_string_literal_dfa_0(0, active0, active1, active2);
				return 1;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		match self.cur_char {
			10 =>  {
				if (active0 & 0x4) != 0x0 {
					return self.jj_stop_at_pos(1, 2);
				}
	
				break;
			}
			'"' =>  {
				return self.jj_move_string_literal_dfa2_0(active0, 0x0, active1, 0x80000000, active2, 0x0);
			}
			'&' =>  {
				if (active1 & 0x1000000000000000) != 0x0 {
					return self.jj_stop_at_pos(1, 124);
				}
	
				break;
			}
			'*' =>  {
				if (active0 & 0x80) != 0x0 {
					return self.jj_start_nfa_with_states_0(1, 7, 77);
				}
	
				break;
			}
			'+' =>  {
				if (active1 & 0x4000000000000000) != 0x0 {
					return self.jj_stop_at_pos(1, 126);
				}
	
				break;
			}
			'-' =>  {
				if (active1 & 0x8000000000000000) != 0x0 {
					return self.jj_stop_at_pos(1, 127);
				}
	
				break;
			}
			'.' =>  {
				return self.jj_move_string_literal_dfa2_0(active0, 0x0, active1, 0x400000000000, active2, 0x0);
			}
			':' =>  {
				if (active1 & 0x1000000000000) != 0x0 {
					return self.jj_stop_at_pos(1, 112);
				}
	
				break;
			}
			'<' =>  {
				if (active2 & 0x100) != 0x0 {
					self.jjmatched_kind = 136;
					self.jjmatched_pos = 1;
				}
				return self.jj_move_string_literal_dfa2_0(active0, 0x0, active1, 0x0, active2, 0x20000);
			}
			'=' =>  {
				if (active1 & 0x100000000000000) != 0x0 {
					return self.jj_stop_at_pos(1, 120);
				}
				else if (active1 & 0x200000000000000) != 0x0 {
					return self.jj_stop_at_pos(1, 121);
				}
				else if (active1 & 0x400000000000000) != 0x0 {
					return self.jj_stop_at_pos(1, 122);
				}
				else if (active1 & 0x800000000000000) != 0x0 {
					return self.jj_stop_at_pos(1, 123);
				}
				else if (active2 & 0x200) != 0x0 {
					return self.jj_stop_at_pos(1, 137);
				}
				else if (active2 & 0x400) != 0x0 {
					return self.jj_stop_at_pos(1, 138);
				}
				else if (active2 & 0x800) != 0x0 {
					return self.jj_stop_at_pos(1, 139);
				}
				else if (active2 & 0x1000) != 0x0 {
					return self.jj_stop_at_pos(1, 140);
				}
				else if (active2 & 0x2000) != 0x0 {
					return self.jj_stop_at_pos(1, 141);
				}
				else if (active2 & 0x4000) != 0x0 {
					return self.jj_stop_at_pos(1, 142);
				}
				else if (active2 & 0x8000) != 0x0 {
					return self.jj_stop_at_pos(1, 143);
				}
				else if (active2 & 0x10000) != 0x0 {
					return self.jj_stop_at_pos(1, 144);
				}
	
				break;
			}
			'>' =>  {
				if (active1 & 0x80000000000000) != 0x0 {
					return self.jj_stop_at_pos(1, 119);
				}
				else if (active2 & 0x200000) != 0x0 {
					self.jjmatched_kind = 149;
					self.jjmatched_pos = 1;
				}
				return self.jj_move_string_literal_dfa2_0(active0, 0x0, active1, 0x0, active2, 0x1c0000);
			}
			'a' =>  {
				return self.jj_move_string_literal_dfa2_0(active0, 0x220010030000, active1, 0x0, active2, 0x0);
			}
			'b' =>  {
				return self.jj_move_string_literal_dfa2_0(active0, 0x800, active1, 0x0, active2, 0x0);
			}
			'e' =>  {
				return self.jj_move_string_literal_dfa2_0(active0, 0x1c440000400000, active1, 0x20, active2, 0x0);
			}
			'f' =>  {
				if (active0 & 0x400000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(1, 34, 161);
				}
	
				break;
			}
			'h' =>  {
				return self.jj_move_string_literal_dfa2_0(active0, 0x3820000000040000, active1, 0x8008, active2, 0x0);
			}
			'i' =>  {
				return self.jj_move_string_literal_dfa2_0(active0, 0x60000000, active1, 0x90, active2, 0x0);
			}
			'l' =>  {
				return self.jj_move_string_literal_dfa2_0(active0, 0x82080000, active1, 0x0, active2, 0x0);
			}
			'm' =>  {
				return self.jj_move_string_literal_dfa2_0(active0, 0x1800000000, active1, 0x0, active2, 0x0);
			}
			'n' =>  {
				return self.jj_move_string_literal_dfa2_0(active0, 0xe004000000, active1, 0x0, active2, 0x0);
			}
			'o' =>  {
				if (active0 & 0x800000) != 0x0 {
					self.jjmatched_kind = 23;
					self.jjmatched_pos = 1;
				} else if (active1 & 0x40) != 0x0 {
					return self.jj_start_nfa_with_states_0(1, 70, 161);
				}
	
				return self.jj_move_string_literal_dfa2_0(active0, 0x90301302000, active1, 0x806, active2, 0x0);
			}
			'p' =>  {
				return self.jj_move_string_literal_dfa2_0(active0, 0x0, active1, 0x300, active2, 0x0);
			}
			'r' =>  {
				return self.jj_move_string_literal_dfa2_0(active0, 0xc001800000004000, active1, 0x6001, active2, 0x0);
			}
			's' =>  {
				return self.jj_move_string_literal_dfa2_0(active0, 0x1000, active1, 0x400, active2, 0x0);
			}
			't' =>  {
				return self.jj_move_string_literal_dfa2_0(active0, 0xc0000000000000, active1, 0x0, active2, 0x0);
			}
			'u' =>  {
				return self.jj_move_string_literal_dfa2_0(active0, 0x102100000000000, active1, 0x0, active2, 0x0);
			}
			'w' =>  {
				return self.jj_move_string_literal_dfa2_0(active0, 0x200000000000000, active1, 0x0, active2, 0x0);
			}
			'x' =>  {
				return self.jj_move_string_literal_dfa2_0(active0, 0x8000000, active1, 0x1000, active2, 0x0);
			}
			'y' =>  {
				return self.jj_move_string_literal_dfa2_0(active0, 0x400000000008000, active1, 0x0, active2, 0x0);
			}
			'|' =>  {
				if (active1 & 0x2000000000000000) != 0x0 {
					return self.jj_stop_at_pos(1, 125);
				}
	
				break;
			}
			_ =>  {
				break;
			}
		}
		return self.jj_start_nfa_0(0, active0, active1, active2);
	}

	fn jj_move_string_literal_dfa2_0(&mut self, old0: i64, mut active0: i64, old1: i64, mut active1: i64, old2: i64, mut active2: i64) /* thrown(java.io.IOException | java.lang.IllegalStateException) */ -> i32 {
		if ((active0 &= old0) | (active1 &= old1) | (active2 &= old2)) == 0 {
			return self.jj_start_nfa_0(0, old0, old1, old2);
		}
	
		let r0 = 'try0: {
			self.cur_char = match self.input_stream.read_char() {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ java.io.IOException) => {
				self.jj_stop_string_literal_dfa_0(1, active0, active1, active2);
				return 2;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		match self.cur_char {
			'"' =>  {
				if (active1 & 0x80000000) != 0x0 {
					return self.jj_stop_at_pos(2, 95);
				}
	
				break;
			}
			'.' =>  {
				if (active1 & 0x400000000000) != 0x0 {
					return self.jj_stop_at_pos(2, 110);
				}
	
				break;
			}
			'=' =>  {
				if (active2 & 0x20000) != 0x0 {
					return self.jj_stop_at_pos(2, 145);
				}
				else if (active2 & 0x40000) != 0x0 {
					return self.jj_stop_at_pos(2, 146);
				}
	
				break;
			}
			'>' =>  {
				if (active2 & 0x100000) != 0x0 {
					self.jjmatched_kind = 148;
					self.jjmatched_pos = 2;
				}
				return self.jj_move_string_literal_dfa3_0(active0, 0x0, active1, 0x0, active2, 0x80000);
			}
			'a' =>  {
				return self.jj_move_string_literal_dfa3_0(active0, 0x40500000000c0000, active1, 0x4000, active2, 0x0);
			}
			'b' =>  {
				return self.jj_move_string_literal_dfa3_0(active0, 0x2000000000000, active1, 0x0, active2, 0x0);
			}
			'c' =>  {
				return self.jj_move_string_literal_dfa3_0(active0, 0x4200000000000, active1, 0x0, active2, 0x0);
			}
			'd' =>  {
				return self.jj_move_string_literal_dfa3_0(active0, 0x0, active1, 0x800, active2, 0x0);
			}
			'e' =>  {
				return self.jj_move_string_literal_dfa3_0(active0, 0x4000, active1, 0x8710, active2, 0x0);
			}
			'f' =>  {
				return self.jj_move_string_literal_dfa3_0(active0, 0x400000, active1, 0x0, active2, 0x0);
			}
			'i' =>  {
				return self.jj_move_string_literal_dfa3_0(active0, 0xa00800000000000, active1, 0xa, active2, 0x0);
			}
			'l' =>  {
				return self.jj_move_string_literal_dfa3_0(active0, 0x100010000000, active1, 0x4, active2, 0x0);
			}
			'n' =>  {
				return self.jj_move_string_literal_dfa3_0(active0, 0x400090060300000, active1, 0x0, active2, 0x0);
			}
			'o' =>  {
				return self.jj_move_string_literal_dfa3_0(active0, 0x21000080002000, active1, 0x2000, active2, 0x0);
			}
			'p' =>  {
				return self.jj_move_string_literal_dfa3_0(active0, 0x100001800000000, active1, 0x1000, active2, 0x0);
			}
			'q' =>  {
				return self.jj_move_string_literal_dfa3_0(active0, 0x0, active1, 0x20, active2, 0x0);
			}
			'r' =>  {
				if (active0 & 0x100000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(2, 32, 161);
				}
	
				return self.jj_move_string_literal_dfa3_0(active0, 0x3080400000000000, active1, 0x0, active2, 0x0);
			}
			's' =>  {
				return self.jj_move_string_literal_dfa3_0(active0, 0x2002011800, active1, 0x0, active2, 0x0);
			}
			't' =>  {
				if (active0 & 0x4000000000) != 0x0 {
					self.jjmatched_kind = 38;
					self.jjmatched_pos = 2;
				}
				return self.jj_move_string_literal_dfa3_0(active0, 0x8028208028000, active1, 0x80, active2, 0x0);
			}
			'u' =>  {
				return self.jj_move_string_literal_dfa3_0(active0, 0x8000000005000000, active1, 0x0, active2, 0x0);
			}
			'w' =>  {
				if (active0 & 0x40000000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(2, 42, 161);
				}
	
				break;
			}
			'y' =>  {
				if (active1 & 0x1) != 0x0 {
					return self.jj_start_nfa_with_states_0(2, 64, 161);
				}
	
				break;
			}
			_ =>  {
				break;
			}
		}
		return self.jj_start_nfa_0(1, active0, active1, active2);
	}

	fn jj_move_string_literal_dfa3_0(&mut self, old0: i64, mut active0: i64, old1: i64, mut active1: i64, old2: i64, mut active2: i64) /* thrown(java.io.IOException | java.lang.IllegalStateException) */ -> i32 {
		if ((active0 &= old0) | (active1 &= old1) | (active2 &= old2)) == 0 {
			return self.jj_start_nfa_0(1, old0, old1, old2);
		}
	
		let r0 = 'try0: {
			self.cur_char = match self.input_stream.read_char() {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ java.io.IOException) => {
				self.jj_stop_string_literal_dfa_0(2, active0, active1, active2);
				return 3;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		match self.cur_char {
			'-' =>  {
				return self.jj_move_string_literal_dfa4_0(active0, 0x80000000000, active1, 0x0, active2, 0x0);
			}
			'=' =>  {
				if (active2 & 0x80000) != 0x0 {
					return self.jj_stop_at_pos(3, 147);
				}
	
				break;
			}
			'a' =>  {
				return self.jj_move_string_literal_dfa4_0(active0, 0xe0404000, active1, 0x4, active2, 0x0);
			}
			'b' =>  {
				return self.jj_move_string_literal_dfa4_0(active0, 0x1000000, active1, 0x0, active2, 0x0);
			}
			'c' =>  {
				return self.jj_move_string_literal_dfa4_0(active0, 0x400000000020000, active1, 0x0, active2, 0x0);
			}
			'd' =>  {
				if (active1 & 0x2) != 0x0 {
					return self.jj_start_nfa_with_states_0(3, 65, 161);
				}
	
				break;
			}
			'e' =>  {
				if (active0 & 0x8000) != 0x0 {
					return self.jj_start_nfa_with_states_0(3, 15, 161);
				}
				else if (active0 & 0x10000) != 0x0 {
					return self.jj_start_nfa_with_states_0(3, 16, 161);
				}
				else if (active0 & 0x2000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(3, 25, 161);
				}
				else if (active0 & 0x8000000000000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(3, 63, 161);
				}
	
				return self.jj_move_string_literal_dfa4_0(active0, 0x100008008001000, active1, 0x0, active2, 0x0);
			}
			'g' =>  {
				if (active0 & 0x10000000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(3, 40, 161);
				}
	
				break;
			}
			'h' =>  {
				if (active1 & 0x80) != 0x0 {
					return self.jj_start_nfa_with_states_0(3, 71, 161);
				}
	
				break;
			}
			'i' =>  {
				return self.jj_move_string_literal_dfa4_0(active0, 0x80020000000000, active1, 0x0, active2, 0x0);
			}
			'k' =>  {
				return self.jj_move_string_literal_dfa4_0(active0, 0x200000000000, active1, 0x0, active2, 0x0);
			}
			'l' =>  {
				if (active0 & 0x100000000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(3, 44, 161);
				}
	
				return self.jj_move_string_literal_dfa4_0(active0, 0x12000800002000, active1, 0x18, active2, 0x0);
			}
			'm' =>  {
				if (active0 & 0x4000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(3, 26, 161);
				}
	
				return self.jj_move_string_literal_dfa4_0(active0, 0x400000000000, active1, 0x0, active2, 0x0);
			}
			'n' =>  {
				if (active1 & 0x100) != 0x0 {
					self.jjmatched_kind = 72;
					self.jjmatched_pos = 3;
				} else if (active1 & 0x8000) != 0x0 {
					return self.jj_start_nfa_with_states_0(3, 79, 161);
				}
	
				return self.jj_move_string_literal_dfa4_0(active0, 0x4000000000000000, active1, 0x4200, active2, 0x0);
			}
			'o' =>  {
				if (active0 & 0x200000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(3, 33, 161);
				}
	
				return self.jj_move_string_literal_dfa4_0(active0, 0x3004001000000000, active1, 0x1000, active2, 0x0);
			}
			'r' =>  {
				if (active0 & 0x40000) != 0x0 {
					return self.jj_start_nfa_with_states_0(3, 18, 161);
				}
	
				return self.jj_move_string_literal_dfa4_0(active0, 0x20000000000000, active1, 0x0, active2, 0x0);
			}
			's' =>  {
				if (active0 & 0x800000000000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(3, 59, 161);
				}
				else if (active1 & 0x400) != 0x0 {
					return self.jj_start_nfa_with_states_0(3, 74, 161);
				}
	
				return self.jj_move_string_literal_dfa4_0(active0, 0x10180000, active1, 0x0, active2, 0x0);
			}
			't' =>  {
				return self.jj_move_string_literal_dfa4_0(active0, 0x241002000200800, active1, 0x0, active2, 0x0);
			}
			'u' =>  {
				return self.jj_move_string_literal_dfa4_0(active0, 0x8000000000000, active1, 0x820, active2, 0x0);
			}
			'v' =>  {
				return self.jj_move_string_literal_dfa4_0(active0, 0x800000000000, active1, 0x2000, active2, 0x0);
			}
			_ =>  {
				break;
			}
		}
		return self.jj_start_nfa_0(2, active0, active1, active2);
	}

	fn jj_move_string_literal_dfa4_0(&mut self, old0: i64, mut active0: i64, old1: i64, mut active1: i64, old2: i64, mut active2: i64) /* thrown(java.io.IOException | java.lang.IllegalStateException) */ -> i32 {
		if ((active0 &= old0) | (active1 &= old1) | (active2 &= old2)) == 0 {
			return self.jj_start_nfa_0(2, old0, old1, old2);
		}
	
		let r0 = 'try0: {
			self.cur_char = match self.input_stream.read_char() {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ java.io.IOException) => {
				self.jj_stop_string_literal_dfa_0(3, active0, active1, 0);
				return 4;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		match self.cur_char {
			'a' =>  {
				return self.jj_move_string_literal_dfa5_0(active0, 0xa02000000000, active1, 0x0);
			}
			'c' =>  {
				return self.jj_move_string_literal_dfa5_0(active0, 0x280000000000000, active1, 0x0);
			}
			'd' =>  {
				if (active1 & 0x10) != 0x0 {
					return self.jj_start_nfa_with_states_0(4, 68, 161);
				}
	
				break;
			}
			'e' =>  {
				if (active0 & 0x10000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(4, 28, 161);
				}
				else if (active1 & 0x8) != 0x0 {
					return self.jj_start_nfa_with_states_0(4, 67, 161);
				}
	
				return self.jj_move_string_literal_dfa5_0(active0, 0x11000800002000, active1, 0x0);
			}
			'h' =>  {
				if (active0 & 0x20000) != 0x0 {
					return self.jj_start_nfa_with_states_0(4, 17, 161);
				}
	
				return self.jj_move_string_literal_dfa5_0(active0, 0x400000000000000, active1, 0x0);
			}
			'i' =>  {
				return self.jj_move_string_literal_dfa5_0(active0, 0x42400000200000, active1, 0x2020);
			}
			'k' =>  {
				if (active0 & 0x4000) != 0x0 {
					return self.jj_start_nfa_with_states_0(4, 14, 161);
				}
	
				break;
			}
			'l' =>  {
				if (active0 & 0x20000000) != 0x0 {
					self.jjmatched_kind = 29;
					self.jjmatched_pos = 4;
				}
				return self.jj_move_string_literal_dfa5_0(active0, 0x41000000, active1, 0x800);
			}
			'n' =>  {
				return self.jj_move_string_literal_dfa5_0(active0, 0x8000000, active1, 0x0);
			}
			'r' =>  {
				if (active0 & 0x100000000000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(4, 56, 161);
				}
	
				return self.jj_move_string_literal_dfa5_0(active0, 0xc009000001800, active1, 0x1000);
			}
			's' =>  {
				if (active0 & 0x80000) != 0x0 {
					return self.jj_start_nfa_with_states_0(4, 19, 161);
				}
				else if (active1 & 0x200) != 0x0 {
					return self.jj_start_nfa_with_states_0(4, 73, 161);
				}
	
				return self.jj_move_string_literal_dfa5_0(active0, 0x4000080000000000, active1, 0x4000);
			}
			't' =>  {
				if (active0 & 0x100000) != 0x0 {
					return self.jj_start_nfa_with_states_0(4, 20, 161);
				}
				else if (active0 & 0x80000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(4, 31, 161);
				}
				else if (active0 & 0x20000000000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(4, 53, 161);
				}
	
				return self.jj_move_string_literal_dfa5_0(active0, 0x0, active1, 0x4);
			}
			'u' =>  {
				return self.jj_move_string_literal_dfa5_0(active0, 0x400000, active1, 0x0);
			}
			'v' =>  {
				return self.jj_move_string_literal_dfa5_0(active0, 0x20000000000, active1, 0x0);
			}
			'w' =>  {
				if (active0 & 0x1000000000000000) != 0x0 {
					self.jjmatched_kind = 60;
					self.jjmatched_pos = 4;
				}
				return self.jj_move_string_literal_dfa5_0(active0, 0x2000000000000000, active1, 0x0);
			}
			_ =>  {
				break;
			}
		}
		return self.jj_start_nfa_0(3, active0, active1, 0);
	}

	fn jj_move_string_literal_dfa5_0(&mut self, old0: i64, mut active0: i64, old1: i64, mut active1: i64) /* thrown(java.io.IOException | java.lang.IllegalStateException) */ -> i32 {
		if ((active0 &= old0) | (active1 &= old1)) == 0 {
			return self.jj_start_nfa_0(3, old0, old1, 0);
		}
	
		let r0 = 'try0: {
			self.cur_char = match self.input_stream.read_char() {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ java.io.IOException) => {
				self.jj_stop_string_literal_dfa_0(4, active0, active1, 0);
				return 5;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		match self.cur_char {
			'a' =>  {
				return self.jj_move_string_literal_dfa6_0(active0, 0x2800, active1, 0x0);
			}
			'c' =>  {
				if (active0 & 0x2000000000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(5, 49, 161);
				}
				else if (active0 & 0x40000000000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(5, 54, 161);
				}
	
				return self.jj_move_string_literal_dfa6_0(active0, 0x1000000000000, active1, 0x0);
			}
			'd' =>  {
				if (active0 & 0x4000000000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(5, 50, 161);
				}
				else if (active0 & 0x10000000000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(5, 52, 161);
				}
	
				return self.jj_move_string_literal_dfa6_0(active0, 0x8000000, active1, 0x2000);
			}
			'e' =>  {
				if (active0 & 0x1000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(5, 24, 161);
				}
				else if (active0 & 0x20000000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(5, 41, 161);
				}
				else if (active1 & 0x800) != 0x0 {
					return self.jj_start_nfa_with_states_0(5, 75, 161);
				}
	
				return self.jj_move_string_literal_dfa6_0(active0, 0x80000000000, active1, 0x0);
			}
			'f' =>  {
				return self.jj_move_string_literal_dfa6_0(active0, 0x8000000000, active1, 0x0);
			}
			'g' =>  {
				return self.jj_move_string_literal_dfa6_0(active0, 0x200000000000, active1, 0x0);
			}
			'h' =>  {
				if (active0 & 0x200000000000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(5, 57, 161);
				}
	
				break;
			}
			'i' =>  {
				return self.jj_move_string_literal_dfa6_0(active0, 0x4000000000000000, active1, 0x4004);
			}
			'l' =>  {
				return self.jj_move_string_literal_dfa6_0(active0, 0x40400000, active1, 0x0);
			}
			'm' =>  {
				return self.jj_move_string_literal_dfa6_0(active0, 0x800000000, active1, 0x0);
			}
			'n' =>  {
				if (active0 & 0x8000000000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(5, 51, 161);
				}
	
				return self.jj_move_string_literal_dfa6_0(active0, 0x2000200000, active1, 0x0);
			}
			'r' =>  {
				return self.jj_move_string_literal_dfa6_0(active0, 0x400000000000000, active1, 0x20);
			}
			's' =>  {
				if (active0 & 0x2000000000000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(5, 61, 161);
				}
	
				break;
			}
			't' =>  {
				if (active0 & 0x1000) != 0x0 {
					return self.jj_start_nfa_with_states_0(5, 12, 161);
				}
				else if (active0 & 0x1000000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(5, 36, 161);
				}
	
				return self.jj_move_string_literal_dfa6_0(active0, 0x80c00000000000, active1, 0x1000);
			}
			_ =>  {
				break;
			}
		}
		return self.jj_start_nfa_0(4, active0, active1, 0);
	}

	fn jj_move_string_literal_dfa6_0(&mut self, old0: i64, mut active0: i64, old1: i64, mut active1: i64) /* thrown(java.io.IOException | java.lang.IllegalStateException) */ -> i32 {
		if ((active0 &= old0) | (active1 &= old1)) == 0 {
			return self.jj_start_nfa_0(4, old0, old1, 0);
		}
	
		let r0 = 'try0: {
			self.cur_char = match self.input_stream.read_char() {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ java.io.IOException) => {
				self.jj_stop_string_literal_dfa_0(5, active0, active1, 0);
				return 6;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		match self.cur_char {
			'a' =>  {
				return self.jj_move_string_literal_dfa7_0(active0, 0x88000000000, active1, 0x0);
			}
			'c' =>  {
				return self.jj_move_string_literal_dfa7_0(active0, 0x2000000800, active1, 0x0);
			}
			'e' =>  {
				if (active0 & 0x200000000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(6, 45, 161);
				}
				else if (active0 & 0x800000000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(6, 47, 161);
				}
	
				return self.jj_move_string_literal_dfa7_0(active0, 0x4000000800000000, active1, 0x2020);
			}
			'f' =>  {
				return self.jj_move_string_literal_dfa7_0(active0, 0x80000000000000, active1, 0x0);
			}
			'l' =>  {
				return self.jj_move_string_literal_dfa7_0(active0, 0x0, active1, 0x4);
			}
			'n' =>  {
				if (active0 & 0x2000) != 0x0 {
					return self.jj_start_nfa_with_states_0(6, 13, 161);
				}
	
				break;
			}
			'o' =>  {
				return self.jj_move_string_literal_dfa7_0(active0, 0x400000000000000, active1, 0x0);
			}
			's' =>  {
				if (active0 & 0x8000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(6, 27, 161);
				}
				else if (active0 & 0x400000000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(6, 46, 161);
				}
				else if (active1 & 0x1000) != 0x0 {
					return self.jj_start_nfa_with_states_0(6, 76, 161);
				}
	
				break;
			}
			't' =>  {
				if (active0 & 0x400000) != 0x0 {
					return self.jj_start_nfa_with_states_0(6, 22, 161);
				}
	
				return self.jj_move_string_literal_dfa7_0(active0, 0x1000000000000, active1, 0x4000);
			}
			'u' =>  {
				return self.jj_move_string_literal_dfa7_0(active0, 0x200000, active1, 0x0);
			}
			'y' =>  {
				if (active0 & 0x40000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(6, 30, 161);
				}
	
				break;
			}
			_ =>  {
				break;
			}
		}
		return self.jj_start_nfa_0(5, active0, active1, 0);
	}

	fn jj_move_string_literal_dfa7_0(&mut self, old0: i64, mut active0: i64, old1: i64, mut active1: i64) /* thrown(java.io.IOException | java.lang.IllegalStateException) */ -> i32 {
		if ((active0 &= old0) | (active1 &= old1)) == 0 {
			return self.jj_start_nfa_0(5, old0, old1, 0);
		}
	
		let r0 = 'try0: {
			self.cur_char = match self.input_stream.read_char() {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ java.io.IOException) => {
				self.jj_stop_string_literal_dfa_0(6, active0, active1, 0);
				return 7;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		match self.cur_char {
			'c' =>  {
				return self.jj_move_string_literal_dfa8_0(active0, 0x8000000000, active1, 0x0);
			}
			'e' =>  {
				if (active0 & 0x200000) != 0x0 {
					return self.jj_start_nfa_with_states_0(7, 21, 161);
				}
				else if (active1 & 0x4) != 0x0 {
					return self.jj_start_nfa_with_states_0(7, 66, 161);
				}
	
				return self.jj_move_string_literal_dfa8_0(active0, 0x1002000000000, active1, 0x0);
			}
			'i' =>  {
				return self.jj_move_string_literal_dfa8_0(active0, 0x0, active1, 0x4000);
			}
			'l' =>  {
				return self.jj_move_string_literal_dfa8_0(active0, 0x80000000000, active1, 0x0);
			}
			'n' =>  {
				return self.jj_move_string_literal_dfa8_0(active0, 0x4400000800000000, active1, 0x0);
			}
			'p' =>  {
				if (active0 & 0x80000000000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(7, 55, 161);
				}
	
				break;
			}
			's' =>  {
				if (active1 & 0x20) != 0x0 {
					return self.jj_start_nfa_with_states_0(7, 69, 161);
				}
				else if (active1 & 0x2000) != 0x0 {
					return self.jj_start_nfa_with_states_0(7, 77, 161);
				}
	
				break;
			}
			't' =>  {
				if (active0 & 0x800) != 0x0 {
					return self.jj_start_nfa_with_states_0(7, 11, 161);
				}
	
				break;
			}
			_ =>  {
				break;
			}
		}
		return self.jj_start_nfa_0(6, active0, active1, 0);
	}

	fn jj_move_string_literal_dfa8_0(&mut self, old0: i64, mut active0: i64, old1: i64, mut active1: i64) /* thrown(java.io.IOException | java.lang.IllegalStateException) */ -> i32 {
		if ((active0 &= old0) | (active1 &= old1)) == 0 {
			return self.jj_start_nfa_0(6, old0, old1, 0);
		}
	
		let r0 = 'try0: {
			self.cur_char = match self.input_stream.read_char() {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ java.io.IOException) => {
				self.jj_stop_string_literal_dfa_0(7, active0, active1, 0);
				return 8;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		match self.cur_char {
			'd' =>  {
				if (active0 & 0x1000000000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(8, 48, 161);
				}
	
				break;
			}
			'e' =>  {
				if (active0 & 0x8000000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(8, 39, 161);
				}
	
				return self.jj_move_string_literal_dfa9_0(active0, 0x80000000000, active1, 0x0);
			}
			'i' =>  {
				return self.jj_move_string_literal_dfa9_0(active0, 0x400000000000000, active1, 0x0);
			}
			'o' =>  {
				return self.jj_move_string_literal_dfa9_0(active0, 0x2000000000, active1, 0x0);
			}
			't' =>  {
				if (active0 & 0x4000000000000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(8, 62, 161);
				}
	
				return self.jj_move_string_literal_dfa9_0(active0, 0x800000000, active1, 0x0);
			}
			'v' =>  {
				return self.jj_move_string_literal_dfa9_0(active0, 0x0, active1, 0x4000);
			}
			_ =>  {
				break;
			}
		}
		return self.jj_start_nfa_0(7, active0, active1, 0);
	}

	fn jj_move_string_literal_dfa9_0(&mut self, old0: i64, mut active0: i64, old1: i64, mut active1: i64) /* thrown(java.io.IOException | java.lang.IllegalStateException) */ -> i32 {
		if ((active0 &= old0) | (active1 &= old1)) == 0 {
			return self.jj_start_nfa_0(7, old0, old1, 0);
		}
	
		let r0 = 'try0: {
			self.cur_char = match self.input_stream.read_char() {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ java.io.IOException) => {
				self.jj_stop_string_literal_dfa_0(8, active0, active1, 0);
				return 9;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		match self.cur_char {
			'd' =>  {
				if (active0 & 0x80000000000) != 0x0 {
					return self.jj_stop_at_pos(9, 43);
				}
	
				break;
			}
			'e' =>  {
				if (active1 & 0x4000) != 0x0 {
					return self.jj_start_nfa_with_states_0(9, 78, 161);
				}
	
				break;
			}
			'f' =>  {
				if (active0 & 0x2000000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(9, 37, 161);
				}
	
				break;
			}
			's' =>  {
				if (active0 & 0x800000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(9, 35, 161);
				}
	
				break;
			}
			'z' =>  {
				return self.jj_move_string_literal_dfa10_0(active0, 0x400000000000000, active1, 0x0);
			}
			_ =>  {
				break;
			}
		}
		return self.jj_start_nfa_0(8, active0, active1, 0);
	}

	fn jj_move_string_literal_dfa10_0(&mut self, old0: i64, mut active0: i64, old1: i64, mut active1: i64) /* thrown(java.io.IOException | java.lang.IllegalStateException) */ -> i32 {
		if ((active0 &= old0) | (active1 &= old1)) == 0 {
			return self.jj_start_nfa_0(8, old0, old1, 0);
		}
	
		let r0 = 'try0: {
			self.cur_char = match self.input_stream.read_char() {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ java.io.IOException) => {
				self.jj_stop_string_literal_dfa_0(9, active0, 0, 0);
				return 10;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		match self.cur_char {
			'e' =>  {
				return self.jj_move_string_literal_dfa11_0(active0, 0x400000000000000);
			}
			_ =>  {
				break;
			}
		}
		return self.jj_start_nfa_0(9, active0, 0, 0);
	}

	fn jj_move_string_literal_dfa11_0(&mut self, old0: i64, mut active0: i64) /* thrown(java.io.IOException | java.lang.IllegalStateException) */ -> i32 {
		if ((active0 &= old0)) == 0 {
			return self.jj_start_nfa_0(9, old0, 0, 0);
		}
	
		let r0 = 'try0: {
			self.cur_char = match self.input_stream.read_char() {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ java.io.IOException) => {
				self.jj_stop_string_literal_dfa_0(10, active0, 0, 0);
				return 11;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		match self.cur_char {
			'd' =>  {
				if (active0 & 0x400000000000000) != 0x0 {
					return self.jj_start_nfa_with_states_0(11, 58, 161);
				}
	
				break;
			}
			_ =>  {
				break;
			}
		}
		return self.jj_start_nfa_0(10, active0, 0, 0);
	}

	fn jj_start_nfa_with_states_0(&mut self, pos: i32, kind: i32, state: i32) /* thrown(java.io.IOException | java.lang.IllegalStateException) */ -> i32 {
		self.jjmatched_kind = kind;
		self.jjmatched_pos = pos;
		let r0 = 'try0: {
			self.cur_char = match self.input_stream.read_char() {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ java.io.IOException) => {
				return pos + 1;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		return self.jj_move_nfa_0(state, pos + 1);
	}

	fn jj_move_nfa_0(&mut self, start_state: i32, cur_pos: i32) /* thrown(java.io.IOException | java.lang.IllegalStateException) */ -> i32 {
		let starts_at: i32 = 0;
		self.jjnew_state_cnt = 160;
		let i: i32 = 1;
		self.jjstate_set[0] = start_state;
		let kind: i32 = 0x7fffffff;
		loop {
			if self.jjround += 1 == 0x7fffffff {
				self.re_init_rounds();
			}
	
			if self.cur_char < 64 {
				let l: i64 = 1 << self.cur_char;
				loop { {
					match self.jjstate_set[i -= 1] {
						160 =>  {
							if (0xfffffffbffffdbff & l) != 0x0 {
								self.jj_checkn_add_states(0, 3);
							} else if self.cur_char == 34 {
								if kind > 94 {
									kind = 94;
								}
	
							}
							break;
						}
						161 =>  {
						}
						54 =>  {
							if (0x3ff00100fffc1ff & l) == 0x0 {
								break;
							}
	
							if kind > 98 {
								kind = 98;
							}
	
							{
								self.jj_checkn_add_two_states(54, 60);
							}
							break;
						}
						162 =>  {
						}
						67 =>  {
							if (0x3ff00100fffc1ff & l) == 0x0 {
								break;
							}
	
							if kind > 98 {
								kind = 98;
							}
	
							{
								self.jj_checkn_add_two_states(67, 73);
							}
							break;
						}
						75 =>  {
							if self.cur_char == 42 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 77;
							}
							else if self.cur_char == 47 {
								if kind > 5 {
									kind = 5;
								}
	
								{
									self.jj_checkn_add(76);
								}
							}
							break;
						}
						0 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								if kind > 81 {
									kind = 81;
								}
	
								{
									self.jj_checkn_add_states(4, 18);
								}
							} else if (0x100001200 & l) != 0x0 {
								if kind > 1 {
									kind = 1;
								}
	
							} else if self.cur_char == 47 {
								self.jj_add_states(19, 20);
							} else if self.cur_char == 36 {
								if kind > 98 {
									kind = 98;
								}
	
								{
									self.jj_checkn_add_two_states(54, 60);
								}
							} else if self.cur_char == 34 {
								self.jj_checkn_add_states(0, 3);
							} else if self.cur_char == 39 {
								self.jj_add_states(21, 23);
							} else if self.cur_char == 46 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 2;
							}
	
							if self.cur_char == 48 {
								self.jj_add_states(24, 31);
							}
							break;
						}
						1 =>  {
							if self.cur_char == 46 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 2;
							}
	
							break;
						}
						2 =>  {
							if (0x3ff000000000000 & l) == 0x0 {
								break;
							}
	
							if kind > 86 {
								kind = 86;
							}
	
							{
								self.jj_checkn_add_states(32, 35);
							}
							break;
						}
						3 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add_two_states(3, 4);
							}
							break;
						}
						4 =>  {
							if (0x3ff000000000000 & l) == 0x0 {
								break;
							}
	
							if kind > 86 {
								kind = 86;
							}
	
							{
								self.jj_checkn_add_two_states(5, 10);
							}
							break;
						}
						6 =>  {
							if (0x280000000000 & l) != 0x0 {
								self.jj_checkn_add(7);
							}
							break;
						}
						7 =>  {
							if (0x3ff000000000000 & l) == 0x0 {
								break;
							}
	
							if kind > 86 {
								kind = 86;
							}
	
							{
								self.jj_checkn_add_states(36, 38);
							}
							break;
						}
						8 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add_two_states(8, 9);
							}
							break;
						}
						9 =>  {
							if (0x3ff000000000000 & l) == 0x0 {
								break;
							}
	
							if kind > 86 {
								kind = 86;
							}
	
							{
								self.jj_checkn_add_two_states(7, 10);
							}
							break;
						}
						11 =>  {
							if (0x3ff000000000000 & l) == 0x0 {
								break;
							}
	
							if kind > 86 {
								kind = 86;
							}
	
							{
								self.jj_checkn_add_states(39, 42);
							}
							break;
						}
						12 =>  {
							if self.cur_char == 39 {
								self.jj_add_states(21, 23);
							}
							break;
						}
						13 =>  {
							if (0xffffff7fffffdbff & l) != 0x0 {
								self.jj_checkn_add(14);
							}
							break;
						}
						14 =>  {
							if self.cur_char == 39 && kind > 93 {
								kind = 93;
							}
	
							break;
						}
						16 =>  {
							if (0x8400000000 & l) != 0x0 {
								self.jj_checkn_add(14);
							}
							break;
						}
						17 =>  {
							if (0xff000000000000 & l) != 0x0 {
								self.jj_checkn_add_two_states(18, 14);
							}
							break;
						}
						18 =>  {
							if (0xff000000000000 & l) != 0x0 {
								self.jj_checkn_add(14);
							}
							break;
						}
						19 =>  {
							if (0xf000000000000 & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 20;
							}
	
							break;
						}
						20 =>  {
							if (0xff000000000000 & l) != 0x0 {
								self.jj_checkn_add(18);
							}
							break;
						}
						22 =>  {
							if self.cur_char == 53 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 23;
							}
	
							break;
						}
						24 =>  {
							if self.cur_char == 53 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 25;
							}
	
							break;
						}
						26 =>  {
							if self.cur_char == 48 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 24;
							}
	
							break;
						}
						27 =>  {
							if self.cur_char == 48 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 26;
							}
	
							break;
						}
						30 =>  {
							if self.cur_char == 48 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 22;
							}
	
							break;
						}
						31 =>  {
							if self.cur_char == 48 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 30;
							}
	
							break;
						}
						34 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 35;
							}
	
							break;
						}
						35 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 36;
							}
	
							break;
						}
						36 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 37;
							}
	
							break;
						}
						37 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add(14);
							}
							break;
						}
						38 =>  {
							if self.cur_char == 34 {
								self.jj_checkn_add_states(0, 3);
							}
							break;
						}
						39 =>  {
							if (0xfffffffbffffdbff & l) != 0x0 {
								self.jj_checkn_add_states(0, 3);
							}
							break;
						}
						41 =>  {
							if (0x8400000000 & l) != 0x0 {
								self.jj_checkn_add_states(0, 3);
							}
							break;
						}
						43 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 44;
							}
	
							break;
						}
						44 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 45;
							}
	
							break;
						}
						45 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 46;
							}
	
							break;
						}
						46 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add_states(0, 3);
							}
							break;
						}
						48 =>  {
							if self.cur_char == 34 && kind > 94 {
								kind = 94;
							}
	
							break;
						}
						49 =>  {
							if (0xff000000000000 & l) != 0x0 {
								self.jj_checkn_add_states(43, 47);
							}
							break;
						}
						50 =>  {
							if (0xff000000000000 & l) != 0x0 {
								self.jj_checkn_add_states(0, 3);
							}
							break;
						}
						51 =>  {
							if (0xf000000000000 & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 52;
							}
	
							break;
						}
						52 =>  {
							if (0xff000000000000 & l) != 0x0 {
								self.jj_checkn_add(50);
							}
							break;
						}
						53 =>  {
							if self.cur_char != 36 {
								break;
							}
	
							if kind > 98 {
								kind = 98;
							}
	
							{
								self.jj_checkn_add_two_states(54, 60);
							}
							break;
						}
						56 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 57;
							}
	
							break;
						}
						57 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 58;
							}
	
							break;
						}
						58 =>  {
						}
						64 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add(59);
							}
							break;
						}
						59 =>  {
							if (0x3ff000000000000 & l) == 0x0 {
								break;
							}
	
							if kind > 98 {
								kind = 98;
							}
	
							{
								self.jj_checkn_add_two_states(54, 60);
							}
							break;
						}
						62 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 63;
							}
	
							break;
						}
						63 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 64;
							}
	
							break;
						}
						69 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 70;
							}
	
							break;
						}
						70 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 71;
							}
	
							break;
						}
						71 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 72;
							}
	
							break;
						}
						72 =>  {
							if (0x3ff000000000000 & l) == 0x0 {
								break;
							}
	
							if kind > 98 {
								kind = 98;
							}
	
							{
								self.jj_checkn_add_two_states(67, 73);
							}
							break;
						}
						74 =>  {
							if self.cur_char == 47 {
								self.jj_add_states(19, 20);
							}
							break;
						}
						76 =>  {
							if (0xffffffffffffdbff & l) == 0x0 {
								break;
							}
	
							if kind > 5 {
								kind = 5;
							}
	
							{
								self.jj_checkn_add(76);
							}
							break;
						}
						77 =>  {
							if self.cur_char == 42 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 78;
							}
	
							break;
						}
						78 =>  {
							if (0xffff7fffffffffff & l) != 0x0 && kind > 6 {
								kind = 6;
							}
	
							break;
						}
						79 =>  {
							if self.cur_char == 42 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 77;
							}
	
							break;
						}
						80 =>  {
							if (0x3ff000000000000 & l) == 0x0 {
								break;
							}
	
							if kind > 81 {
								kind = 81;
							}
	
							{
								self.jj_checkn_add_states(4, 18);
							}
							break;
						}
						81 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add_two_states(81, 82);
							}
							break;
						}
						82 =>  {
						}
						118 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add(83);
							}
							break;
						}
						84 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add_two_states(84, 85);
							}
							break;
						}
						85 =>  {
						}
						129 =>  {
							if (0x3ff000000000000 & l) != 0x0 && kind > 81 {
								kind = 81;
							}
	
							break;
						}
						86 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add_two_states(86, 87);
							}
							break;
						}
						87 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add_two_states(88, 10);
							}
							break;
						}
						89 =>  {
							if (0x280000000000 & l) != 0x0 {
								self.jj_checkn_add(90);
							}
							break;
						}
						90 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add_states(48, 50);
							}
							break;
						}
						91 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add_two_states(91, 92);
							}
							break;
						}
						92 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add_two_states(90, 10);
							}
							break;
						}
						93 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add_states(51, 54);
							}
							break;
						}
						94 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add_two_states(94, 95);
							}
							break;
						}
						95 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add(96);
							}
							break;
						}
						97 =>  {
							if (0x280000000000 & l) != 0x0 {
								self.jj_checkn_add(98);
							}
							break;
						}
						98 =>  {
							if (0x3ff000000000000 & l) == 0x0 {
								break;
							}
	
							if kind > 86 {
								kind = 86;
							}
	
							{
								self.jj_checkn_add_states(55, 57);
							}
							break;
						}
						99 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add_two_states(99, 100);
							}
							break;
						}
						100 =>  {
							if (0x3ff000000000000 & l) == 0x0 {
								break;
							}
	
							if kind > 86 {
								kind = 86;
							}
	
							{
								self.jj_checkn_add_two_states(98, 10);
							}
							break;
						}
						101 =>  {
							if (0x3ff000000000000 & l) == 0x0 {
								break;
							}
	
							if kind > 86 {
								kind = 86;
							}
	
							{
								self.jj_checkn_add_states(58, 61);
							}
							break;
						}
						102 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add_two_states(102, 103);
							}
							break;
						}
						103 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add(104);
							}
							break;
						}
						104 =>  {
							if self.cur_char != 46 {
								break;
							}
	
							if kind > 86 {
								kind = 86;
							}
	
							{
								self.jj_checkn_add_states(62, 64);
							}
							break;
						}
						105 =>  {
							if (0x3ff000000000000 & l) == 0x0 {
								break;
							}
	
							if kind > 86 {
								kind = 86;
							}
	
							{
								self.jj_checkn_add_states(65, 68);
							}
							break;
						}
						106 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add_two_states(106, 107);
							}
							break;
						}
						107 =>  {
							if (0x3ff000000000000 & l) == 0x0 {
								break;
							}
	
							if kind > 86 {
								kind = 86;
							}
	
							{
								self.jj_checkn_add_two_states(108, 10);
							}
							break;
						}
						109 =>  {
							if (0x280000000000 & l) != 0x0 {
								self.jj_checkn_add(110);
							}
							break;
						}
						110 =>  {
							if (0x3ff000000000000 & l) == 0x0 {
								break;
							}
	
							if kind > 86 {
								kind = 86;
							}
	
							{
								self.jj_checkn_add_states(69, 71);
							}
							break;
						}
						111 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add_two_states(111, 112);
							}
							break;
						}
						112 =>  {
							if (0x3ff000000000000 & l) == 0x0 {
								break;
							}
	
							if kind > 86 {
								kind = 86;
							}
	
							{
								self.jj_checkn_add_two_states(110, 10);
							}
							break;
						}
						113 =>  {
							if (0x3ff000000000000 & l) == 0x0 {
								break;
							}
	
							if kind > 86 {
								kind = 86;
							}
	
							{
								self.jj_checkn_add_states(72, 75);
							}
							break;
						}
						114 =>  {
							if self.cur_char == 48 {
								self.jj_add_states(24, 31);
							}
							break;
						}
						116 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add_states(76, 78);
							}
							break;
						}
						117 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add_two_states(117, 118);
							}
							break;
						}
						119 =>  {
							if (0xff000000000000 & l) != 0x0 {
								self.jj_checkn_add_states(79, 81);
							}
							break;
						}
						120 =>  {
							if (0xff000000000000 & l) != 0x0 {
								self.jj_checkn_add_two_states(120, 121);
							}
							break;
						}
						121 =>  {
							if (0xff000000000000 & l) != 0x0 {
								self.jj_checkn_add(83);
							}
							break;
						}
						123 =>  {
							if (0x3000000000000 & l) != 0x0 {
								self.jj_checkn_add_states(82, 84);
							}
							break;
						}
						124 =>  {
							if (0x3000000000000 & l) != 0x0 {
								self.jj_checkn_add_two_states(124, 125);
							}
							break;
						}
						125 =>  {
							if (0x3000000000000 & l) != 0x0 {
								self.jj_checkn_add(83);
							}
							break;
						}
						127 =>  {
							if (0x3ff000000000000 & l) == 0x0 {
								break;
							}
	
							if kind > 81 {
								kind = 81;
							}
	
							{
								self.jj_checkn_add_two_states(128, 129);
							}
							break;
						}
						128 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add_two_states(128, 129);
							}
							break;
						}
						130 =>  {
							if (0xff000000000000 & l) == 0x0 {
								break;
							}
	
							if kind > 81 {
								kind = 81;
							}
	
							{
								self.jj_checkn_add_two_states(131, 132);
							}
							break;
						}
						131 =>  {
							if (0xff000000000000 & l) != 0x0 {
								self.jj_checkn_add_two_states(131, 132);
							}
							break;
						}
						132 =>  {
							if (0xff000000000000 & l) != 0x0 && kind > 81 {
								kind = 81;
							}
	
							break;
						}
						134 =>  {
							if (0x3000000000000 & l) == 0x0 {
								break;
							}
	
							if kind > 81 {
								kind = 81;
							}
	
							{
								self.jj_checkn_add_two_states(135, 136);
							}
							break;
						}
						135 =>  {
							if (0x3000000000000 & l) != 0x0 {
								self.jj_checkn_add_two_states(135, 136);
							}
							break;
						}
						136 =>  {
							if (0x3000000000000 & l) != 0x0 && kind > 81 {
								kind = 81;
							}
	
							break;
						}
						138 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add_states(85, 87);
							}
							break;
						}
						139 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add_two_states(139, 140);
							}
							break;
						}
						140 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add(141);
							}
							break;
						}
						141 =>  {
							if self.cur_char == 46 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 142;
							}
	
							break;
						}
						142 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add_states(88, 90);
							}
							break;
						}
						143 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add_two_states(143, 144);
							}
							break;
						}
						144 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add(145);
							}
							break;
						}
						146 =>  {
							if (0x280000000000 & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 147;
							}
	
							break;
						}
						147 =>  {
							if (0x3ff000000000000 & l) == 0x0 {
								break;
							}
	
							if kind > 86 {
								kind = 86;
							}
	
							{
								self.jj_checkn_add_states(91, 93);
							}
							break;
						}
						148 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add_two_states(148, 149);
							}
							break;
						}
						149 =>  {
						}
						159 =>  {
							if (0x3ff000000000000 & l) == 0x0 {
								break;
							}
	
							if kind > 86 {
								kind = 86;
							}
	
							{
								self.jj_checkn_add(10);
							}
							break;
						}
						151 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add_states(94, 97);
							}
							break;
						}
						152 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add_two_states(152, 153);
							}
							break;
						}
						153 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add_two_states(154, 155);
							}
							break;
						}
						154 =>  {
							if self.cur_char == 46 {
								self.jj_checkn_add(155);
							}
							break;
						}
						156 =>  {
							if (0x280000000000 & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 157;
							}
	
							break;
						}
						157 =>  {
							if (0x3ff000000000000 & l) == 0x0 {
								break;
							}
	
							if kind > 86 {
								kind = 86;
							}
	
							{
								self.jj_checkn_add_states(98, 100);
							}
							break;
						}
						158 =>  {
							if (0x3ff000000000000 & l) != 0x0 {
								self.jj_checkn_add_two_states(158, 159);
							}
							break;
						}
						_ =>  {
							break;
						}
					}
				}if !(i != starts_at) break;}
			} else if self.cur_char < 128 {
				let l: i64 = 1 << (self.cur_char & 077);
				loop { {
					match self.jjstate_set[i -= 1] {
						160 =>  {
							if (0xffffffffefffffff & l) != 0x0 {
								self.jj_checkn_add_states(0, 3);
							} else if self.cur_char == 92 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 42;
							}
	
							if self.cur_char == 92 {
								self.jj_add_states(101, 103);
							}
							break;
						}
						161 =>  {
							if (0x87fffffe87fffffe & l) != 0x0 {
								if kind > 98 {
									kind = 98;
								}
	
								{
									self.jj_checkn_add_two_states(54, 60);
								}
							} else if self.cur_char == 92 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 55;
							}
	
							break;
						}
						162 =>  {
							if (0x87fffffe87fffffe & l) != 0x0 {
								if kind > 98 {
									kind = 98;
								}
	
								{
									self.jj_checkn_add_two_states(67, 73);
								}
							} else if self.cur_char == 92 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 68;
							}
	
							break;
						}
						0 =>  {
							if (0x7fffffe07fffffe & l) != 0x0 {
								if kind > 98 {
									kind = 98;
								}
	
								{
									self.jj_checkn_add_two_states(54, 60);
								}
							} else if self.cur_char == 95 {
								self.jj_checkn_add_two_states(67, 73);
							} else if self.cur_char == 92 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 61;
							}
	
							break;
						}
						3 =>  {
							if self.cur_char == 95 {
								self.jj_add_states(104, 105);
							}
							break;
						}
						5 =>  {
							if (0x2000000020 & l) != 0x0 {
								self.jj_add_states(106, 107);
							}
							break;
						}
						8 =>  {
							if self.cur_char == 95 {
								self.jj_add_states(108, 109);
							}
							break;
						}
						10 =>  {
							if (0x5000000050 & l) != 0x0 && kind > 86 {
								kind = 86;
							}
	
							break;
						}
						13 =>  {
							if (0xffffffffefffffff & l) != 0x0 {
								self.jj_checkn_add(14);
							}
							break;
						}
						15 =>  {
							if self.cur_char == 92 {
								self.jj_checkn_add_states(110, 112);
							}
							break;
						}
						16 =>  {
							if (0x1c404410000000 & l) != 0x0 {
								self.jj_checkn_add(14);
							}
							break;
						}
						21 =>  {
							if self.cur_char == 92 {
								self.jj_add_states(113, 114);
							}
							break;
						}
						23 =>  {
							if (0x800000008 & l) != 0x0 {
								self.jj_checkn_add_two_states(16, 29);
							}
							break;
						}
						25 =>  {
							if (0x800000008 & l) != 0x0 {
								self.jj_checkn_add(14);
							}
							break;
						}
						28 =>  {
							if self.cur_char == 117 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 27;
							}
	
							break;
						}
						29 =>  {
							if self.cur_char == 92 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 28;
							}
	
							break;
						}
						32 =>  {
							if self.cur_char == 117 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 31;
							}
	
							break;
						}
						33 =>  {
							if self.cur_char == 117 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 34;
							}
	
							break;
						}
						34 =>  {
							if (0x7e0000007e & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 35;
							}
	
							break;
						}
						35 =>  {
							if (0x7e0000007e & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 36;
							}
	
							break;
						}
						36 =>  {
							if (0x7e0000007e & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 37;
							}
	
							break;
						}
						37 =>  {
							if (0x7e0000007e & l) != 0x0 {
								self.jj_checkn_add(14);
							}
							break;
						}
						39 =>  {
							if (0xffffffffefffffff & l) != 0x0 {
								self.jj_checkn_add_states(0, 3);
							}
							break;
						}
						40 =>  {
							if self.cur_char == 92 {
								self.jj_add_states(101, 103);
							}
							break;
						}
						41 =>  {
							if (0x1c404410000000 & l) != 0x0 {
								self.jj_checkn_add_states(0, 3);
							}
							break;
						}
						42 =>  {
							if self.cur_char == 117 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 43;
							}
	
							break;
						}
						43 =>  {
							if (0x7e0000007e & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 44;
							}
	
							break;
						}
						44 =>  {
							if (0x7e0000007e & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 45;
							}
	
							break;
						}
						45 =>  {
							if (0x7e0000007e & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 46;
							}
	
							break;
						}
						46 =>  {
							if (0x7e0000007e & l) != 0x0 {
								self.jj_checkn_add_states(0, 3);
							}
							break;
						}
						47 =>  {
							if self.cur_char == 92 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 42;
							}
	
							break;
						}
						53 =>  {
							if (0x7fffffe07fffffe & l) == 0x0 {
								break;
							}
	
							if kind > 98 {
								kind = 98;
							}
	
							{
								self.jj_checkn_add_two_states(54, 60);
							}
							break;
						}
						54 =>  {
							if (0x87fffffe87fffffe & l) == 0x0 {
								break;
							}
	
							if kind > 98 {
								kind = 98;
							}
	
							{
								self.jj_checkn_add_two_states(54, 60);
							}
							break;
						}
						55 =>  {
							if self.cur_char == 117 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 56;
							}
	
							break;
						}
						56 =>  {
							if (0x7e0000007e & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 57;
							}
	
							break;
						}
						57 =>  {
							if (0x7e0000007e & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 58;
							}
	
							break;
						}
						58 =>  {
						}
						64 =>  {
							if (0x7e0000007e & l) != 0x0 {
								self.jj_checkn_add(59);
							}
							break;
						}
						59 =>  {
							if (0x7e0000007e & l) == 0x0 {
								break;
							}
	
							if kind > 98 {
								kind = 98;
							}
	
							{
								self.jj_checkn_add_two_states(54, 60);
							}
							break;
						}
						60 =>  {
							if self.cur_char == 92 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 55;
							}
	
							break;
						}
						61 =>  {
							if self.cur_char == 117 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 62;
							}
	
							break;
						}
						62 =>  {
							if (0x7e0000007e & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 63;
							}
	
							break;
						}
						63 =>  {
							if (0x7e0000007e & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 64;
							}
	
							break;
						}
						65 =>  {
							if self.cur_char == 92 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 61;
							}
	
							break;
						}
						66 =>  {
							if self.cur_char == 95 {
								self.jj_checkn_add_two_states(67, 73);
							}
							break;
						}
						67 =>  {
							if (0x87fffffe87fffffe & l) == 0x0 {
								break;
							}
	
							if kind > 98 {
								kind = 98;
							}
	
							{
								self.jj_checkn_add_two_states(67, 73);
							}
							break;
						}
						68 =>  {
							if self.cur_char == 117 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 69;
							}
	
							break;
						}
						69 =>  {
							if (0x7e0000007e & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 70;
							}
	
							break;
						}
						70 =>  {
							if (0x7e0000007e & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 71;
							}
	
							break;
						}
						71 =>  {
							if (0x7e0000007e & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 72;
							}
	
							break;
						}
						72 =>  {
							if (0x7e0000007e & l) == 0x0 {
								break;
							}
	
							if kind > 98 {
								kind = 98;
							}
	
							{
								self.jj_checkn_add_two_states(67, 73);
							}
							break;
						}
						73 =>  {
							if self.cur_char == 92 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 68;
							}
	
							break;
						}
						76 =>  {
							if kind > 5 {
								kind = 5;
							}
	
							self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 76;
							break;
						}
						78 =>  {
							if kind > 6 {
								kind = 6;
							}
	
							break;
						}
						81 =>  {
							if self.cur_char == 95 {
								self.jj_add_states(115, 116);
							}
							break;
						}
						83 =>  {
							if (0x100000001000 & l) != 0x0 && kind > 80 {
								kind = 80;
							}
	
							break;
						}
						84 =>  {
							if self.cur_char == 95 {
								self.jj_add_states(117, 118);
							}
							break;
						}
						86 =>  {
							if self.cur_char == 95 {
								self.jj_add_states(119, 120);
							}
							break;
						}
						88 =>  {
							if (0x2000000020 & l) != 0x0 {
								self.jj_add_states(121, 122);
							}
							break;
						}
						91 =>  {
							if self.cur_char == 95 {
								self.jj_add_states(123, 124);
							}
							break;
						}
						94 =>  {
							if self.cur_char == 95 {
								self.jj_add_states(125, 126);
							}
							break;
						}
						96 =>  {
							if (0x2000000020 & l) != 0x0 {
								self.jj_add_states(127, 128);
							}
							break;
						}
						99 =>  {
							if self.cur_char == 95 {
								self.jj_add_states(129, 130);
							}
							break;
						}
						102 =>  {
							if self.cur_char == 95 {
								self.jj_add_states(131, 132);
							}
							break;
						}
						106 =>  {
							if self.cur_char == 95 {
								self.jj_add_states(133, 134);
							}
							break;
						}
						108 =>  {
							if (0x2000000020 & l) != 0x0 {
								self.jj_add_states(135, 136);
							}
							break;
						}
						111 =>  {
							if self.cur_char == 95 {
								self.jj_add_states(137, 138);
							}
							break;
						}
						115 =>  {
							if (0x100000001000000 & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 116;
							}
	
							break;
						}
						116 =>  {
							if (0x7e0000007e & l) != 0x0 {
								self.jj_checkn_add_states(76, 78);
							}
							break;
						}
						117 =>  {
							if (0x7e8000007e & l) != 0x0 {
								self.jj_checkn_add_two_states(117, 118);
							}
							break;
						}
						118 =>  {
							if (0x7e0000007e & l) != 0x0 {
								self.jj_checkn_add(83);
							}
							break;
						}
						120 =>  {
							if self.cur_char == 95 {
								self.jj_add_states(139, 140);
							}
							break;
						}
						122 =>  {
							if (0x400000004 & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 123;
							}
	
							break;
						}
						124 =>  {
							if self.cur_char == 95 {
								self.jj_add_states(141, 142);
							}
							break;
						}
						126 =>  {
							if (0x100000001000000 & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 127;
							}
	
							break;
						}
						127 =>  {
							if (0x7e0000007e & l) == 0x0 {
								break;
							}
	
							if kind > 81 {
								kind = 81;
							}
	
							{
								self.jj_checkn_add_two_states(128, 129);
							}
							break;
						}
						128 =>  {
							if (0x7e8000007e & l) != 0x0 {
								self.jj_checkn_add_two_states(128, 129);
							}
							break;
						}
						129 =>  {
							if (0x7e0000007e & l) != 0x0 && kind > 81 {
								kind = 81;
							}
	
							break;
						}
						131 =>  {
							if self.cur_char == 95 {
								self.jj_add_states(143, 144);
							}
							break;
						}
						133 =>  {
							if (0x400000004 & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 134;
							}
	
							break;
						}
						135 =>  {
							if self.cur_char == 95 {
								self.jj_add_states(145, 146);
							}
							break;
						}
						137 =>  {
							if (0x100000001000000 & l) != 0x0 {
								self.jj_checkn_add_two_states(138, 141);
							}
							break;
						}
						138 =>  {
							if (0x7e0000007e & l) != 0x0 {
								self.jj_checkn_add_states(85, 87);
							}
							break;
						}
						139 =>  {
							if (0x7e8000007e & l) != 0x0 {
								self.jj_checkn_add_two_states(139, 140);
							}
							break;
						}
						140 =>  {
							if (0x7e0000007e & l) != 0x0 {
								self.jj_checkn_add(141);
							}
							break;
						}
						142 =>  {
							if (0x7e0000007e & l) != 0x0 {
								self.jj_checkn_add_states(88, 90);
							}
							break;
						}
						143 =>  {
							if (0x7e8000007e & l) != 0x0 {
								self.jj_checkn_add_two_states(143, 144);
							}
							break;
						}
						144 =>  {
							if (0x7e0000007e & l) != 0x0 {
								self.jj_checkn_add(145);
							}
							break;
						}
						145 =>  {
							if (0x1000000010000 & l) != 0x0 {
								self.jj_add_states(147, 148);
							}
							break;
						}
						148 =>  {
							if self.cur_char == 95 {
								self.jj_add_states(149, 150);
							}
							break;
						}
						150 =>  {
							if (0x100000001000000 & l) != 0x0 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 151;
							}
	
							break;
						}
						151 =>  {
							if (0x7e0000007e & l) != 0x0 {
								self.jj_checkn_add_states(94, 97);
							}
							break;
						}
						152 =>  {
							if (0x7e8000007e & l) != 0x0 {
								self.jj_checkn_add_two_states(152, 153);
							}
							break;
						}
						153 =>  {
							if (0x7e0000007e & l) != 0x0 {
								self.jj_checkn_add_two_states(154, 155);
							}
							break;
						}
						155 =>  {
							if (0x1000000010000 & l) != 0x0 {
								self.jj_add_states(151, 152);
							}
							break;
						}
						158 =>  {
							if self.cur_char == 95 {
								self.jj_add_states(153, 154);
							}
							break;
						}
						_ =>  {
							break;
						}
					}
				}if !(i != starts_at) break;}
			} else {
				let hi_byte: i32 = (self.cur_char /* signed */ >> 8);
				let i1: i32 = hi_byte /* signed */ >> 6;
				let l1: i64 = 1 << (hi_byte & 077);
				let i2: i32 = (self.cur_char & 0xff) /* signed */ >> 6;
				let l2: i64 = 1 << (self.cur_char & 077);
				loop { {
					match self.jjstate_set[i -= 1] {
						160 =>  {
						}
						39 =>  {
							if com::github::javaparser::generated_java_parser_token_manager::GeneratedJavaParserTokenManager::jj_can_move_1(hi_byte, i1, i2, l1, l2) {
								self.jj_checkn_add_states(0, 3);
							}
							break;
						}
						161 =>  {
						}
						54 =>  {
							if !com::github::javaparser::generated_java_parser_token_manager::GeneratedJavaParserTokenManager::jj_can_move_3(hi_byte, i1, i2, l1, l2) {
								break;
							}
	
							if kind > 98 {
								kind = 98;
							}
	
							{
								self.jj_checkn_add_two_states(54, 60);
							}
							break;
						}
						162 =>  {
						}
						67 =>  {
							if !com::github::javaparser::generated_java_parser_token_manager::GeneratedJavaParserTokenManager::jj_can_move_3(hi_byte, i1, i2, l1, l2) {
								break;
							}
	
							if kind > 98 {
								kind = 98;
							}
	
							{
								self.jj_checkn_add_two_states(67, 73);
							}
							break;
						}
						0 =>  {
							if com::github::javaparser::generated_java_parser_token_manager::GeneratedJavaParserTokenManager::jj_can_move_0(hi_byte, i1, i2, l1, l2) {
								if kind > 1 {
									kind = 1;
								}
	
							}
							if com::github::javaparser::generated_java_parser_token_manager::GeneratedJavaParserTokenManager::jj_can_move_2(hi_byte, i1, i2, l1, l2) {
								if kind > 98 {
									kind = 98;
								}
	
								{
									self.jj_checkn_add_two_states(54, 60);
								}
							}
							break;
						}
						13 =>  {
							if com::github::javaparser::generated_java_parser_token_manager::GeneratedJavaParserTokenManager::jj_can_move_1(hi_byte, i1, i2, l1, l2) {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 14;
							}
	
							break;
						}
						53 =>  {
							if !com::github::javaparser::generated_java_parser_token_manager::GeneratedJavaParserTokenManager::jj_can_move_2(hi_byte, i1, i2, l1, l2) {
								break;
							}
	
							if kind > 98 {
								kind = 98;
							}
	
							{
								self.jj_checkn_add_two_states(54, 60);
							}
							break;
						}
						76 =>  {
							if !com::github::javaparser::generated_java_parser_token_manager::GeneratedJavaParserTokenManager::jj_can_move_1(hi_byte, i1, i2, l1, l2) {
								break;
							}
	
							if kind > 5 {
								kind = 5;
							}
	
							self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 76;
							break;
						}
						78 =>  {
							if com::github::javaparser::generated_java_parser_token_manager::GeneratedJavaParserTokenManager::jj_can_move_1(hi_byte, i1, i2, l1, l2) && kind > 6 {
								kind = 6;
							}
	
							break;
						}
						_ =>  {
							if i1 == 0 || l1 == 0 || i2 == 0 || l2 == 0 {
								break;
							}
							else {break;
							}
	
						}
					}
				}if !(i != starts_at) break;}
			}
			if kind != 0x7fffffff {
				self.jjmatched_kind = kind;
				self.jjmatched_pos = cur_pos;
				kind = 0x7fffffff;
			}
			cur_pos += 1;
			i = self.jjnew_state_cnt;
			self.jjnew_state_cnt = starts_at;
			starts_at = 160 - self.jjnew_state_cnt;
			if i == starts_at {
				return cur_pos;
			}
	
			let r0 = 'try0: {
				self.cur_char = match self.input_stream.read_char() {
					Err(e) => break 'try0 Err(e),
					Ok(s) => s,
				};
				break 'try0 Ok(());
			};
			match r0 {
				Err(e @ java.io.IOException) => {
					return cur_pos;
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
		}
	
	}

	fn jj_move_string_literal_dfa0_1(&self) -> i32 {
		match self.cur_char {
			'*' =>  {
				return self.jj_move_string_literal_dfa1_1(0x100);
			}
			_ =>  {
				return 1;
			}
		}
	}

	fn jj_move_string_literal_dfa1_1(&mut self, active0: i64) /* thrown(java.io.IOException | java.lang.IllegalStateException) */ -> i32 {
		let r0 = 'try0: {
			self.cur_char = match self.input_stream.read_char() {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ java.io.IOException) => {
				return 1;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		match self.cur_char {
			'/' =>  {
				if (active0 & 0x100) != 0x0 {
					return self.jj_stop_at_pos(1, 8);
				}
	
				break;
			}
			_ =>  {
				return 2;
			}
		}
		return 2;
	}

	fn jj_move_string_literal_dfa0_2(&self) -> i32 {
		match self.cur_char {
			'*' =>  {
				return self.jj_move_string_literal_dfa1_2(0x200);
			}
			_ =>  {
				return 1;
			}
		}
	}

	fn jj_move_string_literal_dfa1_2(&mut self, active0: i64) /* thrown(java.io.IOException | java.lang.IllegalStateException) */ -> i32 {
		let r0 = 'try0: {
			self.cur_char = match self.input_stream.read_char() {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ java.io.IOException) => {
				return 1;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		match self.cur_char {
			'/' =>  {
				if (active0 & 0x200) != 0x0 {
					return self.jj_stop_at_pos(1, 9);
				}
	
				break;
			}
			_ =>  {
				return 2;
			}
		}
		return 2;
	}

	fn jj_stop_string_literal_dfa_3(&mut self, pos: i32, active0: i64, active1: i64) -> i32 {
		match pos {
			0 =>  {
				if (active1 & 0x100000000) != 0x0 {
					self.jjmatched_kind = 97;
					return -1;
				}
				return -1;
			}
			1 =>  {
				if (active1 & 0x100000000) != 0x0 {
					if self.jjmatched_pos == 0 {
						self.jjmatched_kind = 97;
						self.jjmatched_pos = 0;
					}
					return -1;
				}
				return -1;
			}
			_ =>  {
				return -1;
			}
		}
	}

	fn jj_start_nfa_3(&self, pos: i32, active0: i64, active1: i64) -> i32 {
		return self.jj_move_nfa_3(&self.jj_stop_string_literal_dfa_3(pos, active0, active1), pos + 1);
	}

	fn jj_move_string_literal_dfa0_3(&self) -> i32 {
		match self.cur_char {
			'"' =>  {
				return self.jj_move_string_literal_dfa1_3(0x100000000);
			}
			_ =>  {
				return self.jj_move_nfa_3(0, 0);
			}
		}
	}

	fn jj_move_string_literal_dfa1_3(&mut self, active1: i64) /* thrown(java.io.IOException | java.lang.IllegalStateException) */ -> i32 {
		let r0 = 'try0: {
			self.cur_char = match self.input_stream.read_char() {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ java.io.IOException) => {
				self.jj_stop_string_literal_dfa_3(0, 0, active1);
				return 1;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		match self.cur_char {
			'"' =>  {
				return self.jj_move_string_literal_dfa2_3(active1, 0x100000000);
			}
			_ =>  {
				break;
			}
		}
		return self.jj_start_nfa_3(0, 0, active1);
	}

	fn jj_move_string_literal_dfa2_3(&mut self, old1: i64, mut active1: i64) /* thrown(java.io.IOException | java.lang.IllegalStateException) */ -> i32 {
		if ((active1 &= old1)) == 0 {
			return self.jj_start_nfa_3(0, 0, old1);
		}
	
		let r0 = 'try0: {
			self.cur_char = match self.input_stream.read_char() {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ java.io.IOException) => {
				self.jj_stop_string_literal_dfa_3(1, 0, active1);
				return 2;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		match self.cur_char {
			'"' =>  {
				if (active1 & 0x100000000) != 0x0 {
					return self.jj_stop_at_pos(2, 96);
				}
	
				break;
			}
			_ =>  {
				break;
			}
		}
		return self.jj_start_nfa_3(1, 0, active1);
	}

	fn jj_move_nfa_3(&mut self, start_state: i32, cur_pos: i32) /* thrown(java.io.IOException | java.lang.IllegalStateException) */ -> i32 {
		let starts_at: i32 = 0;
		self.jjnew_state_cnt = 3;
		let i: i32 = 1;
		self.jjstate_set[0] = start_state;
		let kind: i32 = 0x7fffffff;
		loop {
			if self.jjround += 1 == 0x7fffffff {
				self.re_init_rounds();
			}
	
			if self.cur_char < 64 {
				let l: i64 = 1 << self.cur_char;
				loop { {
					match self.jjstate_set[i -= 1] {
						0 =>  {
							if kind > 97 {
								kind = 97;
							}
	
							break;
						}
						1 =>  {
							if self.cur_char == 34 && kind > 97 {
								kind = 97;
							}
	
							break;
						}
						_ =>  {
							break;
						}
					}
				}if !(i != starts_at) break;}
			} else if self.cur_char < 128 {
				let l: i64 = 1 << (self.cur_char & 077);
				loop { {
					match self.jjstate_set[i -= 1] {
						0 =>  {
							if kind > 97 {
								kind = 97;
							}
	
							if self.cur_char == 92 {
								self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = 1;
							}
	
							break;
						}
						2 =>  {
							if kind > 97 {
								kind = 97;
							}
	
							break;
						}
						_ =>  {
							break;
						}
					}
				}if !(i != starts_at) break;}
			} else {
				let hi_byte: i32 = (self.cur_char /* signed */ >> 8);
				let i1: i32 = hi_byte /* signed */ >> 6;
				let l1: i64 = 1 << (hi_byte & 077);
				let i2: i32 = (self.cur_char & 0xff) /* signed */ >> 6;
				let l2: i64 = 1 << (self.cur_char & 077);
				loop { {
					match self.jjstate_set[i -= 1] {
						0 =>  {
							if com::github::javaparser::generated_java_parser_token_manager::GeneratedJavaParserTokenManager::jj_can_move_1(hi_byte, i1, i2, l1, l2) && kind > 97 {
								kind = 97;
							}
	
							break;
						}
						_ =>  {
							if i1 == 0 || l1 == 0 || i2 == 0 || l2 == 0 {
								break;
							}
							else {break;
							}
	
						}
					}
				}if !(i != starts_at) break;}
			}
			if kind != 0x7fffffff {
				self.jjmatched_kind = kind;
				self.jjmatched_pos = cur_pos;
				kind = 0x7fffffff;
			}
			cur_pos += 1;
			i = self.jjnew_state_cnt;
			self.jjnew_state_cnt = starts_at;
			starts_at = 3 - self.jjnew_state_cnt;
			if i == starts_at {
				return cur_pos;
			}
	
			let r0 = 'try0: {
				self.cur_char = match self.input_stream.read_char() {
					Err(e) => break 'try0 Err(e),
					Ok(s) => s,
				};
				break 'try0 Ok(());
			};
			match r0 {
				Err(e @ java.io.IOException) => {
					return cur_pos;
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
		}
	
	}

	fn jj_fill_token(&self) -> com::github::javaparser::token::Token {
		/* final */ let t: Token;
		/* final */ let cur_token_image: String;
		/* final */ let begin_line: i32;
		/* final */ let end_line: i32;
		/* final */ let begin_column: i32;
		/* final */ let end_column: i32;
		let im: String = self.jjstr_literal_images[self.jjmatched_kind];
		cur_token_image =  if im == null { self.input_stream.get_image() } else { im };
		begin_line = self.input_stream.get_begin_line();
		begin_column = self.input_stream.get_begin_column();
		end_line = self.input_stream.get_end_line();
		end_column = self.input_stream.get_end_column();
		t = /* Java*/ Token/* */ .newToken(self.jjmatched_kind);
		t.kind = self.jjmatched_kind;
		t.image = cur_token_image;
		t.beginLine = begin_line;
		t.endLine = end_line;
		t.beginColumn = begin_column;
		t.endColumn = end_column;
		return t;
	}

	fn jj_can_move_0(&self, hi_byte: i32, i1: i32, i2: i32, l1: i64, l2: i64) -> bool {
		match hi_byte {
			0 =>  {
				return ((self.jjbit_vec0[i2] & l2) != 0);
			}
			22 =>  {
				return ((self.jjbit_vec1[i2] & l2) != 0);
			}
			24 =>  {
				return ((self.jjbit_vec2[i2] & l2) != 0);
			}
			32 =>  {
				return ((self.jjbit_vec3[i2] & l2) != 0);
			}
			48 =>  {
				return ((self.jjbit_vec4[i2] & l2) != 0);
			}
			254 =>  {
				return ((self.jjbit_vec5[i2] & l2) != 0);
			}
			_ =>  {
				return false;
			}
		}
	}

	fn jj_can_move_1(&self, hi_byte: i32, i1: i32, i2: i32, l1: i64, l2: i64) -> bool {
		match hi_byte {
			0 =>  {
				return ((self.jjbit_vec8[i2] & l2) != 0);
			}
			_ =>  {
				if (self.jjbit_vec6[i1] & l1) != 0 {
					return true;
				}
	
				return false;
			}
		}
	}

	fn jj_can_move_2(&self, hi_byte: i32, i1: i32, i2: i32, l1: i64, l2: i64) -> bool {
		match hi_byte {
			0 =>  {
				return ((self.jjbit_vec10[i2] & l2) != 0);
			}
			2 =>  {
				return ((self.jjbit_vec11[i2] & l2) != 0);
			}
			3 =>  {
				return ((self.jjbit_vec12[i2] & l2) != 0);
			}
			4 =>  {
				return ((self.jjbit_vec13[i2] & l2) != 0);
			}
			5 =>  {
				return ((self.jjbit_vec14[i2] & l2) != 0);
			}
			6 =>  {
				return ((self.jjbit_vec15[i2] & l2) != 0);
			}
			7 =>  {
				return ((self.jjbit_vec16[i2] & l2) != 0);
			}
			8 =>  {
				return ((self.jjbit_vec17[i2] & l2) != 0);
			}
			9 =>  {
				return ((self.jjbit_vec18[i2] & l2) != 0);
			}
			10 =>  {
				return ((self.jjbit_vec19[i2] & l2) != 0);
			}
			11 =>  {
				return ((self.jjbit_vec20[i2] & l2) != 0);
			}
			12 =>  {
				return ((self.jjbit_vec21[i2] & l2) != 0);
			}
			13 =>  {
				return ((self.jjbit_vec22[i2] & l2) != 0);
			}
			14 =>  {
				return ((self.jjbit_vec23[i2] & l2) != 0);
			}
			15 =>  {
				return ((self.jjbit_vec24[i2] & l2) != 0);
			}
			16 =>  {
				return ((self.jjbit_vec25[i2] & l2) != 0);
			}
			18 =>  {
				return ((self.jjbit_vec26[i2] & l2) != 0);
			}
			19 =>  {
				return ((self.jjbit_vec27[i2] & l2) != 0);
			}
			20 =>  {
				return ((self.jjbit_vec6[i2] & l2) != 0);
			}
			22 =>  {
				return ((self.jjbit_vec28[i2] & l2) != 0);
			}
			23 =>  {
				return ((self.jjbit_vec29[i2] & l2) != 0);
			}
			24 =>  {
				return ((self.jjbit_vec30[i2] & l2) != 0);
			}
			25 =>  {
				return ((self.jjbit_vec31[i2] & l2) != 0);
			}
			26 =>  {
				return ((self.jjbit_vec32[i2] & l2) != 0);
			}
			27 =>  {
				return ((self.jjbit_vec33[i2] & l2) != 0);
			}
			28 =>  {
				return ((self.jjbit_vec34[i2] & l2) != 0);
			}
			29 =>  {
				return ((self.jjbit_vec35[i2] & l2) != 0);
			}
			31 =>  {
				return ((self.jjbit_vec36[i2] & l2) != 0);
			}
			32 =>  {
				return ((self.jjbit_vec37[i2] & l2) != 0);
			}
			33 =>  {
				return ((self.jjbit_vec38[i2] & l2) != 0);
			}
			44 =>  {
				return ((self.jjbit_vec39[i2] & l2) != 0);
			}
			45 =>  {
				return ((self.jjbit_vec40[i2] & l2) != 0);
			}
			46 =>  {
				return ((self.jjbit_vec41[i2] & l2) != 0);
			}
			48 =>  {
				return ((self.jjbit_vec42[i2] & l2) != 0);
			}
			49 =>  {
				return ((self.jjbit_vec43[i2] & l2) != 0);
			}
			77 =>  {
				return ((self.jjbit_vec44[i2] & l2) != 0);
			}
			159 =>  {
				return ((self.jjbit_vec45[i2] & l2) != 0);
			}
			164 =>  {
				return ((self.jjbit_vec46[i2] & l2) != 0);
			}
			166 =>  {
				return ((self.jjbit_vec47[i2] & l2) != 0);
			}
			167 =>  {
				return ((self.jjbit_vec48[i2] & l2) != 0);
			}
			168 =>  {
				return ((self.jjbit_vec49[i2] & l2) != 0);
			}
			169 =>  {
				return ((self.jjbit_vec50[i2] & l2) != 0);
			}
			170 =>  {
				return ((self.jjbit_vec51[i2] & l2) != 0);
			}
			171 =>  {
				return ((self.jjbit_vec52[i2] & l2) != 0);
			}
			215 =>  {
				return ((self.jjbit_vec53[i2] & l2) != 0);
			}
			250 =>  {
				return ((self.jjbit_vec54[i2] & l2) != 0);
			}
			251 =>  {
				return ((self.jjbit_vec55[i2] & l2) != 0);
			}
			253 =>  {
				return ((self.jjbit_vec56[i2] & l2) != 0);
			}
			254 =>  {
				return ((self.jjbit_vec57[i2] & l2) != 0);
			}
			255 =>  {
				return ((self.jjbit_vec58[i2] & l2) != 0);
			}
			_ =>  {
				if (self.jjbit_vec9[i1] & l1) != 0 {
					return true;
				}
	
				return false;
			}
		}
	}

	fn jj_can_move_3(&self, hi_byte: i32, i1: i32, i2: i32, l1: i64, l2: i64) -> bool {
		match hi_byte {
			0 =>  {
				return ((self.jjbit_vec59[i2] & l2) != 0);
			}
			2 =>  {
				return ((self.jjbit_vec11[i2] & l2) != 0);
			}
			3 =>  {
				return ((self.jjbit_vec60[i2] & l2) != 0);
			}
			4 =>  {
				return ((self.jjbit_vec61[i2] & l2) != 0);
			}
			5 =>  {
				return ((self.jjbit_vec62[i2] & l2) != 0);
			}
			6 =>  {
				return ((self.jjbit_vec63[i2] & l2) != 0);
			}
			7 =>  {
				return ((self.jjbit_vec64[i2] & l2) != 0);
			}
			8 =>  {
				return ((self.jjbit_vec65[i2] & l2) != 0);
			}
			9 =>  {
				return ((self.jjbit_vec66[i2] & l2) != 0);
			}
			10 =>  {
				return ((self.jjbit_vec67[i2] & l2) != 0);
			}
			11 =>  {
				return ((self.jjbit_vec68[i2] & l2) != 0);
			}
			12 =>  {
				return ((self.jjbit_vec69[i2] & l2) != 0);
			}
			13 =>  {
				return ((self.jjbit_vec70[i2] & l2) != 0);
			}
			14 =>  {
				return ((self.jjbit_vec71[i2] & l2) != 0);
			}
			15 =>  {
				return ((self.jjbit_vec72[i2] & l2) != 0);
			}
			16 =>  {
				return ((self.jjbit_vec73[i2] & l2) != 0);
			}
			18 =>  {
				return ((self.jjbit_vec26[i2] & l2) != 0);
			}
			19 =>  {
				return ((self.jjbit_vec74[i2] & l2) != 0);
			}
			20 =>  {
				return ((self.jjbit_vec6[i2] & l2) != 0);
			}
			22 =>  {
				return ((self.jjbit_vec28[i2] & l2) != 0);
			}
			23 =>  {
				return ((self.jjbit_vec75[i2] & l2) != 0);
			}
			24 =>  {
				return ((self.jjbit_vec76[i2] & l2) != 0);
			}
			25 =>  {
				return ((self.jjbit_vec77[i2] & l2) != 0);
			}
			26 =>  {
				return ((self.jjbit_vec78[i2] & l2) != 0);
			}
			27 =>  {
				return ((self.jjbit_vec79[i2] & l2) != 0);
			}
			28 =>  {
				return ((self.jjbit_vec80[i2] & l2) != 0);
			}
			29 =>  {
				return ((self.jjbit_vec81[i2] & l2) != 0);
			}
			31 =>  {
				return ((self.jjbit_vec36[i2] & l2) != 0);
			}
			32 =>  {
				return ((self.jjbit_vec82[i2] & l2) != 0);
			}
			33 =>  {
				return ((self.jjbit_vec38[i2] & l2) != 0);
			}
			44 =>  {
				return ((self.jjbit_vec83[i2] & l2) != 0);
			}
			45 =>  {
				return ((self.jjbit_vec84[i2] & l2) != 0);
			}
			46 =>  {
				return ((self.jjbit_vec41[i2] & l2) != 0);
			}
			48 =>  {
				return ((self.jjbit_vec85[i2] & l2) != 0);
			}
			49 =>  {
				return ((self.jjbit_vec43[i2] & l2) != 0);
			}
			77 =>  {
				return ((self.jjbit_vec44[i2] & l2) != 0);
			}
			159 =>  {
				return ((self.jjbit_vec45[i2] & l2) != 0);
			}
			164 =>  {
				return ((self.jjbit_vec46[i2] & l2) != 0);
			}
			166 =>  {
				return ((self.jjbit_vec86[i2] & l2) != 0);
			}
			167 =>  {
				return ((self.jjbit_vec48[i2] & l2) != 0);
			}
			168 =>  {
				return ((self.jjbit_vec87[i2] & l2) != 0);
			}
			169 =>  {
				return ((self.jjbit_vec88[i2] & l2) != 0);
			}
			170 =>  {
				return ((self.jjbit_vec89[i2] & l2) != 0);
			}
			171 =>  {
				return ((self.jjbit_vec90[i2] & l2) != 0);
			}
			215 =>  {
				return ((self.jjbit_vec53[i2] & l2) != 0);
			}
			250 =>  {
				return ((self.jjbit_vec54[i2] & l2) != 0);
			}
			251 =>  {
				return ((self.jjbit_vec91[i2] & l2) != 0);
			}
			253 =>  {
				return ((self.jjbit_vec56[i2] & l2) != 0);
			}
			254 =>  {
				return ((self.jjbit_vec92[i2] & l2) != 0);
			}
			255 =>  {
				return ((self.jjbit_vec93[i2] & l2) != 0);
			}
			_ =>  {
				if (self.jjbit_vec9[i1] & l1) != 0 {
					return true;
				}
	
				return false;
			}
		}
	}

	pub fn get_next_token(&mut self) /* thrown(com.github.javaparser.TokenMgrException | java.io.IOException | java.lang.IllegalStateException) */ -> com::github::javaparser::token::Token {
		let special_token: Token = null;
		let matched_token: Token;
		let cur_pos: i32 = 0;
		'EOFLoop: loop {
			let r0 = 'try0: {
				self.cur_char = match self.input_stream.begin_token() {
					Err(e) => break 'try0 Err(e),
					Ok(s) => s,
				};
				break 'try0 Ok(());
			};
			match r0 {
				Err(e @ Exception) => {
					self.jjmatched_kind = 0;
					self.jjmatched_pos = -1;
					matched_token = self.jj_fill_token();
					matched_token.special_token = special_token;
					self.common_token_action(matched_token);
					return matched_token;
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
			self.image = self.jjimage;
			self.image.setLength(0);
			self.jjimage_len = 0;
			loop {
				match self.cur_lex_state {
					0 =>  {
						self.jjmatched_kind = 0x7fffffff;
						self.jjmatched_pos = 0;
						cur_pos = self.jj_move_string_literal_dfa0_0();
						break;
					}
					1 =>  {
						self.jjmatched_kind = 0x7fffffff;
						self.jjmatched_pos = 0;
						cur_pos = self.jj_move_string_literal_dfa0_1();
						if self.jjmatched_pos == 0 && self.jjmatched_kind > 10 {
							self.jjmatched_kind = 10;
						}
						break;
					}
					2 =>  {
						self.jjmatched_kind = 0x7fffffff;
						self.jjmatched_pos = 0;
						cur_pos = self.jj_move_string_literal_dfa0_2();
						if self.jjmatched_pos == 0 && self.jjmatched_kind > 10 {
							self.jjmatched_kind = 10;
						}
						break;
					}
					3 =>  {
						self.jjmatched_kind = 0x7fffffff;
						self.jjmatched_pos = 0;
						cur_pos = self.jj_move_string_literal_dfa0_3();
						break;
					}
				}
				if self.jjmatched_kind != 0x7fffffff {
					if self.jjmatched_pos + 1 < cur_pos {
						if let Err(e) = self.input_stream.backup(cur_pos - self.jjmatched_pos - 1) {
							return Err(e);
						};
					}
	
					if (self.jjto_token[self.jjmatched_kind /* signed */ >> 6] & (1 << (self.jjmatched_kind & 077))) != 0 {
						matched_token = self.jj_fill_token();
						matched_token.special_token = special_token;
						self.token_lexical_actions(matched_token);
						if self.jjnew_lex_state[self.jjmatched_kind] != -1 {
							self.cur_lex_state = self.jjnew_lex_state[self.jjmatched_kind];
						}
	
						self.common_token_action(matched_token);
						return matched_token;
					} else if (self.jjto_skip[self.jjmatched_kind /* signed */ >> 6] & (1 << (self.jjmatched_kind & 077))) != 0 {
						if (self.jjto_special[self.jjmatched_kind /* signed */ >> 6] & (1 << (self.jjmatched_kind & 077))) != 0 {
							matched_token = self.jj_fill_token();
							if special_token == null {
								special_token = matched_token;
							}
							else {
								matched_token.special_token = special_token;
								special_token = (special_token.next = matched_token);
							}
							self.skip_lexical_actions(matched_token);
						} else {self.skip_lexical_actions(null);
						}
	
						if self.jjnew_lex_state[self.jjmatched_kind] != -1 {
							self.cur_lex_state = self.jjnew_lex_state[self.jjmatched_kind];
						}
	
						continue 'EOFLoop;
					}
					self.more_lexical_actions();
					if self.jjnew_lex_state[self.jjmatched_kind] != -1 {
						self.cur_lex_state = self.jjnew_lex_state[self.jjmatched_kind];
					}
	
					cur_pos = 0;
					self.jjmatched_kind = 0x7fffffff;
					let r1 = 'try1: {
						self.cur_char = match self.input_stream.read_char() {
							Err(e) => break 'try1 Err(e),
							Ok(s) => s,
						};
						continue;
						break 'try1 Ok(());
					};
					match r1 {
						Err(e @ java.io.IOException) => {
						},
						Err(e) => Err(e)?,
						Ok => (),
					}
				}
				let error_line: i32 = self.input_stream.get_end_line();
				let error_column: i32 = self.input_stream.get_end_column();
				let error_after: String = null;
				const EOFSeen: bool = false;
				let r2 = 'try2: {
					match self.input_stream.read_char() {
						Err(e) => break 'try2 Err(e),
						Ok(s) => s,
					};
					if let Err(e) = self.input_stream.backup(1) {
						return Err(e);
					};
					break 'try2 Ok(());
				};
				match r2 {
					Err(e @ java.io.IOException) => {
						EOFSeen = true;
						error_after =  if cur_pos <= 1 { "" } else { self.input_stream.get_image() };
						if self.cur_char == '\n' || self.cur_char == '\r' {
							error_line += 1;
							error_column = 0;
						} else {error_column += 1;
						}
	
					},
					Err(e) => Err(e)?,
					Ok => (),
				}
				if !EOFSeen {
					if let Err(e) = self.input_stream.backup(1) {
						return Err(e);
					};
					error_after =  if cur_pos <= 1 { "" } else { self.input_stream.get_image() };
				}
				break 'try2 Err(TokenMgrException::new(EOFSeen, self.cur_lex_state, error_line, error_column, error_after, self.cur_char, /* Java*/ TokenMgrException/* */ .LEXICAL_ERROR));
			}
	
		}
	
	}

	fn skip_lexical_actions(&self, matched_token: &com::github::javaparser::token::Token) {
		match self.jjmatched_kind {
			_ =>  {
				break;
			}
		}
	}

	fn more_lexical_actions(&mut self) /* thrown(java.lang.IllegalStateException) */ {
		self.jjimage_len += (self.length_of_match = self.jjmatched_pos + 1);
		match self.jjmatched_kind {
			6 =>  {
				self.image.append(&self.input_stream.get_suffix(self.jjimage_len));
				self.jjimage_len = 0;
				self.input_stream.backup(1)?;
				break;
			}
			_ =>  {
				break;
			}
		}
	}

	fn token_lexical_actions(&mut self, mut matched_token: &com::github::javaparser::token::Token) /* thrown(java.lang.IllegalStateException) */ {
		match self.jjmatched_kind {
			68 =>  {
				self.image.append(self.jjstr_literal_images[68]);
				self.length_of_match = self.jjstr_literal_images[68].length();
				if !self.yield_supported {
					matched_token.kind = ;
				}
	
				break;
			}
			148 =>  {
				self.image.append(self.jjstr_literal_images[148]);
				self.length_of_match = self.jjstr_literal_images[148].length();
				matched_token.kind = ;
				matched_token.realKind = ;
				self.input_stream.backup(2)?;
				break;
			}
			149 =>  {
				self.image.append(self.jjstr_literal_images[149]);
				self.length_of_match = self.jjstr_literal_images[149].length();
				matched_token.kind = ;
				matched_token.realKind = ;
				self.input_stream.backup(1)?;
				break;
			}
			_ =>  {
				break;
			}
		}
	}

	fn jj_checkn_add(&mut self, mut state: i32) {
		if self.jjrounds[state] != self.jjround {
			self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = state;
			self.jjrounds[state] = self.jjround;
		}
	}

	fn jj_add_states(&mut self, start: i32, end: i32) {
		loop { {
			self.jjstate_set[self.jjnew_state_cnt += 1 !!!check!!! post increment] = self.jjnext_states[start];
		}if !(start += 1 !!!check!!! post increment != end) break;}
	}

	fn jj_checkn_add_two_states(&self, state1: i32, state2: i32) {
		self.jj_checkn_add(state1);
		self.jj_checkn_add(state2);
	}

	fn jj_checkn_add_states(&self, start: i32, end: i32) {
		loop { {
			self.jj_checkn_add(self.jjnext_states[start]);
		}if !(start += 1 !!!check!!! post increment != end) break;}
	}

	pub fn new(stream: &com::github::javaparser::simple_char_stream::SimpleCharStream) -> com::github::javaparser::generated_java_parser_token_manager::GeneratedJavaParserTokenManager {
		self.input_stream = stream;
	}

	pub fn new(stream: &com::github::javaparser::simple_char_stream::SimpleCharStream, lex_state: i32) /* thrown(com.github.javaparser.TokenMgrException) */ -> com::github::javaparser::generated_java_parser_token_manager::GeneratedJavaParserTokenManager {
		self.re_init(stream);
		self.switch_to(lex_state)?;
	}

	pub fn re_init(&mut self, stream: &com::github::javaparser::simple_char_stream::SimpleCharStream) {
		self.jjmatched_pos = self.jjnew_state_cnt = 0;
		self.cur_lex_state = self.default_lex_state;
		self.input_stream = stream;
		self.re_init_rounds();
	}

	fn re_init_rounds(&mut self) {
		let i: i32;
		self.jjround = 0x80000001;
		 {
			i = 160;
			while i -= 1 !!!check!!! post decrement > 0{
				self.jjrounds[i] = 0x80000000;
			}
		 }
	
	}

	pub fn re_init(&self, stream: &com::github::javaparser::simple_char_stream::SimpleCharStream, lex_state: i32) /* thrown(com.github.javaparser.TokenMgrException) */ {
		self.re_init(stream);
		self.switch_to(lex_state)?;
	}

	pub fn switch_to(&mut self, lex_state: i32) /* thrown(com.github.javaparser.TokenMgrException) */ {
		if lex_state >= 4 || lex_state < 0 {
			return Err(TokenMgrException::new("Error: Ignoring invalid lexical state : " + lex_state + ". State unchanged.", /* Java*/ TokenMgrException/* */ .INVALID_LEXICAL_STATE));
		}
		else {self.cur_lex_state = lex_state;
		}
	
	}
}

impl com::github::javaparser::generated_java_parser_constants::GeneratedJavaParserConstants for GeneratedJavaParserTokenManager {}