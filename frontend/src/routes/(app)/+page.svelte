<script lang="ts">
    import { Button } from "$lib/components/ui/button";
    import { Card, CardContent, CardHeader, CardTitle } from "$lib/components/ui/card";
    import { Plus } from "lucide-svelte";
    import { goto } from "$app/navigation";
    import MainNavbar from "$lib/components/MainNavbar.svelte";

    export let data;
    const { workspaces, user } = data;
    
</script>
<MainNavbar user={data.user}/>

    
<div class="p-6">
    <h1 class="text-2xl font-bold mb-6">Your Workspaces</h1>
  
    {#if data.workspaces.length > 0}
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
        {#each data.workspaces as ws}
          <a
            href={`/workspace/${ws.id}`}
            class="block p-4 bg-card rounded-lg shadow hover:shadow-md transition"
          >
            <h2 class="text-lg font-semibold">{ws.name}</h2>
            {#if ws.description}
              <p class="mt-1 text-sm text-muted-foreground">
                {ws.description}
              </p>
            {/if}
          </a>
        {/each}
  
       
        <a
          href="#"
          class="flex items-center justify-center p-4 border-2 border-dashed rounded-lg hover:bg-secondary transition"
        >
          <Plus class="h-6 w-6 mr-2 text-accent" />
          <span class="text-accent font-medium">New Workspace</span>
        </a>
      </div>
    {:else}
      <div class="text-center py-20 text-muted-foreground">
        <p>You don't have any workspaces yet.</p>
        <a href="#" class="inline-flex items-center mt-4 px-4 py-2 bg-primary text-primary-foreground rounded">
          <Plus class="h-5 w-5 mr-2" /> Create your first workspace
        </a>
      </div>
    {/if}
  </div>  