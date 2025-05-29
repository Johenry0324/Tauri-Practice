<script lang="ts">
  // 引入 Tauri API 來控制視窗
  import { getCurrentWindow } from '@tauri-apps/api/window';

  // 關閉應用程式的函數
  async function closeApp() {
    try {
      // 取得當前視窗並關閉它
      const appWindow = getCurrentWindow();
      await appWindow.close();
    } catch (error) {
      console.error('關閉應用程式失敗:', error);
    }
  }
</script>

<!-- 關閉按鈕 -->
<button 
  class="close-button" 
  onclick={closeApp}
  title="關閉"
>
  ✕
</button>

<style>
  .close-button {
    position: absolute; /* 絕對定位 */
    top: 8px; /* 距離頂部 8px */
    right: 8px; /* 距離右側 8px */
    width: 24px; /* 寬度 */
    height: 24px; /* 高度 */
    border: none; /* 無邊框 */
    border-radius: 50%; /* 圓形按鈕 */
    background: rgba(255, 255, 255, 0.1); /* 半透明白色背景 */
    color: rgba(255, 255, 255, 0.7); /* 淺白色文字 */
    font-size: 12px; /* 字體大小 */
    font-weight: bold; /* 粗體 */
    cursor: pointer; /* 指標游標 */
    display: flex; /* 彈性佈局 */
    align-items: center; /* 垂直置中 */
    justify-content: center; /* 水平置中 */
    transition: all 0.2s ease; /* 所有屬性的動畫過渡 */
    z-index: 1000; /* 確保在最上層 */
  }

  /* 滑鼠懸停效果 */
  .close-button:hover {
    background: rgba(255, 0, 0, 0.6); /* 紅色背景 */
    color: white; /* 純白色文字 */
    transform: scale(1.1); /* 放大 1.1 倍 */
  }

  /* 滑鼠按下效果 */
  .close-button:active {
    transform: scale(0.95); /* 縮小到 0.95 倍 */
  }
</style> 