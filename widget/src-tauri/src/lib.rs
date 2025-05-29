// 定義 Tauri 指令：強制將視窗設定到桌面最底層
#[tauri::command]
fn force_to_bottom() {
  // 只在 Windows 系統執行
  #[cfg(target_os = "windows")]
  {
    // 使用 Windows API
    #[cfg(windows)]
    unsafe {
      use windows::Win32::UI::WindowsAndMessaging::{
        SetWindowPos, FindWindowA, HWND_BOTTOM, SWP_NOMOVE, SWP_NOSIZE, SWP_NOACTIVATE
      };
      use windows::core::PCSTR;
      
      // 透過類別名稱尋找 Tauri 視窗
      let class_name = std::ffi::CString::new("tauri").unwrap();
      if let Ok(hwnd) = FindWindowA(PCSTR(class_name.as_ptr() as *const u8), PCSTR::null()) {
        // 檢查視窗是否有效
        if !hwnd.is_invalid() {
          // 使用 SetWindowPos 將視窗設定到最底層
          // HWND_BOTTOM: 將視窗置於所有非置頂視窗的底部
          // SWP_NOMOVE: 不改變位置
          // SWP_NOSIZE: 不改變大小
          // SWP_NOACTIVATE: 不啟動視窗
          let _ = SetWindowPos(
            hwnd,
            HWND_BOTTOM,
            0, 0, 0, 0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE
          );
        }
      }
    }
  }
}

// Tauri 應用程式的主要入口點
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    // 註冊可從前端呼叫的指令
    .invoke_handler(tauri::generate_handler![force_to_bottom])
    // 應用程式初始化設定
    .setup(|app| {
      // 在除錯模式下啟用日誌功能
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }

      // Windows 系統特定的視窗設定
      #[cfg(target_os = "windows")]
      {
        use tauri::Manager;
        // 取得主視窗的參考
        let window = app.get_webview_window("main").unwrap();
        
        // 使用 Windows API 設定視窗層級
        #[cfg(windows)]
        unsafe {
          use windows::Win32::Foundation::HWND;
          use windows::Win32::UI::WindowsAndMessaging::{
            SetWindowPos, SetParent, GetDesktopWindow, 
            HWND_BOTTOM, SWP_NOMOVE, SWP_NOSIZE, SWP_NOACTIVATE
          };
          
          // 將 Tauri 視窗控制代碼轉換為 Windows HWND
          let hwnd = HWND(window.hwnd().unwrap().0 as *mut std::ffi::c_void);
          // 取得桌面視窗控制代碼
          let desktop_hwnd = GetDesktopWindow();
          
          // 將 widget 設定為桌面的子視窗
          // 這使得它會顯示在桌面上，但在其他應用程式視窗之下
          let _ = SetParent(hwnd, desktop_hwnd);
          
          // 強制將視窗設定到最底層
          let _ = SetWindowPos(
            hwnd,
            HWND_BOTTOM,
            0, 0, 0, 0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE
          );

          // 監聽視窗焦點事件
          // 將 HWND 轉換為可在閉包中使用的 isize 型別
          let hwnd_raw = hwnd.0 as isize;
          window.on_window_event(move |event| {
            // 當視窗獲得焦點時
            if let tauri::WindowEvent::Focused(true) = event {
              // 重新建構 HWND 並將視窗設定回底層
              // 這防止了當使用者點擊 widget 時它跳到前景
              let hwnd = HWND(hwnd_raw as *mut std::ffi::c_void);
              let _ = SetWindowPos(
                hwnd,
                HWND_BOTTOM,
                0, 0, 0, 0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE
              );
            }
          });
        }
      }

      // 初始化完成
      Ok(())
    })
    // 啟動 Tauri 應用程式
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
