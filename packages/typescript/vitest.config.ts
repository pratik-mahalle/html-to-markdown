import { defineConfig } from "vitest/config";

export default defineConfig({
	test: {
		environment: "node",
		globals: true,
		pool: process.platform === "win32" ? "forks" : "threads",
		coverage: {
			reporter: ["text", "lcov"],
		},
	},
});
