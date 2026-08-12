import * as assert from "assert";
import * as path from "path";
import { resolveServerPath } from "../serverPath";

const devModeDefault = (extensionPath: string) => path.join(extensionPath, "..", "target", "release", "etanol");

describe("resolveServerPath", () => {
    it("falls back to the dev-mode target/release path when no setting is configured", () => {
        const result = resolveServerPath("/home/user/.vscode/extensions/kotlin-lsp-client", undefined);
        assert.strictEqual(result, devModeDefault("/home/user/.vscode/extensions/kotlin-lsp-client"));
    });

    it("falls back to the dev-mode path when the setting is an empty string", () => {
        const result = resolveServerPath("/ext/path", "");
        assert.strictEqual(result, devModeDefault("/ext/path"));
    });

    it("falls back to the dev-mode path when the setting is only whitespace", () => {
        const result = resolveServerPath("/ext/path", "   ");
        assert.strictEqual(result, devModeDefault("/ext/path"));
    });

    it("uses the configured path when one is set", () => {
        const result = resolveServerPath("/ext/path", "/usr/local/bin/etanol");
        assert.strictEqual(result, "/usr/local/bin/etanol");
    });

    it("trims neither the configured path's content nor rejects surrounding-only whitespace paths", () => {
        // A configured path that is non-empty after trimming is used verbatim,
        // including any of its own leading/trailing space the user intended.
        const result = resolveServerPath("/ext/path", "  /custom/etanol  ");
        assert.strictEqual(result, "  /custom/etanol  ");
    });
});
