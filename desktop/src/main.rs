//http://himawari8.nict.go.jp/himawari8-image.htm

use png::{OutputInfo, HasParameters, ColorType};
use image::{ImageBuffer, Rgb};

//https://stackoverflow.com/questions/42322024/repeating-a-rust-task-with-tokio-timer

//取一张图片
fn download_image(url: &str) -> Result<(OutputInfo, Vec<u8>), Box<std::error::Error>>{
    let resp = reqwest::get(url)?;
    let decoder = png::Decoder::new(resp);
    let (mut info, mut reader) = decoder.read_info().unwrap();
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    //如果是灰度图，转换成rgb
    if info.color_type == ColorType::Grayscale{
        let mut newbuf = vec![];
        for gray in buf{
            newbuf.push(gray);
            newbuf.push(gray);
            newbuf.push(gray);
        }
        buf = newbuf;
        info.color_type = ColorType::RGB;
    }

    Ok((info, buf))
}

/*
合成4x4的图，小图顺序为

00,10,20,30
01,11,21,31
02,12,22,32
03,13,23,33

*/
fn combine_4x4(year:i32, month:u32, day:u32, hour:u32, ten_minute:u32) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, Box<std::error::Error>>{
    println!("合成4x4图片...");
    let img00 = format_url(year, month, day, hour, ten_minute/10, 4, 0, 0);
    let img10 = format_url(year, month, day, hour, ten_minute/10, 4, 1, 0);
    let img20 = format_url(year, month, day, hour, ten_minute/10, 4, 2, 0);
    let img30 = format_url(year, month, day, hour, ten_minute/10, 4, 3, 0);

    let img01 = format_url(year, month, day, hour, ten_minute/10, 4, 0, 1);
    let img11 = format_url(year, month, day, hour, ten_minute/10, 4, 1, 1);
    let img21 = format_url(year, month, day, hour, ten_minute/10, 4, 2, 1);
    let img31 = format_url(year, month, day, hour, ten_minute/10, 4, 3, 1);

    let img02 = format_url(year, month, day, hour, ten_minute/10, 4, 0, 2);
    let img12 = format_url(year, month, day, hour, ten_minute/10, 4, 1, 2);
    let img22 = format_url(year, month, day, hour, ten_minute/10, 4, 2, 2);
    let img32 = format_url(year, month, day, hour, ten_minute/10, 4, 3, 2);

    let img03 = format_url(year, month, day, hour, ten_minute/10, 4, 0, 3);
    let img13 = format_url(year, month, day, hour, ten_minute/10, 4, 1, 3);
    let img23 = format_url(year, month, day, hour, ten_minute/10, 4, 2, 3);
    let img33 = format_url(year, month, day, hour, ten_minute/10, 4, 3, 3);

    println!("下载:{}", img00);
    let (_, buf00) = download_image(&img00)?;
    println!("下载:{}", img10);
    let (_, buf10) = download_image(&img10)?;
    println!("下载:{}", img20);
    let (_, buf20) = download_image(&img20)?;
    // println!("info20:{}x{} {:?}", info20.width, info20.height, info20.color_type);
    println!("下载:{}", img30);
    let (_, buf30) = download_image(&img30)?;
    // println!("info30:{}x{} {:?}", info30.width, info30.height, info30.color_type);

    println!("下载:{}", img01);
    let (_, buf01) = download_image(&img01)?;
    println!("下载:{}", img11);
    let (_, buf11) = download_image(&img11)?;
    println!("下载:{}", img21);
    let (_, buf21) = download_image(&img21)?;
    println!("下载:{}", img31);
    let (_, buf31) = download_image(&img33)?;

    println!("下载:{}", img02);
    let (_, buf02) = download_image(&img02)?;
    println!("下载:{}", img12);
    let (_, buf12) = download_image(&img12)?;
    println!("下载:{}", img22);
    let (_, buf22) = download_image(&img22)?;
    println!("下载:{}", img32);
    let (_, buf32) = download_image(&img32)?;

    println!("下载:{}", img03);
    let (_, buf03) = download_image(&img03)?;
    println!("下载:{}", img13);
    let (_, buf13) = download_image(&img13)?;
    println!("下载:{}", img23);
    let (_, buf23) = download_image(&img23)?;
    println!("下载:{}", img33);
    let (_, buf33) = download_image(&img33)?;

    //创建大图
    let mut data = vec![0u8; 2200*2200*3];
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

    let buffer = ImageBuffer::from_raw(2200, 2200, data).unwrap();

    Ok(buffer)
}

//在大图中填充一个550x550的图块
fn fill_block(target_width: usize, target:&mut Vec<u8>, src:&Vec<u8>, x:usize, y:usize){
    println!("组合:{}x{} src:{}", x, y, src.len());
    for (row, buf) in src.chunks(550*3).enumerate(){
        let i = target_width*3*(row+550*y)+550*3*x;
        target.get_mut(i..i+550*3).unwrap().copy_from_slice(buf);
    }
}

fn format_url(year:i32, month:u32, day:u32, hour:u32, ten_minute:u32, d:u32, x:u32, y:u32) -> String{
    format!("http://himawari8-dl.nict.go.jp/himawari8/img/D531106/{}d/550/{}/{:02}/{:02}/{:02}{}000_{}_{}.png", d, year, month, day, hour, ten_minute/10, x, y)
}

//合成2x2的图
fn combine_2x2(year:i32, month:u32, day:u32, hour:u32, ten_minute:u32) -> Result<(OutputInfo, Vec<u8>), Box<std::error::Error>>{
    println!("合成2x2图片...");
    let img00 = format_url(year, month, day, hour, ten_minute/10, 2, 0, 0);
    let img10 = format_url(year, month, day, hour, ten_minute/10, 2, 1, 0);
    let img01 = format_url(year, month, day, hour, ten_minute/10, 2, 0, 1);
    let img11 = format_url(year, month, day, hour, ten_minute/10, 2, 1, 1);
    println!("下载:{}", img00);
    let (info, buf00) = download_image(&img00)?;
    println!("{}x{},颜色:{:?},位深:{:?}\n下载:{}", info.width, info.height, info.color_type, info.bit_depth, img10);
    let (_, buf10) = download_image(&img10)?;
    println!("{}x{},颜色:{:?},位深:{:?}\n下载:{}", info.width, info.height, info.color_type, info.bit_depth, img01);
    let (_, buf01) = download_image(&img01)?;
    println!("{}x{},颜色:{:?},位深:{:?}\n下载:{}", info.width, info.height, info.color_type, info.bit_depth, img11);
    let (_, buf11) = download_image(&img11)?;
    println!("{}x{},颜色:{:?},位深:{:?}", info.width, info.height, info.color_type, info.bit_depth);

    //创建大图
    let mut image = vec![];
    {
        let mut encoder = png::Encoder::new(&mut image, 1100, 1100);
        encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        let mut data = vec![0u8; 1100*1100*3];
        fill_block(1100, &mut data, &buf00, 0, 0);
        fill_block(1100, &mut data, &buf10, 1, 0);
        fill_block(1100, &mut data, &buf01, 0, 1);
        fill_block(1100, &mut data, &buf11, 1, 1);
        writer.write_image_data(&data).unwrap(); // Save
    }

    Ok((OutputInfo{
        width: 1100,
        height: 1100,
        color_type: png::ColorType::RGB,
        bit_depth: png::BitDepth::Eight,
        line_size: 1100*3
    }, image))
}

use chrono::{Datelike, NaiveDateTime, Timelike, DateTime, Utc};
use image::GenericImageView;

fn get_wallpaper_info() -> Result<(u32, u32), Box<std::error::Error>>{
    let path = wallpaper::get()?;
    let image = image::open(path)?;
    Ok(image.dimensions())
}

fn main() {
    println!("开始下载.");
    let mut timestamp = Utc::now().timestamp_millis();
    //减去15分钟
    timestamp -= 16*60*1000;
    let utc = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp/1000, 0), Utc);
    println!("时间:{:?}", utc);
    //20分钟之前的
    let mut image = combine_4x4(utc.year(), utc.month(), utc.day(), utc.hour(), utc.minute()).unwrap();
    println!("拼接完成:{:?}", image.dimensions());
    
    let mut width = 1920;
    let mut height = 1200;
    //按照桌面最窄缩放
    if let Ok((w, h)) = get_wallpaper_info(){
        width = w;
        height = h;
    }
    let min = (std::cmp::min(width, height) as f64*0.9) as u32;
    image = image::imageops::resize(&image, min, min, image::FilterType::Nearest);

    let mut wallpaper:ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    
    //拼接
    let offset_x = (wallpaper.width()-image.width())/2;
    let ew = image.width() as usize;
    let image = image.into_raw();
    for (y, buf) in image.chunks(ew*3).enumerate(){
        let offset = width as usize*3*y+offset_x as usize*3;
        wallpaper.get_mut(offset..offset+buf.len()).unwrap().copy_from_slice(buf);
    }

    wallpaper.save("wallpaper.png").unwrap();
    wallpaper::set_from_path("C:\\Users\\HaiJuan\\Documents\\GitHub\\himawari-8-wallpaper\\desktop\\wallpaper.png").unwrap();
}