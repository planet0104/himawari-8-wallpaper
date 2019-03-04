#[macro_use]
extern crate lazy_static;
#[macro_use] extern crate log;
extern crate android_logger;
use log::Level;
const LEVEL:Level = Level::Debug;

use jni::{JNIEnv};
use jni::objects::{JObject, JString, JClass, JValue};
use jni::sys::{jint, jbyteArray};

mod himawari8;
mod wallpaper;

pub fn set_wallpaper_from_path(path:&str){
	info!("调用设置壁纸! {}", path);
}

//JNI加载完成
#[no_mangle]
pub extern fn JNI_OnLoad(_vm: jni::JavaVM, _reserved: *mut std::ffi::c_void) -> jint{
	android_logger::init_once(android_logger::Filter::default().with_min_level(LEVEL), Some("lib_wallpaper"));
	info!("JNI_OnLoad.");

	jni::sys::JNI_VERSION_1_6
}

#[no_mangle]
pub extern fn Java_io_github_planet0104_h8w_MainActivity_init<'a>(env: JNIEnv, _activity: JClass, activity:JObject){
	info!("init..");
	info!("start download..");
	if let Err(err) = wallpaper::set_full(
		480,
		800,
		|current: i32, total: i32|{
			info!("下载壁纸{}/{}", current, total);
		},
	){
		info!("壁纸下载失败:{:?}", err);
	}
	// if result.is_err(){
	// 	let err = result.err();
	// 	error!("{:?}", &err);
	// 	let _ = env.throw_new("java/lang/Exception", format!("{:?}", err));
	// }
}