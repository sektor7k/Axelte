<script lang="ts">
    import * as Avatar from "$lib/components/ui/avatar/index.js";
    export let avatarurl;
   </script>
    
   <Avatar.Root>
    <Avatar.Image src={avatarurl} alt="@shadcn" />
    <Avatar.Fallback>CN</Avatar.Fallback>
   </Avatar.Root>