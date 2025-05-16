<script>
    import MainNavbar from "$lib/components/MainNavbar.svelte";

    export let data;

</script>
<MainNavbar user={data.user}/>

    
<div class="h-16"></div>

<div class="flex flex-col items-center justify-center min-h-[calc(100vh-4rem)] ">
    <h1 class="text-2xl font-bold text-gray-800">Welcome to your workspace page!</h1>
</div>