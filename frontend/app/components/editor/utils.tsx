import { SerializedEditorState, SerializedParagraphNode } from 'lexical';

export const EMPTY_RTE = {
  root: {
    children: [
      {
        children: [],
        direction: 'ltr',
        format: '',
        indent: 0,
        type: 'paragraph',
        version: 1,
        textFormat: 0,
      },
    ],
    direction: 'ltr',
    format: '',
    indent: 0,
    type: 'root',
    version: 1,
  },
} as SerializedEditorState<SerializedParagraphNode>;
