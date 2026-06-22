import * as vscode from 'vscode';
import * as fs from 'fs';
import * as path from 'path';
import { spawnSync } from 'child_process';

// ── Activation ──────────────────────────────────
export function activate(context: vscode.ExtensionContext) {
    console.log('BackLang v2 extension activated');

    context.subscriptions.push(
        vscode.languages.registerCompletionItemProvider('backlang', new BackLangCompletionProvider(), '.', ' '),
        vscode.languages.registerHoverProvider('backlang', new BackLangHoverProvider()),
        vscode.languages.registerCodeLensProvider('backlang', new BackLangCodeLensProvider()),
        vscode.languages.registerDocumentFormattingEditProvider('backlang', new BackLangFormatter()),
        vscode.languages.registerInlayHintsProvider('backlang', new BackLangInlayHints()),
        vscode.languages.registerFoldingRangeProvider('backlang', new BackLangFoldingProvider()),
        vscode.languages.registerDocumentSymbolProvider('backlang', new BackLangSymbolProvider()),
        vscode.languages.registerRenameProvider('backlang', new BackLangRenameProvider()),
        vscode.languages.registerReferenceProvider('backlang', new BackLangReferenceProvider()),
        vscode.commands.registerCommand('backlang.run', runFile),
        vscode.commands.registerCommand('backlang.debug', debugFile),
        vscode.commands.registerCommand('backlang.createHtml5Base', createHtmlFile),
        vscode.commands.registerCommand('backlang.createBackendBase', createBackendFile),
        vscode.commands.registerCommand('backlang.build', buildRelease),
        vscode.commands.registerCommand('backlang.repl', openRepl),
    );

    // Diagnostics
    const diag = vscode.languages.createDiagnosticCollection('backlang');
    context.subscriptions.push(diag);
    context.subscriptions.push(vscode.workspace.onDidSaveTextDocument(d => {
        if (d.languageId === 'backlang') checkSyntax(d, diag);
    }));
    vscode.workspace.textDocuments.forEach(d => {
        if (d.languageId === 'backlang') checkSyntax(d, diag);
    });

    // Status bar
    const blPath = findBlBinary();
    const item = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    item.text = blPath ? 'BackLang ✓' : 'BackLang ✗';
    item.tooltip = blPath ? `Binary: ${blPath}` : 'Not built — run Build task';
    item.command = 'backlang.build';
    item.show();
    context.subscriptions.push(item);
}

export function deactivate() {}

// ── Helpers ─────────────────────────────────────
function findBlBinary(): string | null {
    for (const p of [
        '/home/surgeo/Projects/BackLang/target/release/bl',
        '/home/surgeo/Projects/BackLang/target/debug/bl',
    ]) if (fs.existsSync(p)) return p;
    return null;
}

// ── Commands ────────────────────────────────────
async function runFile() {
    const editor = vscode.window.activeTextEditor;
    if (!editor || editor.document.languageId !== 'backlang')
        return vscode.window.showErrorMessage('Open a .bl file first');
    await editor.document.save();
    const bl = findBlBinary();
    if (!bl) return askBuild();
    let term = vscode.window.terminals.find(t => t.name === 'BackLang');
    if (!term) term = vscode.window.createTerminal('BackLang');
    term.show();
    term.sendText(`"${bl}" "${editor.document.uri.fsPath}"`);
}

function debugFile() {
    const editor = vscode.window.activeTextEditor;
    if (!editor || editor.document.languageId !== 'backlang') return;
    vscode.debug.startDebugging(vscode.workspace.workspaceFolders?.[0], {
        name: 'BackLang Debug',
        type: 'backlang',
        request: 'launch',
        program: editor.document.uri.fsPath,
        stopOnEntry: false,
        blPath: findBlBinary(),
    });
}

async function buildRelease() {
    const t = vscode.window.createTerminal('BackLang Build');
    t.show();
    t.sendText('cd ~/Projects/BackLang && cargo build --release && echo "✓ Build OK"');
}

function openRepl() {
    const bl = findBlBinary();
    if (!bl) return askBuild();
    let term = vscode.window.terminals.find(t => t.name === 'BackLang REPL');
    if (!term) term = vscode.window.createTerminal('BackLang REPL');
    term.show();
    term.sendText(`echo 'BackLang REPL — run .bl files with: bl file.bl'`);
}

async function askBuild() {
    const r = await vscode.window.showErrorMessage('BackLang binary not found. Build now?', 'Build');
    if (r === 'Build') buildRelease();
}

async function createBackendFile() {
    const ws = vscode.workspace.workspaceFolders?.[0];
    if (!ws) return vscode.window.showErrorMessage('Open a folder first');
    const name = await vscode.window.showInputBox({ prompt: 'File name', value: 'server' });
    if (!name) return;
    const fp = path.join(ws.uri.fsPath, `${name}.bl`);
    if (fs.existsSync(fp) && await vscode.window.showWarningMessage('Overwrite?', 'Yes') !== 'Yes') return;
    fs.writeFileSync(fp, BACKEND_TEMPLATE);
    const doc = await vscode.workspace.openTextDocument(fp);
    vscode.window.showTextDocument(doc);
}

async function createHtmlFile() {
    const ws = vscode.workspace.workspaceFolders?.[0];
    if (!ws) return;
    const name = await vscode.window.showInputBox({ prompt: 'File name', value: 'index' });
    if (!name) return;
    fs.writeFileSync(path.join(ws.uri.fsPath, `${name}.html`), HTML_TEMPLATE);
}

// ── Templates ────────────────────────────────────
const BACKEND_TEMPLATE = `// BackLang API — generated by BackLang extension
server app { port: 8080; cors: true }
import "std.db"
let dbPath = "/tmp/app.db"
db.open(dbPath)
db.execute(dbPath, "CREATE TABLE IF NOT EXISTS items (id INTEGER PRIMARY KEY, name TEXT)")
endpoint GET "/api/health" { return {status: 200, body: {ok: true}} }
endpoint GET "/api/items" { return {status: 200, body: db.query(dbPath, "SELECT * FROM items")} }
endpoint POST "/api/echo" { return {status: 200, body: {received: true, data: req.body}} }
`;
const HTML_TEMPLATE = `<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><title>BackLang App</title></head><body><h1>BackLang App</h1></body></html>`;

// ── Diagnostics ─────────────────────────────────
function checkSyntax(doc: vscode.TextDocument, diag: vscode.DiagnosticCollection) {
    const bl = findBlBinary();
    if (!bl) return;
    const result = spawnSync(bl, ['--check', doc.uri.fsPath], { timeout: 5000, encoding: 'utf-8' });
    if (result.status === 0 && result.stdout.trim() === 'OK') { diag.set(doc.uri, []); return; }
    const text = (result.stderr || result.stdout || '').trim();
    if (!text) { diag.set(doc.uri, []); return; }
    // Parse "Error: message at line:X" or "Error: message" patterns
    const list: vscode.Diagnostic[] = [];
    for (const line of text.split('\n')) {
        const parts = line.match(/Error:\s*(.*?)(?:\s+at\s+line:?(\d+))?$/i);
        if (parts) {
            const msg = parts[1] || line;
            const lineN = parts[2] ? Math.max(0, parseInt(parts[2]) - 1) : 0;
            const range = new vscode.Range(lineN, 0, lineN, doc.lineAt(Math.min(lineN, doc.lineCount - 1)).text.length);
            list.push(new vscode.Diagnostic(range, msg, vscode.DiagnosticSeverity.Error));
        } else {
            // Fallback: whole-file error
            list.push(new vscode.Diagnostic(
                new vscode.Range(0, 0, Math.min(doc.lineCount - 1, 0), 1000),
                line, vscode.DiagnosticSeverity.Error
            ));
        }
    }
    diag.set(doc.uri, list);
}

// ── Problem Matcher for terminal ─────────────────
// Export a problem matcher pattern for use in tasks.json
// Matches: "Error: <message>"
export const BackLangProblemMatcher = {
    owner: 'backlang',
    pattern: [
        { regexp: /^Error:\s*(.*)$/, message: 1 },
    ],
};

// ── Completion Provider ──────────────────────────
const KEYWORDS = ['let','const','if','else','while','for','break','continue','return',
    'function','true','false','null','server','endpoint','import','middleware','type'];

const BUILTINS: [string,string][] = [
    ['print','Print value'],['len','Length of str/arr'],['push','Append to array'],
    ['pop','Pop from array'],['keys','Object keys'],['str','→ string'],
    ['num','→ number'],['type','Type name'],
];

class BackLangCompletionProvider implements vscode.CompletionItemProvider {
    provideCompletionItems(doc: vscode.TextDocument): vscode.CompletionItem[] {
        const items: vscode.CompletionItem[] = [];
        for (const k of KEYWORDS) {
            const i = new vscode.CompletionItem(k, vscode.CompletionItemKind.Keyword);
            items.push(i);
        }
        for (const [n, d] of BUILTINS) {
            const i = new vscode.CompletionItem(n, vscode.CompletionItemKind.Function);
            i.detail = d; items.push(i);
        }
        for (const f of ['db.open','db.query','db.execute','db_open','db_query','db_execute']) {
            const i = new vscode.CompletionItem(f, vscode.CompletionItemKind.Function);
            i.detail = 'SQLite operation'; items.push(i);
        }
        // Variables from file
        const re = /\b(?:let|const|for)\s+(\w+)/g;
        let m; const text = doc.getText();
        while ((m = re.exec(text)) !== null) {
            const i = new vscode.CompletionItem(m[1], vscode.CompletionItemKind.Variable);
            items.push(i);
        }
        // Route paths
        const er = /endpoint\s+\w+\s+"([^"]+)"/g;
        while ((m = er.exec(text)) !== null) {
            const i = new vscode.CompletionItem(m[1], vscode.CompletionItemKind.Value);
            i.detail = 'Route'; items.push(i);
        }
        // Function names
        const fr = /function\s+(\w+)/g;
        while ((m = fr.exec(text)) !== null) {
            const i = new vscode.CompletionItem(m[1], vscode.CompletionItemKind.Function);
            i.detail = 'User function'; items.push(i);
        }
        return items;
    }
}

// ── Hover Provider ───────────────────────────────
const DOCS: Record<string, string> = {
    'print': '`print(val)` — Print to stdout',
    'len': '`len(val)` → length of string or array',
    'push': '`push(arr, val)` — Append to array',
    'pop': '`pop(arr)` — Remove and return last element',
    'keys': '`keys(obj)` — Get object keys as array',
    'str': '`str(val)` → string representation',
    'num': '`num(val)` → number (NaN if invalid)',
    'type': '`type(val)` → "null"|"bool"|"num"|"str"|"array"|"object"|"function"',
    'db.open': '`db.open(path)` — Open SQLite database connection',
    'db.query': '`db.query(path, sql)` → Array of row objects',
    'db.execute': '`db.execute(path, sql)` → number of affected rows',
    'db_open': 'Alias for `db.open()`',
    'db_query': 'Alias for `db.query()`',
    'db_execute': 'Alias for `db.execute()`',
    'server': '`server name { port; cors }` — HTTP server declaration',
    'endpoint': '`endpoint METHOD "/path" { … }` — Route handler',
    'import': '`import "std.xxx"` or `import "local/file"`',
    'middleware': '`middleware name { … }` — Pre-request hook',
    'return': '`return {status: N, body: …}` — HTTP response',
    'true': '`true` — Boolean literal',
    'false': '`false` — Boolean literal',
    'null': '`null` — Null value',
    'let': '`let name = expr` — Mutable variable',
    'const': '`const name = expr` — Immutable variable',
    'if': '`if cond { … } else { … }` — Conditional',
    'while': '`while cond { … }` — Loop',
    'for': '`for item in iterable { … }` — Iteration',
    'function': '`function name(params) -> Type { … }` — Function definition',
};

class BackLangHoverProvider implements vscode.HoverProvider {
    provideHover(doc: vscode.TextDocument, pos: vscode.Position): vscode.Hover | null {
        const range = doc.getWordRangeAtPosition(pos);
        if (!range) return null;
        const word = doc.getText(range);
        const entry = DOCS[word];
        if (entry) return new vscode.Hover(new vscode.MarkdownString(entry));
        // Show db.* hover for just the member name
        if (['open','query','execute'].includes(word)) {
            const line = doc.lineAt(pos.line).text;
            if (line.includes('db.')) {
                const e = DOCS[`db.${word}`];
                if (e) return new vscode.Hover(new vscode.MarkdownString(e));
            }
        }
        return null;
    }
}

// ── CodeLens ────────────────────────────────────
class BackLangCodeLensProvider implements vscode.CodeLensProvider {
    provideCodeLenses(doc: vscode.TextDocument): vscode.CodeLens[] {
        const lenses: vscode.CodeLens[] = [];
        for (let i = 0; i < doc.lineCount; i++) {
            const line = doc.lineAt(i).text.trim();
            if (line.startsWith('endpoint '))
                lenses.push(new vscode.CodeLens(new vscode.Range(i,0,i,line.length),
                    { title: '▶ Run', command: 'backlang.run' }));
            if (line.startsWith('server '))
                lenses.push(new vscode.CodeLens(new vscode.Range(i,0,i,line.length),
                    { title: '▶ Start Server', command: 'backlang.run' }));
            if (line.startsWith('function '))
                lenses.push(new vscode.CodeLens(new vscode.Range(i,0,i,line.length),
                    { title: '▶ Run', command: 'backlang.run' }));
        }
        return lenses;
    }
}

// ── Formatter ───────────────────────────────────
class BackLangFormatter implements vscode.DocumentFormattingEditProvider {
    provideDocumentFormattingEdits(doc: vscode.TextDocument): vscode.TextEdit[] {
        const edits: vscode.TextEdit[] = [];
        for (let i = 0; i < doc.lineCount; i++) {
            const line = doc.lineAt(i);
            const trimmed = line.text.trimEnd();
            if (trimmed !== line.text)
                edits.push(vscode.TextEdit.delete(new vscode.Range(i, trimmed.length, i, line.text.length)));
        }
        return edits;
    }
}

// ── Inlay Hints ─────────────────────────────────
class BackLangInlayHints implements vscode.InlayHintsProvider {
    provideInlayHints(doc: vscode.TextDocument): vscode.InlayHint[] {
        const hints: vscode.InlayHint[] = [];
        for (let i = 0; i < doc.lineCount; i++) {
            const line = doc.lineAt(i).text;
            // Show `→ type` after function declarations
            const fm = line.match(/^\s*function\s+(\w+)/);
            if (fm) {
                hints.push(new vscode.InlayHint(
                    new vscode.Position(i, line.length),
                    ' → any',
                    vscode.InlayHintKind.Type
                ));
            }
            // Show `← value` after let with literal
            const lm = line.match(/^\s*let\s+(\w+)\s*=\s*("[^"]*"|true|false|\d+)/);
            if (lm) {
                const val = lm[2];
                const label = val.match(/^"/) ? 'string' : val === 'true' || val === 'false' ? 'bool' : 'num';
                hints.push(new vscode.InlayHint(
                    new vscode.Position(i, line.length),
                    `: ${label}`,
                    vscode.InlayHintKind.Type
                ));
            }
        }
        return hints;
    }
}

// ── Folding ─────────────────────────────────────
class BackLangFoldingProvider implements vscode.FoldingRangeProvider {
    provideFoldingRanges(doc: vscode.TextDocument): vscode.FoldingRange[] {
        const ranges: vscode.FoldingRange[] = [];
        const stack: { kind: vscode.FoldingRangeKind; start: number }[] = [];
        for (let i = 0; i < doc.lineCount; i++) {
            const line = doc.lineAt(i).text;
            if (line.includes('{')) stack.push({ kind: vscode.FoldingRangeKind.Region, start: i });
            for (let j = line.length - 1; j >= 0; j--) {
                if (line[j] === '}' && stack.length) {
                    const s = stack.pop()!;
                    if (s.start < i) ranges.push(new vscode.FoldingRange(s.start, i, s.kind));
                }
            }
        }
        // Comments regions
        for (let i = 0; i < doc.lineCount; i++) {
            if (doc.lineAt(i).text.trim().startsWith('// ──') || doc.lineAt(i).text.trim().startsWith('/*')) {
                let end = i;
                while (end < doc.lineCount && !doc.lineAt(end).text.match(/(──|\*\/)/)) end++;
                if (end > i + 1) ranges.push(new vscode.FoldingRange(i, end, vscode.FoldingRangeKind.Comment));
            }
        }
        return ranges;
    }
}

// ── Document Symbols (breadcrumbs / outline) ────
class BackLangSymbolProvider implements vscode.DocumentSymbolProvider {
    provideDocumentSymbols(doc: vscode.TextDocument): vscode.SymbolInformation[] {
        const symbols: vscode.SymbolInformation[] = [];
        for (let i = 0; i < doc.lineCount; i++) {
            const line = doc.lineAt(i).text;
            let m: RegExpExecArray | null;
            if ((m = /^\s*endpoint\s+(GET|POST|PUT|DELETE)\s+"([^"]+)"/.exec(line)))
                symbols.push(new vscode.SymbolInformation(
                    `${m[1]} ${m[2]}`, vscode.SymbolKind.Function,
                    new vscode.Range(i, 0, i, line.length), doc.uri));
            if ((m = /^\s*function\s+(\w+)/.exec(line)))
                symbols.push(new vscode.SymbolInformation(
                    m[1], vscode.SymbolKind.Function,
                    new vscode.Range(i, 0, i, line.length), doc.uri));
            if ((m = /^\s*server\s+(\w+)/.exec(line)))
                symbols.push(new vscode.SymbolInformation(
                    `server ${m[1]}`, vscode.SymbolKind.Module,
                    new vscode.Range(i, 0, i, line.length), doc.uri));
            if ((m = /^\s*(let|const)\s+(\w+)/.exec(line)))
                symbols.push(new vscode.SymbolInformation(
                    `${m[1]} ${m[2]}`, vscode.SymbolKind.Variable,
                    new vscode.Range(i, 0, i, line.length), doc.uri));
        }
        return symbols;
    }
}

// ── Rename / References ─────────────────────────
class BackLangRenameProvider implements vscode.RenameProvider {
    prepareRename(doc: vscode.TextDocument, pos: vscode.Position): vscode.Range | { range: vscode.Range; placeholder: string } | null {
        const r = doc.getWordRangeAtPosition(pos);
        if (!r) return null;
        const w = doc.getText(r);
        if (!/^\w+$/.test(w)) return null;
        return { range: r, placeholder: w };
    }
    provideRenameEdits(doc: vscode.TextDocument, pos: vscode.Position, newName: string): vscode.WorkspaceEdit {
        const edit = new vscode.WorkspaceEdit();
        const wordRange = doc.getWordRangeAtPosition(pos);
        if (!wordRange) return edit;
        const oldName = doc.getText(wordRange);
        const text = doc.getText();
        const re = new RegExp(`\\b${oldName.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}\\b`, 'g');
        let m: RegExpExecArray | null;
        while ((m = re.exec(text)) !== null) {
            const start = doc.positionAt(m.index);
            const end = doc.positionAt(m.index + m[0].length);
            edit.replace(doc.uri, new vscode.Range(start, end), newName);
        }
        return edit;
    }
}

class BackLangReferenceProvider implements vscode.ReferenceProvider {
    provideReferences(doc: vscode.TextDocument, pos: vscode.Position): vscode.Location[] {
        const wordRange = doc.getWordRangeAtPosition(pos);
        if (!wordRange) return [];
        const word = doc.getText(wordRange);
        if (!/^\w+$/.test(word)) return [];
        const locs: vscode.Location[] = [];
        const text = doc.getText();
        const re = new RegExp(`\\b${word.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}\\b`, 'g');
        let m: RegExpExecArray | null;
        while ((m = re.exec(text)) !== null) {
            locs.push(new vscode.Location(doc.uri, doc.positionAt(m.index)));
        }
        return locs;
    }
}
