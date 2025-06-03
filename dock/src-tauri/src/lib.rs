use tauri::Manager;
use std::process::Command;

// 應用程式資訊結構
#[derive(Debug)]
struct AppInfo {
    name: String,
    primary_command: String,
    backup_commands: Vec<String>,
    description: String,
}

impl AppInfo {
    fn new(name: &str, primary: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            primary_command: primary.to_string(),
            backup_commands: Vec::new(),
            description: description.to_string(),
        }
    }

    fn with_backup(mut self, backup: &str) -> Self {
        self.backup_commands.push(backup.to_string());
        self
    }
}

// 定位視窗到螢幕底部的指令
#[tauri::command]
fn position_dock_to_bottom(window: tauri::WebviewWindow) -> Result<(), String> {
    // 獲取螢幕尺寸
    if let Ok(monitor) = window.current_monitor() {
        if let Some(monitor) = monitor {
            let screen_size = monitor.size();
            
            // 設定 dock 視窗的尺寸
            let dock_width = 800;
            let dock_height = 120;
            
            // 計算位置：水平居中，垂直在底部
            let x = (screen_size.width as i32 - dock_width) / 2;
            let y = screen_size.height as i32 - dock_height - 50; // 距離底部 50px
            
            // 設定視窗位置
            window.set_position(tauri::Position::Physical(tauri::PhysicalPosition { x, y }))
                .map_err(|e| format!("設定位置失敗: {}", e))?;
        }
    }
    
    Ok(())
}

// 獲取應用程式配置
fn get_app_config() -> std::collections::HashMap<String, AppInfo> {
    let mut apps = std::collections::HashMap::new();
    
    apps.insert("files".to_string(), 
        AppInfo::new("檔案管理器", "explorer.exe", "開啟 Windows 檔案總管")
    );
    
    apps.insert("browser".to_string(), 
        AppInfo::new("瀏覽器", "msedge.exe", "開啟 Microsoft Edge 瀏覽器")
            .with_backup("start chrome")
            .with_backup("start firefox")
    );
    
    apps.insert("terminal".to_string(), 
        AppInfo::new("終端機", "cmd.exe", "開啟命令提示字元")
            .with_backup("powershell.exe")
            .with_backup("wt.exe")  // Windows Terminal
    );
    
    apps.insert("settings".to_string(), 
        AppInfo::new("設定", "ms-settings:", "開啟 Windows 設定")
            .with_backup("control")
    );
    
    apps.insert("music".to_string(), 
        AppInfo::new("音樂", "spotify.exe", "開啟 Spotify 音樂")
            .with_backup("start mswindowsmusic:")  // Windows 音樂應用程式
    );
    
    apps
}

// 嘗試啟動指令
fn try_launch_command(command: &str) -> Result<(), std::io::Error> {
    println!("嘗試執行指令: {}", command);
    
    if command.starts_with("start ") {
        // 使用 cmd 執行 start 指令
        Command::new("cmd")
            .args(&["/C", command])
            .spawn()?;
    } else if command.ends_with(":") {
        // Windows URI 協議 (如 ms-settings:)
        Command::new("cmd")
            .args(&["/C", "start", command])
            .spawn()?;
    } else {
        // 直接執行程式
        Command::new(command).spawn()?;
    }
    
    Ok(())
}

// 主要的應用程式啟動指令 - 完全由 Rust 處理
#[tauri::command]
async fn handle_app_click(app_id: String) -> Result<String, String> {
    println!("🚀 Rust 後端處理應用程式點擊: {}", app_id);
    
    let apps = get_app_config();
    
    let app_info = apps.get(&app_id)
        .ok_or_else(|| format!("未知的應用程式 ID: {}", app_id))?;
    
    println!("📱 嘗試啟動: {}", app_info.description);
    
    // 嘗試主要指令
    match try_launch_command(&app_info.primary_command) {
        Ok(_) => {
            let success_msg = format!("✅ 成功啟動 {}", app_info.description);
            println!("{}", success_msg);
            return Ok(success_msg);
        }
        Err(e) => {
            println!("❌ 主要指令失敗: {} - {}", app_info.primary_command, e);
        }
    }
    
    // 嘗試備用指令
    for backup_cmd in &app_info.backup_commands {
        println!("🔄 嘗試備用方案: {}", backup_cmd);
        match try_launch_command(backup_cmd) {
            Ok(_) => {
                let success_msg = format!("✅ 成功啟動 {} (備用方案)", app_info.description);
                println!("{}", success_msg);
                return Ok(success_msg);
            }
            Err(e) => {
                println!("❌ 備用指令失敗: {} - {}", backup_cmd, e);
            }
        }
    }
    
    let error_msg = format!("❌ 無法啟動 {} - 所有方案都失敗", app_info.description);
    println!("{}", error_msg);
    Err(error_msg)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      
      // 獲取主視窗並設定位置到螢幕底部
      if let Some(window) = app.get_webview_window("main") {
        let window_clone = window.clone();
        // 延遲一點執行，確保視窗完全載入
        tauri::async_runtime::spawn(async move {
          tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
          if let Err(e) = position_dock_to_bottom(window_clone) {
            eprintln!("定位 dock 失敗: {}", e);
          }
        });
      }
      
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      position_dock_to_bottom,
      handle_app_click
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
