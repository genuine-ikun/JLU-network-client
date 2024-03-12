// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
use config::{check_config_file, js_read_config, js_write_config};
use tauri::{api::process::{Command, CommandEvent}, Manager};

use tokio::sync::{mpsc, Mutex};
use tokio_util::sync::CancellationToken;

pub struct NetworkState
{
    pub tx: tokio::sync::Mutex<mpsc::Sender<bool>>,
}

#[tauri::command]
async fn change_state(run: bool, state: tauri::State<'_, NetworkState>) -> Result<(), ()>
{
    let tx = state.tx.lock().await;
    tx.send(run).await.unwrap();

    Ok(())
}

#[tauri::command(async)]
async fn connect_network(token: CancellationToken, manager: tauri::AppHandle) -> Result<(), ()>
{
    let path_buf = std::env::current_exe()
                        .unwrap()
                        .parent()
                        .unwrap()
                        .join("dogcom.conf");
    let path = path_buf.to_string_lossy().to_string();

    tauri::async_runtime::spawn(async move{
                let (mut rx, child) = Command::new_sidecar("dogcom").expect("无法找到二进制程序").args(["-m", "dhcp", "-c", &path, "-v"]).spawn().expect("无法运行网络连接程序");
        tokio::select! {
            _ = token.cancelled() => {child.kill().unwrap();}
            _ = async {
            
                while let Some(event) = rx.recv().await
                {
                    if let CommandEvent::Stdout(line) = event{
                        println!("{}", line);
                        manager.emit_all("dogcom", line).unwrap();
                    }
                }
            } => {}
        }
    });

    Ok(())
}

fn main() {

    // 前端使用tx发送消息，后端使用rx接受
    let (tx, mut rx) = mpsc::channel::<bool>(1);

    tauri::Builder::default()
        .manage(NetworkState{tx: Mutex::new(tx)}) // 用于前端和后端通信
        .setup(|app| {

            check_config_file();

            // 主循环，根据前端消息启动或停止
            let mut token = CancellationToken::new();

            let app_handle = app.app_handle();

            tauri::async_runtime::spawn(async move{
                loop
                {
                    while let Some(message) = rx.recv().await
                    {
                        // 启动任务
                        if message 
                        {
                            // test(token.clone()).await;
                            connect_network(token.clone(), app_handle.clone()).await.unwrap();
                        }
                        // 取消登陆、注销
                        else
                        {
                            token.cancel();
                            token = CancellationToken::new();
                        }
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![change_state, js_read_config, js_write_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
