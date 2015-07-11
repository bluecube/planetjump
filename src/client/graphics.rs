const WINDOW_COLOR: (u8, u8, u8, u8) = (127, 127, 255, 255);
const GUN_COLOR: (u8, u8, u8, u8) = (0, 0, 0, 255);

fn abs(x: f32) -> f32 {
    if x >= 0.0 { x } else { -x }
}

pub fn particle(color: (u8, u8, u8, u8),
                x: f32, y: f32) -> (u8, u8, u8, u8) {
    let threshold1 = 0.15 * 0.15;
    let threshold2 = 0.5 * 0.5;
    let (r, g, b, a) = color;

    let r2 = x * x + y * y;

    let alpha = if r2 < threshold1 {
         a
    }
    else if r2 < threshold2 {
        let a_float = a as f32;
        let multiplier = (threshold2 - r2) / (threshold2 - threshold1);
        (a_float * multiplier / 2.0) as u8
    }
    else {
        0
    };
    (r, g, b, alpha)
}

pub fn saucer(color1: (u8, u8, u8, u8),
              color2: (u8, u8, u8, u8),
              x: f32, y: f32) -> (u8, u8, u8, u8) {
    let threshold1 = 0.18 * 0.18;
    let threshold2 = 0.40 * 0.40;

    let gun_position = 0.27;
    let gun_y2 = 0.49 * 0.49 - gun_position * gun_position;

    let r2 = x * x + y * y;

    if r2 < threshold1 {
        WINDOW_COLOR
    }
    else if r2 < threshold2 {
        if (x > 0.23 && x < 0.3) || (x > 0.1 && x < 0.17) {
            color2
        }
        else {
            color1
        }
    }
    else if y < 0.0 && y * y < gun_y2 && abs(abs(x) - gun_position) < 0.02 {
        GUN_COLOR
    }
    else {
        (0, 0, 0, 0)
    }
}

pub fn ship1(color1: (u8, u8, u8, u8),
             color2: (u8, u8, u8, u8),
             x: f32, y: f32) -> (u8, u8, u8, u8) {
    let absx = abs(x);

    let fuselage = 0.5 - 8.0 * absx * absx;
    let wings = 0.3 - (0.3 / 0.5) * absx;
    let front = fuselage.max(wings);
    let back = 0.4 - 0.5 * absx * absx;

    let window_front = 0.4 - 14.0 * absx * absx;
    let window_back = 0.0 - 4.0 * absx * absx;

    if y > -window_front && y < window_back {
        WINDOW_COLOR
    }
    else if y > -front && y < back {
        if (x > 0.23 && x < 0.3) || (x > 0.1 && x < 0.17) {
            color2
        }
        else {
            color1
        }
    }
    else if absx < 0.15 && y < 0.45 && y > 0.0 {
        GUN_COLOR
    }
    else {
        (0, 0, 0, 0)
    }
}
