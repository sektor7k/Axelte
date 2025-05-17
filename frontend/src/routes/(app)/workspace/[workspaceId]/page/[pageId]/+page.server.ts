import { error } from '@sveltejs/kit';

export const load = async ({ params, fetch }) => {
    const { pageId } = params;
    const response = await fetch(`${import.meta.env.VITE_SERVER_URL}/api/get-page/${pageId}`, {
        credentials: 'include'
    });
    if (!response.ok) throw error(404, 'Page not found');
    const data = await response.json();
    return { page: data };
};