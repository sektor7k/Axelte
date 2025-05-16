<script lang="ts">
    import FileText from "@lucide/svelte/icons/file-text";
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import { useSidebar } from "$lib/components/ui/sidebar/index.js";
    import { Button } from "$lib/components/ui/button";
    import { LogOut, Plus } from "@lucide/svelte";

    const sidebar = useSidebar();

    const pageItems = [
        {
            title: "Home",
            url: "#",
        },
        {
            title: "Pages",
            url: "#",
        },
        {
            title: "Settings",
            url: "#",
        },
    ];
</script>



<Sidebar.Root>
    <Sidebar.Content>
        <Sidebar.Menu>
            <Sidebar.MenuItem>
                <Sidebar.GroupLabel>Pages</Sidebar.GroupLabel>
                <Sidebar.MenuSub>
                    {#each pageItems as item (item.title)}
                        <Sidebar.MenuSubItem>
                            <Sidebar.MenuButton>
                                {#snippet child({ props })}
                                    <a href={item.url} {...props}>
                                        <FileText />
                                        <span>{item.title}</span>
                                    </a>
                                {/snippet}
                            </Sidebar.MenuButton>
                        </Sidebar.MenuSubItem>
                    {/each}
                    <Sidebar.MenuSubItem>
                        <Sidebar.MenuButton>
                            <Plus />
                            <span>New Page</span>
                        </Sidebar.MenuButton>
                    </Sidebar.MenuSubItem>
                </Sidebar.MenuSub>
            </Sidebar.MenuItem>
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
