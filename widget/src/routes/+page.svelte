<script lang="ts">
  import { onMount } from 'svelte';
  import { _ } from 'svelte-i18n';
  import WidgetContainer from '$lib/components/ui/WidgetContainer.svelte';
  import InfoBlock from '$lib/components/ui/InfoBlock.svelte';
  import '$lib/i18n';

  let currentTime = $state('');
  let currentDate = $state('');

  // 更新時間
  function updateTime() {
    const now = new Date();
    currentTime = now.toLocaleTimeString('zh-TW', { 
      hour12: false,
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit'
    });
    currentDate = now.toLocaleDateString('zh-TW', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      weekday: 'long'
    });
  }

  onMount(() => {
    updateTime();
    const interval = setInterval(updateTime, 1000);
    return () => clearInterval(interval);
  });
</script>

{#snippet timeContent()}
  <div class="time-display">
    <div class="time">{currentTime}</div>
    <div class="date">{currentDate}</div>
  </div>
{/snippet}

{#snippet placeholderContent()}
  <div class="placeholder-content">
    <p>您可以在這裡添加自訂功能</p>
    <p>支援多語系：{$_('settings.language')}</p>
  </div>
{/snippet}

{#snippet widgetContent()}
  <div class="widget-header">
    <h1 class="widget-title">{$_('app.title')}</h1>
  </div>

  <div class="widget-content">
    <!-- 時間區塊 -->
    <InfoBlock title="時間" icon="🕐" children={timeContent} />

    <!-- 範例資訊區塊 -->
    <InfoBlock title="範例數據" value="42" unit="%" icon="📊" />
    
    <InfoBlock title="載入中範例" loading={true} icon="⚡" />
    
    <!-- 佔位區塊 -->
    <InfoBlock title={$_('widget.placeholder')} icon="🔧" children={placeholderContent} />
  </div>
{/snippet}

<WidgetContainer children={widgetContent} />

<style>
  .widget-header {
    text-align: center;
    margin-bottom: 16px;
    padding-bottom: 12px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }

  .widget-title {
    font-size: 16px;
    font-weight: 600;
    margin: 0;
    opacity: 0.9;
  }

  .widget-content {
    flex: 1;
    overflow-y: auto;
  }

  .time-display {
    text-align: center;
  }

  .time {
    font-size: 24px;
    font-weight: 700;
    color: #ffffff;
    margin-bottom: 4px;
  }

  .date {
    font-size: 12px;
    opacity: 0.7;
  }

  .placeholder-content {
    font-size: 12px;
    opacity: 0.8;
    line-height: 1.4;
  }

  .placeholder-content p {
    margin: 4px 0;
  }
</style>
