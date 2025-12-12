use once_cell::sync::Lazy;
use std::{ptr::null_mut, sync::atomic::{AtomicBool, Ordering}};

use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, LRESULT, HINSTANCE};
use winapi::shared::windef::{HWND, HHOOK, HICON, POINT};
use winapi::um::winuser::*;
use winapi::um::shellapi::*;
use winapi::um::libloaderapi::GetModuleHandleW;

// === GLOBAL STATE ===
static IS_LOCKED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
static mut HOOK: HHOOK = null_mut();

const WM_TRAYICON: UINT = WM_APP + 1;
const VK_F12: u32 = 0x7B;

// === KEYBOARD HOOK CALLBACK ===
extern "system" fn keyboard_proc(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if code == HC_ACTION {
        unsafe {
            let kb = &*(l_param as *const KBDLLHOOKSTRUCT);
            let vk = kb.vkCode;

            let is_key_up = (kb.flags & LLKHF_UP) != 0;

            // Toggle only on KEY DOWN, not KEY UP
            if vk == VK_F12 && !is_key_up {
                let new_state = !IS_LOCKED.load(Ordering::SeqCst);
                IS_LOCKED.store(new_state, Ordering::SeqCst);
                println!("Keyless state: {}", if new_state { "LOCKED" } else { "UNLOCKED" });

                return CallNextHookEx(HOOK, code, w_param, l_param);
            }

            if IS_LOCKED.load(Ordering::SeqCst) {
                return 1; // block
            }
        }
    }

    unsafe { CallNextHookEx(HOOK, code, w_param, l_param) }
}


// === WINDOW CALLBACK FOR TRAY ===
extern "system" fn window_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        if msg == WM_TRAYICON {
            if lparam as UINT == WM_RBUTTONUP {
                show_tray_menu(hwnd);
            }
        }
        DefWindowProcW(hwnd, msg, wparam, lparam)
    }
}

// === ADD TRAY ICON ===
unsafe fn add_tray_icon(hwnd: HWND, hicon: HICON) {
    let mut nid: NOTIFYICONDATAW = std::mem::zeroed();
    nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
    nid.hWnd = hwnd;
    nid.uID = 1;
    nid.uCallbackMessage = WM_TRAYICON;
    nid.hIcon = hicon;
    nid.uFlags = NIF_MESSAGE | NIF_ICON | NIF_TIP;

    let tip = widestring("Keyless – Keyboard Locker");
    nid.szTip[..tip.len()].copy_from_slice(&tip);

    Shell_NotifyIconW(NIM_ADD, &mut nid);
}

unsafe fn remove_tray_icon(hwnd: HWND) {
    let mut nid: NOTIFYICONDATAW = std::mem::zeroed();
    nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
    nid.hWnd = hwnd;
    nid.uID = 1;
    Shell_NotifyIconW(NIM_DELETE, &mut nid);
}

// === TRAY MENU ===
unsafe fn show_tray_menu(hwnd: HWND) {
    let menu = CreatePopupMenu();
    AppendMenuW(menu, MF_STRING, 1001, widestring("Lock Keyboard\0").as_ptr());
    AppendMenuW(menu, MF_STRING, 1002, widestring("Unlock Keyboard\0").as_ptr());
    AppendMenuW(menu, MF_STRING, 1003, widestring("Exit\0").as_ptr());

    let mut pt = POINT { x: 0, y: 0 };
    GetCursorPos(&mut pt);
    SetForegroundWindow(hwnd);

    let cmd = TrackPopupMenu(
        menu,
        TPM_RETURNCMD,
        pt.x,
        pt.y,
        0,
        hwnd,
        null_mut(),
    );

    if cmd == 1001 {
        IS_LOCKED.store(true, Ordering::SeqCst);
        println!("Locked via tray");
    } else if cmd == 1002 {
        IS_LOCKED.store(false, Ordering::SeqCst);
        println!("Unlocked via tray");
    } else if cmd == 1003 {
        println!("Exiting...");
        UnhookWindowsHookEx(HOOK);
        remove_tray_icon(hwnd);
        std::process::exit(0);
    }
}

// === HELPERS ===
fn widestring(s: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    std::ffi::OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}

fn main() {
    unsafe {
        // === Create hidden window for tray ===
        let class_name = widestring("KeylessWindowClass");
        let wc = WNDCLASSW {
            lpfnWndProc: Some(window_proc),
            hInstance: GetModuleHandleW(null_mut()),
            lpszClassName: class_name.as_ptr(),
            ..std::mem::zeroed()
        };

        RegisterClassW(&wc);

        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            widestring("Keyless").as_ptr(),
            WS_OVERLAPPEDWINDOW,
            0,
            0,
            0,
            0,
            null_mut(),
            null_mut(),
            wc.hInstance,
            null_mut(),
        );

        // === Load tray icon ===
        let icon = LoadImageW(
            null_mut(),
            widestring("icon.ico").as_ptr(),
            IMAGE_ICON,
            16,
            16,
            LR_LOADFROMFILE,
        ) as HICON;

        add_tray_icon(hwnd, icon);

        // === Install keyboard hook ===
        HOOK = SetWindowsHookExW(
            WH_KEYBOARD_LL,
            Some(keyboard_proc),
            GetModuleHandleW(null_mut()),
            0,
        );

        println!("Keyless running. Press F12 or use tray menu.");

        // === Message loop ===
        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
