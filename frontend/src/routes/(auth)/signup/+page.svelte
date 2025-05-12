<script lang="ts">
    import { Button } from "$lib/components/ui/button";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import { Card, CardContent, CardHeader, CardTitle, CardFooter } from "$lib/components/ui/card";
    import { toast } from "svelte-sonner";
    import axios from "axios";
    import { goto } from "$app/navigation";
    
    let username = "";
    let email = "";
    let password = "";
    let confirmPassword = "";
    let isLoading = false;

    async function handleSubmit() {
        if (password !== confirmPassword) {
            toast.error("Passwords do not match!");
            return;
        }

        try {
            isLoading = true;
            const response = await axios.post(`${import.meta.env.VITE_SERVER_URL}/auth/signup`, {
                username,
                email,
                password
            });           
                toast.success("Account created successfully!");
                goto("/login");
          
        } catch (error:any) {
            toast.error(error.response.data.message|| "Something went wrong!");
        } finally {
            isLoading = false;
        }
    }
</script>

<div class="flex justify-center items-center min-h-screen">
    <Card class="w-[400px]">
        <CardHeader>
            <CardTitle class="text-2xl font-bold text-center">Create Account</CardTitle>
        </CardHeader>
        <CardContent>
            <form on:submit|preventDefault={handleSubmit}>
                <div class="space-y-4">
                    <div class="space-y-2">
                        <Label for="username">Username</Label>
                        <Input 
                            type="text" 
                            id="username" 
                            placeholder="Enter your username" 
                            bind:value={username}
                            required
                            disabled={isLoading}
                        />
                    </div>
                    <div class="space-y-2">
                        <Label for="email">Email</Label>
                        <Input 
                            type="email" 
                            id="email" 
                            placeholder="Enter your email" 
                            bind:value={email}
                            required
                            disabled={isLoading}
                        />
                    </div>
                    <div class="space-y-2">
                        <Label for="password">Password</Label>
                        <Input 
                            type="password" 
                            id="password" 
                            placeholder="Create a password" 
                            bind:value={password}
                            required
                            disabled={isLoading}
                        />
                    </div>
                    <div class="space-y-2">
                        <Label for="confirmPassword">Confirm Password</Label>
                        <Input 
                            type="password" 
                            id="confirmPassword" 
                            placeholder="Confirm your password" 
                            bind:value={confirmPassword}
                            required
                            disabled={isLoading}
                        />
                    </div>
                    <Button type="submit" class="w-full" disabled={isLoading}>
                        {isLoading ? "Creating Account..." : "Create Account"}
                    </Button>
                </div>
            </form>
        </CardContent>
        <CardFooter class="flex justify-center">
            <p class="text-sm text-gray-500">
                Already have an account? 
                <a href="/login" class="text-primary hover:underline">Sign in</a>
            </p>
        </CardFooter>
    </Card>
</div>
