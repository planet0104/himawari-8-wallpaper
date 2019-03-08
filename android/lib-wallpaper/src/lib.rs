#[macro_use]
extern crate lazy_static;
#[macro_use] extern crate log;
extern crate android_logger;

use jni::{JavaVM, JNIEnv};
use jni::objects::{JObject, JClass, JValue};
use jni::sys::{jboolean, jint};
use std::sync::Mutex;
use std::io::BufWriter;
use png::HasParameters;

mod himawari8;
mod wallpaper;

lazy_static! {
    static ref JVM: Mutex<Option<JavaVM>> = Mutex::new(None);
}

//读取完整的图片文件
pub fn open_image(file_name: &str) -> Option<image::ImageBuffer<image::Rgb<u8>, Vec<u8>>>{
    info!("打开图片文件:{}", file_name);
	if let Ok(image) = || -> Result<image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, Box<std::error::Error>>{
		let jvm = JVM.lock()?;
		let env = jvm.as_ref().unwrap().attach_current_thread()?;
		let file_name = env.new_string(file_name)?;
		info!("调用openFile");
		let result = env.call_static_method("io/github/planet0104/h8w/MainActivity", "openFile", "(Ljava/lang/String;)[B", &[JValue::from(JObject::from(file_name))])?;
		info!("调用openFile result={:?}", result);
		let bytes = env.convert_byte_array(result.l()?.into_inner())?;
		Ok(image::load_from_memory(&bytes)?.to_rgb())
	}(){
		Some(image)
	}else{
		None
	}
}

//保存完整的图片文件
pub fn save_image(_utc:chrono::DateTime<chrono::Utc>, file_name: &str, image:&image::ImageBuffer<image::Rgb<u8>, Vec<u8>>){
    info!("save_image>>保存图片文件:{} {}x{}", file_name, image.width(), image.height());

	let _ = || -> Result<(), Box<std::error::Error>>{
		//生成png
		let mut buf = vec![];
		{
			let ref mut w = BufWriter::new(&mut buf);
			let mut encoder = png::Encoder::new(w, image.width(), image.height());
			encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
			let mut writer = encoder.write_header()?;
			writer.write_image_data(&image)?;
		}
		info!("save_image>>png壁纸, len={}", buf.len());

		let jvm = JVM.lock()?;
		let env = jvm.as_ref().unwrap().attach_current_thread()?;
		let file_name = env.new_string(file_name)?;
		let bytes = env.byte_array_from_slice(&buf)?;
		info!("调用saveFile");
		let result = env.call_static_method("io/github/planet0104/h8w/MainActivity", "saveFile", "(Ljava/lang/String;[B)Ljava/lang/String;", &[JValue::from(JObject::from(file_name)), JValue::from(JObject::from(bytes))])?;
		info!("调用saveFile result={:?}", result);
		Ok(())
	}();
}

//设置壁纸
pub fn set_wallpaper(
    wallpaper: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
) -> Result<(), Box<std::error::Error>> {
	info!("设置壁纸 大小{}x{}", wallpaper.width(), wallpaper.height());

	//生成png
	let mut buf = vec![];
	{
		let ref mut w = BufWriter::new(&mut buf);
		let mut encoder = png::Encoder::new(w, wallpaper.width(), wallpaper.height());
		encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
		let mut writer = encoder.write_header()?;
		writer.write_image_data(&wallpaper)?;
	}
	info!("png壁纸, len={}", buf.len());

	let jvm = JVM.lock()?;
	let env = jvm.as_ref().unwrap().attach_current_thread()?;
	let bytes = env.byte_array_from_slice(buf.as_slice())?;
	info!("png壁纸, bytes已转换");
	let result = env.call_static_method("io/github/planet0104/h8w/MainActivity", "setWallpaper", "([B)Ljava/lang/String;", &[JValue::from(JObject::from(bytes))])?;
	info!("png壁纸, setWallpaper已调用 result={:?}", result);
	Ok(())
}

//JNI加载完成
#[no_mangle]
pub extern fn JNI_OnLoad(jvm: JavaVM, _reserved: *mut std::ffi::c_void) -> jint{
	android_logger::init_once(android_logger::Filter::default().with_min_level(log::Level::Info), Some("lib_wallpaper"));
	info!("JNI_OnLoad.");
	*JVM.lock().unwrap() = Some(jvm);
	jni::sys::JNI_VERSION_1_6
}

#[no_mangle]
pub extern fn Java_io_github_planet0104_h8w_MainActivity_downloadAndSetWallpaper<'a>(env: JNIEnv, _activity: JClass, wallpaper_type:jint) -> jboolean{
	info!("downloadAndSetWallpaper...");
	//获取屏幕宽、高
	let (width, height) = || -> Result<(i32, i32), Box<std::error::Error>>{
		Ok((env.call_static_method("io/github/planet0104/h8w/MainActivity", "getScreenWidth", "()I", &[])?.i()?,
		env.call_static_method("io/github/planet0104/h8w/MainActivity", "getScreenHeight", "()I", &[])?.i()?))
	}().unwrap_or((720, 1280));
	info!("壁纸大小:{}x{}", width, height);

	info!("wallpaper_type:{:?}", wallpaper_type);

	if wallpaper_type==0{
		if let Err(err) = wallpaper::set_full(
			width,
			height,
			|current: i32, total: i32|{
				info!("下载壁纸{}/{}", current, total);
				info!("调用openFile");
				if let Ok(jvm) = JVM.lock(){
					if let Ok(env) = jvm.as_ref().unwrap().attach_current_thread(){
						if let Err(err) = env.call_static_method("io/github/planet0104/h8w/MainActivity", "notifyDownloadProgress", "(II)V", &[JValue::from(current), JValue::from(total)]){
							error!("下载进度通知失败:{:?}", err);
						}
					}
				}
			},
		){
			info!("壁纸下载失败:{:?}", err);
			return 0;
		}
	}else{
		if let Err(err) = wallpaper::set_half(
			width,
			height,
			|current: i32, total: i32|{
				info!("下载壁纸{}/{}", current, total);
			},
		){
			info!("壁纸下载失败:{:?}", err);
			return 0;
		}
	}
	1
}