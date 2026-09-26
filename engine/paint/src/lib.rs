use aether_layout::TextRun;
pub struct Frame {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u32>,
}
pub fn paint(runs: &[TextRun], width: u32, height: u32) -> Frame {
    paint_browser_frame(runs, width, height, "", false)
}

pub fn paint_browser_frame(
    runs: &[TextRun],
    width: u32,
    height: u32,
    address: &str,
    editing_address: bool,
) -> Frame {
    let mut pixels = vec![0xfff7f4ed; (width * height) as usize];
    fill_rect(&mut pixels, width, height, 0, 0, width, 72, 0xff20252b);
    fill_rect(&mut pixels, width, height, 12, 12, 34, 28, 0xff313942);
    fill_rect(&mut pixels, width, height, 52, 12, 34, 28, 0xff313942);
    fill_rect(&mut pixels, width, height, 92, 12, 34, 28, 0xff313942);
    fill_rect(
        &mut pixels,
        width,
        height,
        136,
        10,
        width.saturating_sub(148),
        32,
        if editing_address {
            0xfffefefe
        } else {
            0xff303840
        },
    );
    draw_text(&mut pixels, width, height, 22, 22, "<", 0xffd8dee4);
    draw_text(&mut pixels, width, height, 62, 22, ">", 0xffd8dee4);
    draw_text(&mut pixels, width, height, 102, 22, "R", 0xffd8dee4);
    draw_text(
        &mut pixels,
        width,
        height,
        148,
        22,
        if address.is_empty() {
            "Search or enter address"
        } else {
            address
        },
        if editing_address {
            0xff20252b
        } else {
            0xffd8dee4
        },
    );
    for run in runs {
        let mut x = run.x;
        for byte in run.text.bytes() {
            if x + 6 >= width {
                break;
            }
            for row in 0..7 {
                for col in 0..5 {
                    if (byte.rotate_left(row) >> col) & 1 == 1 {
                        let px = x + col;
                        let py = run.y + row + 72;
                        if px < width && py < height {
                            pixels[(py * width + px) as usize] = run.color;
                        }
                    }
                }
            }
            x += 7;
        }
    }
    Frame {
        width,
        height,
        pixels,
    }
}

#[allow(clippy::too_many_arguments)]
fn fill_rect(
    pixels: &mut [u32],
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    color: u32,
) {
    for py in y..y.saturating_add(h).min(height) {
        for px in x..x.saturating_add(w).min(width) {
            pixels[(py * width + px) as usize] = color;
        }
    }
}

fn draw_text(pixels: &mut [u32], width: u32, height: u32, x: u32, y: u32, text: &str, color: u32) {
    let mut cursor = x;
    for byte in text.bytes() {
        for row in 0..7 {
            for col in 0..5 {
                if (byte.rotate_left(row) >> col) & 1 == 1 {
                    let px = cursor + col;
                    let py = y + row;
                    if px < width && py < height {
                        pixels[(py * width + px) as usize] = color;
                    }
                }
            }
        }
        cursor += 7;
        if cursor >= width {
            break;
        }
    }
}
