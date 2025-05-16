// src/routes/(app)/workspace/[workspaceId]/+page.ts
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params }) => {
    return {
        workspaceId: params.workspaceId
    };
};