import * as path from "path";

/**
 * Resolves the path to the etanol server binary.
 *
 * Prefers the `kotlinLsp.serverPath` setting so a packaged install (where
 * there is no `../target/release/etanol` next to the extension) can point at
 * a real binary. Falls back to the dev-mode layout used when running the
 * extension straight out of this repository.
 *
 * Deliberately has no `vscode` import, so it can be unit tested with plain
 * Node/mocha instead of needing a running VS Code extension host.
 */
export function resolveServerPath(extensionPath: string, configuredPath: string | undefined): string {
    if (configuredPath && configuredPath.trim().length > 0) {
        return configuredPath;
    }
    return path.join(extensionPath, "..", "target", "release", "etanol");
}
