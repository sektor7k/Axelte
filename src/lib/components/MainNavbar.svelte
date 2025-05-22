<script lang="ts">
    import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { PlusIcon, X } from "@lucide/svelte";
    import NavUser from "./NavUser.svelte";
    import { Textarea } from "./ui/textarea";
    import axios from "axios";
    import { toast } from "svelte-sonner";
    import { goto } from "$app/navigation";

    export let user;
    let emailInput = "";
    let emails: string[] = [];
    
    let wname = "";
    let wdescription = "";
    function addEmail() {
        if (emailInput && !emails.includes(emailInput)) {
            emails = [...emails, emailInput];
            emailInput = "";
        }
    }
    function removeEmail(email: string) {
        emails = emails.filter(e => e !== email);
    }
    async function createWorkspace() {
        try {
            const response = await axios.post(`${import.meta.env.VITE_SERVER_URL}/api/createworkspace`, {
                name: wname,
                description: wdescription,
                emails: emails
            },{
                withCredentials: true
            });
            goto(`/workspace/${response.data.workspaceId}`);
            toast.success("Workspace created successfully");
            console.log(response);
        } catch (error) {
            console.error("Error creating workspace:", error);
            toast.error("Error creating workspace");
        }
    }
</script>

<nav
    class="fixed top-0 left-0 w-full h-16 border-b flex items-center justify-between px-6 z-50 shadow-sm"
>
    <div class="flex-1 flex justify-center">
        <Dialog.Root>
            <Dialog.Trigger class={buttonVariants({ variant: "default" })}>
                    <PlusIcon class="w-4 h-4 mr-2" />Create Workspace
                
            </Dialog.Trigger>
            <Dialog.Content class="sm:max-w-[425px]">
                <Dialog.Header>
                    <Dialog.Title>Create Workspace</Dialog.Title>
                    <Dialog.Description>  
                        Create a new workspace to start collaborating with your team.
                    </Dialog.Description>
                </Dialog.Header>
                <div class="flex flex-col gap-4 py-4 items-start">
                    <div class="flex flex-col gap-2 w-full">
                        <Label for="wname" class="text-left">Workspace Name</Label>
                        <Input id="wname" class="w-full" bind:value={wname} />
                    </div>
                    <div class="flex flex-col gap-2 w-full">
                        <Label for="wdescription" class="text-left">Workspace Description</Label>
                        <Textarea id="wdescription" class="w-full" bind:value={wdescription} />
                    </div>
                    <div class="flex flex-col gap-2 w-full">
                        <Label for="emails" class="text-left">Add User Emails</Label>
                        <div class="flex gap-2 w-full">
                            <Input
                                id="emails"
                                type="email"
                                class="w-full"
                                bind:value={emailInput}
                                placeholder="user@example.com"
                                onkeydown={(e) => e.key === 'Enter' && (e.preventDefault(), addEmail())}
                            />
                            <Button type="button" onclick={addEmail} variant="secondary">Ekle</Button>
                        </div>
                        <div class="flex flex-wrap gap-2 mt-2">
                            {#each emails as email}
                                <span class="flex items-center bg-gray-900 rounded px-2 py-1 text-sm">
                                    {email}
                                    <button type="button" class="ml-1" on:click={() => removeEmail(email)}>
                                        <X class="w-3 h-3" />
                                    </button>
                                </span>
                            {/each}
                        </div>
                    </div>
                </div>
                <Dialog.Footer>
                    <Button type="submit" onclick={createWorkspace}>Create Workspace</Button>
                </Dialog.Footer>
            </Dialog.Content>
        </Dialog.Root>
    </div>
    <div class=" flex justify-end">
        <NavUser {user} />
    </div>
</nav>
