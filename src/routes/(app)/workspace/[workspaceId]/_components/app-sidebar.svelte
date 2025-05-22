<script lang="ts" module>
	import AudioWaveform from "@lucide/svelte/icons/audio-waveform";
	import BookOpen from "@lucide/svelte/icons/book-open";
	import Bot from "@lucide/svelte/icons/bot";
	import ChartPie from "@lucide/svelte/icons/chart-pie";
	import Command from "@lucide/svelte/icons/command";
	import Frame from "@lucide/svelte/icons/frame";
	import GalleryVerticalEnd from "@lucide/svelte/icons/gallery-vertical-end";
	import Map from "@lucide/svelte/icons/map";
	import Settings2 from "@lucide/svelte/icons/settings-2";
	import SquareTerminal from "@lucide/svelte/icons/square-terminal";

	// This is sample data.
	const data = {
		
		projects: [
			{
				name: "Design Engineering",
				url: "#",
				icon: Frame,
			},
			{
				name: "Sales & Marketing",
				url: "#",
				icon: ChartPie,
			},
			{
				name: "Travel",
				url: "#",
				icon: Map,
			},
		],
	};


    


    
</script>

<script lang="ts">
	import NavMain from "./nav-main.svelte";
	import NavProjects from "./nav-projects.svelte";
	import TeamSwitcher from "./team-switcher.svelte";
	import * as Sidebar from "$lib/components/ui/sidebar";
	import type { ComponentProps } from "svelte";
	import NavUser from "./nav-user.svelte";

	let { 
		workspaces=[], 
		pages, 
		user,
		userRole,
		ref = $bindable(null),
		collapsible = "icon" as "icon" | "none" | "offcanvas",
		...restProps
	} = $props();

</script>

<Sidebar.Root bind:ref {collapsible} {...restProps}>
	<Sidebar.Header>
		<TeamSwitcher workspaces={workspaces}/>
	</Sidebar.Header>
	<Sidebar.Content>
		<NavMain userRole={userRole} />
		<NavProjects projects={data.projects} />
	</Sidebar.Content>
	<Sidebar.Footer>
		<NavUser user={user} />
	</Sidebar.Footer>
	<Sidebar.Rail />
</Sidebar.Root>