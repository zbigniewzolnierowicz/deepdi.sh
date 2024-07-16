import type { FC } from 'react';
import { ParagraphNode, type SerializedEditorState } from 'lexical';

import { AutoFocusPlugin } from '@lexical/react/LexicalAutoFocusPlugin';
import type { InitialConfigType } from '@lexical/react/LexicalComposer';
import { LexicalComposer } from '@lexical/react/LexicalComposer';
import { RichTextPlugin } from '@lexical/react/LexicalRichTextPlugin';
import { ContentEditable } from '@lexical/react/LexicalContentEditable';
import { HistoryPlugin } from '@lexical/react/LexicalHistoryPlugin';
import { LexicalErrorBoundary } from '@lexical/react/LexicalErrorBoundary';
import { OnChangePlugin } from '@lexical/react/LexicalOnChangePlugin';
import { MarkdownShortcutPlugin } from '@lexical/react/LexicalMarkdownShortcutPlugin';
import { HorizontalRulePlugin } from '@lexical/react/LexicalHorizontalRulePlugin';

import { HeadingNode, QuoteNode } from '@lexical/rich-text';
import { CodeNode } from '@lexical/code';
import { ListItemNode, ListNode } from '@lexical/list';
import { LinkNode } from '@lexical/link';
import { HorizontalRuleNode } from '@lexical/react/LexicalHorizontalRuleNode';

import { clsx } from 'clsx';

type EditorProps<T> = {
  onChange: (data: T) => void;
  value?: T;
  className?: string;
  editable?: boolean;
};

export const EDITOR_NODES = [
  HeadingNode,
  QuoteNode,
  ParagraphNode,
  HorizontalRuleNode,
  CodeNode,
  ListNode,
  ListItemNode,
  LinkNode,
];

export const Editor: FC<EditorProps<SerializedEditorState>> = ({ onChange, value, className, editable = true }) => {
  const initialConfig: InitialConfigType = {
    namespace: 'MyEditor',
    theme: {
      paragraph: 'mb-1 last:mb-0 font-body',
      heading: {
        h1: 'font-heading mb-1 text-2xl',
        h2: 'font-heading mb-1 text-xl',
        h3: 'font-heading mb-1 text-lg font-extrabold',
        h4: 'font-heading mb-1 text-lg',
      },
      hr: 'background-red',
    },
    onError: console.error,
    editorState(editor) {
      if (!value) return;
      const state = editor.parseEditorState(value);

      editor.setEditorState(state);
    },
    nodes: EDITOR_NODES,
    editable,
  };

  return (
    <div className={clsx('grid grid-cols-1 grid-rows-1', className)}>
      <LexicalComposer initialConfig={initialConfig}>
        <HorizontalRulePlugin />
        <MarkdownShortcutPlugin />
        <RichTextPlugin
          contentEditable={(
            <ContentEditable
              className="col-start-1 col-end-1 row-start-1 row-end-1"
            />
          )}
          placeholder={(
            <div
              className="col-start-1 col-end-1 row-start-1 row-end-1 pointer-events-none font-extrabold text-text-800"
            >
              Enter some text...
            </div>
          )}
          ErrorBoundary={LexicalErrorBoundary}
        />
        <HistoryPlugin />
        <AutoFocusPlugin />
        <OnChangePlugin onChange={state => onChange(state.toJSON())} />
      </LexicalComposer>
    </div>
  );
};
