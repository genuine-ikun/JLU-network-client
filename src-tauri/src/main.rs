// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{api::process::{Command, CommandEvent}};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command(async)]
async fn connect_network() -> String
{
    let (mut rx, _) = Command::new_sidecar("dogcom").expect("无法找到二进制程序").args(["-m", "dhcp", "-c", "./bin/dogcom.conf", "-v"]).spawn().expect("无法运行网络连接程序");

    let mut result = String::new();

    while let Some(event) = rx.recv().await
    {
        if let CommandEvent::Stdout(line) = event{
            // print!("{}", line);
            result.push_str(&line);
        }
    }
    result
}

fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![connect_network])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
