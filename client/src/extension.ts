import * as vscode from "vscode";
import * as path from "path";
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient;

export async function activate(context: vscode.ExtensionContext) {
    const serverPath = path.join(context.extensionPath, "..", "target", "release", "etanol");
    
    console.log("LSP server path %s", serverPath);

    let serverOptions: ServerOptions = {
        run: { command: serverPath },
        debug: { command: serverPath },
    };

    let clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: "file", language: "kotlin" }],
    };

    client = new LanguageClient("kotlinLsp", "Kotlin Language Server", serverOptions, clientOptions);
    
    console.log("Starting LSP client...")
    
    // Start the client and wrap it in a custom disposable object
    const clientDisposable = await client.start();
    console.log("Language Client Started.");

    // Create a custom disposable object that calls client.stop() when disposed
    context.subscriptions.push({
        dispose: async () => {
            await client.stop();
        }
    });
}

export function deactivate(): Thenable<void> | undefined {
    return client ? client.stop() : undefined;
}
