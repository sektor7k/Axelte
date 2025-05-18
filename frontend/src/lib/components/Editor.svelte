<script lang="ts">
    import { onMount, onDestroy, createEventDispatcher, tick } from 'svelte';
    import { Editor } from '@tiptap/core';
    import StarterKit from '@tiptap/starter-kit';
    import Underline from '@tiptap/extension-underline';
    import Link from '@tiptap/extension-link';
    import Blockquote from '@tiptap/extension-blockquote';
    import CodeBlock from '@tiptap/extension-code-block';
    import ListItem from '@tiptap/extension-list-item';
    import BulletList from '@tiptap/extension-bullet-list';
    import OrderedList from '@tiptap/extension-ordered-list';
    import Placeholder from '@tiptap/extension-placeholder';
    import { fade } from 'svelte/transition';
    
  
  export let page;
    const dispatch = createEventDispatcher();
    let editor: Editor;
    let container: HTMLDivElement;
    let showToolbar = false;
    let selection = false;
  
    onMount(async () => {
      await tick(); // Wait for container to be assigned
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
            placeholder: ({ node }) => {
              if (node.type.name === 'heading') {
                return `Heading ${node.attrs.level}`;
              }
              return 'Type "/" for commands...';
            }
          })
        ],
        content: page.content || '',
        autofocus: 'start',
        editorProps: {
          attributes: {
            class: 'outline-none prose prose-invert max-w-full px-4 py-2'
          }
        },
        onUpdate: ({ editor }) => dispatch('update', editor.getJSON()),
        onSelectionUpdate: ({ editor }) => {
          selection = !editor.state.selection.empty;
          showToolbar = selection;
        }
      });
  
      // blink cursor CSS
      const style = document.createElement('style');
      style.innerHTML = `
        .ProseMirror-focused .ProseMirror-cursor {
          animation: blink 1s step-end infinite;
          border-color: #e2e8f0;
        }
        @keyframes blink { 50% { visibility: hidden; } }
        
        .ProseMirror {
          color: #e2e8f0;
        }
        
        .ProseMirror h1 {
          font-size: 1.875rem;
          font-weight: 700;
          margin-top: 1.5rem;
          margin-bottom: 0.5rem;
          color: #f8fafc;
        }
        
        .ProseMirror h2 {
          font-size: 1.5rem;
          font-weight: 600;
          margin-top: 1.25rem;
          margin-bottom: 0.5rem;
          color: #f8fafc;
        }
        
        .ProseMirror h3 {
          font-size: 1.25rem;
          font-weight: 600;
          margin-top: 1rem;
          margin-bottom: 0.5rem;
          color: #f8fafc;
        }
        
        .ProseMirror p {
          margin-bottom: 0.5rem;
        }
        
        .ProseMirror ul, .ProseMirror ol {
          padding-left: 1.5rem;
        }
        
        .ProseMirror blockquote {
          border-left: 3px solid #475569;
          padding-left: 1rem;
          color: #94a3b8;
        }
        
        .ProseMirror pre {
          background-color: #1e293b;
          border-radius: 0.375rem;
          padding: 0.75rem;
          font-family: monospace;
          color: #e2e8f0;
        }
        
        .ProseMirror code {
          background-color: #1e293b;
          border-radius: 0.25rem;
          padding: 0.125rem 0.25rem;
          font-family: monospace;
          color: #e2e8f0;
        }
      `;
      document.head.append(style);
    });
  
    $: if (editor) {
      if (page.content) {
        editor.commands.setContent(page.content);
      } else {
        editor.commands.clearContent();
      }
    }
  
    onDestroy(() => {
      editor?.destroy(); 
    });
    
    function isActive(type:any, attrs = {}) {
      if (!editor) return false;
      return editor.isActive(type, attrs);
    }
  </script>
  
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
  </style>
  
  <div class="editor-container">
    <div class="editor-header">
      <div class="page-title">{page.title}</div>
      <div class="saving-status">
        <span>Auto-saving</span>
      </div>
    </div>
  
    {#if showToolbar && editor}
      <div class="toolbar" transition:fade={{ duration: 100 }}>
        <button 
          class:active={isActive('heading', { level: 1 })}
          on:click={() => editor.chain().focus().toggleHeading({ level: 1 }).run()}>
          H1
        </button>
        <button 
          class:active={isActive('heading', { level: 2 })}
          on:click={() => editor.chain().focus().toggleHeading({ level: 2 }).run()}>
          H2
        </button>
        <button 
          class:active={isActive('heading', { level: 3 })}
          on:click={() => editor.chain().focus().toggleHeading({ level: 3 }).run()}>
          H3
        </button>
        <button 
          class:active={isActive('bold')}
          on:click={() => editor.chain().focus().toggleBold().run()}>
          <b>B</b>
        </button>
        <button 
          class:active={isActive('italic')}
          on:click={() => editor.chain().focus().toggleItalic().run()}>
          <i>I</i>
        </button>
        <button 
          class:active={isActive('underline')}
          on:click={() => editor.chain().focus().toggleUnderline().run()}>
          <u>U</u>
        </button>
        <button 
          class:active={isActive('strike')}
          on:click={() => editor.chain().focus().toggleStrike().run()}>
          S
        </button>
        <button 
          class:active={isActive('bulletList')}
          on:click={() => editor.chain().focus().toggleBulletList().run()}>
          • List
        </button>
        <button 
          class:active={isActive('orderedList')}
          on:click={() => editor.chain().focus().toggleOrderedList().run()}>
          1. List
        </button>
        <button 
          class:active={isActive('blockquote')}
          on:click={() => editor.chain().focus().toggleBlockquote().run()}>
          ❝ Quote
        </button>
        <button 
          class:active={isActive('codeBlock')}
          on:click={() => editor.chain().focus().toggleCodeBlock().run()}>
          {"{}"} Code
        </button>
      </div>
    {/if}
    
    <div bind:this={container} class="editor-content"></div>
  </div>