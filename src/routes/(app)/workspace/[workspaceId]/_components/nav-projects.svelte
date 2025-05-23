<script lang="ts">
	import * as Dialog from "$lib/components/ui/dialog";
	import { Label } from "$lib/components/ui/label";
	import { Input } from "$lib/components/ui/input";
	import { Button } from "$lib/components/ui/button";
	import { Brain } from "@lucide/svelte";
	import axios from "axios";
	import { toast } from "svelte-sonner";
	import { onDestroy } from "svelte";
	import * as Sidebar from "$lib/components/ui/sidebar";
	import { goto } from "$app/navigation";
	import { page } from "$app/stores";
	import { pages } from "$lib/stores/pages";
	import { textToDoc } from "$lib/utils/prosemirror";

	// URL to scrape
	let url = "";
	let isDialogOpen = false;
	let jobId: string;
	let summary: string | null = null;
	let polling: ReturnType<typeof setInterval>;

	// route param’dan workspaceId alın
	const workspaceId = $page.params.workspaceId;

	function openDialog() {
		isDialogOpen = true;
	}

	async function handleScrape() {
		try {
			const { data } = await axios.post(
				`${import.meta.env.VITE_SERVER_URL}/api/scrape`,
				{ url },
				{ withCredentials: true },
			);
			jobId = data.job_id;
			isDialogOpen = false;
			toast.success("Job submitted! Waiting for summary…");
			polling = setInterval(checkStatus, 2000);
		} catch (err) {
			console.error(err);
			toast.error("Failed to start scraping job.");
		}
	}

	async function checkStatus() {
		try {
			const { data } = await axios.get(
				`${import.meta.env.VITE_SERVER_URL}/api/jobs/${jobId}`,
				{ withCredentials: true },
			);
			if (data.status === "done") {
				clearInterval(polling);
				const summary = data.summary as string;
				toast.success("Summary is ready, creating page…");

				// 1) Boş page oluştur
				const { data: created } = await axios.post(
					`${import.meta.env.VITE_SERVER_URL}/api/create-page`,
					{
						title: url,
						workspace_id: workspaceId,
					},
					{ withCredentials: true },
				);
				pages.addPage({
					id: created.page_id,
					title: url,
				});

				const newPageId = created.page_id;
				toast.success("Page created, updating content");
				const doc = textToDoc(summary);
				await axios.post(
					`${import.meta.env.VITE_SERVER_URL}/api/update-page`,
					{
						id: newPageId,
						content: doc,
					},
					{ withCredentials: true },
				);
				toast.success("Page content updated!");
				// → Yeni sayfaya yönlendir
				await goto(`/workspace/${workspaceId}/page/${newPageId}`, {
					replaceState: true,
					keepFocus: true,
				});
			} else if (data.status === "failed") {
				clearInterval(polling);
				toast.error("Summary generation failed: " + data.error);
			}
			// pending ise bekle
		} catch (err) {
			clearInterval(polling);
			console.error(err);
			toast.error("Error polling job status.");
		}
	}

	onDestroy(() => {
		if (polling) clearInterval(polling);
	});
</script>

<!-- Tools Sidebar -->
<Sidebar.Group class="group-data-[collapsible=icon]:hidden">
	<Sidebar.GroupLabel>Tools</Sidebar.GroupLabel>
	<Sidebar.Menu>
		<Sidebar.MenuItem>
			<Sidebar.MenuButton class="w-full">
				<Dialog.Root bind:open={isDialogOpen}>
					<Dialog.Trigger class="w-full" onclick={openDialog}>
						<div class="flex items-center gap-2">
							<Brain />
							<span>Web Scraper</span>
						</div>
					</Dialog.Trigger>
					<Dialog.Content class="sm:max-w-[425px]">
						<Dialog.Header>
							<Dialog.Title>Web Scraper</Dialog.Title>
							<Dialog.Description>
								Enter a URL and get back an AI‐generated
								summary.
							</Dialog.Description>
						</Dialog.Header>
						<div class="grid gap-4 py-4">
							<div class="grid grid-cols-4 items-center gap-4">
								<Label for="url">Website URL:</Label>
								<Input
									id="url"
									type="url"
									placeholder="https://example.com"
									bind:value={url}
									class="col-span-3"
								/>
							</div>
						</div>
						<Dialog.Footer>
							<Button type="button" onclick={handleScrape}>
								Scrape & Create Page
							</Button>
						</Dialog.Footer>
					</Dialog.Content>
				</Dialog.Root>
			</Sidebar.MenuButton>
		</Sidebar.MenuItem>
	</Sidebar.Menu>
</Sidebar.Group>
