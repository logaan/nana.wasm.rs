import * as monaco from 'monaco-editor';
import { nana } from './js-transpile/nana.js';

async function loadInitialValue() {
  const response = await fetch('/learn_x_in_y_minutes.nana');
  const value = await response.text();
  myEditor.setValue(value);
}

// Register a new language
monaco.languages.register({ id: "nana" });

// Register a tokens provider for the language
monaco.languages.setMonarchTokensProvider("nana", {
  defaultToken: 'invalid',

  tokenizer: {
    root: [
      [/[a-z_$][\w-\.$]*/, 'identifier'],
      [/[A-Z][\w-\.\$]*/, 'keyword'],
      { include: '@whitespace' },
      [/[{}()\[\]]/, '@brackets'],
      [/\d+/, 'number'],
      [/"/, { token: 'string.quote', bracket: '@open', next: '@string' }],
    ],

    string: [
      [/[^\\"]+/, 'string'],
      [/"/, { token: 'string.quote', bracket: '@close', next: '@pop' }]
    ],

    whitespace: [
      [/[ \t\r\n]+/, 'white'],
      [/#.*$/, 'comment'],
    ],
  },
});


function evaluateEditor() {
  try {
    const result = nana.evaluate(myEditor.getValue());
    resultsEditor.setValue(result);
  } catch (error) {
    if (error instanceof Error) {
      resultsEditor.setValue(`${error.name}: ${error.message}`);
    } else {
      resultsEditor.setValue(`Unexpected exception: ${error}`);
    }
  }
}

export const myEditor = monaco.editor.create(document.getElementById("container")!, {
  value: '',
  language: "nana",
  minimap: { enabled: false },
  theme: "vs-light",
});

loadInitialValue();

myEditor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyE, () => {
  evaluateEditor();
});

export const resultsEditor = monaco.editor.create(document.getElementById("results")!, {
  value: `# Evalute code by clicking the button above or pressing cmd + e.
# The result of each top level expression will be shown here, prefixed by \`>\`.`,
  language: "nana",
  readOnly: true,
  minimap: { enabled: false },
});

document.getElementById("evaluate")!.onclick = evaluateEditor;
