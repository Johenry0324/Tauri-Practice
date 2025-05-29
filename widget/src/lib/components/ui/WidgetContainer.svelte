<script lang="ts">
  // 引入 Svelte 的生命週期函數
  import { onMount, onDestroy } from 'svelte';
  // 引入 Tauri API 來控制視窗
  import { getCurrentWindow } from '@tauri-apps/api/window';
  // 引入 Tauri API 來呼叫後端指令
  import { invoke } from '@tauri-apps/api/core';
  // 引入關閉按鈕元件
  import CloseButton from './CloseButton.svelte';

  // 定義元件的 props 型別
  interface Props {
    children: any; // 子內容（使用 Svelte 5 的 snippet 語法）
  }

  // 解構 props
  let { children }: Props = $props();
  let appWindow: any; // 儲存視窗參考
  let interval: number; // 儲存定時器 ID

  // 元件掛載時執行
  onMount(async () => {
    try {
      // 取得當前視窗的參考
      appWindow = getCurrentWindow();
      console.log('Window initialized:', appWindow);

      // 設定定時器：每 3 秒強制將視窗設定到底層
      // 這確保即使有其他程式干擾，widget 仍保持在桌面層級
      interval = setInterval(async () => {
        try {
          await invoke('force_to_bottom');
        } catch (error) {
          console.error('Failed to force window to bottom:', error);
        }
      }, 3000);
    } catch (error) {
      console.error('Failed to get window:', error);
    }
  });

  // 元件銷毀時清理定時器
  onDestroy(() => {
    if (interval) {
      clearInterval(interval);
    }
  });

  // 處理滑鼠按下事件（用於拖曳功能）
  async function handleMouseDown(event: MouseEvent) {
    
    // 防止在互動元素（如按鈕）上觸發拖曳
    const target = event.target as HTMLElement;
    if (target.tagName === 'BUTTON' || target.tagName === 'INPUT' || target.closest('button')) {
      return;
    }

    // 如果視窗參考存在，開始拖曳
    if (appWindow) {
      try {
        // 呼叫 Tauri API 開始拖曳視窗
        await appWindow.startDragging();
        
        // 拖曳完成後，稍等片刻再強制設定回底層
        // 這防止拖曳後視窗停留在前景
        setTimeout(async () => {
          try {
            await invoke('force_to_bottom');
          } catch (error) {
            console.error('Failed to force window to bottom after drag:', error);
          }
        }, 100);
      } catch (error) {
        console.error('Drag failed:', error);
      }
    } else {
      console.error('Window not available');
    }
  }
</script>

<!-- 主要的 widget 容器 -->
<div 
  class="widget-container"
  role="button"
  tabindex="0"
  onmousedown={handleMouseDown}
>
  <!-- 關閉按鈕（右上角） -->
  <CloseButton />
  <!-- 渲染子內容 -->
  {@render children()}
</div>

<style>
  .widget-container {
    position: relative; /* 讓子元素可以絕對定位 */
    width: 100%; /* 佔滿視窗寬度 */
    height: 100vh; /* 佔滿視窗高度 */
    background: rgba(0, 0, 0, 0.8); /* 半透明黑色背景 */
    backdrop-filter: blur(10px); /* 毛玻璃模糊效果 */
    border-radius: 12px; /* 圓角 */
    border: 1px solid rgba(255, 255, 255, 0.1); /* 淺色邊框 */
    padding: 16px; /* 內邊距 */
    color: white; /* 白色文字 */
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; /* 字體 */
    cursor: move; /* 顯示移動游標 */
    user-select: none; /* 防止文字選取 */
    overflow: hidden; /* 隱藏溢出內容 */
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3); /* 陰影效果 */
  }

  /* 滑鼠懸停時的效果 */
  .widget-container:hover {
    border-color: rgba(255, 255, 255, 0.2); /* 邊框更亮 */
  }
</style> 