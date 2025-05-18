<script lang="ts">
    import AppSidebar from './_components/app-sidebar.svelte';
    import * as Breadcrumb from '$lib/components/ui/breadcrumb';
    import { Separator } from '$lib/components/ui/separator';
    import * as Sidebar from '$lib/components/ui/sidebar';
    import { pages } from '$lib/stores/pages';
    import { page } from '$app/stores';
    import { activeUsers } from '$lib/stores/awareness';
    import { onMount } from 'svelte';

    export let data;
    let { workspaces, pages: initialPages, user } = data;

    onMount(() => {
    activeUsers.set([]);
  });
    
    // Initialize pages store with server data
    $: pages.set(initialPages);

    // Aktif workspace ve page'ı bul
    $: currentWorkspace = workspaces.find(ws => ws.id === $page.params.workspaceId);
    $: currentPage = initialPages.find(p => p.id === $page.params.pageId);
</script>
  
<Sidebar.Provider>
  <div class="flex h-full w-full">
    <!-- 1) Sol: AppSidebar -->
    <AppSidebar workspaces={workspaces} pages={pages} user={user} />

    <!-- 2) Sağ: Header + İçerik -->
    <div class="flex flex-1 flex-col ">
      <!-- Navbar / Breadcrumb -->
      <header
        class="sticky top-0 z-10 flex h-16 shrink-0 items-center gap-2 border-b bg-background px-4 transition-[height] ease-linear group-has-[[data-collapsible=icon]]/sidebar-wrapper:h-12"
      >
        <Sidebar.Trigger class="-ml-1" />
        <Separator orientation="vertical" class="mr-2 h-4" />

        <Breadcrumb.Root>
          <Breadcrumb.List class="flex items-center space-x-1">
            <Breadcrumb.Item class="hidden md:block">
              <Breadcrumb.Link href="/">Dashboard</Breadcrumb.Link>
            </Breadcrumb.Item>
            <Breadcrumb.Separator class="hidden md:block" />
            <Breadcrumb.Item>
              <Breadcrumb.Link href={`/workspace/${$page.params.workspaceId}`}>
                {currentWorkspace ? currentWorkspace.name : 'Workspace'}
              </Breadcrumb.Link>
            </Breadcrumb.Item>
            <Breadcrumb.Separator class="hidden md:block" />
            <Breadcrumb.Item>
              <Breadcrumb.Page>
                {currentPage ? currentPage.title : 'Current Page'}
              </Breadcrumb.Page>
            </Breadcrumb.Item>
          </Breadcrumb.List>
        </Breadcrumb.Root>
        {#if $activeUsers.length}
    <div class="flex items-center gap-2">
      {#each $activeUsers as u}
        <img src={u.avatar} alt={u.name} class="w-6 h-6 rounded-full" title={u.name} style="border:2px solid {u.color}" />
      {/each}
    </div>
  {/if}
      </header>

      <!-- Dinamik Sayfa İçeriği -->
      <main class="h-full w-full overflow-hidden">
        <slot />
      </main>
    </div>
  </div>
</Sidebar.Provider>
  
<style>
  /* mobile'de sidebar collapse animasyonu için (shadcn class'ları örnek) */
  :global(.sidebar-wrapper[data-collapsible='icon']) + div header {
    /* örnek: collapsed state'de header height = 3rem */
  }
</style>
  