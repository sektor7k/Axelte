// src/lib/utils/prosemirror.ts

/**
 * Düz metni paragraflara bölüp ProseMirror 'doc' formatına çevirir.
 */
export function textToDoc(text: string) {
    const paras = text
      .split(/\n{2,}/g)      // çift boşluktan paragraf ayır
      .map(p => p.trim())    // kenar boşlukları kırp
      .filter(Boolean);
  
    return {
      type: "doc",
      content: paras.map(p => ({
        type: "paragraph",
        content: [{ type: "text", text: p }]
      }))
    };
  }
  