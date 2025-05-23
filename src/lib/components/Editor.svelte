<script lang="ts">
  import { onMount, onDestroy, createEventDispatcher, tick } from "svelte";
  import { Editor } from "@tiptap/core";
  import StarterKit from "@tiptap/starter-kit";
  import Underline from "@tiptap/extension-underline";
  import Link from "@tiptap/extension-link";
  import Blockquote from "@tiptap/extension-blockquote";
  import CodeBlock from "@tiptap/extension-code-block";
  import ListItem from "@tiptap/extension-list-item";
  import BulletList from "@tiptap/extension-bullet-list";
  import OrderedList from "@tiptap/extension-ordered-list";
  import Placeholder from "@tiptap/extension-placeholder";
  import { fade } from "svelte/transition";

  import Collaboration from "@tiptap/extension-collaboration";
  import CollaborationCursor from "@tiptap/extension-collaboration-cursor";
  import * as Y from "yjs";
  import { WebsocketProvider } from "y-websocket";
  import { activeUsers } from "$lib/stores/awareness";

  export let page;
  export let user; // { id, name, color, avatar }
  export let disabled = false;
  const dispatch = createEventDispatcher();
  let editor: Editor;
  let container: HTMLDivElement;
  let showToolbar = false;
  let selection = false;

  let ydoc: Y.Doc;
  let provider: WebsocketProvider;

  onMount(async () => {
    console.log(page.content);
    await tick(); // Wait for container to be assigned

    // Setup Yjs doc and WebSocket provider for collaboration
    ydoc = new Y.Doc();
    provider = new WebsocketProvider(
      import.meta.env.VITE_WS_URL,
      page.id,
      ydoc,
    );

    const randomHexColor =
      "#" +
      Math.floor(Math.random() * 0xffffff)
        .toString(16)
        .padStart(6, "0");

    // Set local user state for awareness
    provider.awareness.setLocalStateField("user", {
      ...user,
      color: randomHexColor,
    });
    provider.awareness.on("change", () => {
      // 1) Tüm awareness durumlarını al
      const states = Array.from(provider.awareness.getStates().values());

      // 2) Sadece user objelerini çıkar, undefined'ları at
      const users = states.map((s) => s.user).filter(Boolean) as Array<{
        id: string;
        name: string;
        color: string;
        avatar?: string;
      }>;

      // 3) ID'ye göre tekilleştir
      const uniqueById = Array.from(
        new Map(users.map((u) => [u.id, u])).values(),
      );

      // 4) Svelte store'u güncelle
      activeUsers.set(uniqueById);
    });

    editor = new Editor({
      element: container,
      extensions: [
        StarterKit,
        Underline,
        Link.configure({ openOnClick: false }),
        Blockquote,
        CodeBlock,
        ListItem,
        BulletList,
        OrderedList,
        Placeholder.configure({
          placeholder: ({ node }) =>
            node.type.name === "heading"
              ? `Heading ${node.attrs.level}`
              : 'Type "/" for commands...',
        }),
        Collaboration.configure({ document: ydoc }),
        CollaborationCursor.configure({
          provider,
          user,
          render: (remoteUser) => {
            // Create main wrapper element
            const wrapper = document.createElement("div");
            wrapper.className =
              "absolute flex items-center -left-28 transform -translate-y-6 pointer-events-none z-20 transition-all duration-200";

            // Get user color or use default
            const userColor = remoteUser.color || "#3b82f6";

            // Create HTML with improved styling
            wrapper.innerHTML = `
    <div class="flex items-center justify-center gap-2 bg-white dark:bg-gray-800 rounded-full px-0 pr-2 shadow-sm border" style="border-color: ${userColor}">
      <img src="${remoteUser.avatar}" class="w-6 h-6 rounded-full object-cover border border-gray-100" alt="${remoteUser.username}'s avatar" />
      <span class="font-medium text-xs text-gray-800 dark:text-gray-200">${remoteUser.username}</span>
    </div>
    <span class="text-lg mx-1" style="color: ${userColor}">▶</span>
  `;

            return wrapper;
          },
          selectionRender: () => ({ class: "" }),
        }),
      ],
      content: page.content || "",
      autofocus: "start",
      editable: !disabled,
      editorProps: {
        attributes: {
          class: "outline-none prose prose-invert max-w-full px-4 py-2",
        },
      },
      onSelectionUpdate: ({ editor }) => {
        if (!disabled) {
          selection = !editor.state.selection.empty;
          showToolbar = selection;
        }
      },
    });

    // Eğer content JSON ise yükle
    // if (page.content && typeof page.content === "object") {
    //   editor.commands.setContent(page.content);
    // }

    ydoc.on("update", (update: Uint8Array, origin: any) => {
      if (!disabled && origin !== provider) {
        dispatch("update", editor.getJSON());
      }
    });
  });

  onDestroy(() => {
    editor?.destroy();
    activeUsers.set([]);
  });

  function isActive(type: any, attrs = {}) {
    if (!editor) return false;
    return editor.isActive(type, attrs);
  }
</script>

<div class="editor-container">
  <div class="editor-header">
    <div class="page-title">{page.title}</div>
    <div class="saving-status">
      <span>Auto-saving</span>
    </div>
  </div>

  {#if showToolbar && editor && !disabled}
    <div class="toolbar" transition:fade={{ duration: 100 }}>
      <button
        class:active={isActive("heading", { level: 1 })}
        on:click={() =>
          editor.chain().focus().toggleHeading({ level: 1 }).run()}
      >
        H1
      </button>
      <button
        class:active={isActive("heading", { level: 2 })}
        on:click={() =>
          editor.chain().focus().toggleHeading({ level: 2 }).run()}
      >
        H2
      </button>
      <button
        class:active={isActive("heading", { level: 3 })}
        on:click={() =>
          editor.chain().focus().toggleHeading({ level: 3 }).run()}
      >
        H3
      </button>
      <button
        class:active={isActive("bold")}
        on:click={() => editor.chain().focus().toggleBold().run()}
      >
        <b>B</b>
      </button>
      <button
        class:active={isActive("italic")}
        on:click={() => editor.chain().focus().toggleItalic().run()}
      >
        <i>I</i>
      </button>
      <button
        class:active={isActive("underline")}
        on:click={() => editor.chain().focus().toggleUnderline().run()}
      >
        <u>U</u>
      </button>
      <button
        class:active={isActive("strike")}
        on:click={() => editor.chain().focus().toggleStrike().run()}
      >
        S
      </button>
      <button
        class:active={isActive("bulletList")}
        on:click={() => editor.chain().focus().toggleBulletList().run()}
      >
        • List
      </button>
      <button
        class:active={isActive("orderedList")}
        on:click={() => editor.chain().focus().toggleOrderedList().run()}
      >
        1. List
      </button>
      <button
        class:active={isActive("blockquote")}
        on:click={() => editor.chain().focus().toggleBlockquote().run()}
      >
        ❝ Quote
      </button>
      <button
        class:active={isActive("codeBlock")}
        on:click={() => editor.chain().focus().toggleCodeBlock().run()}
      >
        {"{}"} Code
      </button>
    </div>
  {/if}

  <div bind:this={container} class="editor-content" class:disabled></div>
</div>

<style>
  .editor-container {
    position: relative;
    width: 100%;
    max-width: 800px;
    margin: 0 auto;
    background-color: #121212;
    border-radius: 8px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
    min-height: calc(100vh - 80px);
  }

  .disabled {
    cursor: not-allowed;
    opacity: 0.7;
  }

  .toolbar {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;
    padding: 6px 8px;
    background-color: #1e1e1e;
    border-radius: 4px;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.3);
    position: absolute;
    z-index: 20;
    left: 50%;
    transform: translateX(-50%);
  }

  .toolbar button {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 4px 8px;
    border-radius: 4px;
    background: transparent;
    color: #d1d5db;
    border: none;
    cursor: pointer;
    font-size: 14px;
    transition: all 0.2s;
  }

  .toolbar button:hover {
    background-color: #2d2d2d;
  }

  .toolbar button.active {
    background-color: #2d2d2d;
    color: #60a5fa;
  }

  .slash-menu {
    position: absolute;
    background: #1e1e1e;
    border-radius: 4px;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.3);
    padding: 4px;
    z-index: 30;
  }

  .editor-header {
    padding: 12px 16px;
    border-bottom: 1px solid #2d2d2d;
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .page-title {
    font-size: 14px;
    color: #9ca3af;
    font-weight: 500;
  }

  .saving-status {
    font-size: 12px;
    color: #9ca3af;
  }
  /* Caret & avatar wrapper */
  :global(.remote-cursor-wrapper) {
    position: absolute;
    left: -20px;
    display: flex;
    align-items: center;
    transform: translateY(-50%);
    pointer-events: none;
    z-index: 20;
  }
  :global(.remote-cursor-avatar) {
    width: 16px;
    height: 16px;
    border-radius: 50%;
    border: 1px solid white;
    box-shadow: 0 0 2px rgba(0, 0, 0, 0.5);
    margin-bottom: 20px;
  }
  :global(.remote-cursor-arrow) {
    margin-left: 4px;
    color: var(--cursor-color, #f59e0b);
    font-size: 16px;
    margin-bottom: 20px;
  }

  /* Hide default Yjs cursor */
  :global(.yjs-cursor),
  :global(.yjs-cursor-label) {
    display: none !important;
  }
</style>
