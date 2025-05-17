<script lang="ts">
	import * as Sidebar from "$lib/components/ui/sidebar";
	import { FileText, Plus } from "@lucide/svelte";
	import * as Dialog from "$lib/components/ui/dialog";
	import { Input } from "$lib/components/ui/input";
	import { Label } from "$lib/components/ui/label/index.js";
	import { Button } from "$lib/components/ui/button";
	import axios from "axios";
	import { page } from "$app/stores";
	import { toast } from "svelte-sonner";
	import { goto } from "$app/navigation";
	import { pages } from '$lib/stores/pages';

	let workspaceId = $page.params.workspaceId;
	let pageName = $state("");
	let isDialogOpen = $state(false);

	async function handleCreatePage() {
		try {
			const response = await axios.post(`${import.meta.env.VITE_SERVER_URL}/api/create-page`, {
				title: pageName,
				workspace_id: workspaceId,
			},
			{
				withCredentials: true,
			}
		);
		isDialogOpen = false;
		
		// Add the new page to the store
		pages.addPage({
			id: response.data.page_id,
			title: pageName
		});

		await goto(`/workspace/${workspaceId}/page/${response.data.page_id}`, {
			replaceState: true,
			keepFocus: true
		});
		toast.success("Page created successfully");
		} catch (error) {
			console.error(error);
			toast.error("Failed to create page");
		}
	}

	function navigateToPage(pageId: string) {
		goto(`/workspace/${workspaceId}/page/${pageId}`, {
			replaceState: true,
			keepFocus: true
		});
	}
</script>

<Sidebar.Group>
	<Sidebar.GroupLabel>Pages</Sidebar.GroupLabel>
	<Sidebar.Menu>
		{#each $pages as page (page.id)}
			<Sidebar.MenuItem>
				<Sidebar.MenuButton onclick={() => navigateToPage(page.id)}>
					<FileText />
					<span>{page.title}</span>
				</Sidebar.MenuButton>
			</Sidebar.MenuItem>
		{/each}
		<Sidebar.MenuItem>
			<Sidebar.MenuButton>
				<Dialog.Root bind:open={isDialogOpen}>
					<Dialog.Trigger>
						<div class="flex w-full items-center gap-2 overflow-hidden">
							<Plus />
							<span>New Page</span>
						</div>
					</Dialog.Trigger>
					<Dialog.Content class="sm:max-w-[425px]">
						<Dialog.Header>
							<Dialog.Title>Create Page</Dialog.Title>
							<Dialog.Description>
								Create a new page for your workspace.
							</Dialog.Description>
						</Dialog.Header>
						<div class="grid gap-4 py-4">
							<div class="grid grid-cols-4 items-center gap-4">
								<Label for="title" class="text-right">Page Name:</Label>
								<Input
									id="title"
									class="col-span-3"
									bind:value={pageName}
								/>
							</div>
						</div>
						<Dialog.Footer>
							<Button type="submit" onclick={handleCreatePage}>Create</Button>
						</Dialog.Footer>
					</Dialog.Content>
				</Dialog.Root>
			</Sidebar.MenuButton>
		</Sidebar.MenuItem>
	</Sidebar.Menu>
</Sidebar.Group>
