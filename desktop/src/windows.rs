use crate::wallpaper;
use crate::Config;
use crate::TYPE_FULL;
use crate::TYPE_HALF;
use std::cell::RefCell;
use std::mem;
use std::path::Path;
use std::ptr::null_mut;
use std::sync::Mutex;
use winapi::shared::basetsd::UINT_PTR;
use winapi::shared::minwindef::{MAX_PATH, DWORD, HINSTANCE, LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::ntdef::LPSTR;
use winapi::shared::windef::{HBRUSH, HICON, HMENU, HWND, POINT};
use winapi::um::shlobj::SHGetSpecialFolderPathW;
use winapi::um::shlobj::CSIDL_STARTUP;
use winapi::um::shellapi::{
    ShellExecuteW, Shell_NotifyIconW, NIF_ICON, NIF_INFO, NIF_MESSAGE, NIF_TIP, NIIF_NONE, NIM_ADD,
    NIM_DELETE, NIM_MODIFY, NOTIFYICONDATAW,
};
use winapi::um::wingdi::{GetStockObject, WHITE_BRUSH};
use winapi::um::winnt::LPCWSTR;
use winapi::um::winuser::*;

//https://blog.csdn.net/end_ing/article/details/19168679

pub static APP_NAME: &str = "himawari8壁纸";

static TEMPLATE:&str = r"[InternetShortcut]
URL=--
IconIndex=0
IconFile=--
";

const IDR_EXIT: usize = 10;
const IDR_HOME: usize = 20;
const IDR_DOWNLOAD: usize = 40;
const IDR_STARTUP: usize = 30;
const IDR_TP_FULL: usize = 210;
const IDR_TP_HALF: usize = 211;
const IDR_FQ_10: usize = 110;
const IDR_FQ_20: usize = 111;
const IDR_FQ_30: usize = 112;
const IDR_FQ_60: usize = 113;

const MSG_ERROR: u32 = WM_USER + 100;
const MSG_OK: u32 = WM_USER + 101;
const MSG_PROGRESS: u32 = WM_USER + 102;

lazy_static! {
    static ref SCREEN_WIDTH: i32 = unsafe { GetSystemMetrics(SM_CXSCREEN) as i32 };
    static ref SCREEN_HEIGHT: i32 = unsafe { GetSystemMetrics(SM_CYSCREEN) as i32 };
    static ref WM_TASKBAR_CREATED: UINT =
        unsafe { RegisterWindowMessageW(convert("TaskbarCreated")) };
    static ref H_MENU: Mutex<isize> = Mutex::new(0);
    static ref TY_MENU: Mutex<isize> = Mutex::new(0);
    static ref TIMER_ID: Mutex<usize> = Mutex::new(0);
    static ref CONFIG: Mutex<Config> = Mutex::new(Config::default());
}

thread_local! {
    static NID:RefCell<NOTIFYICONDATAW> = RefCell::new(unsafe{std::mem::zeroed()});
}

//切换到整福图
fn switch_to_full() {
    let tid = thread_id::get();
    std::thread::spawn(move || {
        match wallpaper::download_full(*SCREEN_WIDTH, *SCREEN_HEIGHT, move |current: i32, total: i32| unsafe {
                PostThreadMessageW(tid as u32, MSG_PROGRESS, current as usize, total as isize);
        }){
            Err(_err) => {
                unsafe { PostThreadMessageW(tid as u32, MSG_ERROR, 0, 0); }
            }
            Ok(wallpaper) => {
                if crate::set_wallpaper(wallpaper).is_err(){
                    unsafe { PostThreadMessageW(tid as u32, MSG_ERROR, 0, 0); }
                }else{
                    unsafe { PostThreadMessageW(tid as u32, MSG_OK, 0, 0); }
                }
            }
        }
    });
}

//切换到半幅图
fn switch_to_half() {
    let tid = thread_id::get();
    std::thread::spawn(move || {
        match wallpaper::download_half(*SCREEN_WIDTH, *SCREEN_HEIGHT, move |current: i32, total: i32| unsafe {
                PostThreadMessageW(tid as u32, MSG_PROGRESS, current as usize, total as isize);
        }){
            Err(_err) => {
                unsafe { PostThreadMessageW(tid as u32, MSG_ERROR, 0, 0); }
            }
            Ok(wallpaper) => {
                if crate::set_wallpaper(wallpaper).is_err(){
                    unsafe { PostThreadMessageW(tid as u32, MSG_ERROR, 0, 0); }
                }else{
                    unsafe { PostThreadMessageW(tid as u32, MSG_OK, 0, 0); }
                }
            }
        }
    });
}

fn init_timer(h_wnd: HWND, min: i32) {
    //销毁时钟
    NID.with(|nid| {
        let nid = nid.borrow_mut();
        unsafe {
            KillTimer(nid.hWnd, *TIMER_ID.lock().unwrap());
        }
    });
    //启动定时器 10分钟一次, 30分钟一次, 60分钟一次
    unsafe extern "system" fn task(_: HWND, _: UINT, _: UINT_PTR, _: DWORD) {
        match CONFIG.lock().unwrap().show_type {
            TYPE_HALF => switch_to_half(),
            TYPE_FULL => switch_to_full(),
            _ => (),
        };
    }
    *TIMER_ID.lock().unwrap() = unsafe { SetTimer(h_wnd, 1, min as u32 * 60 * 1000, Some(task)) };
}

//窗口消息函数
pub unsafe extern "system" fn window_proc(
    h_wnd: HWND,
    u_msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    let app_name = convert_u16(APP_NAME);
    match u_msg {
        WM_CREATE => {
            NID.with(|nid| {
                let mut nid = nid.borrow_mut();
                nid.cbSize = mem::size_of::<NOTIFYICONDATAW>() as u32;
                nid.hWnd = h_wnd;
                nid.uID = 0;
                nid.uFlags = NIF_ICON | NIF_MESSAGE | NIF_TIP;
                nid.uCallbackMessage = WM_USER;
                nid.hIcon = if Path::new("icon.ico").exists() {
                    LoadImageW(
                        0 as HINSTANCE,
                        convert("icon.ico"),
                        IMAGE_ICON,
                        0,
                        0,
                        LR_LOADFROMFILE,
                    ) as HICON
                } else {
                    LoadIconW(0 as HINSTANCE, IDI_APPLICATION)
                }; //图标
                nid.szTip
                    .get_mut(0..app_name.len())
                    .unwrap()
                    .copy_from_slice(&app_name);
                Shell_NotifyIconW(NIM_ADD, &mut *nid);
            });

            //二级菜单
            let fq_menu = CreatePopupMenu();
            AppendMenuW(fq_menu, MF_STRING, IDR_FQ_10, convert("10分钟"));
            AppendMenuW(fq_menu, MF_STRING, IDR_FQ_20, convert("20分钟"));
            AppendMenuW(fq_menu, MF_STRING, IDR_FQ_30, convert("30分钟"));
            AppendMenuW(fq_menu, MF_STRING, IDR_FQ_60, convert("1小时"));
            let ty_menu = CreatePopupMenu();
            match CONFIG.lock().unwrap().show_type {
                TYPE_HALF => {
                    AppendMenuW(ty_menu, MF_STRING, IDR_TP_FULL, convert("整幅图"));
                    AppendMenuW(ty_menu, MF_STRING, IDR_TP_HALF, convert("半幅图√"));
                }
                TYPE_FULL => {
                    AppendMenuW(ty_menu, MF_STRING, IDR_TP_FULL, convert("整幅图√"));
                    AppendMenuW(ty_menu, MF_STRING, IDR_TP_HALF, convert("半幅图"));
                }
                _ => (),
            };
            // AppendMenuW(ty_menu, MF_STRING, IDR_TP_FULL, convert("整幅图"));
            // AppendMenuW(ty_menu, MF_STRING, IDR_TP_HALF, convert("半幅图"));

            //一级菜单
            let h_menu = CreatePopupMenu();
            AppendMenuW(h_menu, MF_POPUP, fq_menu as usize, convert("更新频率"));
            AppendMenuW(h_menu, MF_POPUP, ty_menu as usize, convert("展示方式"));
            //开机启动
            if let Ok(run_on_startup) = is_app_registered_for_startup(APP_NAME){
                if run_on_startup{
                    AppendMenuW(h_menu, MF_STRING, IDR_STARTUP, convert("开机启动(已开启)"));
                }else{
                    AppendMenuW(h_menu, MF_STRING, IDR_STARTUP, convert("开机启动(已关闭)"));
                }
            }
            AppendMenuW(h_menu, MF_STRING, IDR_DOWNLOAD, convert("下载壁纸"));
            AppendMenuW(h_menu, MF_STRING, IDR_HOME, convert("项目主页"));
            AppendMenuW(h_menu, MF_STRING, IDR_EXIT, convert("退出"));
            *H_MENU.lock().unwrap() = h_menu as isize;
            *TY_MENU.lock().unwrap() = ty_menu as isize;

            //启动时第一次下载
            //读取配置: 更新频率、展示方式
            let conf = CONFIG.lock().unwrap();
            if conf.show_type == TYPE_FULL {
                switch_to_full();
            } else {
                switch_to_half();
            }
            init_timer(h_wnd, conf.freq);

            //弹出气泡
            show_bubble("已启动");
        }
        WM_USER => {
            match l_param as u32 {
                WM_LBUTTONDBLCLK => {
                    SendMessageW(h_wnd, WM_CLOSE, w_param, l_param);
                }
                WM_RBUTTONDOWN | WM_LBUTTONDOWN => {
                    let mut pt: POINT = POINT { x: 0, y: 0 };
                    GetCursorPos(&mut pt); //取鼠标坐标
                    SetForegroundWindow(h_wnd); //解决在菜单外单击左键菜单不消失的问题
                                                // EnableMenuItem(hmenu,IDR_PAUSE,MF_GRAYED);//让菜单中的某一项变灰
                    let h_menu = *H_MENU.lock().unwrap() as HMENU;
                    let ty_menu = *TY_MENU.lock().unwrap() as HMENU;
                    match TrackPopupMenu(
                        h_menu,
                        TPM_RETURNCMD,
                        pt.x,
                        pt.y,
                        0,
                        h_wnd,
                        null_mut(),
                    ) as usize
                    {
                        //显示菜单并获取选项ID
                        IDR_EXIT => {
                            SendMessageW(h_wnd, WM_CLOSE, w_param, l_param);
                        }
                        IDR_DOWNLOAD => {
                            std::thread::spawn(move || {
                                open_downloader();
                            });
                        }
                        IDR_HOME => {
                            //打开github主页链接
                            ShellExecuteW(
                                h_wnd,
                                convert("open"),
                                convert("https://github.com/planet0104/himawari-8-wallpaper"),
                                null_mut(),
                                null_mut(),
                                SW_SHOWNORMAL,
                            );
                        }
                        IDR_STARTUP => {
                            if let Ok(run_on_startup) = is_app_registered_for_startup(APP_NAME){
                                if run_on_startup{
                                    info!("删除启动项:{:?}", remove_app_for_startup(APP_NAME));
                                    ModifyMenuW(h_menu, IDR_STARTUP as u32, MF_BYCOMMAND, IDR_STARTUP, convert("开机启动(已关闭)"));
                                }else{
                                    info!("添加启动项={:?}", register_app_for_startup(APP_NAME));
                                    ModifyMenuW(h_menu, IDR_STARTUP as u32, MF_BYCOMMAND, IDR_STARTUP, convert("开机启动(已开启)"));
                                }
                            }
                        }
                        IDR_TP_FULL => {
                            let mut conf = CONFIG.lock().unwrap();
                            if conf.show_type != TYPE_FULL {
                                conf.show_type = TYPE_FULL;
                                switch_to_full();
                                crate::write_config(&conf);
                                ModifyMenuW(ty_menu, IDR_TP_FULL as u32, MF_BYCOMMAND, IDR_TP_FULL, convert("整幅图√"));
                                ModifyMenuW(ty_menu, IDR_TP_HALF as u32, MF_BYCOMMAND, IDR_TP_HALF, convert("半幅图"));
                            }
                        }
                        IDR_TP_HALF => {
                            let mut conf = CONFIG.lock().unwrap();
                            if conf.show_type != TYPE_HALF {
                                conf.show_type = TYPE_HALF;
                                switch_to_half();
                                crate::write_config(&conf);
                                ModifyMenuW(ty_menu, IDR_TP_FULL as u32, MF_BYCOMMAND, IDR_TP_FULL, convert("整幅图"));
                                ModifyMenuW(ty_menu, IDR_TP_HALF as u32, MF_BYCOMMAND, IDR_TP_HALF, convert("半幅图√"));
                            }
                        }
                        IDR_FQ_10 => {
                            init_timer(h_wnd, 10);
                            let mut conf = CONFIG.lock().unwrap();
                            conf.freq = 10;
                            crate::write_config(&conf);
                        }
                        IDR_FQ_20 => {
                            init_timer(h_wnd, 20);
                            let mut conf = CONFIG.lock().unwrap();
                            conf.freq = 10;
                            crate::write_config(&conf);
                        }
                        IDR_FQ_30 => {
                            init_timer(h_wnd, 30);
                            let mut conf = CONFIG.lock().unwrap();
                            conf.freq = 30;
                            crate::write_config(&conf);
                        }
                        IDR_FQ_60 => {
                            init_timer(h_wnd, 60);
                            let mut conf = CONFIG.lock().unwrap();
                            conf.freq = 60;
                            crate::write_config(&conf);
                        }
                        0 => {
                            PostMessageW(h_wnd, WM_LBUTTONDOWN, 0, 0);
                        }
                        _ => {}
                    }
                }
                _ => (),
            }
        }
        WM_DESTROY => {
            info!("程序结束");
            NID.with(|nid| {
                let mut nid = nid.borrow_mut();
                //销毁时钟
                KillTimer(nid.hWnd, *TIMER_ID.lock().unwrap());
                //删除托盘
                Shell_NotifyIconW(NIM_DELETE, &mut *nid);
            });
            PostQuitMessage(0);
        }
        _ => {
            /*
             * 防止当Explorer.exe 崩溃以后，程序在系统系统托盘中的图标就消失
             *
             * 原理：Explorer.exe 重新载入后会重建系统任务栏。当系统任务栏建立的时候会向系统内所有
             * 注册接收TaskbarCreated 消息的顶级窗口发送一条消息，我们只需要捕捉这个消息，并重建系
             * 统托盘的图标即可。
             */
            if u_msg == *WM_TASKBAR_CREATED {
                SendMessageW(h_wnd, WM_CREATE, w_param, l_param);
            }
        }
    }
    DefWindowProcW(h_wnd, u_msg, w_param, l_param)
}

pub fn alert(title: &str, msg: &str) {
    unsafe {
        MessageBoxW(null_mut(), convert(msg), convert(title), MB_OK);
    }
}

#[allow(non_snake_case)]
pub fn win_main(
    hInstance: HINSTANCE,
    _hPrevInstance: HINSTANCE,
    _szCmdLine: LPSTR,
    iCmdShow: i32,
    conf: Config,
) -> i32 {
    let app_name = convert(APP_NAME);
    *CONFIG.lock().unwrap() = conf;

    let handle = unsafe { FindWindowW(null_mut(), app_name) };
    if !handle.is_null() {
        alert(APP_NAME, "程序已经运行");
        return 0;
    }

    let mut wndclass: WNDCLASSW = unsafe { std::mem::zeroed() };

    wndclass.style = CS_HREDRAW | CS_VREDRAW;
    wndclass.lpfnWndProc = Some(window_proc);
    wndclass.cbClsExtra = 0;
    wndclass.cbWndExtra = 0;
    wndclass.hInstance = hInstance;
    wndclass.hIcon = unsafe { LoadIconW(null_mut(), IDI_APPLICATION) };
    wndclass.hCursor = unsafe { LoadCursorW(null_mut(), IDC_ARROW) };
    wndclass.hbrBackground = unsafe { GetStockObject(WHITE_BRUSH as i32) as HBRUSH };
    wndclass.lpszMenuName = null_mut();
    wndclass.lpszClassName = app_name;

    if unsafe { RegisterClassW(&wndclass) == 0 } {
        alert(APP_NAME, "程序需要在Windows NT运行！");
        return 0;
    }

    // 此处使用WS_EX_TOOLWINDOW 属性来隐藏显示在任务栏上的窗口程序按钮
    let hwnd = unsafe {
        CreateWindowExW(
            WS_EX_TOOLWINDOW,
            app_name,
            app_name,
            WS_POPUP,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            null_mut(),
            null_mut(),
            hInstance,
            null_mut(),
        )
    };

    let mut msg: MSG = unsafe { std::mem::zeroed() };
    unsafe {
        ShowWindow(hwnd, iCmdShow);
        UpdateWindow(hwnd);
        while GetMessageW(&mut msg, null_mut(), 0, 0) != 0 {
            match msg.message {
                MSG_ERROR => {
                    show_bubble(&format!(
                        "图片下载出错，右键击菜单重试 {}",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
                    ));
                }
                MSG_PROGRESS => {
                    show_tip(&format!(
                        "正在下载卫星图片({}/{})",
                        msg.wParam, msg.lParam
                    ));
                }
                MSG_OK => {
                    show_tip(&format!(
                        "壁纸下载完成 {}",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
                    ));
                }
                _ => {}
            }
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
    msg.wParam as i32
}

fn show_tip(tip: &str) {
    NID.with(|nid| {
        let mut nid = nid.borrow_mut();

        // _tcscpy(m_nid.szInfoTitle,"提醒你");
        // _tcscpy(m_nid.szInfo,"内容改变");
        // m_nid.uTimeout=1000;
        // m_nid.uVersion=NOTIFYICON_VERSION;
        // Shell_NotifyIcon(NIM_MODIFY,&m_nid);

        nid.uFlags = NIF_TIP;
        let tip = convert_u16(tip);
        nid.szTip
            .get_mut(0..tip.len())
            .unwrap()
            .copy_from_slice(&tip);
        unsafe {
            Shell_NotifyIconW(NIM_MODIFY, &mut *nid);
        }
    });
}

fn show_bubble(info: &str) {
    NID.with(|nid| {
        let mut nid = nid.borrow_mut();
        let title = convert_u16(APP_NAME);
        nid.szInfoTitle
            .get_mut(0..title.len())
            .unwrap()
            .copy_from_slice(&title);
        let info = convert_u16(info);
        nid.szInfo
            .get_mut(0..info.len())
            .unwrap()
            .copy_from_slice(&info);
        nid.uFlags = NIF_INFO;
        nid.dwInfoFlags = NIIF_NONE;
        unsafe {
            Shell_NotifyIconW(NIM_MODIFY, &mut *nid);
        }
    });
}

fn remove_app_for_startup(app_name:&str) -> Result<(), Box<std::error::Error>>{
    let mut path:[u16; MAX_PATH+1] = [0; MAX_PATH+1];
    unsafe{ SHGetSpecialFolderPathW(HWND_DESKTOP, path.as_mut_ptr(), CSIDL_STARTUP, 0) };
    let path = String::from_utf16(&path)?.replace("\u{0}", "");
    std::fs::remove_file(format!("{}\\{}.url", path, app_name))?;
    Ok(())
}

fn register_app_for_startup(app_name:&str) -> Result<(), Box<std::error::Error>>{
    let mut path:[u16; MAX_PATH+1] = [0; MAX_PATH+1];
    unsafe{ SHGetSpecialFolderPathW(HWND_DESKTOP, path.as_mut_ptr(), CSIDL_STARTUP, 0) };
    let path = String::from_utf16(&path)?.replace("\u{0}", "");
    let url_file = format!("{}\\{}.url", path, app_name);
    //写入url文件
    use std::io::Write;
    let mut file = std::fs::File::create(url_file)?;
    let exe_path = ::std::env::current_exe()?;
    if let Some(exe_path) = exe_path.to_str(){
        file.write_all(TEMPLATE.replace("--", exe_path).as_bytes())?;
        Ok(())
    }else{
        use std::io::{Error, ErrorKind};
        Err(Box::new(Error::new(ErrorKind::Other, "exe路径读取失败!")))
    }
}

fn is_app_registered_for_startup(app_name:&str) -> Result<bool, Box<std::error::Error>>{
    let mut path:[u16; MAX_PATH+1] = [0; MAX_PATH+1];
    unsafe{ SHGetSpecialFolderPathW(HWND_DESKTOP, path.as_mut_ptr(), CSIDL_STARTUP, 0) };
    let path = String::from_utf16(&path)?.replace("\u{0}", "");
    Ok(Path::new(&format!("{}\\{}.url", path, app_name)).exists())
}

pub fn convert(s: &str) -> LPCWSTR {
    let mut v: Vec<u16> = s.encode_utf16().collect();
    v.push(0);
    v.as_ptr()
}

/** 字符串转换成双字 0结尾的数组 */
pub fn convert_u16(s: &str) -> Vec<u16> {
    let mut v: Vec<u16> = s.encode_utf16().collect();
    v.push(0);
    v
}

fn open_downloader(){
    crate::set_current_dir();
    use sciter::Value;
    let width = 360;
    let height = 265;
    let mut window = sciter::window::Builder::main_window()
    .with_rect((*SCREEN_WIDTH/2-width/2, *SCREEN_HEIGHT/2-height/2, width, height))
    .fixed()
    .create();

    struct EventHandler{
        hwnd:isize
    };

    impl EventHandler {
        fn download_wallpaper(&self, tp:String, width: i32, height:i32, progress: Value, done: Value){
            use std::thread;
            use winapi::um::commdlg::{GetSaveFileNameW, OFN_HIDEREADONLY, OFN_PATHMUSTEXIST, OPENFILENAMEW};
            let hwnd = self.hwnd;
            thread::spawn(move || {
                crate::set_current_dir();
                let result = if tp=="full"{
                    wallpaper::download_full(width, height, move |current: i32, total: i32| {
                        progress.call(None, &make_args!(current, total), None).unwrap();
                    })
                }else{
                    wallpaper::download_half(width, height, move |current: i32, total: i32| {
                        progress.call(None, &make_args!(current, total), None).unwrap();
                    })
                };

                match result{
                    Err(err) => {
                        if format!("{:?}", err).contains("正在下载中"){
                            done.call(None, &make_args!(4), None).unwrap();
                        }else{
                            done.call(None, &make_args!(1), None).unwrap();
                        }
                    }
                    Ok(wallpaper) => {
                        //另存为对话框
                        let mut ofn: OPENFILENAMEW = unsafe { std::mem::zeroed() };
                        let mut file_name:[u16; MAX_PATH] = [0; MAX_PATH];
                        let mut title_name:[u16; MAX_PATH] = [0; MAX_PATH];
                        ofn.lpstrFile = file_name.as_mut_ptr();//初始化文件名​​
                        ofn.nMaxFile = MAX_PATH as u32;
                        ofn.lpstrFileTitle = title_name.as_mut_ptr();
                        ofn.nMaxFileTitle = MAX_PATH as u32;
                        use std::ffi::OsStr;
                        use std::os::windows::ffi::OsStrExt;
                        let text = OsStr::new("图片文件(*.png;*.jpg;*.jpeg;*.bmp)\0*.png;*.jpg;*.jpeg;*.bmp\0全部文件(*.*)\0*.*\0\0").encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>();
                        ofn.lpstrFilter = text.as_ptr();
                        ofn.lpstrDefExt = convert("png");//默认扩展名
                        ofn.lpstrTitle = null_mut();
                        ofn.Flags = OFN_HIDEREADONLY | OFN_PATHMUSTEXIST;
                        ofn.lStructSize = mem::size_of::<OPENFILENAMEW>() as u32;
                        ofn.hwndOwner = hwnd as HWND;
                        if unsafe{ GetSaveFileNameW(&mut ofn) != 0 }{
                            let path = String::from_utf16(&file_name).unwrap().replace("\u{0}", "");
                            // info!("文件名:{}", path);
                            // let title = String::from_utf16(&title_name).unwrap().replace("\u{0}", "");
                            // info!("标题:{}", title);
                            
                            //保存文件
                            let result = wallpaper.save(path);
                            if result.is_err(){
                                error!("壁纸图片保存失败:{:?}", result.err());
                                done.call(None, &make_args!(2), None).unwrap();
                            }else{
                                done.call(None, &make_args!(0), None).unwrap();
                            }
                        }else{
                            done.call(None, &make_args!(3), None).unwrap();
                        }
                    }
                }
            });
        }
    }

    impl EventHandler{
        fn new(hwnd: isize) -> EventHandler{
            EventHandler{hwnd}
        }
    }

    impl sciter::EventHandler for EventHandler {
        dispatch_script_call! {
            fn download_wallpaper(String, i32, i32, Value, Value);
        }
    }

    window.event_handler(EventHandler::new(window.get_hwnd() as isize));

    let html = include_bytes!("../main.html");
    window.load_html(html, Some("\\main.html"));
    // let path = std::env::current_dir().unwrap();
    // window.load_file(&format!("{}\\main.html", path.display()));
    window.run_app();
}