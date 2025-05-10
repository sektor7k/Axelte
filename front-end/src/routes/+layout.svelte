<script lang="ts">
	import '../app.css'
  
	let { children } = $props()
</script>

<svelte:head>
	<meta charset="utf-8" />
	<meta name="viewport" content="width=device-width, initial-scale=1" />
	<link rel="icon" href="/favicon.svg" type="image/svg+xml" />
	<title>Axelete</title>
</svelte:head>

<main class="min-h-screen bg-gray-900 text-white">
	{@render children()}
</main>