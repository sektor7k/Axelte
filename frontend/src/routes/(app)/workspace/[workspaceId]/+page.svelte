<script lang="ts">
    export let data: { workspaceId: string };
    // Demo veriler
    let workspace = {
        name: "Demo Workspace",
        description: "This is a collaborative space for your team projects.",
        members: [
            { name: "Alice", email: "alice@example.com" },
            { name: "Bob", email: "bob@example.com" },
            { name: "Charlie", email: "charlie@example.com" }
        ]
    };
</script>

<div class="max-w-2xl mx-auto mt-12 p-6  rounded-lg shadow">
    <h1 class="text-2xl font-bold mb-2">{workspace.name}</h1>
    <p class="text-gray-600 mb-4">{workspace.description}</p>
    <div class="mb-6">
        <h2 class="font-semibold mb-2">Members</h2>
        <ul class="space-y-1">
            {#each workspace.members as member}
                <li class="flex items-center gap-2">
                    <span class="font-medium">{member.name}</span>
                    <span class="text-gray-400 text-xs">{member.email}</span>
                </li>
            {/each}
        </ul>
    </div>
    <button class="bg-blue-600 hover:bg-blue-700  px-4 py-2 rounded transition">
        Invite User
    </button>
</div>