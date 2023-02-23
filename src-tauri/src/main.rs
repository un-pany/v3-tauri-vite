#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// #[macro_use]
extern crate lazy_static;

use lazy_static::lazy_static;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{process, sync::Mutex};
use tauri::{
    api::dialog, generate_context, generate_handler, AppHandle, CustomMenuItem, GlobalWindowEvent,
    Manager, Menu, MenuItem, Submenu, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, Window, WindowEvent, WindowMenuEvent,
};

// 全局变量
struct GlobalData {
    vector: Vec<String>,
    timestamp: String,
}
impl GlobalData {
    fn new() -> GlobalData {
        GlobalData {
            vector: Vec::new(),
            timestamp: String::from(""),
        }
    }
}
lazy_static! {
    static ref GLOBAL_DATA: Mutex<GlobalData> = Mutex::new(GlobalData::new());
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    if !name.is_empty() {
        format!("Hello, {}!", name)
    } else {
        "Hello!".to_string()
    }
}

fn main() {
    println!("\nMy pid is {} ", process::id());

    // 构建
    tauri::Builder::default()
        // 注册命令
        .invoke_handler(generate_handler![greet])
        // 设置 插件
        .plugin(tauri_plugin_single_instance::init(
            |app: &AppHandle, argv: Vec<String>, cwd: String| {
                println!("\n{:?}\n{}\n", argv, cwd);
                // 二次打开软件时，显示已打开窗口，单例运行 app
                let window: Window = app.get_window("main").unwrap();
                window.set_focus().unwrap();
                window.show().unwrap();
            },
        ))
        // 设置 系统菜单
        .menu(get_sys_menu())
        // 设置 系统托盘
        .system_tray(get_tray_menu())
        // 处理 窗口事件
        .on_window_event(|event: GlobalWindowEvent| win_event_handle(event))
        // 处理 系统菜单事件
        .on_menu_event(|event: WindowMenuEvent| sys_menu_handle(event))
        // 处理 系统托盘事件
        .on_system_tray_event(|app: &AppHandle, event: SystemTrayEvent| {
            tray_menu_handle(app, event)
        })
        .run(generate_context!())
        .expect("error while running tauri application");
}

/** 系统菜单 */
fn get_sys_menu() -> Menu {
    let quit: CustomMenuItem = CustomMenuItem::new("quit".to_string(), "Quit");
    let close: CustomMenuItem = CustomMenuItem::new("close".to_string(), "Close");
    let submenu: Submenu = Submenu::new("Tools", Menu::new().add_item(quit).add_item(close));
    let hide: CustomMenuItem = CustomMenuItem::new("hide".to_string(), "Hide")
        .accelerator("CmdOrCtrl+H")
        .into();
    Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(hide)
        .add_submenu(submenu)
}

//** 处理系统菜单事件 */
fn sys_menu_handle(event: WindowMenuEvent) {
    match event.menu_item_id() {
        "quit" => {
            process::exit(0);
        }
        "close" => {
            event.window().close().unwrap();
        }
        "hide" => toggle_win_show(&event.window().app_handle()),
        _ => {}
    }
}

/** 切换窗口的显示与隐藏 */
fn toggle_win_show(app_handle: &AppHandle) {
    let window: Window = app_handle.get_window("main").unwrap();
    let flag: bool = window.is_visible().unwrap();
    let text: String;
    if flag {
        window.hide().unwrap();
        text = "显示".to_string();
    } else {
        window.unminimize().unwrap();
        window.center().unwrap();
        window.show().unwrap();
        window.set_always_on_top(true).unwrap();
        text = "隐藏".to_string();
    }
    println!("\n{} => {}", flag, text);
    let item_handle = app_handle.tray_handle().get_item(&"hide");
    item_handle.set_title(text).unwrap();
}

/** 处理窗口事件 */
fn win_event_handle(event: GlobalWindowEvent) {
    match event.event() {
        WindowEvent::Focused(..) => {
            let window: Window = event.window().clone();
            window.set_always_on_top(false).unwrap();
        }
        WindowEvent::CloseRequested { api, .. } => {
            println!("CloseRequested");
            api.prevent_close();
            let window: Window = event.window().clone();
            dialog::confirm(
                Some(&(event.window())),
                "提示",
                "确定退出客户端？",
                move |answer: bool| {
                    if answer {
                        window.close().unwrap();
                    }
                },
            )
        }
        WindowEvent::Destroyed => {
            println!("Destroyed");
        }
        _ => {} // 其他事件
    }
}

/** 托盘菜单 */
fn get_tray_menu() -> SystemTray {
    let hide: CustomMenuItem = CustomMenuItem::new("hide".to_string(), "隐藏");
    let quit: CustomMenuItem = CustomMenuItem::new("quit".to_string(), "退出");
    let devtools: CustomMenuItem = CustomMenuItem::new("devtools".to_string(), "控制台");
    let tray_menu: SystemTrayMenu = SystemTrayMenu::new()
        .add_item(hide)
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(devtools);
    SystemTray::new().with_menu(tray_menu)
}

/** 处理托盘事件 */
fn tray_menu_handle(app_handle: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::DoubleClick { .. } => {
            toggle_win_show(app_handle);
            // 修改全局变量
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("error by get time")
                .as_millis()
                .to_string();
            GLOBAL_DATA.lock().unwrap().vector.clear();
            GLOBAL_DATA.lock().unwrap().vector.push(now.clone());
            GLOBAL_DATA.lock().unwrap().timestamp = now;
            println!("全局变量 : {:?}", GLOBAL_DATA.lock().unwrap().vector);
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "hide" => {
                toggle_win_show(app_handle);
            }
            "quit" => {
                process::exit(0);
            }
            // "devtools" => {
            //     let window: Window = app_handle.get_window("main").unwrap();
            //     println!("{}", window.is_devtools_open());
            //     if !window.is_devtools_open() {
            //         window.open_devtools();
            //     }
            // }
            _ => {}
        },
        _ => {}
    }
}
