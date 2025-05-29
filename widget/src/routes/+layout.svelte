<script lang="ts">
  import { onMount } from 'svelte';
  import { isLoading } from 'svelte-i18n';
  import '$lib/i18n';

  interface Props {
    children: any;
  }

  let { children }: Props = $props();
  let ready = $state(false);

  onMount(() => {
    const unsubscribe = isLoading.subscribe((loading) => {
      if (!loading) {
        ready = true;
      }
    });

    return unsubscribe;
  });
</script>

{#if ready}
  {@render children()}
{:else}
  <div class="loading-screen">
    <div class="spinner"></div>
  </div>
{/if}

<style>
  .loading-screen {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100vw;
    height: 100vh;
    background: rgba(0, 0, 0, 0.8);
  }

  .spinner {
    width: 32px;
    height: 32px;
    border: 3px solid rgba(255, 255, 255, 0.3);
    border-top: 3px solid #ffffff;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }
</style> 