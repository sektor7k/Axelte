// src/lib/server/auth.ts
import { env } from '$env/dynamic/private';

export interface User {
  id:     string;
  username: string;
  email:    string;
  avatar: string;
  role: string;
}

export async function getUserById(
  fetchFn: typeof fetch,
  cookieHeader: string,
  userId: string
): Promise<User> {
  const res = await fetchFn(`${env.VITE_SERVER_URL}/api/me`, {
    headers: {
      // Aynı cookie ile session/auth header’larını backend’e ilet
      cookie: cookieHeader
    },
    credentials: 'include'
  });

  if (!res.ok) {
    throw new Error(`Kullanıcı bilgisi alınamadı (status: ${res.status})`);
  }

  const data = await res.json();
  // Geri dönen objeyi User tipine uydur
  return data as User;
}
