const vscode = require('vscode');
const { execFile } = require('child_process');
function getRadheBin() {
  return vscode.workspace.getConfiguration('radhe').get('binaryPath', 'radhe');
}
function getLangArgs() {
  const lang = vscode.workspace.getConfiguration('radhe').get('language', 'en');
  return lang && lang !== 'en' ? ['--lang', lang] : [];
}
function runRadhe(args, input, callback) {
  const bin = getRadheBin();
  const fullArgs = [...args, ...getLangArgs()];
  const proc = execFile(bin, fullArgs, (err, stdout, stderr) => {
    if (err) {
      vscode.window.showErrorMessage(`Radhe Error: ${stderr || err.message}`);
      return;
    }
    callback(stdout.trim());
  });
  if (input && proc.stdin) {
    proc.stdin.write(input);
    proc.stdin.end();
  }
}
function activate(context) {
  // Command 1: Explain Selection
  context.subscriptions.push(
    vscode.commands.registerCommand('radhe.explain', () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) return;
      const selection = editor.document.getText(editor.selection);
      if (!selection) { vscode.window.showWarningMessage('Radhe: Select some text first.'); return; }
      vscode.window.showInformationMessage('Radhe: Explaining...');
      runRadhe(['--explain', selection], null, (output) => {
        const panel = vscode.window.createWebviewPanel('radheExplain', 'Radhe: Explanation', vscode.ViewColumn.Beside, {});
        panel.webview.html = `<html><body style="font-family:sans-serif;padding:16px"><h2>Explanation</h2><pre style="white-space:pre-wrap">${output}</pre></body></html>`;
      });
    })
  );
  // Command 2: Fix Selection
  context.subscriptions.push(
    vscode.commands.registerCommand('radhe.fix', () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) return;
      const selection = editor.document.getText(editor.selection);
      if (!selection) { vscode.window.showWarningMessage('Radhe: Select some code first.'); return; }
      vscode.window.showInformationMessage('Radhe: Fixing...');
      // Write selection to a temp file and pass to --fix
      const fs = require('fs');
      const os = require('os');
      const path = require('path');
      const tmpFile = path.join(os.tmpdir(), 'radhe_fix_tmp.txt');
      fs.writeFileSync(tmpFile, selection);
      runRadhe(['--fix', tmpFile], null, (output) => {
        editor.edit(editBuilder => {
          editBuilder.replace(editor.selection, output);
        });
        fs.unlinkSync(tmpFile);
      });
    })
  );
  // Command 3: Generate Quiz from File
  context.subscriptions.push(
    vscode.commands.registerCommand('radhe.quiz', () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) return;
      const filePath = editor.document.fileName;
      vscode.window.showInformationMessage('Radhe: Generating quiz...');
      runRadhe(['--quiz-file', filePath], null, (output) => {
        const panel = vscode.window.createWebviewPanel('radheQuiz', 'Radhe: Quiz', vscode.ViewColumn.Beside, {});
        panel.webview.html = `<html><body style="font-family:sans-serif;padding:16px"><h2>Quiz</h2><pre style="white-space:pre-wrap">${output}</pre></body></html>`;
      });
    })
  );
  // Command 4: Create Custom Pack
  context.subscriptions.push(
    vscode.commands.registerCommand('radhe.createPack', () => {
      const terminal = vscode.window.createTerminal('Radhe: Create Pack');
      terminal.show();
      terminal.sendText(`${getRadheBin()} --create-pack`);
    })
  );
}
function deactivate() {}
module.exports = { activate, deactivate };
