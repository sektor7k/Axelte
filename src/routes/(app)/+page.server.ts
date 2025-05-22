// +page.server.ts
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, locals }) => {
  const res = await fetch(`${import.meta.env.VITE_SERVER_URL}/api/get-workspaces`, {
    credentials: 'include'
  });

  if (!res.ok) {
    console.log("error");
    return { workspaces: [] };
  }

  const data = await res.json();
  return { workspaces: data.workspaces };
};