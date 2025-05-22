import { writable } from 'svelte/store';

export const activeUsers = writable<
  Array<{ name: string; color: string; avatar?: string }>
>([]);
