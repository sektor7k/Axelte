<script lang="ts">
    import LogOut from "@lucide/svelte/icons/log-out";
    import Settings from "@lucide/svelte/icons/settings";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
    import AvatarUser from "./AvatarUser.svelte";
    import { BadgeCheck, Bell, ChevronsUpDown, CreditCard, User } from "@lucide/svelte";
    import axios from "axios";
    import { toast } from "svelte-sonner";
    import { goto } from "$app/navigation";
    import * as Avatar  from "./ui/avatar";

    export let user;


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

<DropdownMenu.Root>
    <DropdownMenu.Trigger>
        <div
            class="flex items-center gap-2 px-1 py-1.5 text-left text-sm rounded-lg hover:bg-sidebar-accent hover:text-sidebar-accent-foreground"
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
        </div>
    </DropdownMenu.Trigger>
    <DropdownMenu.Content
        class="w-[var(--bits-dropdown-menu-anchor-width)] min-w-56 rounded-lg"
        side={"bottom"}
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
