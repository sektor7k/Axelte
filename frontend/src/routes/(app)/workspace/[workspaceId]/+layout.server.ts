// src/routes/workspaces/[workspaceId]/+layout.server.ts
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ params, fetch }) => {

  // 1) Workspace meta
  const wsRes = await fetch(`${import.meta.env.VITE_SERVER_URL}/api/get-workspaces`,
    {
        credentials: 'include'
      }
  );
  const wsData = await wsRes.json();
  const workspaces: { id: string; name: string; description: string, owner_id:string }[] = wsData.workspaces;


  // 2) Sadece sayfa listesi: id ve title
  const pagesRes = await fetch(`${import.meta.env.VITE_SERVER_URL}/api/workspaces/${params.workspaceId}/pages`,
    {
        credentials: 'include'
      }
  );
  const pages: { id: string; title: string }[] = await pagesRes.json();

  return { workspaces, pages };
};
