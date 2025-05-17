<script lang="ts">
    import { goto } from "$app/navigation";
	import * as Avatar from "$lib/components/ui/avatar";
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
	import * as Sidebar from "$lib/components/ui/sidebar";
    import { useSidebar } from "$lib/components/ui/sidebar";
	import BadgeCheck from "@lucide/svelte/icons/badge-check";
	import Bell from "@lucide/svelte/icons/bell";
	import ChevronsUpDown from "@lucide/svelte/icons/chevrons-up-down";
	import CreditCard from "@lucide/svelte/icons/credit-card";
	import LogOut from "@lucide/svelte/icons/log-out";
	import Sparkles from "@lucide/svelte/icons/sparkles";
    import axios from "axios";
    import { toast } from "svelte-sonner";

	let { user }: { user: { username: string; email: string; avatar: string } } = $props();
	const sidebar = useSidebar();

	async function logout() {
        try {
            let response = await axios.post(
                `${import.meta.env.VITE_SERVER_URL}/auth/logout`,
                {},

                {
                    withCredentials: true,
                },
            );
            toast.success("Logout successful!");
            goto("/login");
        } catch (error) {
            toast.error("Logout failed!");
        }
    }
</script>

<Sidebar.Menu>
	<Sidebar.MenuItem>
		<DropdownMenu.Root>
			<DropdownMenu.Trigger>
				{#snippet child({ props })}
					<Sidebar.MenuButton
						size="lg"
						class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
						{...props}
					>
						<Avatar.Root class="h-8 w-8 rounded-lg">
							<Avatar.Image src={user.avatar} alt={user.username} />
							<Avatar.Fallback class="rounded-lg">CN</Avatar.Fallback>
						</Avatar.Root>
						<div class="grid flex-1 text-left text-sm leading-tight">
							<span class="truncate font-semibold">{user.username}</span>
							<span class="truncate text-xs">{user.email}</span>
						</div>
						<ChevronsUpDown class="ml-auto size-4" />
					</Sidebar.MenuButton>
				{/snippet}
			</DropdownMenu.Trigger>
			<DropdownMenu.Content
				class="w-[var(--bits-dropdown-menu-anchor-width)] min-w-56 rounded-lg"
				side={sidebar.isMobile ? "bottom" : "right"}
				align="end"
				sideOffset={4}
			>
				<DropdownMenu.Label class="p-0 font-normal">
					<div class="flex items-center gap-2 px-1 py-1.5 text-left text-sm">
						<Avatar.Root class="h-8 w-8 rounded-lg">
							<Avatar.Image src={user.avatar} alt={user.username} />
							<Avatar.Fallback class="rounded-lg">CN</Avatar.Fallback>
						</Avatar.Root>
						<div class="grid flex-1 text-left text-sm leading-tight">
							<span class="truncate font-semibold">{user.username}</span>
							<span class="truncate text-xs">{user.email}</span>
						</div>
					</div>
				</DropdownMenu.Label>
				<DropdownMenu.Separator />
				<DropdownMenu.Group>
					<DropdownMenu.Item onSelect={() => goto(`/profile/${user.username}`)}>
						<BadgeCheck />
						Account
					</DropdownMenu.Item>
					<DropdownMenu.Item>
						<CreditCard />
						Billing
					</DropdownMenu.Item>
					<DropdownMenu.Item>
						<Bell />
						Notifications
					</DropdownMenu.Item>
				</DropdownMenu.Group>
				<DropdownMenu.Separator />
				<DropdownMenu.Item onSelect={logout}>
					<LogOut />
					Log out
				</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</Sidebar.MenuItem>
</Sidebar.Menu>