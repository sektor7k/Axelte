<script lang="ts">
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
	import * as Sidebar from "$lib/components/ui/sidebar";
	import { useSidebar } from "$lib/components/ui/sidebar";
	import { page } from "$app/stores";
	import { goto } from "$app/navigation";
	import ChevronsUpDown from "@lucide/svelte/icons/chevrons-up-down";
	import Plus from "@lucide/svelte/icons/plus";
    import { Check } from "@lucide/svelte";

	interface ButtonProps {
		[key: string]: any;
	}

	let { workspaces = []} = $props();
	const sidebar = useSidebar();
	
	// URL'deki workspace ID'sine göre active workspace'i bul
	 let activeWorkspace = $derived(workspaces.find((ws: { id: string; }) => ws.id === $page.params.workspaceId) || workspaces[0]);
</script>



<Sidebar.Menu>
	<Sidebar.MenuItem>
		<DropdownMenu.Root>
			<DropdownMenu.Trigger>
				{#snippet child({ props }: { props: ButtonProps })}
					<Sidebar.MenuButton
						{...props}
						size="lg"
						class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
					>
						<div
							class="bg-sidebar-primary text-sidebar-primary-foreground flex aspect-square size-8 items-center justify-center rounded-lg"
						>
							<span class="text-lg font-bold">{activeWorkspace.name[0]}</span>
						</div>
						<div class="grid flex-1 text-left text-sm leading-tight">
							<span class="truncate font-semibold">
								{activeWorkspace.name}
							</span>
							<span class="truncate text-xs">{activeWorkspace.description}</span>
						</div>
						<ChevronsUpDown class="ml-auto" />
					</Sidebar.MenuButton>
				{/snippet}
			</DropdownMenu.Trigger>
			<DropdownMenu.Content
				class="w-[var(--bits-dropdown-menu-anchor-width)] min-w-56 rounded-lg"
				align="start"
				side={sidebar.isMobile ? "bottom" : "right"}
				sideOffset={4}
			>
				<DropdownMenu.Label class="text-muted-foreground text-xs">Workspaces</DropdownMenu.Label>
				{#each workspaces as workspace, index (workspace.id)}
					<DropdownMenu.Item 
						onSelect={() => window.location.href = `/workspace/${workspace.id}`} 
						class="gap-2 p-2"
					>
						<div class="flex size-6 items-center justify-center rounded-sm border">
							<span class="text-sm font-bold">{workspace.name[0]}</span>
						</div>
						{workspace.name}
						{#if workspace.id === activeWorkspace.id }
							<Check class="ml-auto" />
						{/if}
					</DropdownMenu.Item>
				{/each}
				<DropdownMenu.Separator />
				<DropdownMenu.Item class="gap-2 p-2">
					<div
						class="bg-background flex size-6 items-center justify-center rounded-md border"
					>
						<Plus class="size-4" />
					</div>
					<div class="text-muted-foreground font-medium">Add workspace</div>
				</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</Sidebar.MenuItem>
</Sidebar.Menu>