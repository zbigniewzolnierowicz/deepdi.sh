import type { FC } from 'react';
import type { SerializedEditorState } from 'lexical';

import { AutoFocusPlugin } from '@lexical/react/LexicalAutoFocusPlugin';
import type { InitialConfigType } from '@lexical/react/LexicalComposer';
import { LexicalComposer } from '@lexical/react/LexicalComposer';
import { RichTextPlugin } from '@lexical/react/LexicalRichTextPlugin';
import { ContentEditable } from '@lexical/react/LexicalContentEditable';
import { HistoryPlugin } from '@lexical/react/LexicalHistoryPlugin';
import { LexicalErrorBoundary } from '@lexical/react/LexicalErrorBoundary';
import { OnChangePlugin } from '@lexical/react/LexicalOnChangePlugin';

type EditorProps<T> = {
  onChange: (data: T) => void;
  value?: T;
  name: string;
};

export const Editor: FC<EditorProps<SerializedEditorState>> = ({ onChange, value }) => {
  const initialConfig: InitialConfigType = {
    namespace: 'MyEditor',
    theme: {},
    onError: console.error,
    editorState(editor) {
      if (!value) return;
      const state = editor.parseEditorState(value);

      editor.setEditorState(state);
    },
  };

  return (
    <LexicalComposer initialConfig={initialConfig}>
      <RichTextPlugin
        contentEditable={<ContentEditable />}
        placeholder={<div>Enter some text...</div>}
        ErrorBoundary={LexicalErrorBoundary}
      />
      <HistoryPlugin />
      <AutoFocusPlugin />
      <OnChangePlugin onChange={state => onChange(state.toJSON())} />
    </LexicalComposer>
  );
};
