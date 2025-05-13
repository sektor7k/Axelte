// src/lib/server/auth.ts
import { env } from '$env/dynamic/private';

export async function verifyJwtAndFetchUser(fetchFn: typeof fetch, cookieHeader: string) {
  const res = await fetchFn(`${env.VITE_SERVER_URL}/api/me`, {
    headers: { cookie: cookieHeader },
    credentials: 'include'
  });

  if (!res.ok) {
    throw new Error('Not authenticated');
  }

  return await res.json();  // { id, username, email }
}
