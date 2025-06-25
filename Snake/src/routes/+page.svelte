<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';

  let testResult = '';
  let gameState = '';
  let isLoading = false;

  async function testGameLogic() {
    isLoading = true;
    try {
      const result = await invoke('test_game_logic');
      testResult = `✅ ${result}`;
    } catch (error) {
      testResult = `❌ Error: ${error}`;
    } finally {
      isLoading = false;
    }
  }

  async function getGameState() {
    try {
      const state = await invoke('get_game_state');
      gameState = JSON.stringify(state, null, 2);
    } catch (error) {
      gameState = `Error: ${error}`;
    }
  }

  onMount(() => {
    getGameState();
  });
</script>

<main class="container">
  <h1>🐍 Snake Game - Rust Logic Test</h1>
  
  <div class="test-section">
    <h2>遊戲邏輯測試</h2>
    <button 
      on:click={testGameLogic} 
      disabled={isLoading}
      class="test-button"
    >
      {isLoading ? '測試中...' : '執行遊戲邏輯測試'}
    </button>
    
    {#if testResult}
      <div class="result">
        <pre>{testResult}</pre>
      </div>
    {/if}
  </div>

  <div class="test-section">
    <h2>遊戲狀態</h2>
    <button on:click={getGameState} class="test-button">
      重新載入遊戲狀態
    </button>
    
    {#if gameState}
      <div class="result">
        <h3>當前遊戲狀態：</h3>
        <pre>{gameState}</pre>
      </div>
    {/if}
  </div>

  <div class="info">
    <h2>測試說明</h2>
    <ul>
      <li>✅ 蛇的移動邏輯</li>
      <li>✅ 方向改變邏輯</li>
      <li>✅ 暫停/繼續功能</li>
      <li>✅ 食物生成邏輯</li>
      <li>✅ 遊戲狀態管理</li>
    </ul>
  </div>
</main>

<style>
  .container {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  }

  h1 {
    color: #333;
    text-align: center;
    margin-bottom: 2rem;
  }

  .test-section {
    background: #f8f9fa;
    border: 1px solid #dee2e6;
    border-radius: 8px;
    padding: 1.5rem;
    margin-bottom: 2rem;
  }

  h2 {
    color: #495057;
    margin-top: 0;
  }

  .test-button {
    background: #007bff;
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 4px;
    cursor: pointer;
    font-size: 1rem;
    margin-bottom: 1rem;
  }

  .test-button:hover:not(:disabled) {
    background: #0056b3;
  }

  .test-button:disabled {
    background: #6c757d;
    cursor: not-allowed;
  }

  .result {
    background: white;
    border: 1px solid #ced4da;
    border-radius: 4px;
    padding: 1rem;
    margin-top: 1rem;
  }

  pre {
    margin: 0;
    white-space: pre-wrap;
    word-wrap: break-word;
  }

  .info {
    background: #e7f3ff;
    border: 1px solid #b3d9ff;
    border-radius: 8px;
    padding: 1.5rem;
  }

  .info ul {
    margin: 0;
    padding-left: 1.5rem;
  }

  .info li {
    margin-bottom: 0.5rem;
  }
</style>
