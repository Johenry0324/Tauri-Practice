<script>
    import { invoke } from '@tauri-apps/api/core';
</script>

<div data-tauri-drag-region class="titlebar">
    <div class="titlebar-left">
        <img src="/logo.svg" alt="logo" />
        <span class="titlebar-title">Statistic</span>
    </div>
    <div class="titlebar-buttons">
        <button type="button" class="titlebar-button" on:click={() => invoke('minimize_window')}>
            <img src="/minimize.svg" alt="minimize" />
        </button>
        <button type="button" class="titlebar-button" on:click={() => invoke('toggle_maximize_window')}>
            <img src="/maximize.svg" alt="maximize" />
        </button>
        <button type="button" class="titlebar-button" on:click={() => invoke('close_window')}>
            <img src="/close.svg" alt="close" />
        </button>
    </div>
</div>

<style>
  :global(body) {
    background: #181a20;
    color: #e0e0e0;
    margin: 0;
    font-family: 'Segoe UI', 'Arial', sans-serif;
    min-height: 100vh;
    overflow: hidden;
  }
  .titlebar {
    height: 36px;
    background: #2b2c31;
    user-select: none;
    display: flex;
    justify-content: space-between;
    align-items: center;
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    z-index: 1000;
  }
  .titlebar-left {
    display: flex;
    align-items: center;
    margin-left: 12px;
  }
  .titlebar-buttons {
    display: flex;
    align-items: center;
  }
  .titlebar-button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    width: 40px;
    height: 32px;
    cursor: pointer;
    filter: invert(80%);
  }
  .titlebar-button:hover {
    background: #23272f;
  }
  .titlebar-title {
    color: #e0e0e0;
    font-size: 1.1rem;
    font-weight: 600;
    margin-left: 10px;
    letter-spacing: 1px;
    user-select: none;
  }
</style>