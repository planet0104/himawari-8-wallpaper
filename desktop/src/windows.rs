use crate::wallpaper;
use std::cell::RefCell;
use std::mem;
use std::ptr::null_mut;
use winapi::shared::minwindef::{DWORD, HINSTANCE, LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::ntdef::LPSTR;
use winapi::shared::windef::{HBRUSH, HICON, HMENU, HWND, POINT};
use winapi::um::shellapi::{
    Shell_NotifyIconW, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_DELETE, NIM_MODIFY,
    NOTIFYICONDATAW,
};
use winapi::um::wingdi::{GetStockObject, WHITE_BRUSH};
use winapi::um::winuser::*;
use std::sync::Mutex;
use winapi::shared::basetsd::UINT_PTR;

//https://blog.csdn.net/end_ing/article/details/19168679

static APP_NAME: &str = "himawari8壁纸";

const IDR_EXIT: usize = 10;
const IDR_FULL: usize = 11;
const IDR_HALF: usize = 12;
const MSG_ERROR: u32 = WM_USER + 100;
const MSG_OK: u32 = WM_USER + 101;
const MSG_PROGRESS: u32 = WM_USER + 102;

const TYPE_FULL:i32 = 0;//整幅图
const TYPE_HALF:i32 = 1;//半副图

lazy_static!{
    static ref SCREEN_WIDTH:i32 = unsafe{ GetSystemMetrics(SM_CXSCREEN) as i32 };
    static ref SCREEN_HEIGHT:i32 = unsafe{ GetSystemMetrics(SM_CYSCREEN) as i32 };
    static ref WM_TASKBAR_CREATED:UINT = unsafe{ RegisterWindowMessageW(str_to_ws("TaskbarCreated").as_ptr()) };
    static ref H_MENU:Mutex<isize> = Mutex::new(0);
    static ref WALLPAPER_TYPE:Mutex<i32> = Mutex::new(TYPE_FULL);
}

thread_local! {
    static NID:RefCell<NOTIFYICONDATAW> = RefCell::new(unsafe{std::mem::zeroed()});
}

//切换到整福图
fn switch_to_full() {
    let tid = thread_id::get();
    std::thread::spawn(move || {
        if wallpaper::set_full(*SCREEN_WIDTH, *SCREEN_HEIGHT, move |current:i32, total:i32|{
            unsafe{ PostThreadMessageW(tid as u32, MSG_PROGRESS, current as usize, total as isize); }
        }).is_err() {
            unsafe {
                PostThreadMessageW(tid as u32, MSG_ERROR, 0, 0);
            }
        } else {
            unsafe {
                PostThreadMessageW(tid as u32, MSG_OK, 0, 0);
            }
        }
    });
}

//切换到半幅图
fn switch_to_half() {
    let tid = thread_id::get();
    std::thread::spawn(move || {
        if wallpaper::set_half(*SCREEN_WIDTH, *SCREEN_HEIGHT, move |current:i32, total:i32|{
            unsafe{ PostThreadMessageW(tid as u32, MSG_PROGRESS, current as usize, total as isize); }
        }).is_err() {
            unsafe {
                PostThreadMessageW(tid as u32, MSG_ERROR, 0, 0);
            }
        } else {
            unsafe {
                PostThreadMessageW(tid as u32, MSG_OK, 0, 0);
            }
        }
    });
}

//窗口消息函数
pub unsafe extern "system" fn window_proc(
    h_wnd: HWND,
    u_msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    let app_name = str_to_ws(APP_NAME);
    match u_msg {
        WM_CREATE => {
            NID.with(|nid| {
                let mut nid = nid.borrow_mut();
                nid.cbSize = mem::size_of::<NOTIFYICONDATAW>() as u32;;
                nid.hWnd = h_wnd;
                nid.uID = 0;
                nid.uFlags = NIF_ICON | NIF_MESSAGE | NIF_TIP;
                nid.uCallbackMessage = WM_USER;
                nid.hIcon = LoadImageW(
                    0 as HINSTANCE,
                    str_to_ws("icon.ico").as_ptr(),
                    IMAGE_ICON,
                    0,
                    0,
                    LR_LOADFROMFILE,
                ) as HICON; //图标
                nid.szTip
                    .get_mut(0..app_name.len())
                    .unwrap()
                    .copy_from_slice(&app_name);
                Shell_NotifyIconW(NIM_ADD, &mut *nid);
            });

            let h_menu = CreatePopupMenu();
            AppendMenuW(h_menu, MF_STRING, IDR_FULL, str_to_ws("整幅图").as_ptr());
            AppendMenuW(h_menu, MF_STRING, IDR_HALF, str_to_ws("半幅图").as_ptr());
            AppendMenuW(h_menu, MF_STRING, IDR_EXIT, str_to_ws("退出").as_ptr());
            *H_MENU.lock().unwrap() = h_menu as isize;

            //启动时第一次下载
            switch_to_full();
            //启动定时器
            unsafe extern "system" fn task(_: HWND, _: UINT, _: UINT_PTR, _: DWORD){
                match *WALLPAPER_TYPE.lock().unwrap(){
                    TYPE_HALF => switch_to_half(),
                    TYPE_FULL => switch_to_full(),
                    _ => ()
                };
            }
            SetTimer(h_wnd, 1, 5000, Some(task));
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
                    match TrackPopupMenu(
                        *H_MENU.lock().unwrap() as HMENU,
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
                        IDR_FULL => {
                            let mut ptype = WALLPAPER_TYPE.lock().unwrap();
                            if *ptype != TYPE_FULL{
                                *ptype = TYPE_FULL;
                                switch_to_full();
                            }
                        }
                        IDR_HALF => {
                            let mut ptype = WALLPAPER_TYPE.lock().unwrap();
                            if *ptype != TYPE_HALF{
                                *ptype = TYPE_HALF;
                                switch_to_half();
                            }
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
            NID.with(|nid| {
                let mut nid = nid.borrow_mut();
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

#[allow(non_snake_case)]
pub fn win_main(
    hInstance: HINSTANCE,
    _hPrevInstance: HINSTANCE,
    _szCmdLine: LPSTR,
    iCmdShow: i32,
) -> i32 {
    let app_name = str_to_ws(APP_NAME);

    let handle = unsafe { FindWindowW(null_mut(), app_name.as_ptr()) };
    if !handle.is_null() {
        unsafe {
            MessageBoxW(
                null_mut(),
                str_to_ws("程序已经运行").as_ptr(),
                app_name.as_ptr(),
                MB_OK,
            );
        }
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
    wndclass.lpszClassName = app_name.as_ptr();

    if unsafe { RegisterClassW(&wndclass) == 0 } {
        unsafe {
            MessageBoxW(
                null_mut(),
                str_to_ws("程序需要在Windows NT运行！").as_ptr(),
                app_name.as_ptr(),
                MB_ICONERROR,
            );
        }
        return 0;
    }

    // 此处使用WS_EX_TOOLWINDOW 属性来隐藏显示在任务栏上的窗口程序按钮
    let hwnd = unsafe {
        CreateWindowExW(
            WS_EX_TOOLWINDOW,
            app_name.as_ptr(),
            app_name.as_ptr(),
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
                    show_tip(&format!("图片下载出错，右键击菜单重试 {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()));
                },
                MSG_PROGRESS => {
                    show_tip(&format!("正在下载图片({}/{})", msg.wParam, msg.lParam));
                }
                MSG_OK => {
                    show_tip(&format!("壁纸下载完成 {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()));
                }
                _ => {}
            }
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
    msg.wParam as i32
}

fn show_tip(tip:&str){
    NID.with(|nid| {
        let mut nid = nid.borrow_mut();

        // _tcscpy(m_nid.szInfoTitle,"提醒你");
        // _tcscpy(m_nid.szInfo,"内容改变");
        // m_nid.uTimeout=1000;
        // m_nid.uVersion=NOTIFYICON_VERSION;
        // Shell_NotifyIcon(NIM_MODIFY,&m_nid);

        let tip = str_to_ws(tip);
        nid.szTip
            .get_mut(0..tip.len())
            .unwrap()
            .copy_from_slice(&tip);
        unsafe{ Shell_NotifyIconW(NIM_MODIFY, &mut *nid); }
    });
}

/** 字符串转换成双字 0结尾的数组 */
pub fn str_to_ws(s: &str) -> Vec<u16> {
    let mut v: Vec<u16> = s.encode_utf16().collect();
    v.push(0);
    v
}
