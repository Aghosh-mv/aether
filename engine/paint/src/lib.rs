use aether_layout::TextRun;
pub struct Frame {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u32>,
}
pub fn paint(runs: &[TextRun], width: u32, height: u32) -> Frame {
    let mut pixels = vec![0xfff7f4ed; (width * height) as usize];
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
                        let py = run.y + row;
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
