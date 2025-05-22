<script lang="ts">
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import AppSidebar from "../_components/app-sidebar.svelte";
   
</script>
   
<Sidebar.Provider>
    <div class="flex min-h-screen">
        <AppSidebar />
        <main class="flex-1 p-4 md:ml-64">
            <slot />  
        </main>
    </div>
</Sidebar.Provider>