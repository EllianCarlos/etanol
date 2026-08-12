import * as vscode from "vscode";
import * as fs from "fs";
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
} from "vscode-languageclient/node";
import { resolveServerPath } from "./serverPath";

let client: LanguageClient;

export async function activate(context: vscode.ExtensionContext) {
    const configuredPath = vscode.workspace.getConfiguration("kotlinLsp").get<string>("serverPath");
    const serverPath = resolveServerPath(context.extensionPath, configuredPath);

    const outputChannel = vscode.window.createOutputChannel("Kotlin LSP");
    outputChannel.appendLine(`LSP server path: ${serverPath}`);

    if (!fs.existsSync(serverPath)) {
        const message =
            `Kotlin LSP server binary not found at "${serverPath}". ` +
            `Build it with "cargo build --release", or set the "kotlinLsp.serverPath" setting.`;
        outputChannel.appendLine(message);
        vscode.window.showErrorMessage(message);
        return;
    }

    let serverOptions: ServerOptions = {
        run: { command: serverPath },
        debug: { command: serverPath },
    };

    let clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: "file", language: "kotlin" }],
        outputChannel,
    };

    client = new LanguageClient("kotlinLsp", "Kotlin Language Server", serverOptions, clientOptions);

    outputChannel.appendLine("Starting LSP client...");

    try {
        await client.start();
        outputChannel.appendLine("Language Client Started.");
    } catch (e) {
        const message = `Failed to start Kotlin LSP client: ${e}`;
        outputChannel.appendLine(message);
        vscode.window.showErrorMessage(message);
    }
}

export function deactivate(): Thenable<void> | undefined {
    return client ? client.stop() : undefined;
}
