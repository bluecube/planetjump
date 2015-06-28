pub const FIRST_GLYPH: usize = 32;
pub const GLYPH_COUNT: usize = 96;
pub const GLYPH_HEIGHT: usize = 9;
pub const GLYPH_ASCENT: u32 = 7;
pub const GLYPH_SPACING: u32 = 1;
pub const GLYPH_FP_MULTIPLIER: u32 = 16;

#[derive(Copy,Clone,Debug)]
pub struct Glyph (pub u8, pub [u8; GLYPH_HEIGHT]);

pub const GLYPHS: [Glyph; GLYPH_COUNT] = [
    Glyph(0x40, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), /*   */
    Glyph(0x1b, [0x01, 0x01, 0x01, 0x01, 0x01, 0x00, 0x01, 0x00, 0x00]), /* ! */
    Glyph(0x40, [0x05, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), /* " */
    Glyph(0x57, [0x00, 0x00, 0x0a, 0x1f, 0x0a, 0x1f, 0x0a, 0x00, 0x00]), /* # */
    Glyph(0x56, [0x0e, 0x15, 0x05, 0x0e, 0x14, 0x15, 0x0e, 0x00, 0x00]), /* $ */
    Glyph(0x51, [0x00, 0x00, 0x11, 0x08, 0x04, 0x02, 0x11, 0x00, 0x00]), /* % */
    Glyph(0x68, [0x0c, 0x12, 0x12, 0x0c, 0x4a, 0x11, 0x2e, 0x00, 0x00]), /* & */
    Glyph(0x20, [0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), /* ' */
    Glyph(0x2b, [0x04, 0x02, 0x01, 0x01, 0x01, 0x02, 0x04, 0x00, 0x00]), /* ( */
    Glyph(0x37, [0x01, 0x02, 0x04, 0x04, 0x04, 0x02, 0x01, 0x00, 0x00]), /* ) */
    Glyph(0x59, [0x00, 0x15, 0x0e, 0x1f, 0x0e, 0x15, 0x00, 0x00, 0x00]), /* * */
    Glyph(0x4e, [0x00, 0x04, 0x04, 0x1f, 0x04, 0x04, 0x00, 0x00, 0x00]), /* + */
    Glyph(0x30, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x01, 0x00]), /* , */
    Glyph(0x60, [0x00, 0x00, 0x00, 0x1f, 0x00, 0x00, 0x00, 0x00, 0x00]), /* - */
    Glyph(0x20, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00]), /* . */
    Glyph(0x2c, [0x04, 0x04, 0x02, 0x02, 0x02, 0x01, 0x01, 0x00, 0x00]), /* / */
    Glyph(0x5b, [0x0e, 0x11, 0x19, 0x15, 0x13, 0x11, 0x0e, 0x00, 0x00]), /* 0 */
    Glyph(0x2e, [0x02, 0x03, 0x02, 0x02, 0x02, 0x02, 0x07, 0x00, 0x00]), /* 1 */
    Glyph(0x51, [0x0e, 0x11, 0x10, 0x08, 0x04, 0x02, 0x1f, 0x00, 0x00]), /* 2 */
    Glyph(0x58, [0x0e, 0x11, 0x10, 0x08, 0x10, 0x11, 0x0e, 0x00, 0x00]), /* 3 */
    Glyph(0x51, [0x08, 0x0c, 0x0a, 0x09, 0x1f, 0x08, 0x08, 0x00, 0x00]), /* 4 */
    Glyph(0x57, [0x1f, 0x01, 0x0f, 0x10, 0x10, 0x11, 0x0e, 0x00, 0x00]), /* 5 */
    Glyph(0x55, [0x0e, 0x11, 0x01, 0x0f, 0x11, 0x11, 0x0e, 0x00, 0x00]), /* 6 */
    Glyph(0x46, [0x1f, 0x10, 0x08, 0x04, 0x02, 0x02, 0x02, 0x00, 0x00]), /* 7 */
    Glyph(0x58, [0x0e, 0x11, 0x11, 0x0e, 0x11, 0x11, 0x0e, 0x00, 0x00]), /* 8 */
    Glyph(0x5b, [0x0e, 0x11, 0x11, 0x1e, 0x10, 0x11, 0x0e, 0x00, 0x00]), /* 9 */
    Glyph(0x1b, [0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00]), /* : */
    Glyph(0x2b, [0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x01, 0x00]), /* ; */
    Glyph(0x4e, [0x00, 0x18, 0x06, 0x01, 0x06, 0x18, 0x00, 0x00, 0x00]), /* < */
    Glyph(0x60, [0x00, 0x00, 0x1f, 0x00, 0x1f, 0x00, 0x00, 0x00, 0x00]), /* = */
    Glyph(0x50, [0x00, 0x03, 0x0c, 0x10, 0x0c, 0x03, 0x00, 0x00, 0x00]), /* > */
    Glyph(0x50, [0x0e, 0x11, 0x10, 0x08, 0x04, 0x00, 0x04, 0x00, 0x00]), /* ? */
    Glyph(0x87, [0x00, 0x3c, 0x42, 0xb9, 0xa5, 0xa5, 0x79, 0x00, 0x00]), /* @ */
    Glyph(0x5b, [0x04, 0x0a, 0x11, 0x11, 0x1f, 0x11, 0x11, 0x00, 0x00]), /* A */
    Glyph(0x58, [0x0f, 0x11, 0x11, 0x0f, 0x11, 0x11, 0x0f, 0x00, 0x00]), /* B */
    Glyph(0x49, [0x0e, 0x11, 0x01, 0x01, 0x01, 0x11, 0x0e, 0x00, 0x00]), /* C */
    Glyph(0x5b, [0x0f, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0f, 0x00, 0x00]), /* D */
    Glyph(0x44, [0x1f, 0x01, 0x01, 0x0f, 0x01, 0x01, 0x1f, 0x00, 0x00]), /* E */
    Glyph(0x33, [0x1f, 0x01, 0x01, 0x0f, 0x01, 0x01, 0x01, 0x00, 0x00]), /* F */
    Glyph(0x58, [0x0e, 0x11, 0x01, 0x19, 0x11, 0x11, 0x0e, 0x00, 0x00]), /* G */
    Glyph(0x5c, [0x11, 0x11, 0x11, 0x1f, 0x11, 0x11, 0x11, 0x00, 0x00]), /* H */
    Glyph(0x30, [0x07, 0x02, 0x02, 0x02, 0x02, 0x02, 0x07, 0x00, 0x00]), /* I */
    Glyph(0x4b, [0x0e, 0x08, 0x08, 0x08, 0x08, 0x09, 0x06, 0x00, 0x00]), /* J */
    Glyph(0x51, [0x11, 0x09, 0x05, 0x07, 0x09, 0x11, 0x11, 0x00, 0x00]), /* K */
    Glyph(0x31, [0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x1f, 0x00, 0x00]), /* L */
    Glyph(0x7c, [0x41, 0x63, 0x55, 0x49, 0x41, 0x41, 0x41, 0x00, 0x00]), /* M */
    Glyph(0x6c, [0x21, 0x23, 0x25, 0x29, 0x31, 0x21, 0x21, 0x00, 0x00]), /* N */
    Glyph(0x5b, [0x0e, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0e, 0x00, 0x00]), /* O */
    Glyph(0x49, [0x0f, 0x11, 0x11, 0x0f, 0x01, 0x01, 0x01, 0x00, 0x00]), /* P */
    Glyph(0x5b, [0x0e, 0x11, 0x11, 0x11, 0x11, 0x15, 0x0e, 0x08, 0x10]), /* Q */
    Glyph(0x57, [0x0f, 0x11, 0x11, 0x0f, 0x09, 0x11, 0x11, 0x00, 0x00]), /* R */
    Glyph(0x55, [0x0e, 0x11, 0x01, 0x0e, 0x10, 0x11, 0x0e, 0x00, 0x00]), /* S */
    Glyph(0x40, [0x1f, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x00, 0x00]), /* T */
    Glyph(0x5b, [0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0e, 0x00, 0x00]), /* U */
    Glyph(0x58, [0x11, 0x11, 0x11, 0x11, 0x11, 0x0a, 0x04, 0x00, 0x00]), /* V */
    Glyph(0x7b, [0x41, 0x41, 0x41, 0x49, 0x49, 0x49, 0x36, 0x00, 0x00]), /* W */
    Glyph(0x54, [0x11, 0x11, 0x0a, 0x04, 0x0a, 0x11, 0x11, 0x00, 0x00]), /* X */
    Glyph(0x4a, [0x11, 0x11, 0x0a, 0x04, 0x04, 0x04, 0x04, 0x00, 0x00]), /* Y */
    Glyph(0x4d, [0x1f, 0x10, 0x08, 0x04, 0x02, 0x01, 0x1f, 0x00, 0x00]), /* Z */
    Glyph(0x28, [0x07, 0x01, 0x01, 0x01, 0x01, 0x01, 0x07, 0x00, 0x00]), /* [ */
    Glyph(0x31, [0x01, 0x01, 0x02, 0x02, 0x02, 0x04, 0x04, 0x00, 0x00]), /* \ */
    Glyph(0x3c, [0x07, 0x04, 0x04, 0x04, 0x04, 0x04, 0x07, 0x00, 0x00]), /* ] */
    Glyph(0x4b, [0x04, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), /* ^ */
    Glyph(0x60, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1f, 0x00]), /* _ */
    Glyph(0x2b, [0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), /* ` */
    Glyph(0x5c, [0x00, 0x00, 0x1e, 0x11, 0x11, 0x11, 0x1e, 0x00, 0x00]), /* a */
    Glyph(0x55, [0x01, 0x01, 0x0f, 0x11, 0x11, 0x11, 0x0f, 0x00, 0x00]), /* b */
    Glyph(0x54, [0x00, 0x00, 0x0e, 0x11, 0x01, 0x11, 0x0e, 0x00, 0x00]), /* c */
    Glyph(0x5c, [0x10, 0x10, 0x1e, 0x11, 0x11, 0x11, 0x1e, 0x00, 0x00]), /* d */
    Glyph(0x55, [0x00, 0x00, 0x0e, 0x11, 0x1f, 0x01, 0x0e, 0x00, 0x00]), /* e */
    Glyph(0x22, [0x06, 0x01, 0x01, 0x03, 0x01, 0x01, 0x01, 0x00, 0x00]), /* f */
    Glyph(0x5c, [0x00, 0x00, 0x1e, 0x11, 0x11, 0x11, 0x1e, 0x10, 0x0e]), /* g */
    Glyph(0x56, [0x01, 0x01, 0x0f, 0x11, 0x11, 0x11, 0x11, 0x00, 0x00]), /* h */
    Glyph(0x1c, [0x01, 0x00, 0x01, 0x01, 0x01, 0x01, 0x01, 0x00, 0x00]), /* i */
    Glyph(0x3b, [0x04, 0x00, 0x04, 0x04, 0x04, 0x04, 0x03, 0x00, 0x00]), /* j */
    Glyph(0x4d, [0x01, 0x01, 0x11, 0x09, 0x07, 0x09, 0x11, 0x00, 0x00]), /* k */
    Glyph(0x22, [0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x06, 0x00, 0x00]), /* l */
    Glyph(0x7b, [0x00, 0x00, 0x36, 0x49, 0x49, 0x49, 0x49, 0x00, 0x00]), /* m */
    Glyph(0x5b, [0x00, 0x00, 0x0e, 0x11, 0x11, 0x11, 0x11, 0x00, 0x00]), /* n */
    Glyph(0x5a, [0x00, 0x00, 0x0e, 0x11, 0x11, 0x11, 0x0e, 0x00, 0x00]), /* o */
    Glyph(0x59, [0x00, 0x00, 0x0f, 0x11, 0x11, 0x11, 0x0f, 0x01, 0x01]), /* p */
    Glyph(0x5c, [0x00, 0x00, 0x1e, 0x11, 0x11, 0x11, 0x1e, 0x10, 0x10]), /* q */
    Glyph(0x34, [0x00, 0x00, 0x0e, 0x01, 0x01, 0x01, 0x01, 0x00, 0x00]), /* r */
    Glyph(0x52, [0x00, 0x00, 0x1e, 0x01, 0x0e, 0x10, 0x0f, 0x00, 0x00]), /* s */
    Glyph(0x25, [0x01, 0x01, 0x03, 0x01, 0x01, 0x01, 0x06, 0x00, 0x00]), /* t */
    Glyph(0x5b, [0x00, 0x00, 0x11, 0x11, 0x11, 0x11, 0x0e, 0x00, 0x00]), /* u */
    Glyph(0x58, [0x00, 0x00, 0x11, 0x11, 0x11, 0x0a, 0x04, 0x00, 0x00]), /* v */
    Glyph(0x7b, [0x00, 0x00, 0x41, 0x41, 0x41, 0x49, 0x36, 0x00, 0x00]), /* w */
    Glyph(0x53, [0x00, 0x00, 0x11, 0x0a, 0x04, 0x0a, 0x11, 0x00, 0x00]), /* x */
    Glyph(0x57, [0x00, 0x00, 0x11, 0x11, 0x11, 0x0a, 0x04, 0x02, 0x00]), /* y */
    Glyph(0x51, [0x00, 0x00, 0x1f, 0x08, 0x04, 0x02, 0x1f, 0x00, 0x00]), /* z */
    Glyph(0x2e, [0x04, 0x02, 0x02, 0x01, 0x02, 0x02, 0x04, 0x00, 0x00]), /* { */
    Glyph(0x1c, [0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x00, 0x00]), /* | */
    Glyph(0x30, [0x01, 0x02, 0x02, 0x04, 0x02, 0x02, 0x01, 0x00, 0x00]), /* } */
    Glyph(0x54, [0x00, 0x00, 0x02, 0x15, 0x08, 0x00, 0x00, 0x00, 0x00]), /* ~ */
    Glyph(0x5c, [0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x00, 0x00]), /* Unknown character */
];