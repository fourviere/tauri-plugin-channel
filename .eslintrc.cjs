module.exports = {
  extends: [
    "eslint:recommended",
    "plugin:@typescript-eslint/recommended",
    "plugin:@typescript-eslint/recommended-type-checked",
    "plugin:prettier/recommended",
  ],
  parser: "@typescript-eslint/parser",
  parserOptions: {
    tsconfigRootDir: __dirname,
    project: [
      "./packages/plugin/webview-src/tsconfig.json",
      "./packages/examples/tauri-app/tsconfig.json",
    ],
  },
  rules: {
    "prettier/prettier": "error",
  },
  plugins: ["@typescript-eslint", "svelte", "prettier"],
  root: true,
};
