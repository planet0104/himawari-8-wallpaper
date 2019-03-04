//http://himawari8.nict.go.jp/himawari8-image.htm

use chrono::Local;
use image::{ImageBuffer, Rgb};
use png::{ColorType, OutputInfo};

/*
合成4x4的图，小图顺序为

00,10,20,30
01,11,21,31
02,12,22,32
03,13,23,33

*/
pub fn combine_4x4<C>(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    ten_minute: u32,
    callback: C,
) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, Box<std::error::Error>>
where
    C: Fn(i32, i32) + 'static,
{
    println!("{:?} 下载4x4图片..", Local::now());
    let img00 = format_url(year, month, day, hour, ten_minute / 10, 4, 0, 0);
    let img10 = format_url(year, month, day, hour, ten_minute / 10, 4, 1, 0);
    let img20 = format_url(year, month, day, hour, ten_minute / 10, 4, 2, 0);
    let img30 = format_url(year, month, day, hour, ten_minute / 10, 4, 3, 0);

    let img01 = format_url(year, month, day, hour, ten_minute / 10, 4, 0, 1);
    let img11 = format_url(year, month, day, hour, ten_minute / 10, 4, 1, 1);
    let img21 = format_url(year, month, day, hour, ten_minute / 10, 4, 2, 1);
    let img31 = format_url(year, month, day, hour, ten_minute / 10, 4, 3, 1);

    let img02 = format_url(year, month, day, hour, ten_minute / 10, 4, 0, 2);
    let img12 = format_url(year, month, day, hour, ten_minute / 10, 4, 1, 2);
    let img22 = format_url(year, month, day, hour, ten_minute / 10, 4, 2, 2);
    let img32 = format_url(year, month, day, hour, ten_minute / 10, 4, 3, 2);

    let img03 = format_url(year, month, day, hour, ten_minute / 10, 4, 0, 3);
    let img13 = format_url(year, month, day, hour, ten_minute / 10, 4, 1, 3);
    let img23 = format_url(year, month, day, hour, ten_minute / 10, 4, 2, 3);
    let img33 = format_url(year, month, day, hour, ten_minute / 10, 4, 3, 3);

    callback(1, 16);
    let (_, buf00) = download_image(&img00)?;
    callback(2, 16);
    let (_, buf10) = download_image(&img10)?;
    callback(3, 16);
    let (_, buf20) = download_image(&img20)?;
    callback(4, 16);
    let (_, buf30) = download_image(&img30)?;

    callback(5, 16);
    let (_, buf01) = download_image(&img01)?;
    callback(6, 16);
    let (_, buf11) = download_image(&img11)?;
    callback(7, 16);
    let (_, buf21) = download_image(&img21)?;
    callback(8, 16);
    let (_, buf31) = download_image(&img31)?;

    callback(9, 16);
    let (_, buf02) = download_image(&img02)?;
    callback(10, 16);
    let (_, buf12) = download_image(&img12)?;
    callback(11, 16);
    let (_, buf22) = download_image(&img22)?;
    callback(12, 16);
    let (_, buf32) = download_image(&img32)?;

    callback(13, 16);
    let (_, buf03) = download_image(&img03)?;
    callback(14, 16);
    let (_, buf13) = download_image(&img13)?;
    callback(15, 16);
    let (_, buf23) = download_image(&img23)?;
    callback(16, 16);
    let (_, buf33) = download_image(&img33)?;

    println!("{:?} 合成4x4图片..", Local::now());
    let mut data = vec![0u8; 2200 * 2200 * 3];
    fill_block(2200, &mut data, &buf00, 0, 0);
    fill_block(2200, &mut data, &buf10, 1, 0);
    fill_block(2200, &mut data, &buf20, 2, 0);
    fill_block(2200, &mut data, &buf30, 3, 0);

    fill_block(2200, &mut data, &buf01, 0, 1);
    fill_block(2200, &mut data, &buf11, 1, 1);
    fill_block(2200, &mut data, &buf21, 2, 1);
    fill_block(2200, &mut data, &buf31, 3, 1);

    fill_block(2200, &mut data, &buf02, 0, 2);
    fill_block(2200, &mut data, &buf12, 1, 2);
    fill_block(2200, &mut data, &buf22, 2, 2);
    fill_block(2200, &mut data, &buf32, 3, 2);

    fill_block(2200, &mut data, &buf03, 0, 3);
    fill_block(2200, &mut data, &buf13, 1, 3);
    fill_block(2200, &mut data, &buf23, 2, 3);
    fill_block(2200, &mut data, &buf33, 3, 3);
    if let Some(buffer) = ImageBuffer::from_raw(2200, 2200, data) {
        Ok(buffer)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "图片解析失败",
        )))
    }
}

//合成2x2的图
pub fn combine_2x2<C>(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    ten_minute: u32,
    callback: C,
) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, Box<std::error::Error>>
where
    C: Fn(i32, i32) + 'static,
{
    println!("{:?} 下载2x2图片..", Local::now());
    let img00 = format_url(year, month, day, hour, ten_minute / 10, 2, 0, 0);
    let img10 = format_url(year, month, day, hour, ten_minute / 10, 2, 1, 0);
    let img01 = format_url(year, month, day, hour, ten_minute / 10, 2, 0, 1);
    let img11 = format_url(year, month, day, hour, ten_minute / 10, 2, 1, 1);
    callback(1, 4);
    let (_, buf00) = download_image(&img00)?;
    callback(2, 4);
    let (_, buf10) = download_image(&img10)?;
    callback(3, 4);
    let (_, buf01) = download_image(&img01)?;
    callback(4, 4);
    let (_, buf11) = download_image(&img11)?;

    println!("{:?} 合成2x2图片..", Local::now());
    let mut data = vec![0u8; 1100 * 1100 * 3];
    fill_block(1100, &mut data, &buf00, 0, 0);
    fill_block(1100, &mut data, &buf10, 1, 0);
    fill_block(1100, &mut data, &buf01, 0, 1);
    fill_block(1100, &mut data, &buf11, 1, 1);

    if let Some(buffer) = ImageBuffer::from_raw(1100, 1100, data) {
        Ok(buffer)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "图片解析失败",
        )))
    }
}

//取一张图片
fn download_image(url: &str) -> Result<(OutputInfo, Vec<u8>), Box<std::error::Error>> {
    let (info, buf) = {
        // println!("开始下载:{}", url);
        let decoder = png::Decoder::new(reqwest::get(url)?);
        let (mut info, mut reader) = decoder.read_info()?;
        // println!("下载完成:{} {}x{}", url, info.width, info.height);
        let mut buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut buf)?;
        //如果是灰度图，转换成rgb
        if info.color_type == ColorType::Grayscale {
            let mut newbuf = vec![];
            for gray in buf {
                newbuf.push(gray);
                newbuf.push(gray);
                newbuf.push(gray);
            }
            buf = newbuf;
            info.color_type = ColorType::RGB;
        }
        (info, buf)
    };

    Ok((info, buf))
}

fn format_url(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    ten_minute: u32,
    d: u32,
    x: u32,
    y: u32,
) -> String {
    format!("http://himawari8-dl.nict.go.jp/himawari8/img/D531106/{}d/550/{}/{:02}/{:02}/{:02}{}000_{}_{}.png", d, year, month, day, hour, ten_minute/10, x, y)
}

//在大图中填充一个550x550的图块
fn fill_block(target_width: usize, target: &mut Vec<u8>, src: &Vec<u8>, x: usize, y: usize) {
    // println!("组合:{}x{} src:{}", x, y, src.len());
    for (row, buf) in src.chunks(550 * 3).enumerate() {
        let i = target_width * 3 * (row + 550 * y) + 550 * 3 * x;
        if let Some(t) = target.get_mut(i..i + 550 * 3) {
            if t.len() == buf.len() {
                t.copy_from_slice(buf);
            }
        }
    }
}
