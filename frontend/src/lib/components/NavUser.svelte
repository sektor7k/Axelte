<script lang="ts">
    import LogOut from "@lucide/svelte/icons/log-out";
    import Settings from "@lucide/svelte/icons/settings";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
    import AvatarUser from "./AvatarUser.svelte";
    import { User } from "@lucide/svelte";
    import axios from "axios";
    import { toast } from "svelte-sonner";
    import { goto } from "$app/navigation";

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
    <DropdownMenu.Trigger class="">
        <AvatarUser avatarurl={user.avatar}/>
    </DropdownMenu.Trigger>
    <DropdownMenu.Content class="w-56">
        <DropdownMenu.Group>
            <DropdownMenu.GroupHeading class="text-sm font-normal text-gray-400"
                >{user.username}</DropdownMenu.GroupHeading
            >
            <DropdownMenu.GroupHeading class="text-sm font-normal text-gray-400"
                >{user.email}</DropdownMenu.GroupHeading
            >
            <DropdownMenu.Separator />
            <DropdownMenu.Group>
                <DropdownMenu.Item onclick={() => goto(`/profile/${user.username}`)}>
                    <User class="mr-2 size-4" />
                    <span>Profile</span>
                </DropdownMenu.Item>
                <DropdownMenu.Item>
                    <Settings class="mr-2 size-4" />
                    <span>Settings</span>
                </DropdownMenu.Item>
            </DropdownMenu.Group>
            <DropdownMenu.Separator />
            <DropdownMenu.Item onclick={logout}>
                <LogOut class="mr-2 size-4" />
                <span>Log out</span>
            </DropdownMenu.Item>
        </DropdownMenu.Group>
    </DropdownMenu.Content>
</DropdownMenu.Root>
