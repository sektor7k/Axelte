
<script lang="ts">
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import AppSidebar from "./_components/app-sidebar.svelte";
   
</script>
   
<Sidebar.Provider>
    <div class="flex min-h-screen">
        <AppSidebar />
        <Sidebar.Trigger />
        <main class="">    
            <slot />  
        </main>
    </div>
</Sidebar.Provider>