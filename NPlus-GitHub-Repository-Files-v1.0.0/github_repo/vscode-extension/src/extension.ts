import * as vscode from 'vscode';
import { execFile } from 'child_process';
import * as path from 'path';
import * as fs from 'fs';

function findCompiler(file: string): string {
  const workspace = vscode.workspace.getWorkspaceFolder(vscode.Uri.file(file));
  if (workspace) {
    const root = workspace.uri.fsPath;
    const candidates = process.platform === 'win32'
      ? [
          path.join(root, 'compiler', 'target', 'debug', 'nplus.exe'),
          path.join(root, 'compiler', 'target', 'release', 'nplus.exe')
        ]
      : [
          path.join(root, 'compiler', 'target', 'debug', 'nplus'),
          path.join(root, 'compiler', 'target', 'release', 'nplus')
        ];

    const local = candidates.find((candidate) => fs.existsSync(candidate));
    if (local) return local;
  }

  return process.platform === 'win32' ? 'nplus.exe' : 'nplus';
}

function runNPlus(command: 'run' | 'check') {
  const editor = vscode.window.activeTextEditor;
  if (!editor || editor.document.languageId !== 'nplus') return;

  const file = editor.document.uri.fsPath;
  const compiler = findCompiler(file);
  const output = vscode.window.createOutputChannel('N+');
  output.clear();
  output.show(true);

  output.appendLine(`N+ ${command} • ${path.basename(file)}`);
  output.appendLine('────────────────────────────────────────');

  execFile(compiler, [command, file], {}, (error, stdout, stderr) => {
    if (stdout) output.append(stdout);
    if (stderr) output.append(stderr);

    if (error) {
      vscode.window.showErrorMessage(`N+ ${command} failed. See the N+ output panel.`);
      return;
    }

    if (command === 'check') {
      vscode.window.showInformationMessage('N+ check passed.');
    }
  });
}

export function activate(context: vscode.ExtensionContext) {
  context.subscriptions.push(
    vscode.commands.registerCommand('nplus.runFile', () => runNPlus('run')),
    vscode.commands.registerCommand('nplus.checkFile', () => runNPlus('check'))
  );
}

export function deactivate() {}
