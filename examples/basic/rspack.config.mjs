import { defineConfig } from "@rspack/cli";
import rspackHtmlPlugin from "@rspack/plugin-html";

const HtmlPlugin =
  rspackHtmlPlugin.default?.default || rspackHtmlPlugin.default;

export default defineConfig({
  entry: {
    index: "./app/entry.tsx",
  },
  output: {
    distPath: { root: "dist" },
  },
  plugins: [new HtmlPlugin({ template: "./public/index.html" })],
  resolve: {
    extensions: [".tsx", ".ts", ".jsx", ".js", ".css"],
  },
  experiments: {
    css: true,
  },
  module: {
    rules: [
      {
        test: /\.(tsx|ts|jsx|js)$/,
        use: {
          loader: "builtin:swc-loader",
          options: {
            jsc: {
              parser: { syntax: "typescript", tsx: true },
              transform: { react: { runtime: "automatic" } },
            },
          },
        },
        exclude: /node_modules/,
      },
      {
        test: /\.css$/,
        use: ["postcss-loader"],
      },
    ],
  },
  devServer: { port: 3000, hot: true },
});
