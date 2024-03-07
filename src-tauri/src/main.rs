// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, io::{BufRead, Write}, time::Duration};

use tauri::api::process::{Command, CommandEvent};
use tokio::{sync::{mpsc, Mutex}, time::sleep};
use std::collections::HashMap;

struct NetworkState{
    tx: tokio::sync::Mutex<mpsc::Sender<String>>,
}


#[tauri::command(async)]
async fn connect_network() -> String
{
    let (mut rx, _) = Command::new_sidecar("dogcom").expect("无法找到二进制程序").args(["-m", "dhcp", "-c", "dogcom.conf", "-v"]).spawn().expect("无法运行网络连接程序");

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

#[tauri::command]
async fn change_state(message: String, state: tauri::State<'_, NetworkState>) -> Result<(), ()>
{
    // println!("change_state trigger");
    let tx = state.tx.lock().await;
    tx.send(message).await.unwrap();

    Ok(())
}

const CONFIG: &str = r#"server = '10.100.61.3'
username=''
password=''
host_ip = ''
mac = 
host_name = 'LISYSDKJ'
host_os = 'Windows'
CONTROLCHECKSTATUS = '\x20'
ADAPTERNUM = '\x03'
IPDOG = '\x01'
PRIMARY_DNS = '10.10.10.10'
dhcp_server = '0.0.0.0'
AUTH_VERSION = '\x68\x00'
KEEP_ALIVE_VERSION = '\xdc\x02'
"#;

#[derive(serde::Serialize, Debug)]
pub struct UserConfig
{
    username: String,
    password: String,
    host_ip: String,
    mac: String
}

fn check_config_file()
{
    // 获取运行程序所在路径
    let path = std::env::current_exe()
                        .unwrap()
                        .parent()
                        .unwrap()
                        .join("dogcom.conf");


    let file = match fs::File::open(&path){
        Ok(_) => None, // 存在则返回空
        Err(_) => {
            match fs::File::create(&path){
                Ok(file) => Some(file),
                Err(_) => None
            }
        }
    };

    match file{
        Some(mut conf_file) => {
            conf_file.write(CONFIG.as_bytes()).unwrap();
        },
        None => ()
    }

}

#[tauri::command]
fn js_read_config() -> UserConfig
{
    let path = std::env::current_exe()
                        .unwrap()
                        .parent()
                        .unwrap()
                        .join("dogcom.conf");
    let file = fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut config_map = HashMap::new();

    for line in reader.lines()
    {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
        config_map.insert(parts[0].to_string(), parts[1].trim_matches('\'').to_string());
    }

    UserConfig{
        username: config_map.get("username").unwrap().clone(),
        password: config_map.get("password").unwrap().to_string(),
        host_ip: config_map.get("host_ip").unwrap().to_string(),
        mac: config_map.get("mac").unwrap().to_string()
    }
}

#[tauri::command]
fn js_write_config(username: String, password: String, host_ip: String, mac: String)
{
    let path = std::env::current_exe()
                        .unwrap()
                        .parent()
                        .unwrap()
                        .join("dogcom.conf");
    let file = fs::File::open(&path).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut config_map = HashMap::new();

    for line in reader.lines()
    {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
        config_map.insert(parts[0].to_string(), parts[1].trim_matches('\'').to_string());
    }

    config_map.insert("username".to_string(), username);
    config_map.insert("password".to_string(), password);
    config_map.insert("host_ip".to_string(), host_ip);
    config_map.insert("mac".to_string(), mac);

    // 写入配置文件
    let mut file = fs::File::create(&path).unwrap();
    for (key, value) in config_map
    {
        // mac地址不需要单引号
        if key == "mac"
        {
            let line = format!("{} = {}\n", key, value);
            file.write(line.as_bytes()).unwrap();
            continue;
        }

        let line = format!("{} = '{}'\n", key, value);
        file.write(line.as_bytes()).unwrap();
    }
}

async fn test()
{
    loop 
    {
        sleep(Duration::from_secs(1)).await;
        println!("连接网络");
    }
}


fn main() {

    let mut handles = Vec::new();

    // 前端使用tx发送消息，后端使用rx接受
    let (tx, mut rx) = mpsc::channel::<String>(1);

    tauri::Builder::default()
        .manage(NetworkState{tx: Mutex::new(tx)}) // 用于前端和后端通信
        .setup(|_| {

            check_config_file();

            // 主循环，根据前端消息启动或停止
            tauri::async_runtime::spawn(async move{
                loop 
                {
                    while let Some(message) = rx.recv().await
                    {
                        println!("Rust Received: {}", message);
                        match message.as_str() {
                            "1" => handles.push(tokio::spawn(test())),
                            "2" => handles[0].abort(),
                            _ => ()
                        }
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![connect_network, change_state, js_read_config, js_write_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
