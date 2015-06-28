extern crate sdl2;

use font_data::*;

impl Glyph {
    fn draw(self, renderer: &mut sdl2::render::Renderer,
            x: i32, y: i32, scale: u32) {
        for (i, bits) in self.1.into_iter().enumerate() {
            for j in 0..8 {
                if bits & (1 << j) != 0 {
                    let box_size = (9 * scale + 5) / 10 - 1;
                    let rect = sdl2::rect::Rect::new_unwrap(x + (j as u32 * scale) as i32,
                                                            y + (i as u32 * scale) as i32,
                                                            box_size, box_size);
                    renderer.fill_rect(rect);
                }
            }
        }
    }

    fn find_by_char(c: char) -> Glyph {
        let mut index = c as usize;

        if index < FIRST_GLYPH {
            return GLYPHS[GLYPH_COUNT - 1];
        }
        index = index - FIRST_GLYPH;
        if index >= GLYPH_COUNT {
            return GLYPHS[GLYPH_COUNT - 1];
        }

        return GLYPHS[index];
    }
}

pub fn draw_text(text: &str,
                 renderer: &mut sdl2::render::Renderer,
                 x: i32, y: i32, scale: u32) {
    let mut cursor_x = x;
    for glyph in text.chars().map(Glyph::find_by_char) {
        glyph.draw(renderer, cursor_x, y, scale);
        cursor_x += (glyph.0 as u32 * scale / GLYPH_FP_MULTIPLIER) as i32
    }
}

pub fn measure_text(text: &str, scale: u32) -> (u32, u32) {
    let mut cursor_x = 0;
    for glyph in text.chars().map(Glyph::find_by_char) {
        cursor_x += glyph.0 as u32 * scale / GLYPH_FP_MULTIPLIER;
    }
    cursor_x -= GLYPH_SPACING * scale;

    (cursor_x, GLYPH_HEIGHT as u32 * scale)
}

pub fn line_spacing(scale: u32) -> i32 {
    (scale * GLYPH_HEIGHT as u32) as i32
}
