<script lang="ts">
  import Editor from '$lib/components/Editor.svelte';
  import axios from 'axios';
  export let data;
 
      $: page = data.page;
      const{user,} = data;

  let saveTimeout: ReturnType<typeof setTimeout>;
  let isSaving = false;
  let lastSaved = '';


  
  function handleUpdate(event: CustomEvent<string>) {
    const json = event.detail;
    
    clearTimeout(saveTimeout);
    isSaving = true;
    
    saveTimeout = setTimeout(() => {
      console.log("istek");
      axios.post(`${import.meta.env.VITE_SERVER_URL}/api/update-page`, 
      {
          id: page.id,
          content: json
      },
      { 
          withCredentials: true
      })
      .then(() => {
        isSaving = false;
        lastSaved = new Date().toLocaleTimeString();
      })
      .catch(err => {
        console.error("Kaydetme hatası:", err);
        isSaving = false;
      });
    }, 3000);
  }
</script>

<style>
  .page-container {
    width: 100%;
    min-height: 100vh;
    background-color: #0a0a0a;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 20px 0;
  }
  
  .status-indicator {
    position: fixed;
    bottom: 16px;
    right: 16px;
    padding: 6px 12px;
    background-color: #1e1e1e;
    border-radius: 4px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
    font-size: 12px;
    color: #9ca3af;
    display: flex;
    align-items: center;
    gap: 6px;
  }
  
  .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
  }
  
  .saving {
    background-color: #fbbf24;
  }
  
  .saved {
    background-color: #10b981;
  }
</style>

<div class="page-container">
  <div class="w-full max-w-4xl flex-1">
    {#key page.id}
    <Editor page={page} user={user} on:update={handleUpdate} />
    {/key}
  </div>
  
  <div class="status-indicator">
    {#if isSaving}
      <div class="dot saving"></div>
      <span>Saving...</span>
    {:else}
      <div class="dot saved"></div>
      <span>Last saved: {lastSaved || ''}</span>
    {/if}
  </div>
</div>