<script lang="ts">
    export let data;
    
    $: page = data.page;

</script>

<div>
    <h1>{page.title}</h1>
</div>