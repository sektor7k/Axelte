<script lang="ts">
    import {
        Card,
        CardContent,
        CardHeader,
        CardTitle,
    } from "$lib/components/ui/card";
    import { Button } from "$lib/components/ui/button";
    import axios from "axios";
    import { goto } from "$app/navigation";
    import { toast } from "svelte-sonner";

    export let data: { user: { id: string; username: string; email: string; avatar: string } };
    const { user } = data;


    async function logout() {
        try {
            let response = await axios.post(
                `${import.meta.env.VITE_SERVER_URL}/auth/logout`,
                {},

                {
                    withCredentials: true,
                },
            );
            console.log(response);
            toast.success("Logout successful!");
            goto("/login");
        } catch (error) {
            console.error(error);
            toast.error("Logout failed!");
        }
    }
</script>

<div
    class="flex flex-col gap-4 justify-center items-center min-h-screen bg-gray-50"
>
    <Card class="w-[350px]">
        <CardHeader class="flex flex-col items-center gap-2">
            <img
                src={user.avatar}
                alt="Profile"
                class="w-24 h-24 rounded-full object-cover border-4 border-primary shadow"
            />
            <CardTitle class="text-xl mt-2">{user.username}</CardTitle>
            <span class="text-gray-500 text-sm">{user.email}</span>
        </CardHeader>
        <CardContent class="flex flex-col items-center">
            <Button variant="outline" class="mt-4 w-full"
                >Profili Düzenle</Button
            >
        </CardContent>
    </Card>
    <Button onclick={logout}>Logout</Button>
</div>
