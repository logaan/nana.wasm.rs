import * as monaco from 'monaco-editor';
import { nana } from './js-transpile/nana.js';

async function loadInitialValue() {
  const response = await fetch('./learn_x_in_y_minutes.nana');
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

export const myEditor = monaco.editor.create(document.getElementById("container")!, {
  value: '',
  language: "nana",
  minimap: { enabled: false },
  automaticLayout: true,
  scrollBeyondLastLine: false,
  theme: "vs-light",
  padding: {
    top: 16
  },
});

export const resultsEditor = monaco.editor.create(document.getElementById("output")!, {
  value: `# Evalute code by clicking the button above or pressing cmd + e.
# The result of each top level expression will show here, prefixed by \`>\`.`,
  language: "nana",
  readOnly: true,
  automaticLayout: true,
  scrollBeyondLastLine: false,
  minimap: { enabled: false },
  theme: "vs-light",
  padding: {
    top: 16
  }
});

loadInitialValue();

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

myEditor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyE, evaluateEditor);
myEditor.addCommand(monaco.KeyMod.WinCtrl | monaco.KeyCode.KeyE, evaluateEditor);

document.getElementById("evaluate")!.onclick = evaluateEditor;

function clearEditor() {
  if (confirm("Are you sure you want to clear the editor?")) {
    myEditor.setValue(`# Write Nana code here\n\n"Hello, world!"\n`);
  }
}

document.getElementById("clear")!.onclick = clearEditor;

function setLineNumbersForWidth() {
  const showLineNumbers = window.innerWidth >= 600;
  myEditor.updateOptions({ lineNumbers: showLineNumbers ? 'on' : 'off' });
  resultsEditor.updateOptions({ lineNumbers: showLineNumbers ? 'on' : 'off' });
}

window.addEventListener('resize', setLineNumbersForWidth);
setLineNumbersForWidth();
