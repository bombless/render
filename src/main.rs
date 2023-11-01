use image::{Rgb, ImageBuffer};

mod lib;

fn main() {
    // 定义图像的宽度和高度
    let width = 256;
    let height = 256;

    // 创建一个图像缓冲区，使用RGB颜色模式
    let mut image_buffer = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(width, height);

    // 生成RGB序列
    let rgb_sequence = generate_rgb_sequence(width, height);

    // 将RGB序列写入图像缓冲区
    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        *pixel = rgb_sequence[y as usize][x as usize];
    }

    // 保存图像为PNG文件
    image_buffer.save("output.png").unwrap();
}

fn generate_rgb_sequence(width: u32, height: u32) -> Vec<Vec<Rgb<u8>>> {
    let mut sequence = vec![vec![Rgb([255, 0, 0]); width as usize]; height as usize];

    // 在这里生成RGB序列的逻辑
    // 可以根据需要修改

    sequence
}
