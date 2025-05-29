# Windows 桌面 Widget 專案 - 中文註釋說明

## 📁 專案結構與功能說明

### 🎯 核心配置檔案

#### `src-tauri/tauri.conf.json` - Tauri 應用程式配置
```json
{
  "productName": "widget",        // 產品名稱
  "version": "0.1.0",            // 版本號
  "identifier": "com.tauri.widget", // 應用程式唯一識別碼
  
  "build": {
    "frontendDist": "../build",           // 前端建置輸出目錄
    "devUrl": "http://localhost:5173",    // 開發伺服器 URL
    "beforeDevCommand": "npm run dev",    // 開發前執行的指令
    "beforeBuildCommand": "npm run build" // 建置前執行的指令
  },
  
  "app": {
    "windows": [{
      "title": "Desktop Widget",     // 視窗標題
      "width": 320,                  // 預設寬度
      "height": 480,                 // 預設高度
      "minWidth": 280,               // 最小寬度
      "minHeight": 400,              // 最小高度
      "maxWidth": 500,               // 最大寬度
      "maxHeight": 800,              // 最大高度
      "resizable": true,             // 可調整大小
      "fullscreen": false,           // 非全螢幕
      "decorations": false,          // 無視窗裝飾（標題列、邊框）
      "transparent": true,           // 透明背景
      "alwaysOnTop": false,          // 不總是置頂
      "skipTaskbar": true,           // 不顯示在工作列
      "center": false,               // 不自動置中
      "x": 100,                      // 初始 X 座標
      "y": 100,                      // 初始 Y 座標
      "visibleOnAllWorkspaces": true // 在所有工作區可見
    }]
  }
}
```

#### `src-tauri/capabilities/default.json` - 權限設定
```json
{
  "permissions": [
    "core:default",                        // 核心預設權限
    "core:window:allow-start-dragging",    // 允許開始拖曳視窗
    "core:window:allow-minimize",          // 允許最小化視窗
    "core:window:allow-close",             // 允許關閉視窗
    "core:window:allow-set-always-on-top", // 允許設定總是置頂
    "core:window:allow-set-position",      // 允許設定視窗位置
    "core:window:allow-set-size"           // 允許設定視窗大小
  ]
}
```

### 🔧 後端 Rust 程式碼

#### `src-tauri/src/lib.rs` - 主要邏輯
```rust
// 定義可從前端呼叫的 Tauri 指令
#[tauri::command]
fn force_to_bottom() {
    // 使用 Windows API 強制將視窗設定到桌面底層
    // FindWindowA: 尋找視窗
    // SetWindowPos: 設定視窗位置和層級
    // HWND_BOTTOM: 將視窗置於所有非置頂視窗的底部
}

// 應用程式初始化函數
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![force_to_bottom]) // 註冊指令
        .setup(|app| {
            // 取得主視窗並設定為桌面層級
            let window = app.get_webview_window("main").unwrap();
            
            // Windows API 呼叫：
            // SetParent: 將視窗設為桌面的子視窗
            // SetWindowPos: 強制置於底層
            // on_window_event: 監聽焦點事件，防止跳到前景
        })
}
```

### 🎨 前端 Svelte 元件

#### `src/lib/components/ui/WidgetContainer.svelte` - 主容器
```typescript
// 主要功能：
// 1. 視窗拖曳處理
// 2. 定期強制設定到底層（每3秒）
// 3. 包含關閉按鈕
// 4. 半透明毛玻璃效果

async function handleMouseDown(event: MouseEvent) {
    // 防止在按鈕上觸發拖曳
    // 呼叫 Tauri API 開始拖曳
    // 拖曳完成後重新設定到底層
}
```

#### `src/lib/components/ui/InfoBlock.svelte` - 資訊區塊
```typescript
// 可復用的資訊顯示元件
// 支援：圖示、標題、數值、單位、載入狀態、自訂內容
// 使用 Svelte 5 的 snippet 語法
```

#### `src/lib/components/ui/CloseButton.svelte` - 關閉按鈕
```typescript
// 右上角浮動關閉按鈕
// 呼叫 Tauri API 關閉應用程式
// 紅色懸停效果
```

#### `src/routes/+page.svelte` - 主頁面
```typescript
// 整合所有元件
// 即時時間顯示（每秒更新）
// 多語系支援
// 使用 Svelte 5 snippet 語法組織內容
```

#### `src/routes/+layout.svelte` - 佈局檔案
```typescript
// 多語系初始化
// 確保 i18n 載入完成後才顯示內容
// 載入動畫
```

### 🌐 多語系系統

#### `src/lib/i18n/index.ts` - 多語系初始化
```typescript
// 註冊語言檔案
register('zh-TW', () => import('./locales/zh-TW.json'));
register('en', () => import('./locales/en.json'));

// 初始化設定
init({
    fallbackLocale: 'zh-TW',    // 預設語言
    initialLocale: localStorage.getItem('locale') ?? 'zh-TW' // 從瀏覽器儲存讀取
});
```

#### 語言檔案結構
```json
{
  "app": {
    "title": "桌面小工具",
    "close": "關閉",
    "settings": "設定"
  },
  "widget": {
    "placeholder": "功能開發中...",
    "loading": "載入中..."
  }
}
```

## 🔄 運作流程詳解

### 1. 應用程式啟動流程
```
1. Tauri 載入 Rust 後端
2. 初始化視窗設定（透明、無邊框、桌面層級）
3. 載入 SvelteKit 前端
4. 多語系初始化
5. 顯示 Widget 介面
6. 開始定時器（時間更新、層級檢查）
```

### 2. 桌面層級控制機制
```
1. SetParent(hwnd, desktop) - 設為桌面子視窗
2. SetWindowPos(HWND_BOTTOM) - 強制置於底層
3. 焦點事件監聽 - 獲得焦點時立即重設
4. 定期檢查 - 每3秒執行 force_to_bottom
5. 拖曳後重設 - 防止拖曳後停留在前景
```

### 3. 拖曳功能實作
```
1. 偵測滑鼠按下 - handleMouseDown
2. 排除互動元素 - 按鈕、輸入框等
3. 呼叫 Tauri API - startDragging()
4. 系統接管拖曳 - Windows 原生拖曳
5. 完成後重設層級 - force_to_bottom
```

## 🎯 關鍵設計決策

### 為什麼使用這些技術？

1. **Tauri**: 輕量、安全、效能佳的桌面應用框架
2. **Svelte 5**: 現代化、效能優異的前端框架
3. **Windows API**: 直接控制視窗層級，確保桌面效果
4. **多語系**: 提供更好的使用者體驗
5. **TypeScript**: 型別安全，減少開發錯誤

### 架構優勢

1. **分離關注點**: 前端專注 UI，後端處理系統整合
2. **模組化設計**: 元件可復用，易於擴展
3. **效能優化**: 原生效能 + 現代前端技術
4. **跨平台潛力**: Tauri 支援多平台（目前專注 Windows）

## 🚀 擴展建議

### 可以添加的功能
1. **系統監控**: CPU、記憶體、磁碟使用率
2. **天氣資訊**: 整合氣象 API
3. **快捷工具**: 計算機、便條紙、截圖
4. **主題系統**: 多種視覺風格
5. **設定介面**: 透明度、大小、功能開關

### 技術改進方向
1. **更好的層級控制**: 研究更穩定的桌面層級方案
2. **效能優化**: 減少定時器頻率，優化渲染
3. **記憶體管理**: 長時間運行的穩定性
4. **使用者設定**: 持久化配置儲存 