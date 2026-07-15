import * as monaco from 'monaco-editor';

monaco.languages.register({ id: 'python' });

const tm = monaco.editor.createModel(
    `print('Hello, world!')
1 + 1`,
    'python', monaco.Uri.parse('file:///main.ts'));

const editor = monaco.editor.create(document.getElementById('editor')!, {
    model: tm,
    language: 'python',
    theme: 'vs-dark',
})