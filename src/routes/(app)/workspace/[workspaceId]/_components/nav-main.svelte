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
	import PageActions from "./page-actions.svelte";

	let workspaceId = $page.params.workspaceId;
	let { userRole } = $props<{ userRole: 'owner' | 'editor' | 'viewer' }>();
	let pageName = $state("");
	let isDialogOpen = $state(false);

	const currentPage = $derived(() => $pages.find((p: { id: string }) => p.id === $page.params.pageId));
	const canEdit = $derived(userRole === 'owner' || userRole === 'editor');


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
		pages.addPage({
			id: response.data.page_id,
			title: pageName
		});
		isDialogOpen = false;
		pageName = "";
		
		// Add the new page to the store
		

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
		console.log(userRole);
	}

	function truncateTitle(title: string, maxLength: number = 20) {
		return title.length > maxLength ? title.slice(0, maxLength) + '...' : title;
	}
</script>

<Sidebar.Group>
	<Sidebar.GroupLabel>Pages</Sidebar.GroupLabel>
	<Sidebar.Menu>
		{#each $pages as page (page.id)}
			<Sidebar.MenuItem>
				<Sidebar.MenuButton
					class={currentPage()?.id === page.id ? "bg-sidebar-accent text-sidebar-accent-foreground" : ""}
					onclick={() => navigateToPage(page.id)}
				>
					<div class="flex items-center justify-between w-full">
						<div class="flex items-center gap-2">
							<FileText class="w-5 h-5"/>
							<span>{truncateTitle(page.title, 20)}</span>
						</div>
						{#if canEdit}
						<PageActions
							pageId={page.id}
							pageTitle={page.title}
						/>
						{/if}
					</div>
				</Sidebar.MenuButton>
			</Sidebar.MenuItem>
		{/each}
		{#if canEdit}
		<Sidebar.MenuItem>
			<Sidebar.MenuButton class="w-full">
				<Dialog.Root bind:open={isDialogOpen}>
					<Dialog.Trigger class="w-full" disabled={!canEdit}>
						
						<div class="flex items-center justify-start gap-2 overflow-hidden">
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
		{/if}
	</Sidebar.Menu>
</Sidebar.Group>
