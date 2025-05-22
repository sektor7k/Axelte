// src/hooks.server.ts
import type { Handle } from '@sveltejs/kit';
import jwt from 'jsonwebtoken';
import { redirect } from '@sveltejs/kit';
import { getUserById, type User} from '$lib/server/auth';

const JWT_SECRET = import.meta.env.VITE_JWT_SECRET;

export const handle: Handle = async ({ event, resolve }) => {

  const cookieHeader = event.request.headers.get('cookie') || '';
  const token = event.cookies.get('axtoken');

  if (token) {
    try {

      const { sub: userId } = jwt.verify(token, JWT_SECRET) as { sub: string };

      const user: User = await getUserById(fetch, cookieHeader, userId);

      event.locals.user = user;
    } catch (err) {
      console.log('auth error:', err);
      event.locals.user = null;
    }
  } else {
    event.locals.user = null;
  }


const PUBLIC_ROUTES = ['/login', '/register'];
const path = event.url.pathname;

if (!event.locals.user && !PUBLIC_ROUTES.some(r => path.startsWith(r))) {
  throw redirect(303, '/login');
}
if (event.locals.user && PUBLIC_ROUTES.includes(path)) {
  throw redirect(303, '/');
}

return resolve(event);
};
