// src/hooks.server.ts
import type { Handle } from '@sveltejs/kit';
import { verifyJwtAndFetchUser } from '$lib/server/auth';

export const handle: Handle = async ({ event, resolve }) => {
  try {
    // Gelen isteğin cookie başlığını al:
    const cookieHeader = event.request.headers.get('cookie') || '';
    // API üzerinden me’yi çek:
    event.locals.user = await verifyJwtAndFetchUser(fetch, cookieHeader);
  } catch {
    event.locals.user = null;
  }

  return resolve(event);
};
