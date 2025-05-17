<script lang="ts">
    import { MoreVertical, Pencil, Trash2 } from "@lucide/svelte";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import * as Dialog from "$lib/components/ui/dialog";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import { Button } from "$lib/components/ui/button";
    import { toast } from "svelte-sonner";
    import axios from "axios";
    import { pages } from '$lib/stores/pages';
    import { goto } from "$app/navigation";
    import { page } from "$app/stores";

    const { pageId, pageTitle } = $props<{
        pageId: string;
        pageTitle: string;
    }>();

    let isRenameDialogOpen = $state(false);
    let isDeleteDialogOpen = $state(false);
    let newTitle = $state(pageTitle);
    let workspaceId = $page.params.workspaceId;

    async function handleRename() {
        try {
            await axios.post(`${import.meta.env.VITE_SERVER_URL}/api/rename-page`, {
                id: pageId,
                title: newTitle
            }, {
                withCredentials: true
            });
            pages.updatePage(pageId, { title: newTitle });
            isRenameDialogOpen = false;
            toast.success("Page renamed successfully");
        } catch (error) {
            console.error(error);
            toast.error("Failed to rename page");
        }
    }

    async function handleDelete() {
        try {
            await axios.post(`${import.meta.env.VITE_SERVER_URL}/api/delete-page`, {
                id: pageId
            }, {
                withCredentials: true
            });
            pages.removePage(pageId);
            isDeleteDialogOpen = false;
            toast.success("Page deleted successfully");
            
            // If we're on the deleted page, redirect to workspace
            if ($page.params.pageId === pageId) {
                goto(`/workspace/${workspaceId}`);
            }
        } catch (error) {
            console.error(error);
            toast.error("Failed to delete page");
        }
    }
</script>

<DropdownMenu.Root>
    <DropdownMenu.Trigger class="p-1 hover:bg-sidebar-accent rounded-md">
        <MoreVertical class="h-4 w-4" />
    </DropdownMenu.Trigger>
    <DropdownMenu.Content>
        <DropdownMenu.Item onclick={() => isRenameDialogOpen = true}>
            <Pencil class="mr-2 h-4 w-4" />
            <span>Rename</span>
        </DropdownMenu.Item>
        <DropdownMenu.Item onclick={() => isDeleteDialogOpen = true}>
            <Trash2 class="mr-2 h-4 w-4" />
            <span>Delete</span>
        </DropdownMenu.Item>
    </DropdownMenu.Content>
</DropdownMenu.Root>

<!-- Rename Dialog -->
<Dialog.Root bind:open={isRenameDialogOpen}>
    <Dialog.Content class="sm:max-w-[425px]">
        <Dialog.Header>
            <Dialog.Title>Rename Page</Dialog.Title>
            <Dialog.Description>
                Enter a new name for your page.
            </Dialog.Description>
        </Dialog.Header>
        <div class="grid gap-4 py-4">
            <div class="grid grid-cols-4 items-center gap-4">
                <Label for="title" class="text-right">New Name:</Label>
                <Input
                    id="title"
                    class="col-span-3"
                    bind:value={newTitle}
                />
            </div>
        </div>
        <Dialog.Footer>
            <Button type="submit" onclick={handleRename}>Rename</Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>

<!-- Delete Confirmation Dialog -->
<Dialog.Root bind:open={isDeleteDialogOpen}>
    <Dialog.Content class="sm:max-w-[425px]">
        <Dialog.Header>
            <Dialog.Title>Delete Page</Dialog.Title>
            <Dialog.Description>
                Are you sure you want to delete this page? This action cannot be undone.
            </Dialog.Description>
        </Dialog.Header>
        <Dialog.Footer>
            <Button variant="destructive" onclick={handleDelete}>Delete</Button>
            <Button variant="outline" onclick={() => isDeleteDialogOpen = false}>Cancel</Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root> 