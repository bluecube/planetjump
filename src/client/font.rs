extern crate sdl2;

const FIRST_GLYPH: usize = 32;
const GLYPH_COUNT: usize = 95;
const GLYPH_HEIGHT: usize = 9;
const GLYPH_ASCENT: u32 = 7;

#[derive(Copy,Clone,Debug)]
struct Glyph (u8, [u8; GLYPH_HEIGHT]);

const GLYPHS: [Glyph; GLYPH_COUNT] = [
    Glyph(5, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), /*   */
    Glyph(2, [0x01, 0x01, 0x01, 0x01, 0x01, 0x00, 0x01, 0x00, 0x00]), /* ! */
    Glyph(4, [0x05, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), /* " */
    Glyph(6, [0x00, 0x00, 0x0a, 0x1f, 0x0a, 0x1f, 0x0a, 0x00, 0x00]), /* # */
    Glyph(6, [0x0e, 0x15, 0x05, 0x0e, 0x14, 0x15, 0x0e, 0x00, 0x00]), /* $ */
    Glyph(6, [0x00, 0x00, 0x11, 0x08, 0x04, 0x02, 0x11, 0x00, 0x00]), /* % */
    Glyph(8, [0x0c, 0x12, 0x12, 0x0c, 0x4a, 0x11, 0x2e, 0x00, 0x00]), /* & */
    Glyph(2, [0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), /* ' */
    Glyph(4, [0x04, 0x02, 0x01, 0x01, 0x01, 0x02, 0x04, 0x00, 0x00]), /* ( */
    Glyph(4, [0x01, 0x02, 0x04, 0x04, 0x04, 0x02, 0x01, 0x00, 0x00]), /* ) */
    Glyph(6, [0x00, 0x15, 0x0e, 0x1f, 0x0e, 0x15, 0x00, 0x00, 0x00]), /* * */
    Glyph(6, [0x00, 0x04, 0x04, 0x1f, 0x04, 0x04, 0x00, 0x00, 0x00]), /* + */
    Glyph(3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x01, 0x00]), /* , */
    Glyph(6, [0x00, 0x00, 0x00, 0x1f, 0x00, 0x00, 0x00, 0x00, 0x00]), /* - */
    Glyph(2, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00]), /* . */
    Glyph(4, [0x04, 0x04, 0x02, 0x02, 0x02, 0x01, 0x01, 0x00, 0x00]), /* / */
    Glyph(6, [0x0e, 0x11, 0x19, 0x15, 0x13, 0x11, 0x0e, 0x00, 0x00]), /* 0 */
    Glyph(4, [0x02, 0x03, 0x02, 0x02, 0x02, 0x02, 0x07, 0x00, 0x00]), /* 1 */
    Glyph(6, [0x0e, 0x11, 0x10, 0x08, 0x04, 0x02, 0x1f, 0x00, 0x00]), /* 2 */
    Glyph(6, [0x0e, 0x11, 0x10, 0x08, 0x10, 0x11, 0x0e, 0x00, 0x00]), /* 3 */
    Glyph(6, [0x08, 0x0c, 0x0a, 0x09, 0x1f, 0x08, 0x08, 0x00, 0x00]), /* 4 */
    Glyph(6, [0x1f, 0x01, 0x0f, 0x10, 0x10, 0x11, 0x0e, 0x00, 0x00]), /* 5 */
    Glyph(6, [0x0e, 0x11, 0x01, 0x0f, 0x11, 0x11, 0x0e, 0x00, 0x00]), /* 6 */
    Glyph(6, [0x1f, 0x10, 0x08, 0x04, 0x02, 0x02, 0x02, 0x00, 0x00]), /* 7 */
    Glyph(6, [0x0e, 0x11, 0x11, 0x0e, 0x11, 0x11, 0x0e, 0x00, 0x00]), /* 8 */
    Glyph(6, [0x0e, 0x11, 0x11, 0x1e, 0x10, 0x11, 0x0e, 0x00, 0x00]), /* 9 */
    Glyph(2, [0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00]), /* : */
    Glyph(3, [0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x01, 0x00]), /* ; */
    Glyph(6, [0x00, 0x18, 0x06, 0x01, 0x06, 0x18, 0x00, 0x00, 0x00]), /* < */
    Glyph(6, [0x00, 0x00, 0x1f, 0x00, 0x1f, 0x00, 0x00, 0x00, 0x00]), /* = */
    Glyph(6, [0x00, 0x03, 0x0c, 0x10, 0x0c, 0x03, 0x00, 0x00, 0x00]), /* > */
    Glyph(6, [0x0e, 0x11, 0x10, 0x08, 0x04, 0x00, 0x04, 0x00, 0x00]), /* ? */
    Glyph(9, [0x00, 0x3c, 0x42, 0xb9, 0xa5, 0xa5, 0x79, 0x00, 0x00]), /* @ */
    Glyph(6, [0x04, 0x0a, 0x11, 0x11, 0x1f, 0x11, 0x11, 0x00, 0x00]), /* A */
    Glyph(6, [0x0f, 0x11, 0x11, 0x0f, 0x11, 0x11, 0x0f, 0x00, 0x00]), /* B */
    Glyph(6, [0x0e, 0x11, 0x01, 0x01, 0x01, 0x11, 0x0e, 0x00, 0x00]), /* C */
    Glyph(6, [0x0f, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0f, 0x00, 0x00]), /* D */
    Glyph(6, [0x1f, 0x01, 0x01, 0x0f, 0x01, 0x01, 0x1f, 0x00, 0x00]), /* E */
    Glyph(6, [0x1f, 0x01, 0x01, 0x0f, 0x01, 0x01, 0x01, 0x00, 0x00]), /* F */
    Glyph(6, [0x0e, 0x11, 0x01, 0x19, 0x11, 0x11, 0x0e, 0x00, 0x00]), /* G */
    Glyph(6, [0x11, 0x11, 0x11, 0x1f, 0x11, 0x11, 0x11, 0x00, 0x00]), /* H */
    Glyph(4, [0x07, 0x02, 0x02, 0x02, 0x02, 0x02, 0x07, 0x00, 0x00]), /* I */
    Glyph(5, [0x0e, 0x08, 0x08, 0x08, 0x08, 0x09, 0x06, 0x00, 0x00]), /* J */
    Glyph(6, [0x11, 0x09, 0x05, 0x07, 0x09, 0x11, 0x11, 0x00, 0x00]), /* K */
    Glyph(6, [0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x1f, 0x00, 0x00]), /* L */
    Glyph(8, [0x41, 0x63, 0x55, 0x49, 0x41, 0x41, 0x41, 0x00, 0x00]), /* M */
    Glyph(7, [0x21, 0x23, 0x25, 0x29, 0x31, 0x21, 0x21, 0x00, 0x00]), /* N */
    Glyph(6, [0x0e, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0e, 0x00, 0x00]), /* O */
    Glyph(6, [0x0f, 0x11, 0x11, 0x0f, 0x01, 0x01, 0x01, 0x00, 0x00]), /* P */
    Glyph(6, [0x0e, 0x11, 0x11, 0x11, 0x11, 0x15, 0x0e, 0x08, 0x10]), /* Q */
    Glyph(6, [0x0f, 0x11, 0x11, 0x0f, 0x09, 0x11, 0x11, 0x00, 0x00]), /* R */
    Glyph(6, [0x0e, 0x11, 0x01, 0x0e, 0x10, 0x11, 0x0e, 0x00, 0x00]), /* S */
    Glyph(6, [0x1f, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x00, 0x00]), /* T */
    Glyph(6, [0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0e, 0x00, 0x00]), /* U */
    Glyph(6, [0x11, 0x11, 0x11, 0x11, 0x11, 0x0a, 0x04, 0x00, 0x00]), /* V */
    Glyph(8, [0x41, 0x41, 0x41, 0x49, 0x49, 0x49, 0x36, 0x00, 0x00]), /* W */
    Glyph(6, [0x11, 0x11, 0x0a, 0x04, 0x0a, 0x11, 0x11, 0x00, 0x00]), /* X */
    Glyph(6, [0x11, 0x11, 0x0a, 0x04, 0x04, 0x04, 0x04, 0x00, 0x00]), /* Y */
    Glyph(6, [0x1f, 0x10, 0x08, 0x04, 0x02, 0x01, 0x1f, 0x00, 0x00]), /* Z */
    Glyph(4, [0x07, 0x01, 0x01, 0x01, 0x01, 0x01, 0x07, 0x00, 0x00]), /* [ */
    Glyph(4, [0x01, 0x01, 0x02, 0x02, 0x02, 0x04, 0x04, 0x00, 0x00]), /* \ */
    Glyph(4, [0x07, 0x04, 0x04, 0x04, 0x04, 0x04, 0x07, 0x00, 0x00]), /* ] */
    Glyph(5, [0x04, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), /* ^ */
    Glyph(6, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1f, 0x00]), /* _ */
    Glyph(3, [0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), /* ` */
    Glyph(6, [0x00, 0x00, 0x1e, 0x11, 0x11, 0x11, 0x1e, 0x00, 0x00]), /* a */
    Glyph(6, [0x01, 0x01, 0x0f, 0x11, 0x11, 0x11, 0x0f, 0x00, 0x00]), /* b */
    Glyph(6, [0x00, 0x00, 0x0e, 0x11, 0x01, 0x11, 0x0e, 0x00, 0x00]), /* c */
    Glyph(6, [0x10, 0x10, 0x1e, 0x11, 0x11, 0x11, 0x1e, 0x00, 0x00]), /* d */
    Glyph(6, [0x00, 0x00, 0x0e, 0x11, 0x1f, 0x01, 0x0e, 0x00, 0x00]), /* e */
    Glyph(4, [0x06, 0x01, 0x01, 0x03, 0x01, 0x01, 0x01, 0x00, 0x00]), /* f */
    Glyph(6, [0x00, 0x00, 0x1e, 0x11, 0x11, 0x11, 0x1e, 0x10, 0x0e]), /* g */
    Glyph(6, [0x01, 0x01, 0x0f, 0x11, 0x11, 0x11, 0x11, 0x00, 0x00]), /* h */
    Glyph(2, [0x01, 0x00, 0x01, 0x01, 0x01, 0x01, 0x01, 0x00, 0x00]), /* i */
    Glyph(4, [0x04, 0x00, 0x04, 0x04, 0x04, 0x04, 0x03, 0x00, 0x00]), /* j */
    Glyph(6, [0x01, 0x01, 0x11, 0x09, 0x07, 0x09, 0x11, 0x00, 0x00]), /* k */
    Glyph(4, [0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x06, 0x00, 0x00]), /* l */
    Glyph(8, [0x00, 0x00, 0x36, 0x49, 0x49, 0x49, 0x49, 0x00, 0x00]), /* m */
    Glyph(6, [0x00, 0x00, 0x0e, 0x11, 0x11, 0x11, 0x11, 0x00, 0x00]), /* n */
    Glyph(6, [0x00, 0x00, 0x0e, 0x11, 0x11, 0x11, 0x0e, 0x00, 0x00]), /* o */
    Glyph(6, [0x00, 0x00, 0x0f, 0x11, 0x11, 0x11, 0x0f, 0x01, 0x01]), /* p */
    Glyph(6, [0x00, 0x00, 0x1e, 0x11, 0x11, 0x11, 0x1e, 0x10, 0x10]), /* q */
    Glyph(5, [0x00, 0x00, 0x0e, 0x01, 0x01, 0x01, 0x01, 0x00, 0x00]), /* r */
    Glyph(6, [0x00, 0x00, 0x1e, 0x01, 0x0e, 0x10, 0x0f, 0x00, 0x00]), /* s */
    Glyph(4, [0x01, 0x01, 0x03, 0x01, 0x01, 0x01, 0x06, 0x00, 0x00]), /* t */
    Glyph(6, [0x00, 0x00, 0x11, 0x11, 0x11, 0x11, 0x0e, 0x00, 0x00]), /* u */
    Glyph(6, [0x00, 0x00, 0x11, 0x11, 0x11, 0x0a, 0x04, 0x00, 0x00]), /* v */
    Glyph(8, [0x00, 0x00, 0x41, 0x41, 0x41, 0x49, 0x36, 0x00, 0x00]), /* w */
    Glyph(6, [0x00, 0x00, 0x11, 0x0a, 0x04, 0x0a, 0x11, 0x00, 0x00]), /* x */
    Glyph(6, [0x00, 0x00, 0x11, 0x11, 0x11, 0x0a, 0x04, 0x02, 0x00]), /* y */
    Glyph(6, [0x00, 0x00, 0x1f, 0x08, 0x04, 0x02, 0x1f, 0x00, 0x00]), /* z */
    Glyph(4, [0x04, 0x02, 0x02, 0x01, 0x02, 0x02, 0x04, 0x00, 0x00]), /* { */
    Glyph(2, [0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x00, 0x00]), /* | */
    Glyph(4, [0x01, 0x02, 0x02, 0x04, 0x02, 0x02, 0x01, 0x00, 0x00]), /* } */
    Glyph(6, [0x00, 0x00, 0x02, 0x15, 0x08, 0x00, 0x00, 0x00, 0x00]), /* ~ */
];
const UNKNOWN_CHAR_GLYPH: Glyph = Glyph(6, [0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x00, 0x00]);

impl Glyph {
    fn draw(self, drawer: &mut sdl2::render::RenderDrawer,
            x: i32, y: i32, scale: i32) {
        for (i, bits) in self.1.into_iter().enumerate() {
            for j in 0..8 {
                if bits & (1 << j) != 0 {
                    let box_size = (4 * scale) / 5;
                    let rect = sdl2::rect::Rect::new(x + ((j as i32) * scale),
                                                     y + ((i as i32) * scale),
                                                     box_size, box_size);
                    drawer.fill_rect(rect);
                }
            }
        }
    }

    fn find_by_char(c: char) -> Glyph {
        let index = c as usize;

        if index < FIRST_GLYPH {
            return UNKNOWN_CHAR_GLYPH;
        }
        let glyph_index = index - FIRST_GLYPH;
        if glyph_index >= GLYPH_COUNT {
            return UNKNOWN_CHAR_GLYPH;
        }

        return GLYPHS[glyph_index];
    }
}

pub fn draw_text(text: &str,
                 drawer: &mut sdl2::render::RenderDrawer,
                 x: i32, y: i32, scale: i32) {
    let mut cursor_x = x;
    for glyph in text.chars().map(Glyph::find_by_char) {
        glyph.draw(drawer, cursor_x, y, scale);
        cursor_x += (glyph.0 as i32) * scale
    }
}

pub fn measure_text(text: &str, scale: i32) -> sdl2::rect::Rect {
    let mut cursor_x = 0;
    for glyph in text.chars().map(Glyph::find_by_char) {
        cursor_x += (glyph.0 as i32) * scale
    }

    sdl2::rect::Rect::new(0, 0, cursor_x, (GLYPH_HEIGHT as i32) * scale)
}
