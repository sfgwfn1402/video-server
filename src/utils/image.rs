use image::codecs::png::PngEncoder;
use image::ImageEncoder;

pub fn create_error_image(_error_msg: &str) -> Vec<u8> {
    // 创建一个错误提示图片
    let imgx = 400;
    let imgy = 200;
    let mut img = image::RgbImage::new(imgx, imgy);
    
    // 填充红色背景
    for pixel in img.pixels_mut() {
        *pixel = image::Rgb([255, 200, 200]);
    }
    
    // 编码为 PNG
    let mut buf = Vec::new();
    {
        let encoder = PngEncoder::new(&mut buf);
        encoder.write_image(
            img.as_raw(),
            imgx,
            imgy,
            image::ExtendedColorType::Rgb8,
        ).unwrap();
    }
    
    buf
} 