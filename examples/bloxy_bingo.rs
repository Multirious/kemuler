use windows::core::Result as WindowsResult;
use windows::Win32::Foundation::BOOL as WindowsBool;
use windows::Win32::Foundation::HWND as WindowsHWND;
use windows::Win32::Foundation::LPARAM as WindowsLparam;

use device_query::DeviceQuery;
use std::sync::atomic::AtomicBool;

use kemuler::control_flow::wait;
use kemuler::prelude::*;

struct Window {
    handle: WindowsHWND,
    process_id: u32,
    thread_id: u32,
}

impl Window {
    fn new(handle: WindowsHWND) -> WindowsResult<Window> {
        let (process_id, thread_id) = Window::window_process_thread_id(&handle)?;
        Ok(Window {
            handle,
            process_id,
            thread_id,
        })
    }

    fn windows() -> WindowsResult<Vec<WindowsHWND>> {
        unsafe extern "system" fn enum_windows_callback(
            handle: WindowsHWND,
            vec: WindowsLparam,
        ) -> WindowsBool {
            let vec: *mut Vec<WindowsHWND> = vec.0 as _;
            unsafe {
                (*vec).push(handle);
            }
            WindowsBool::from(true)
        }

        let mut vec: Vec<WindowsHWND> = vec![];

        unsafe {
            windows::Win32::UI::WindowsAndMessaging::EnumWindows(
                Some(enum_windows_callback),
                WindowsLparam(&mut vec as *mut _ as isize),
            )
        }
        .ok()
        .map(|_| vec)
    }

    fn window_process_thread_id(window: &WindowsHWND) -> WindowsResult<(u32, u32)> {
        let mut process_id = 0;
        let thread_id = unsafe {
            windows::Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId(
                *window,
                Some(&mut process_id),
            )
        };
        if thread_id == 0 {
            return Err(windows::core::Error::from_win32());
        }
        Ok((process_id, thread_id))
    }

    fn title(window: &WindowsHWND) -> WindowsResult<String> {
        let mut title = [0; 200];
        let res =
            unsafe { windows::Win32::UI::WindowsAndMessaging::GetWindowTextW(*window, &mut title) };
        if res == 0 {
            return Err(windows::core::Error::from_win32());
        }
        Ok(String::from_utf16(&title[..(res as usize)]).unwrap())
    }
}

fn current_thread_id() -> u32 {
    unsafe { windows::Win32::System::Threading::GetCurrentThreadId() }
}

fn attach_thread_input(id_attach: u32, id_attach_to: u32, attach: bool) -> WindowsResult<()> {
    unsafe {
        windows::Win32::System::Threading::AttachThreadInput(
            id_attach,
            id_attach_to,
            WindowsBool::from(attach),
        )
    }
    .ok()
}

static IS_QUIT: AtomicBool = AtomicBool::new(false);

fn is_quit() -> bool {
    IS_QUIT.load(std::sync::atomic::Ordering::Relaxed)
}

fn notify_quit() {
    IS_QUIT.store(true, std::sync::atomic::Ordering::Relaxed);
}

fn positions_in_area(
    size_x: i32,
    size_y: i32,
    step_x: i32,
    step_y: i32,
) -> impl Iterator<Item = (i32, i32)> {
    let nx = size_x / step_x;
    let ny = size_y / step_y;
    (0..ny).into_iter().flat_map(move |y| {
        (0..nx).into_iter().map(move |x| {
            let x = x * step_x;
            let y = y * step_y;
            (x, y)
        })
    })
}

fn main() {
    // let windows = Window::windows().unwrap();
    // let (roblox_window, title) = windows
    //     .into_iter()
    //     .find_map(|v| {
    //         let Ok(title) = Window::title(&v) else {
    //             return None;
    //         };
    //         if title.to_lowercase().contains("roblox") {
    //             // println!("{title}");
    //             Some((v, title))
    //         } else {
    //             None
    //         }
    //     })
    //     .unwrap();
    // let roblox_window = Window::new(roblox_window).unwrap();
    // let current_thread_id = current_thread_id();

    // attach_thread_input(roblox_window.thread_id, current_thread_id, true).unwrap();

    // bingo area
    // let area = (519, 684, 880, 354);

    // lreader board area
    let area = (1677, 60, 233, 508);

    let alt_tab = || {
        (KeyCommon::Alt + KeyCommon::Tab).pulse();
    };

    alt_tab();

    wait(2);

    let quit_detect_loop = || {
        let device_state = device_query::DeviceState::new();
        loop {
            if device_state.get_keys().contains(&device_query::Keycode::Q) {
                notify_quit();
                break;
            }
        }
    };

    let macro_loop = || {
        let mut previous_pos: Option<(i32, i32)> = None;
        loop {
            for (x, y) in positions_in_area(area.2, area.3, 5, 5) {
                wait(0.01);
                let x = x + area.0;
                let y = y + area.1;
                match previous_pos {
                    Some((px, py)) => {
                        let dx = x - px;
                        let dy = y - py;
                        previous_pos = Some((px + dx, py + dy));
                        MousePosition.change_by((dx, dy));
                    }
                    None => {
                        MousePosition.change_to((x, y));
                        previous_pos = Some((x, y))
                    }
                }

                if is_quit() {
                    return;
                }
            }
            if is_quit() {
                return;
            }
        }
    };

    std::thread::scope(|scope| {
        scope.spawn(macro_loop);
        scope.spawn(quit_detect_loop);
    });

    // attach_thread_input(current_thread_id, roblox_window.thread_id, false).unwrap();
}
