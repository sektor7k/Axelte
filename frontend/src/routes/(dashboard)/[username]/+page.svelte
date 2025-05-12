<script>
    import { Card, CardContent, CardHeader, CardTitle } from "$lib/components/ui/card";
    import { Button } from "$lib/components/ui/button";
    // Bu değerler örnek, backend'den çekebilirsin
    export let data;
    let username = "johndoe";
    let email =  "johndoe@example.com";
    let avatar = "https://ui-avatars.com/api/?name=John+Doe&background=random";
</script>

<div class="flex justify-center items-center min-h-screen bg-gray-50">
    <Card class="w-[350px]">
        <CardHeader class="flex flex-col items-center gap-2">
            <img src={avatar} alt="Profile" class="w-24 h-24 rounded-full object-cover border-4 border-primary shadow" />
            <CardTitle class="text-xl mt-2">{username}</CardTitle>
            <span class="text-gray-500 text-sm">{email}</span>
        </CardHeader>
        <CardContent class="flex flex-col items-center">
            <Button variant="outline" class="mt-4 w-full">Profili Düzenle</Button>
        </CardContent>
    </Card>
</div>
