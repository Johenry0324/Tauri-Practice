<script lang="ts">
  // 定義元件的 props 型別
  interface Props {
    title: string; // 區塊標題
    value?: string | number; // 顯示的數值（可選）
    unit?: string; // 數值單位（可選）
    icon?: string; // 圖示（可選）
    loading?: boolean; // 是否顯示載入狀態
    children?: any; // 自訂內容（使用 Svelte 5 snippet）
  }

  // 解構 props，設定預設值
  let { title, value, unit = '', icon, loading = false, children }: Props = $props();
</script>

<!-- 資訊區塊容器 -->
<div class="info-block">
  <!-- 區塊標題區域 -->
  <div class="info-header">
    <!-- 如果有圖示則顯示 -->
    {#if icon}
      <span class="icon">{icon}</span>
    {/if}
    <!-- 顯示標題 -->
    <span class="title">{title}</span>
  </div>
  
  <!-- 區塊內容區域 -->
  <div class="info-content">
    <!-- 載入狀態 -->
    {#if loading}
      <div class="loading">
        <div class="spinner"></div>
      </div>
    <!-- 有數值時顯示數值和單位 -->
    {:else if value !== undefined}
      <span class="value">{value}</span>
      {#if unit}
        <span class="unit">{unit}</span>
      {/if}
    <!-- 有自訂內容時顯示自訂內容 -->
    {:else if children}
      {@render children()}
    {/if}
  </div>
</div>

<style>
  .info-block {
    background: rgba(255, 255, 255, 0.05); /* 淺色半透明背景 */
    border-radius: 8px; /* 圓角 */
    padding: 12px; /* 內邊距 */
    margin-bottom: 8px; /* 底部外邊距 */
    border: 1px solid rgba(255, 255, 255, 0.1); /* 淺色邊框 */
    transition: all 0.2s ease; /* 動畫過渡 */
  }

  /* 滑鼠懸停效果 */
  .info-block:hover {
    background: rgba(255, 255, 255, 0.08); /* 背景稍亮 */
    border-color: rgba(255, 255, 255, 0.2); /* 邊框更亮 */
  }

  .info-header {
    display: flex; /* 彈性佈局 */
    align-items: center; /* 垂直置中 */
    gap: 8px; /* 元素間距 */
    margin-bottom: 8px; /* 底部外邊距 */
  }

  .icon {
    font-size: 14px; /* 圖示大小 */
    opacity: 0.8; /* 透明度 */
  }

  .title {
    font-size: 12px; /* 標題字體大小 */
    opacity: 0.7; /* 透明度 */
    font-weight: 500; /* 字體粗細 */
  }

  .info-content {
    display: flex; /* 彈性佈局 */
    align-items: baseline; /* 基線對齊 */
    gap: 4px; /* 元素間距 */
  }

  .value {
    font-size: 18px; /* 數值字體大小 */
    font-weight: 600; /* 字體粗細 */
    color: #ffffff; /* 純白色 */
  }

  .unit {
    font-size: 12px; /* 單位字體大小 */
    opacity: 0.6; /* 透明度 */
  }

  /* 載入動畫容器 */
  .loading {
    display: flex; /* 彈性佈局 */
    align-items: center; /* 垂直置中 */
    justify-content: center; /* 水平置中 */
    height: 24px; /* 高度 */
  }

  /* 載入動畫轉輪 */
  .spinner {
    width: 16px; /* 寬度 */
    height: 16px; /* 高度 */
    border: 2px solid rgba(255, 255, 255, 0.3); /* 淺色邊框 */
    border-top: 2px solid #ffffff; /* 頂部白色邊框 */
    border-radius: 50%; /* 圓形 */
    animation: spin 1s linear infinite; /* 旋轉動畫 */
  }

  /* 旋轉動畫定義 */
  @keyframes spin {
    0% { transform: rotate(0deg); } /* 起始角度 */
    100% { transform: rotate(360deg); } /* 結束角度 */
  }
</style> 