use image::GenericImageView;
use std::fs;
use std::io;
use text_to_png::TextRenderer;
fn get_str_ascii(intent: u8) -> &'static str {
    let index = intent / 32;
    let ascii = [" ", "-", "+", "?", "*", "$", "#", "@"];
    // let ascii = ["N", "I", "T", "H", "i", "n", "#", "@"];
    return ascii[index as usize].into();
}
fn get_image(dir: &str, scale: u32) -> String {
    let mut image_str = String::new();
    let img = image::open(dir).unwrap();
    println!("{:?}", img.dimensions());
    let (width, height) = img.dimensions();
    for y in 0..height {
        for x in 0..width {
            if y % (scale * 2) == 0 && x % scale == 2 {
                let pix = img.get_pixel(x, y);
                let mut intent = pix[0] / 3 + pix[1] / 3 + pix[2] / 3;
                if pix[3] == 0 {
                    intent = 0;
                }
                image_str.push_str(get_str_ascii(intent));
                // print!("{}", get_str_ascii(intent))
            }
        }
        if y % (scale * 2) == 0 {
            image_str.push_str("\n");
            // println!("");
        }
    }

    image_str
}
fn main() {
    let mut scale = String::new();
    print!("enter the scale");
    let read = io::stdin();
    read.read_line(&mut scale).unwrap();
    let scale_int: u32 = scale.trim().parse().unwrap();

    let img = get_image("nithin_1.jpg", scale_int);
    print!("{}", img);
    fs::write("output/text.txt", &img).expect("Unable to write file");
    let renderer = TextRenderer::default();

    let text_png = renderer.render_text_to_png_data(img, 10, "black");
    fs::write("output/text.png", text_png.unwrap().data).expect("Unable to write file");
}
