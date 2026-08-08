pub struct EntityArrays;

impl EntityArrays {
	static ISO8859_1_ESCAPE: &[&[/* Java */ java::lang::String /**/]] = vec![// non-breaking space
	vec!["\u00A0", "&nbsp;", ]
	, // inverted exclamation mark
	vec!["\u00A1", "&iexcl;", ]
	, // cent sign
	vec!["\u00A2", "&cent;", ]
	, // pound sign
	vec!["\u00A3", "&pound;", ]
	, // currency sign
	vec!["\u00A4", "&curren;", ]
	, // yen sign = yuan sign
	vec!["\u00A5", "&yen;", ]
	, // broken bar = broken vertical bar
	vec!["\u00A6", "&brvbar;", ]
	, // section sign
	vec!["\u00A7", "&sect;", ]
	, // dieresis = spacing dieresis
	vec!["\u00A8", "&uml;", ]
	, // © - copyright sign
	vec!["\u00A9", "&copy;", ]
	, // feminine ordinal indicator
	vec!["\u00AA", "&ordf;", ]
	, // left-pointing double angle quotation mark = left pointing guillemet
	vec!["\u00AB", "&laquo;", ]
	, // not sign
	vec!["\u00AC", "&not;", ]
	, // soft hyphen = discretionary hyphen
	vec!["\u00AD", "&shy;", ]
	, // ® - registered trademark sign
	vec!["\u00AE", "&reg;", ]
	, // macron = spacing macron = overline = APL overbar
	vec!["\u00AF", "&macr;", ]
	, // degree sign
	vec!["\u00B0", "&deg;", ]
	, // plus-minus sign = plus-or-minus sign
	vec!["\u00B1", "&plusmn;", ]
	, // superscript two = superscript digit two = squared
	vec!["\u00B2", "&sup2;", ]
	, // superscript three = superscript digit three = cubed
	vec!["\u00B3", "&sup3;", ]
	, // acute accent = spacing acute
	vec!["\u00B4", "&acute;", ]
	, // micro sign
	vec!["\u00B5", "&micro;", ]
	, // pilcrow sign = paragraph sign
	vec!["\u00B6", "&para;", ]
	, // middle dot = Georgian comma = Greek middle dot
	vec!["\u00B7", "&middot;", ]
	, // cedilla = spacing cedilla
	vec!["\u00B8", "&cedil;", ]
	, // superscript one = superscript digit one
	vec!["\u00B9", "&sup1;", ]
	, // masculine ordinal indicator
	vec!["\u00BA", "&ordm;", ]
	, // right-pointing double angle quotation mark = right pointing guillemet
	vec!["\u00BB", "&raquo;", ]
	, // vulgar fraction one quarter = fraction one quarter
	vec!["\u00BC", "&frac14;", ]
	, // vulgar fraction one half = fraction one half
	vec!["\u00BD", "&frac12;", ]
	, // vulgar fraction three quarters = fraction three quarters
	vec!["\u00BE", "&frac34;", ]
	, // inverted question mark = turned question mark
	vec!["\u00BF", "&iquest;", ]
	, // À - uppercase A, grave accent
	vec!["\u00C0", "&Agrave;", ]
	, // Á - uppercase A, acute accent
	vec!["\u00C1", "&Aacute;", ]
	, // Â - uppercase A, circumflex accent
	vec!["\u00C2", "&Acirc;", ]
	, // Ã - uppercase A, tilde
	vec!["\u00C3", "&Atilde;", ]
	, // Ä - uppercase A, umlaut
	vec!["\u00C4", "&Auml;", ]
	, // Å - uppercase A, ring
	vec!["\u00C5", "&Aring;", ]
	, // Æ - uppercase AE
	vec!["\u00C6", "&AElig;", ]
	, // Ç - uppercase C, cedilla
	vec!["\u00C7", "&Ccedil;", ]
	, // È - uppercase E, grave accent
	vec!["\u00C8", "&Egrave;", ]
	, // É - uppercase E, acute accent
	vec!["\u00C9", "&Eacute;", ]
	, // Ê - uppercase E, circumflex accent
	vec!["\u00CA", "&Ecirc;", ]
	, // Ë - uppercase E, umlaut
	vec!["\u00CB", "&Euml;", ]
	, // Ì - uppercase I, grave accent
	vec!["\u00CC", "&Igrave;", ]
	, // Í - uppercase I, acute accent
	vec!["\u00CD", "&Iacute;", ]
	, // Î - uppercase I, circumflex accent
	vec!["\u00CE", "&Icirc;", ]
	, // Ï - uppercase I, umlaut
	vec!["\u00CF", "&Iuml;", ]
	, // Ð - uppercase Eth, Icelandic
	vec!["\u00D0", "&ETH;", ]
	, // Ñ - uppercase N, tilde
	vec!["\u00D1", "&Ntilde;", ]
	, // Ò - uppercase O, grave accent
	vec!["\u00D2", "&Ograve;", ]
	, // Ó - uppercase O, acute accent
	vec!["\u00D3", "&Oacute;", ]
	, // Ô - uppercase O, circumflex accent
	vec!["\u00D4", "&Ocirc;", ]
	, // Õ - uppercase O, tilde
	vec!["\u00D5", "&Otilde;", ]
	, // Ö - uppercase O, umlaut
	vec!["\u00D6", "&Ouml;", ]
	, // multiplication sign
	vec!["\u00D7", "&times;", ]
	, // Ø - uppercase O, slash
	vec!["\u00D8", "&Oslash;", ]
	, // Ù - uppercase U, grave accent
	vec!["\u00D9", "&Ugrave;", ]
	, // Ú - uppercase U, acute accent
	vec!["\u00DA", "&Uacute;", ]
	, // Û - uppercase U, circumflex accent
	vec!["\u00DB", "&Ucirc;", ]
	, // Ü - uppercase U, umlaut
	vec!["\u00DC", "&Uuml;", ]
	, // Ý - uppercase Y, acute accent
	vec!["\u00DD", "&Yacute;", ]
	, // Þ - uppercase THORN, Icelandic
	vec!["\u00DE", "&THORN;", ]
	, // ß - lowercase sharps, German
	vec!["\u00DF", "&szlig;", ]
	, // à - lowercase a, grave accent
	vec!["\u00E0", "&agrave;", ]
	, // á - lowercase a, acute accent
	vec!["\u00E1", "&aacute;", ]
	, // â - lowercase a, circumflex accent
	vec!["\u00E2", "&acirc;", ]
	, // ã - lowercase a, tilde
	vec!["\u00E3", "&atilde;", ]
	, // ä - lowercase a, umlaut
	vec!["\u00E4", "&auml;", ]
	, // å - lowercase a, ring
	vec!["\u00E5", "&aring;", ]
	, // æ - lowercase ae
	vec!["\u00E6", "&aelig;", ]
	, // ç - lowercase c, cedilla
	vec!["\u00E7", "&ccedil;", ]
	, // è - lowercase e, grave accent
	vec!["\u00E8", "&egrave;", ]
	, // é - lowercase e, acute accent
	vec!["\u00E9", "&eacute;", ]
	, // ê - lowercase e, circumflex accent
	vec!["\u00EA", "&ecirc;", ]
	, // ë - lowercase e, umlaut
	vec!["\u00EB", "&euml;", ]
	, // ì - lowercase i, grave accent
	vec!["\u00EC", "&igrave;", ]
	, // í - lowercase i, acute accent
	vec!["\u00ED", "&iacute;", ]
	, // î - lowercase i, circumflex accent
	vec!["\u00EE", "&icirc;", ]
	, // ï - lowercase i, umlaut
	vec!["\u00EF", "&iuml;", ]
	, // ð - lowercase eth, Icelandic
	vec!["\u00F0", "&eth;", ]
	, // ñ - lowercase n, tilde
	vec!["\u00F1", "&ntilde;", ]
	, // ò - lowercase o, grave accent
	vec!["\u00F2", "&ograve;", ]
	, // ó - lowercase o, acute accent
	vec!["\u00F3", "&oacute;", ]
	, // ô - lowercase o, circumflex accent
	vec!["\u00F4", "&ocirc;", ]
	, // õ - lowercase o, tilde
	vec!["\u00F5", "&otilde;", ]
	, // ö - lowercase o, umlaut
	vec!["\u00F6", "&ouml;", ]
	, // division sign
	vec!["\u00F7", "&divide;", ]
	, // ø - lowercase o, slash
	vec!["\u00F8", "&oslash;", ]
	, // ù - lowercase u, grave accent
	vec!["\u00F9", "&ugrave;", ]
	, // ú - lowercase u, acute accent
	vec!["\u00FA", "&uacute;", ]
	, // û - lowercase u, circumflex accent
	vec!["\u00FB", "&ucirc;", ]
	, // ü - lowercase u, umlaut
	vec!["\u00FC", "&uuml;", ]
	, // ý - lowercase y, acute accent
	vec!["\u00FD", "&yacute;", ]
	, // þ - lowercase thorn, Icelandic
	vec!["\u00FE", "&thorn;", ]
	, // ÿ - lowercase y, umlaut
	vec!["\u00FF", "&yuml;", ]
	, ]
	;

	static ISO8859_1_UNESCAPE: &[&[/* Java */ java::lang::String /**/]] = org::apache::commons::lang3::text::translate::entity_arrays::EntityArrays::invert(ISO8859_1_ESCAPE);

	static HTML40_EXTENDED_ESCAPE: &[&[/* Java */ java::lang::String /**/]] = vec![// latin small f with hook = function= florin, U+0192 ISOtech -->
	vec!["\u0192", "&fnof;", ]
	, // greek capital letter alpha, U+0391 -->
	vec!["\u0391", "&Alpha;", ]
	, // greek capital letter beta, U+0392 -->
	vec!["\u0392", "&Beta;", ]
	, // greek capital letter gamma, U+0393 ISOgrk3 -->
	vec!["\u0393", "&Gamma;", ]
	, // greek capital letter delta, U+0394 ISOgrk3 -->
	vec!["\u0394", "&Delta;", ]
	, // greek capital letter epsilon, U+0395 -->
	vec!["\u0395", "&Epsilon;", ]
	, // greek capital letter zeta, U+0396 -->
	vec!["\u0396", "&Zeta;", ]
	, // greek capital letter eta, U+0397 -->
	vec!["\u0397", "&Eta;", ]
	, // greek capital letter theta, U+0398 ISOgrk3 -->
	vec!["\u0398", "&Theta;", ]
	, // greek capital letter iota, U+0399 -->
	vec!["\u0399", "&Iota;", ]
	, // greek capital letter kappa, U+039A -->
	vec!["\u039A", "&Kappa;", ]
	, // greek capital letter lambda, U+039B ISOgrk3 -->
	vec!["\u039B", "&Lambda;", ]
	, // greek capital letter mu, U+039C -->
	vec!["\u039C", "&Mu;", ]
	, // greek capital letter nu, U+039D -->
	vec!["\u039D", "&Nu;", ]
	, // greek capital letter xi, U+039E ISOgrk3 -->
	vec!["\u039E", "&Xi;", ]
	, // greek capital letter omicron, U+039F -->
	vec!["\u039F", "&Omicron;", ]
	, // greek capital letter pi, U+03A0 ISOgrk3 -->
	vec!["\u03A0", "&Pi;", ]
	, // greek capital letter rho, U+03A1 -->
	vec!["\u03A1", "&Rho;", ]
	, // greek capital letter sigma, U+03A3 ISOgrk3 -->
	vec!["\u03A3", "&Sigma;", ]
	, // greek capital letter tau, U+03A4 -->
	vec!["\u03A4", "&Tau;", ]
	, // greek capital letter upsilon, U+03A5 ISOgrk3 -->
	vec!["\u03A5", "&Upsilon;", ]
	, // greek capital letter phi, U+03A6 ISOgrk3 -->
	vec!["\u03A6", "&Phi;", ]
	, // greek capital letter chi, U+03A7 -->
	vec!["\u03A7", "&Chi;", ]
	, // greek capital letter psi, U+03A8 ISOgrk3 -->
	vec!["\u03A8", "&Psi;", ]
	, // greek capital letter omega, U+03A9 ISOgrk3 -->
	vec!["\u03A9", "&Omega;", ]
	, // greek small letter alpha, U+03B1 ISOgrk3 -->
	vec!["\u03B1", "&alpha;", ]
	, // greek small letter beta, U+03B2 ISOgrk3 -->
	vec!["\u03B2", "&beta;", ]
	, // greek small letter gamma, U+03B3 ISOgrk3 -->
	vec!["\u03B3", "&gamma;", ]
	, // greek small letter delta, U+03B4 ISOgrk3 -->
	vec!["\u03B4", "&delta;", ]
	, // greek small letter epsilon, U+03B5 ISOgrk3 -->
	vec!["\u03B5", "&epsilon;", ]
	, // greek small letter zeta, U+03B6 ISOgrk3 -->
	vec!["\u03B6", "&zeta;", ]
	, // greek small letter eta, U+03B7 ISOgrk3 -->
	vec!["\u03B7", "&eta;", ]
	, // greek small letter theta, U+03B8 ISOgrk3 -->
	vec!["\u03B8", "&theta;", ]
	, // greek small letter iota, U+03B9 ISOgrk3 -->
	vec!["\u03B9", "&iota;", ]
	, // greek small letter kappa, U+03BA ISOgrk3 -->
	vec!["\u03BA", "&kappa;", ]
	, // greek small letter lambda, U+03BB ISOgrk3 -->
	vec!["\u03BB", "&lambda;", ]
	, // greek small letter mu, U+03BC ISOgrk3 -->
	vec!["\u03BC", "&mu;", ]
	, // greek small letter nu, U+03BD ISOgrk3 -->
	vec!["\u03BD", "&nu;", ]
	, // greek small letter xi, U+03BE ISOgrk3 -->
	vec!["\u03BE", "&xi;", ]
	, // greek small letter omicron, U+03BF NEW -->
	vec!["\u03BF", "&omicron;", ]
	, // greek small letter pi, U+03C0 ISOgrk3 -->
	vec!["\u03C0", "&pi;", ]
	, // greek small letter rho, U+03C1 ISOgrk3 -->
	vec!["\u03C1", "&rho;", ]
	, // greek small letter final sigma, U+03C2 ISOgrk3 -->
	vec!["\u03C2", "&sigmaf;", ]
	, // greek small letter sigma, U+03C3 ISOgrk3 -->
	vec!["\u03C3", "&sigma;", ]
	, // greek small letter tau, U+03C4 ISOgrk3 -->
	vec!["\u03C4", "&tau;", ]
	, // greek small letter upsilon, U+03C5 ISOgrk3 -->
	vec!["\u03C5", "&upsilon;", ]
	, // greek small letter phi, U+03C6 ISOgrk3 -->
	vec!["\u03C6", "&phi;", ]
	, // greek small letter chi, U+03C7 ISOgrk3 -->
	vec!["\u03C7", "&chi;", ]
	, // greek small letter psi, U+03C8 ISOgrk3 -->
	vec!["\u03C8", "&psi;", ]
	, // greek small letter omega, U+03C9 ISOgrk3 -->
	vec!["\u03C9", "&omega;", ]
	, // greek small letter theta symbol, U+03D1 NEW -->
	vec!["\u03D1", "&thetasym;", ]
	, // greek upsilon with hook symbol, U+03D2 NEW -->
	vec!["\u03D2", "&upsih;", ]
	, // greek pi symbol, U+03D6 ISOgrk3 -->
	vec!["\u03D6", "&piv;", ]
	, // bullet = black small circle, U+2022 ISOpub -->
	vec!["\u2022", "&bull;", ]
	, // horizontal ellipsis = three dot leader, U+2026 ISOpub -->
	vec!["\u2026", "&hellip;", ]
	, // prime = minutes = feet, U+2032 ISOtech -->
	vec!["\u2032", "&prime;", ]
	, // double prime = seconds = inches, U+2033 ISOtech -->
	vec!["\u2033", "&Prime;", ]
	, // overline = spacing overscore, U+203E NEW -->
	vec!["\u203E", "&oline;", ]
	, // fraction slash, U+2044 NEW -->
	vec!["\u2044", "&frasl;", ]
	, // script capital P = power set= Weierstrass p, U+2118 ISOamso -->
	vec!["\u2118", "&weierp;", ]
	, // blackletter capital I = imaginary part, U+2111 ISOamso -->
	vec!["\u2111", "&image;", ]
	, // blackletter capital R = real part symbol, U+211C ISOamso -->
	vec!["\u211C", "&real;", ]
	, // trade mark sign, U+2122 ISOnum -->
	vec!["\u2122", "&trade;", ]
	, // alef symbol = first transfinite cardinal, U+2135 NEW -->
	vec!["\u2135", "&alefsym;", ]
	, // leftwards arrow, U+2190 ISOnum -->
	vec!["\u2190", "&larr;", ]
	, // upwards arrow, U+2191 ISOnum-->
	vec!["\u2191", "&uarr;", ]
	, // rightwards arrow, U+2192 ISOnum -->
	vec!["\u2192", "&rarr;", ]
	, // downwards arrow, U+2193 ISOnum -->
	vec!["\u2193", "&darr;", ]
	, // left right arrow, U+2194 ISOamsa -->
	vec!["\u2194", "&harr;", ]
	, // downwards arrow with corner leftwards= carriage return, U+21B5 NEW -->
	vec!["\u21B5", "&crarr;", ]
	, // leftwards double arrow, U+21D0 ISOtech -->
	vec!["\u21D0", "&lArr;", ]
	, // upwards double arrow, U+21D1 ISOamsa -->
	vec!["\u21D1", "&uArr;", ]
	, // rightwards double arrow, U+21D2 ISOtech -->
	vec!["\u21D2", "&rArr;", ]
	, // downwards double arrow, U+21D3 ISOamsa -->
	vec!["\u21D3", "&dArr;", ]
	, // left right double arrow, U+21D4 ISOamsa -->
	vec!["\u21D4", "&hArr;", ]
	, // for all, U+2200 ISOtech -->
	vec!["\u2200", "&forall;", ]
	, // partial differential, U+2202 ISOtech -->
	vec!["\u2202", "&part;", ]
	, // there exists, U+2203 ISOtech -->
	vec!["\u2203", "&exist;", ]
	, // empty set = null set = diameter, U+2205 ISOamso -->
	vec!["\u2205", "&empty;", ]
	, // nabla = backward difference, U+2207 ISOtech -->
	vec!["\u2207", "&nabla;", ]
	, // element of, U+2208 ISOtech -->
	vec!["\u2208", "&isin;", ]
	, // not an element of, U+2209 ISOtech -->
	vec!["\u2209", "&notin;", ]
	, // contains as member, U+220B ISOtech -->
	vec!["\u220B", "&ni;", ]
	, // n-ary product = product sign, U+220F ISOamsb -->
	vec!["\u220F", "&prod;", ]
	, // n-ary summation, U+2211 ISOamsb -->
	vec!["\u2211", "&sum;", ]
	, // minus sign, U+2212 ISOtech -->
	vec!["\u2212", "&minus;", ]
	, // asterisk operator, U+2217 ISOtech -->
	vec!["\u2217", "&lowast;", ]
	, // square root = radical sign, U+221A ISOtech -->
	vec!["\u221A", "&radic;", ]
	, // proportional to, U+221D ISOtech -->
	vec!["\u221D", "&prop;", ]
	, // infinity, U+221E ISOtech -->
	vec!["\u221E", "&infin;", ]
	, // angle, U+2220 ISOamso -->
	vec!["\u2220", "&ang;", ]
	, // logical and = wedge, U+2227 ISOtech -->
	vec!["\u2227", "&and;", ]
	, // logical or = vee, U+2228 ISOtech -->
	vec!["\u2228", "&or;", ]
	, // intersection = cap, U+2229 ISOtech -->
	vec!["\u2229", "&cap;", ]
	, // union = cup, U+222A ISOtech -->
	vec!["\u222A", "&cup;", ]
	, // integral, U+222B ISOtech -->
	vec!["\u222B", "&int;", ]
	, // therefore, U+2234 ISOtech -->
	vec!["\u2234", "&there4;", ]
	, // tilde operator = varies with = similar to, U+223C ISOtech -->
	vec!["\u223C", "&sim;", ]
	, // approximately equal to, U+2245 ISOtech -->
	vec!["\u2245", "&cong;", ]
	, // almost equal to = asymptotic to, U+2248 ISOamsr -->
	vec!["\u2248", "&asymp;", ]
	, // not equal to, U+2260 ISOtech -->
	vec!["\u2260", "&ne;", ]
	, // identical to, U+2261 ISOtech -->
	vec!["\u2261", "&equiv;", ]
	, // less-than or equal to, U+2264 ISOtech -->
	vec!["\u2264", "&le;", ]
	, // greater-than or equal to, U+2265 ISOtech -->
	vec!["\u2265", "&ge;", ]
	, // subset of, U+2282 ISOtech -->
	vec!["\u2282", "&sub;", ]
	, // superset of, U+2283 ISOtech -->
	vec!["\u2283", "&sup;", ]
	, // not a subset of, U+2284 ISOamsn -->
	vec!["\u2284", "&nsub;", ]
	, // subset of or equal to, U+2286 ISOtech -->
	vec!["\u2286", "&sube;", ]
	, // superset of or equal to, U+2287 ISOtech -->
	vec!["\u2287", "&supe;", ]
	, // circled plus = direct sum, U+2295 ISOamsb -->
	vec!["\u2295", "&oplus;", ]
	, // circled times = vector product, U+2297 ISOamsb -->
	vec!["\u2297", "&otimes;", ]
	, // up tack = orthogonal to = perpendicular, U+22A5 ISOtech -->
	vec!["\u22A5", "&perp;", ]
	, // dot operator, U+22C5 ISOamsb -->
	vec!["\u22C5", "&sdot;", ]
	, // left ceiling = apl upstile, U+2308 ISOamsc -->
	vec!["\u2308", "&lceil;", ]
	, // right ceiling, U+2309 ISOamsc -->
	vec!["\u2309", "&rceil;", ]
	, // left floor = apl downstile, U+230A ISOamsc -->
	vec!["\u230A", "&lfloor;", ]
	, // right floor, U+230B ISOamsc -->
	vec!["\u230B", "&rfloor;", ]
	, // left-pointing angle bracket = bra, U+2329 ISOtech -->
	vec!["\u2329", "&lang;", ]
	, // right-pointing angle bracket = ket, U+232A ISOtech -->
	vec!["\u232A", "&rang;", ]
	, // lozenge, U+25CA ISOpub -->
	vec!["\u25CA", "&loz;", ]
	, // black spade suit, U+2660 ISOpub -->
	vec!["\u2660", "&spades;", ]
	, // black club suit = shamrock, U+2663 ISOpub -->
	vec!["\u2663", "&clubs;", ]
	, // black heart suit = valentine, U+2665 ISOpub -->
	vec!["\u2665", "&hearts;", ]
	, // black diamond suit, U+2666 ISOpub -->
	vec!["\u2666", "&diams;", ]
	, // -- latin capital ligature OE, U+0152 ISOlat2 -->
	vec!["\u0152", "&OElig;", ]
	, // -- latin small ligature oe, U+0153 ISOlat2 -->
	vec!["\u0153", "&oelig;", ]
	, // -- latin capital letter S with caron, U+0160 ISOlat2 -->
	vec!["\u0160", "&Scaron;", ]
	, // -- latin small letter s with caron, U+0161 ISOlat2 -->
	vec!["\u0161", "&scaron;", ]
	, // -- latin capital letter Y with dieresis, U+0178 ISOlat2 -->
	vec!["\u0178", "&Yuml;", ]
	, // -- modifier letter circumflex accent, U+02C6 ISOpub -->
	vec!["\u02C6", "&circ;", ]
	, // small tilde, U+02DC ISOdia -->
	vec!["\u02DC", "&tilde;", ]
	, // en space, U+2002 ISOpub -->
	vec!["\u2002", "&ensp;", ]
	, // em space, U+2003 ISOpub -->
	vec!["\u2003", "&emsp;", ]
	, // thin space, U+2009 ISOpub -->
	vec!["\u2009", "&thinsp;", ]
	, // zero width non-joiner, U+200C NEW RFC 2070 -->
	vec!["\u200C", "&zwnj;", ]
	, // zero width joiner, U+200D NEW RFC 2070 -->
	vec!["\u200D", "&zwj;", ]
	, // left-to-right mark, U+200E NEW RFC 2070 -->
	vec!["\u200E", "&lrm;", ]
	, // right-to-left mark, U+200F NEW RFC 2070 -->
	vec!["\u200F", "&rlm;", ]
	, // en dash, U+2013 ISOpub -->
	vec!["\u2013", "&ndash;", ]
	, // em dash, U+2014 ISOpub -->
	vec!["\u2014", "&mdash;", ]
	, // left single quotation mark, U+2018 ISOnum -->
	vec!["\u2018", "&lsquo;", ]
	, // right single quotation mark, U+2019 ISOnum -->
	vec!["\u2019", "&rsquo;", ]
	, // single low-9 quotation mark, U+201A NEW -->
	vec!["\u201A", "&sbquo;", ]
	, // left double quotation mark, U+201C ISOnum -->
	vec!["\u201C", "&ldquo;", ]
	, // right double quotation mark, U+201D ISOnum -->
	vec!["\u201D", "&rdquo;", ]
	, // double low-9 quotation mark, U+201E NEW -->
	vec!["\u201E", "&bdquo;", ]
	, // dagger, U+2020 ISOpub -->
	vec!["\u2020", "&dagger;", ]
	, // double dagger, U+2021 ISOpub -->
	vec!["\u2021", "&Dagger;", ]
	, // per mille sign, U+2030 ISOtech -->
	vec!["\u2030", "&permil;", ]
	, // single left-pointing angle quotation mark, U+2039 ISO proposed -->
	vec!["\u2039", "&lsaquo;", ]
	, // single right-pointing angle quotation mark, U+203A ISO proposed -->
	vec!["\u203A", "&rsaquo;", ]
	, // -- euro sign, U+20AC NEW -->
	vec!["\u20AC", "&euro;", ]
	, ]
	;

	static HTML40_EXTENDED_UNESCAPE: &[&[/* Java */ java::lang::String /**/]] = org::apache::commons::lang3::text::translate::entity_arrays::EntityArrays::invert(HTML40_EXTENDED_ESCAPE);

	static BASIC_ESCAPE: &[&[/* Java */ java::lang::String /**/]] = vec![// " - double-quote
	vec!["\"", "&quot;", ]
	, // & - ampersand
	vec!["&", "&amp;", ]
	, // < - less-than
	vec!["<", "&lt;", ]
	, // > - greater-than
	vec![">", "&gt;", ]
	, ]
	;

	static BASIC_UNESCAPE: &[&[/* Java */ java::lang::String /**/]] = org::apache::commons::lang3::text::translate::entity_arrays::EntityArrays::invert(BASIC_ESCAPE);

	static APOS_ESCAPE: &[&[/* Java */ java::lang::String /**/]] = vec![// XML apostrophe
	vec!["'", "&apos;", ]
	, ]
	;

	static APOS_UNESCAPE: &[&[/* Java */ java::lang::String /**/]] = org::apache::commons::lang3::text::translate::entity_arrays::EntityArrays::invert(APOS_ESCAPE);

	static JAVA_CTRL_CHARS_ESCAPE: &[&[/* Java */ java::lang::String /**/]] = vec![vec!["\b", "\\b", ]
	, vec!["\n", "\\n", ]
	, vec!["\t", "\\t", ]
	, vec!["\f", "\\f", ]
	, vec!["\r", "\\r", ]
	, ]
	;

	static JAVA_CTRL_CHARS_UNESCAPE: &[&[/* Java */ java::lang::String /**/]] = org::apache::commons::lang3::text::translate::entity_arrays::EntityArrays::invert(JAVA_CTRL_CHARS_ESCAPE);

	pub fn apo_s_escape(&self) -> &[&[/* Java */ java::lang::String /**/]] {
		return self.APOS_ESCAPE.clone();
	}

	pub fn apo_s_unescape(&self) -> &[&[/* Java */ java::lang::String /**/]] {
		return self.APOS_UNESCAPE.clone();
	}

	pub fn basi_c_escape(&self) -> &[&[/* Java */ java::lang::String /**/]] {
		return self.BASIC_ESCAPE.clone();
	}

	pub fn basi_c_unescape(&self) -> &[&[/* Java */ java::lang::String /**/]] {
		return self.BASIC_UNESCAPE.clone();
	}

	pub fn htm_l40_extende_d_escape(&self) -> &[&[/* Java */ java::lang::String /**/]] {
		return self.HTML40_EXTENDED_ESCAPE.clone();
	}

	pub fn htm_l40_extende_d_unescape(&self) -> &[&[/* Java */ java::lang::String /**/]] {
		return self.HTML40_EXTENDED_UNESCAPE.clone();
	}

	pub fn invert(&self, array: &&[&[/* Java */ java::lang::String /**/]]) -> &[&[/* Java */ java::lang::String /**/]] {
		/* final */ let new_array: [[Option<String>; 2]; array.length] = [[None; 2]; array.length];
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					new_array[i][0] = array[i][1];
					new_array[i][1] = array[i][0];
				}
				i += 1;
			 }
		 }
	
		return new_array;
	}

	pub fn is_o8859_1_escape(&self) -> &[&[/* Java */ java::lang::String /**/]] {
		return self.ISO8859_1_ESCAPE.clone();
	}

	pub fn is_o8859_1_unescape(&self) -> &[&[/* Java */ java::lang::String /**/]] {
		return self.ISO8859_1_UNESCAPE.clone();
	}

	pub fn jav_a_ctr_l_char_s_escape(&self) -> &[&[/* Java */ java::lang::String /**/]] {
		return self.JAVA_CTRL_CHARS_ESCAPE.clone();
	}

	pub fn jav_a_ctr_l_char_s_unescape(&self) -> &[&[/* Java */ java::lang::String /**/]] {
		return self.JAVA_CTRL_CHARS_UNESCAPE.clone();
	}

	pub fn new() -> org::apache::commons::lang3::text::translate::entity_arrays::EntityArrays {
	// empty
	}
}