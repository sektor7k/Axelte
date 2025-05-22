<script lang="ts">
    import Calendar from "@lucide/svelte/icons/calendar";
    import House from "@lucide/svelte/icons/house";
    import Inbox from "@lucide/svelte/icons/inbox";
    import Search from "@lucide/svelte/icons/search";
    import Settings from "@lucide/svelte/icons/settings";
    import Menu from "@lucide/svelte/icons/menu";
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import { useSidebar } from "$lib/components/ui/sidebar/index.js";
    import { Button } from "$lib/components/ui/button";
    import { LogOut } from "@lucide/svelte";
    
    const sidebar = useSidebar();
    
    // Menu items.
    const items = [
     {
      title: "Home",
      url: "#",
      icon: House,
     },
     {
      title: "Inbox",
      url: "#",
      icon: Inbox,
     },
     {
      title: "Calendar",
      url: "#",
      icon: Calendar,
     },
     {
      title: "Search",
      url: "#",
      icon: Search,
     },
     {
      title: "Settings",
      url: "#",
      icon: Settings,
     },
    ];
</script>

<Button
    class="fixed top-4 left-4 z-50 md:hidden" variant="ghost"
    onclick={() => sidebar.toggle()}
>
    <Menu class="h-6 w-6" />
</Button>

<Sidebar.Root>
    <Sidebar.Content>
       
                <Sidebar.Menu>
                    {#each items as item (item.title)}
                        <Sidebar.MenuItem>
                            <Sidebar.MenuButton>
                                {#snippet child({ props })}
                                    <a href={item.url} {...props}>
                                        <item.icon />
                                        <span>{item.title}</span>
                                    </a>
                                {/snippet}
                            </Sidebar.MenuButton>
                        </Sidebar.MenuItem>
                    {/each}
                </Sidebar.Menu>
    </Sidebar.Content>
    <Sidebar.Footer>
        <Button variant="ghost" class="w-full">
            <LogOut class="h-6 w-6" />
            <span>Logout</span>
        </Button>
    </Sidebar.Footer>
</Sidebar.Root>

<style>
    :global(.sidebar-root) {
        @apply fixed inset-y-0 left-0 z-40 w-64 transform transition-transform duration-200 ease-in-out;
    }

    @media (max-width: 768px) {
        :global(.sidebar-root:not([data-state="open"])) {
            @apply -translate-x-full;
        }
    }
</style>