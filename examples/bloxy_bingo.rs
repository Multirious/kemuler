use kemuler::backend;
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

    fn get_windows() -> WindowsResult<Vec<WindowsHWND>> {
        type PanicResult<T> = Result<T, Box<dyn std::any::Any + Send>>;

        unsafe extern "system" fn enum_windows_callback(
            handle: WindowsHWND,
            vec: WindowsLparam,
        ) -> WindowsBool {
            let vec: *mut PanicResult<Vec<WindowsHWND>> = vec.0 as _;
            let vec: &mut _ = &mut *vec;
            let res = {
                let vec = vec.as_mut().unwrap();
                let mut vec = std::panic::AssertUnwindSafe(vec);
                std::panic::catch_unwind(move || vec.push(handle))
            };
            match res {
                Ok(()) => WindowsBool::from(true),
                Err(e) => {
                    *vec = Err(e);
                    WindowsBool::from(false)
                }
            }
        }

        // Should error if vec capacity reached `isize::MAX`
        let mut vec: PanicResult<Vec<WindowsHWND>> = Ok(vec![]);

        let res = unsafe {
            windows::Win32::UI::WindowsAndMessaging::EnumWindows(
                Some(enum_windows_callback),
                WindowsLparam(&mut vec as *mut _ as isize),
            )
        }
        .ok();
        let vec = match vec {
            Ok(o) => o,
            Err(e) => std::panic::resume_unwind(e),
        };
        res.map(|_| vec)
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

    backend::set_keyboard_common_backend(Box::new(backend::default::DefaultBackend));
    backend::set_mouse_backend(Box::new(backend::directx::General));

    let area = (518, 792, 846, 237);

    // lreader board area
    // let area = (1677, 60, 233, 508);

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
        let step_x = 30;
        let step_y = 30;
        loop {
            for (x, y) in positions_in_area(area.2, area.3, step_x, step_y) {
                wait(0.01);
                let x = x + area.0;
                let y = y + area.1;
                MousePosition.change_to((x, y));
                MouseButton::Left.pulse();

                if is_quit() {
                    return;
                }
            }

            for y in (543..=(543 + 488)).step_by(step_y as usize) {
                wait(0.01);
                MousePosition.change_to((1920 / 2, y));
                MouseButton::Left.pulse();
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
