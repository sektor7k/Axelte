import { writable } from 'svelte/store';

type Page = {
    id: string;
    title: string;
};

function createPagesStore() {
    const { subscribe, set, update } = writable<Page[]>([]);

    return {
        subscribe,
        set,
        addPage: (page: Page) => update(pages => [...pages, page]),
        removePage: (pageId: string) => update(pages => pages.filter(p => p.id !== pageId)),
        updatePage: (pageId: string, newData: Partial<Page>) => 
            update(pages => pages.map(p => p.id === pageId ? { ...p, ...newData } : p))
    };
}

export const pages = createPagesStore(); 