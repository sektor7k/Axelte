<script>
    import { Button } from "$lib/components/ui/button";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import { Card, CardContent, CardHeader, CardTitle, CardFooter } from "$lib/components/ui/card";
    import { toast } from "svelte-sonner";
    
    let name = "";
    let email = "";
    let password = "";
    let confirmPassword = "";

    function handleSubmit() {
        // Handle signup logic here
        if (password !== confirmPassword) {
            toast.error("Passwords do not match!");
            return;
        }
        console.log({ name, email, password });
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
                        <Label for="name">Full Name</Label>
                        <Input 
                            type="text" 
                            id="name" 
                            placeholder="Enter your full name" 
                            bind:value={name}
                            required
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
                        />
                    </div>
                    <Button type="submit" class="w-full">
                        Create Account
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
