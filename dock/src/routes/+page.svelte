<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { getCurrentWindow } from '@tauri-apps/api/window';

  // 簡單的點擊處理 - 所有邏輯由 Rust 處理
  async function clickApp(appId: string): Promise<void> {
    try {
      // 直接將事件傳遞給 Rust 後端處理
      const result = await invoke('handle_app_click', { appId });
      console.log('🎉', result);
    } catch (error) {
      console.error('❌ 啟動失敗:', error);
      // 可選：顯示錯誤提示
      // alert(`啟動失敗: ${error}`);
    }
  }

  // 關閉應用程式
  async function closeApp(): Promise<void> {
    try {
      const window = getCurrentWindow();
      await window.close();
    } catch (error) {
      console.error('關閉失敗:', error);
    }
  }
</script>

<div class="dock">
  <div class="dock-item" title="檔案管理器" on:click={() => clickApp('files')}>
    <span class="icon">📁</span>
  </div>
  <div class="dock-item" title="瀏覽器" on:click={() => clickApp('browser')}>
    <span class="icon">🌐</span>
  </div>
  <div class="dock-item" title="終端機" on:click={() => clickApp('terminal')}>
    <span class="icon">💻</span>
  </div>
  <div class="dock-item" title="設定" on:click={() => clickApp('settings')}>
    <span class="icon">⚙️</span>
  </div>
  <div class="dock-item" title="音樂" on:click={() => clickApp('music')}>
    <span class="icon">🎵</span>
  </div>
  
  <!-- 關閉按鈕 -->
  <div class="dock-separator"></div>
  <div class="dock-item close-btn" title="關閉 Dock" on:click={closeApp}>
    <span class="icon">❌</span>
  </div>
</div>

<style>
  .page {
    width: 100vw;
    height: 100vh;
    display: flex;
    align-items: flex-end;    /* 將內容對齊到底部 */
    justify-content: center;  /* 水平居中 */
    padding-bottom: 20px;     /* 距離底部 20px */
    box-sizing: border-box;
  }

  .dock {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 16px;
    padding: 20px;
    margin: 20px;
    
    /* 暗色系背景 */
    background: rgba(20, 20, 20, 0.9);
    backdrop-filter: blur(20px);
    border-radius: 24px;
    border: 1px solid rgba(255, 255, 255, 0.1);
    
    /* 陰影效果 */
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
  }

  .dock-separator {
    width: 1px;
    height: 40px;
    background: rgba(255, 255, 255, 0.2);
    margin: 0 8px;
  }

  .dock-item {
    width: 64px;
    height: 64px;
    display: flex;
    align-items: center;
    justify-content: center;
    
    background: rgba(40, 40, 40, 0.8);
    border-radius: 16px;
    border: 1px solid rgba(255, 255, 255, 0.05);
    cursor: pointer;
    
    transition: all 0.3s ease;
    
    /* 改善點擊體驗 */
    user-select: none;
  }

  .close-btn {
    background: rgba(60, 20, 20, 0.8);
    border-color: rgba(255, 100, 100, 0.2);
  }

  .close-btn:hover {
    background: rgba(80, 30, 30, 0.9);
    border-color: rgba(255, 150, 150, 0.4);
  }

  .icon {
    font-size: 32px;
    transition: all 0.3s ease;
    pointer-events: none; /* 防止圖示本身被點擊 */
  }

  .dock-item:hover {
    transform: scale(1.2) translateY(-8px);
    background: rgba(60, 60, 60, 0.9);
    border-color: rgba(255, 255, 255, 0.2);
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.5);
  }

  .dock-item:hover .icon {
    transform: scale(1.1);
  }

  .dock-item:active {
    transform: scale(1.05) translateY(-4px);
  }
</style>
